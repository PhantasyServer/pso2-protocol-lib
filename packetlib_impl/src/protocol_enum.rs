use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TS2};
use quote::quote;
use syn::{
    parse::Parse, punctuated::Punctuated, spanned::Spanned, Data, DataEnum, Fields, FieldsUnnamed,
    LitInt, MetaList, Token, Type, TypePath,
};

pub fn protocol_deriver(ast: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;

    let mut read = quote! {};
    let mut write = quote! {};
    let mut category = quote! {};

    if let Data::Enum(data) = &ast.data {
        parse_enum_field(&mut read, &mut write, &mut category, data)?
    }

    let gen = quote! {
        #[automatically_derived]
        impl ProtocolRW for #name {
            fn write(&self, is_ngs: bool) -> Vec<u8> {
                let mut buf = vec![];
                buf.write_u32::<LittleEndian>(0).unwrap();
                buf.extend(match self {
                    #write
                    Self::Unknown(data) => {
                        let mut out_data = data.0.write(is_ngs);
                        out_data.extend_from_slice(&data.1);
                        out_data
                    }
                });
                let len = (buf.len() + 3) & (usize::MAX ^ 3);
                buf.resize(len, 0);
                let len = (len as u32).to_le_bytes();
                buf[..4].copy_from_slice(&len);
                buf
            }
            fn read(input: &[u8], is_ngs: bool) -> std::io::Result<Vec<Self>> {
                let mut packets: Vec<Self> = vec![];
                let buffer_length = input.len();
                let mut pointer = 0;
                loop {
                    if pointer >= buffer_length {
                        break;
                    }
                    if input[pointer..].len() <= 4 {
                        break;
                    }
                    let len = (&input[pointer..pointer + 4]).read_u32::<LittleEndian>()? as usize - 4;
                    pointer += 4;
                    if input[pointer..].len() < len {
                        return Err(std::io::ErrorKind::UnexpectedEof.into());
                    }
                    let mut buf_tmp = Cursor::new(&input[pointer..pointer + len]);
                    let header = PacketHeader::read(&mut buf_tmp, is_ngs)?;
                    let flags = header.flag.clone();

                    let tmp_header = header.clone();

                    pointer += len;
                    match (header.id, header.subid, is_ngs) {
                        #read
                        (_, _, _) => {
                            packets.push(Self::Unknown({
                                let mut data = vec![];
                                buf_tmp.read_to_end(&mut data)?;
                                (header, data)
                            }));
                        }
                    }
                    // let cur_pos = buf_tmp.stream_position()?;
                    // let diff = len - cur_pos as usize;
                    // if diff >= 4 {
                    //     println!("id: {}, subid: {} - diff: {}", tmp_header.id, tmp_header.subid, diff);
                    // }
                }

                Ok(packets)
            }
            fn get_category(&self) -> PacketCategory {
                let cat = match self {
                    #category
                    _ => Default::default(),
                };
                cat
            }
        }
    };
    Ok(gen.into())
}

fn parse_enum_field(
    read: &mut TS2,
    write: &mut TS2,
    category: &mut TS2,
    data: &DataEnum,
) -> syn::Result<()> {
    let mut category_stream = quote! {Default::default()};
    for variant in &data.variants {
        let name = &variant.ident;
        let mut settings = Settings::default();

        for attr in &variant.attrs {
            match &attr.meta {
                syn::Meta::NameValue(_) => {}
                syn::Meta::Path(path) => {
                    let string = path.get_ident().unwrap().to_string();
                    get_attrs(&mut settings, &string, None, path.span())?;
                }
                syn::Meta::List(list) => {
                    let string = list.path.get_ident().unwrap().to_string();
                    get_attrs(&mut settings, &string, Some(&list), list.span())?;
                }
            }
        }
        if settings.skip {
            continue;
        }
        if settings.id == 0
            && settings.subid == 0
            && !matches!(settings.packet_type, PacketType::Empty)
        {
            return Err(syn::Error::new(variant.span(), "No Id defined"));
        }
        let id = settings.id;
        let subid = settings.subid;
        if let PacketType::Empty = settings.packet_type {
            write.extend(quote! {
                Self::#name => return vec![],
            })
        }
        let mut push_string = quote! {};
        category_stream = if settings.category.is_empty() {
            category_stream
        } else {
            settings.category
        };
        match &variant.fields {
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                if let Type::Path(TypePath { path, .. }) = &unnamed.first().unwrap().ty {
                    let struct_field = path.get_ident().unwrap();
                    push_string = quote! {packets.push(Self::#name(#struct_field::read(&mut buf_tmp, flags)?))};
                    write.extend(quote! {
                        Self::#name(packet) => packet.write(is_ngs),
                    });
                    category.extend(quote! {
                        Self::#name(_) => {#category_stream},
                    })
                }
            }
            Fields::Unit => {
                push_string = quote! {packets.push(Self::#name)};
                write.extend(quote! {
                    Self::#name => PacketHeader::new(#id, #subid, Flags::default()).write(is_ngs),
                });
                category.extend(quote! {
                    Self::#name => {#category_stream},
                })
            }
            _ => {}
        }
        match settings.packet_type {
            PacketType::Both => read.extend(quote! {
                (#id, #subid, _) => {#push_string},
            }),
            PacketType::Base => read.extend(quote! {
                (#id, #subid, false) => {#push_string},
            }),
            PacketType::NGS => read.extend(quote! {
                (#id, #subid, true) => {#push_string},
            }),
            PacketType::Empty => {}
        }
    }
    Ok(())
}

fn get_attrs(
    set: &mut Settings,
    string: &str,
    list: Option<&MetaList>,
    span: Span,
) -> syn::Result<()> {
    match string {
        "Empty" => set.packet_type = PacketType::Empty,
        "Unknown" => {
            set.skip = true;
        }
        "Base" => set.packet_type = PacketType::Base,
        "NGS" => set.packet_type = PacketType::NGS,
        "Id" => {
            let attrs: AttributeList = match list {
                Some(x) => x.parse_args()?,
                None => {
                    return Err(syn::Error::new(
                        span,
                        "Invalid syntax \nPerhaps you ment Id(..)?",
                    ))
                }
            };
            if attrs.fields.len() != 2 {
                return Err(syn::Error::new(span, "Invalid number of arguments"));
            }
            set.id = attrs.fields[0].base10_parse()?;
            set.subid = attrs.fields[1].base10_parse()?;
        }
        "Category" => {
            let attrs = match list {
                Some(x) => &x.tokens,
                None => {
                    return Err(syn::Error::new(
                        span,
                        "Invalid syntax \nPerhaps you ment Id(..)?",
                    ))
                }
            };
            set.category = attrs.clone();
        }
        _ => {}
    }
    Ok(())
}

#[derive(Default)]
struct Settings {
    id: u8,
    subid: u16,
    packet_type: PacketType,
    skip: bool,
    category: TS2,
}

#[derive(Default)]
enum PacketType {
    #[default]
    Both,
    Base,
    NGS,
    Empty,
}

struct AttributeList {
    fields: Punctuated<LitInt, Token![,]>,
}
impl Parse for AttributeList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            fields: Punctuated::parse_separated_nonempty(input)?,
        })
    }
}
