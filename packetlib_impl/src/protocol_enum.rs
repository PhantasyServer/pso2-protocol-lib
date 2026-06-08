use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TS2};
use quote::quote;
use syn::{
    parse::Parse, punctuated::Punctuated, spanned::Spanned, Data, DataEnum, Fields, FieldsUnnamed,
    LitInt, MetaList, Token, Type, TypePath,
};

#[derive(Default)]
struct OutputCode {
    read: TS2,
    write: TS2,
    category: TS2,
    read_raw: TS2,

    tests: TS2,
}

pub fn protocol_deriver(ast: &syn::DeriveInput, is_internal: bool) -> syn::Result<TokenStream> {
    let name = &ast.ident;

    let mut out_code = OutputCode::default();
    let mut proto_settings = ProtocolSettings {
        generate_tests: false,
        enum_name: name.clone(),
        crate_location: if is_internal {
            quote! {crate}
        } else {
            quote! {pso2packetlib}
        },
    };

    let Data::Enum(data) = &ast.data else {
        return Err(syn::Error::new(
            ast.span(),
            "ProtocolRW is only defined for enums",
        ));
    };
    get_attr_iter(&mut proto_settings, &ast.attrs, get_proto_attrs)?;
    parse_enum_field(&mut out_code, data, &proto_settings)?;

    let OutputCode {
        read,
        write,
        category,
        read_raw,
        tests,
    } = out_code;

    let tests = if proto_settings.generate_tests {
        quote! {
            #[cfg(test)]
            mod auto_protocol_tests {
                #tests
            }
        }
    } else {
        quote! {}
    };
    let crate_location = proto_settings.crate_location;

    let gen = quote! {
        #[automatically_derived]
        impl #crate_location::protocol::ProtocolRW for #name {
            fn write(&self, packet_type: #crate_location::protocol::PacketType) -> Vec<u8> {
                use #crate_location::derive_reexports::*;
                use #crate_location::protocol::PacketError;

                let mut buf: Vec<u8> = vec![0; 4];
                let packet_out: Vec<u8> = match self {
                    #write
                };
                buf.extend(packet_out);
                let len = buf.len().next_multiple_of(4);
                buf.resize(len, 0);
                let len = (len as u32).to_le_bytes();
                buf[..4].copy_from_slice(&len);
                buf
            }
            fn read(
                input: &[u8],
                packet_type: #crate_location::protocol::PacketType,
            ) -> Result<Vec<Self>, #crate_location::protocol::PacketError> {
                use #crate_location::derive_reexports::*;
                use #crate_location::protocol::PacketError;
                let packet_name = stringify!(#name);

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
                    let len = (&input[pointer..pointer + 4]).read_u32::<LittleEndian>().map_err(|e| {
                        PacketError::PacketLengthError{
                            error: e,
                        }
                    })? as usize - 4;
                    pointer += 4;
                    if input[pointer..].len() < len {
                        return Err(PacketError::PacketLengthError{
                            error: std::io::ErrorKind::UnexpectedEof.into()
                        });
                    }
                    #read_raw
                    let mut buf_tmp = std::io::Cursor::new(&input[pointer..pointer + len]);
                    let header = PacketHeader::read(&mut buf_tmp, packet_type).map_err(|e| {
                        PacketError::CompositeFieldError {
                            packet_name: stringify!(#name),
                            field_name: "header",
                            error: Box::new(e),
                        }
                    })?;
                    let flags = &header.flag;

                    pointer += len;
                    match (header.id, header.subid, packet_type) {
                        #read
                    }
                }

                Ok(packets)
            }
            fn get_category(&self) -> #crate_location::protocol::PacketCategory {
                let cat = match self {
                    #category
                    _ => Default::default(),
                };
                cat
            }
        }
        #tests
    };
    Ok(gen.into())
}

fn parse_enum_field(
    out_code: &mut OutputCode,
    data: &DataEnum,
    proto_set: &ProtocolSettings,
) -> syn::Result<()> {
    let mut category_stream = quote! {Default::default()};
    let OutputCode {
        read,
        write,
        category,
        read_raw,
        tests,
    } = out_code;
    for variant in &data.variants {
        let name = &variant.ident;
        let mut settings = Settings::default();

        get_attr_iter(&mut settings, &variant.attrs, get_attrs)?;
        if settings.skip {
            continue;
        }
        if settings.id == 0
            && settings.subid == 0
            && !settings.raw
            && !settings.unknown
            && !matches!(settings.packet_type, PacketType::Empty)
        {
            return Err(syn::Error::new(variant.span(), "No Id defined"));
        }

        if proto_set.generate_tests
            && !settings.raw
            && !settings.unknown
            && !matches!(settings.packet_type, PacketType::Empty)
        {
            let test_name = quote::format_ident!("test_proto_{name}");
            let enum_name = &proto_set.enum_name;
            let crate_loc = &proto_set.crate_location;
            let constructor = if matches!(variant.fields, Fields::Unit) {
                quote! {}
            } else {
                quote! {(Default::default())}
            };
            let mut read_write_quote = quote! {};

            fn create_packet_type(ty: TS2, enum_name: &syn::Ident) -> TS2 {
                quote! {
                    {
                        println!("Testing: {:?}", #ty);
                        let write_data = packet.write(#ty);
                        let read_packet = #enum_name::read(&write_data, #ty).unwrap().remove(0);
                        assert_eq!(packet, read_packet);
                    }
                }
            }

            match settings.packet_type {
                PacketType::Both => {
                    read_write_quote
                        .extend(create_packet_type(quote! {PacketType::NGS}, enum_name));
                    read_write_quote
                        .extend(create_packet_type(quote! {PacketType::Classic}, enum_name));
                    read_write_quote.extend(create_packet_type(quote! {PacketType::NA}, enum_name));
                    read_write_quote.extend(create_packet_type(quote! {PacketType::JP}, enum_name));
                    read_write_quote
                        .extend(create_packet_type(quote! {PacketType::Vita}, enum_name));
                }
                PacketType::Classic => {
                    read_write_quote
                        .extend(create_packet_type(quote! {PacketType::Classic}, enum_name));
                    read_write_quote.extend(create_packet_type(quote! {PacketType::NA}, enum_name));
                    read_write_quote.extend(create_packet_type(quote! {PacketType::JP}, enum_name));
                    read_write_quote
                        .extend(create_packet_type(quote! {PacketType::Vita}, enum_name));
                }
                PacketType::Ngs => {
                    read_write_quote
                        .extend(create_packet_type(quote! {PacketType::NGS}, enum_name));
                }
                PacketType::Na => {
                    read_write_quote.extend(create_packet_type(quote! {PacketType::NA}, enum_name));
                }
                PacketType::Jp => {
                    read_write_quote.extend(create_packet_type(quote! {PacketType::JP}, enum_name));
                }
                PacketType::Vita => {
                    read_write_quote
                        .extend(create_packet_type(quote! {PacketType::Vita}, enum_name));
                }
                PacketType::Empty => {}
            }

            tests.extend(quote! {
                #[test]
                fn #test_name() {
                    use super::*;
                    use #crate_loc::{derive_reexports::*, protocol::PacketType};
                    let packet = #enum_name::#name #constructor;
                    #read_write_quote
                }
            });
        }

        // set ids to a wildcard for unknown packets
        let Settings { id, subid, .. } = settings;
        let (id, subid) = if settings.unknown {
            (quote! {_}, quote! {_})
        } else {
            (quote! {#id}, quote! {#subid})
        };

        if let PacketType::Empty = settings.packet_type {
            write.extend(quote! {
                Self::#name => return vec![],
            })
        }
        let mut push_string = quote! {};
        if !settings.category.is_empty() {
            category_stream = settings.category
        }
        match &variant.fields {
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                if let Type::Path(TypePath { path, .. }) = &unnamed.first().unwrap().ty {
                    if settings.raw {
                        read_raw.extend(quote! {
                            if matches!(packet_type, PacketType::Raw) {
                                let data = &input[pointer - 4..pointer + len];
                                packets.push(Self::#name(data.to_vec()));
                                pointer += len;
                                continue;
                            }
                        });
                        write.extend(quote! {
                            Self::#name(data) => data[4..].to_vec(),
                        });
                        continue;
                    }
                    if settings.unknown {
                        return Err(syn::Error::new(
                            variant.span(),
                            "Unknown packets with fields should only contain tuple of (PacketHeader, Vec<u8>).",
                        ));
                    }
                    let struct_field = path.get_ident().unwrap();
                    push_string = quote! {packets.push(Self::#name(#struct_field::read(&mut buf_tmp, flags, packet_type)?))};
                    write.extend(quote! {
                        Self::#name(packet) => packet.write(packet_type),
                    });
                    category.extend(quote! {
                        Self::#name(_) => {#category_stream},
                    })
                }

                if settings.unknown {
                    push_string = quote! {
                        packets.push(Self::#name({
                            let mut data = vec![];
                            buf_tmp.read_to_end(&mut data).map_err(|e| PacketError::FieldError{
                                packet_name: packet_name,
                                field_name: stringify!(#name),
                                error: e
                            })?;
                            (header, data)
                        }));
                    };
                    write.extend(quote! {
                        Self::#name((header, data)) => {
                            let mut out_data = header.write(packet_type);
                            out_data.extend_from_slice(&data);
                            out_data
                        }
                    });
                }
            }
            Fields::Unit => {
                push_string = quote! {packets.push(Self::#name)};
                if settings.raw {
                    read_raw.extend(quote! {
                        if matches!(packet_type, PacketType::Raw) {
                            packets.push(Self::#name);
                            pointer += len;
                            continue;
                        }
                    });
                    write.extend(quote! {
                        Self::#name => vec![],
                    });
                    continue;
                }
                if settings.unknown {
                    write.extend(quote! {
                        Self::#name => vec![],
                    });
                } else {
                    write.extend(quote! {
                        Self::#name => PacketHeader::new(#id, #subid, Flags::default()).write(packet_type),
                    });
                }
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
            PacketType::Classic => read.extend(quote! {
                (#id, #subid, PacketType::Classic | PacketType::NA | PacketType::JP | PacketType::Vita) => {#push_string},
            }),
            PacketType::Na => read.extend(quote! {
                (#id, #subid, PacketType::NA) => {#push_string},
            }),
            PacketType::Jp => read.extend(quote! {
                (#id, #subid, PacketType::JP) => {#push_string},
            }),
            PacketType::Vita => read.extend(quote! {
                (#id, #subid, PacketType::Vita) => {#push_string},
            }),
            PacketType::Ngs => read.extend(quote! {
                (#id, #subid, PacketType::NGS) => {#push_string},
            }),
            PacketType::Empty => {}
        }
    }
    Ok(())
}

fn get_attr_iter<S>(
    set: &mut S,
    attrs: &[syn::Attribute],
    cb: fn(&mut S, &str, Option<&MetaList>, Span) -> syn::Result<()>,
) -> syn::Result<()> {
    for attr in attrs {
        match &attr.meta {
            syn::Meta::NameValue(_) => {}
            syn::Meta::Path(path) => {
                let string = path.get_ident().unwrap().to_string();
                get_attr_stub(set, &string, None, path.span(), cb)?;
            }
            syn::Meta::List(list) => {
                let string = list.path.get_ident().unwrap().to_string();
                get_attr_stub(set, &string, Some(list), list.span(), cb)?;
            }
        }
    }
    Ok(())
}

fn get_attr_stub<S>(
    set: &mut S,
    string: &str,
    list: Option<&MetaList>,
    span: Span,
    cb: fn(&mut S, &str, Option<&MetaList>, Span) -> syn::Result<()>,
) -> syn::Result<()> {
    if string == "pso2packet" {
        let Some(list) = list else {
            return Err(syn::Error::new(
                span,
                "Invalid syntax \nPerhaps you ment pso2packet(..)?",
            ));
        };
        let meta: syn::Meta = list.parse_args()?;
        match &meta {
            syn::Meta::NameValue(_) => {
                return Err(syn::Error::new(span, "Invalid syntax"));
            }
            syn::Meta::Path(path) => {
                let string = path.get_ident().unwrap().to_string();
                cb(set, &string, None, path.span())?;
            }
            syn::Meta::List(list) => {
                let string = list.path.get_ident().unwrap().to_string();
                cb(set, &string, Some(list), list.span())?;
            }
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
        "empty" => set.packet_type = PacketType::Empty,
        "unknown" => set.unknown = true,
        "raw" => set.raw = true,
        "NGS" => set.packet_type = PacketType::Ngs,
        "Classic" => set.packet_type = PacketType::Classic,
        "NA" => set.packet_type = PacketType::Na,
        "JP" => set.packet_type = PacketType::Jp,
        "Vita" => set.packet_type = PacketType::Vita,
        "id" => {
            let Some(list) = list else {
                return Err(syn::Error::new(
                    span,
                    "Invalid syntax \nPerhaps you ment id(..)?",
                ));
            };
            let attrs: AttributeList = list.parse_args()?;

            if attrs.fields.len() != 2 {
                return Err(syn::Error::new(span, "Invalid number of arguments"));
            }
            set.id = attrs.fields[0].base10_parse()?;
            set.subid = attrs.fields[1].base10_parse()?;
        }
        "category" => {
            let Some(attrs) = list.map(|l| &l.tokens) else {
                return Err(syn::Error::new(
                    span,
                    "Invalid syntax \nPerhaps you ment category(..)?",
                ));
            };
            set.category = attrs.clone();
        }
        _ => return Err(syn::Error::new(span, "Unknown attribute")),
    }
    Ok(())
}

fn get_proto_attrs(
    set: &mut ProtocolSettings,
    string: &str,
    _: Option<&MetaList>,
    span: Span,
) -> syn::Result<()> {
    match string {
        "gen_tests" => set.generate_tests = true,
        _ => return Err(syn::Error::new(span, "Unknown attribute")),
    }
    Ok(())
}

#[derive(Default)]
struct Settings {
    id: u8,
    subid: u16,
    packet_type: PacketType,
    raw: bool,
    unknown: bool,
    skip: bool,
    category: TS2,
}

struct ProtocolSettings {
    generate_tests: bool,
    enum_name: syn::Ident,
    crate_location: TS2,
}

#[derive(Default)]
enum PacketType {
    #[default]
    Both,
    Classic,
    Ngs,
    Na,
    Jp,
    Vita,
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
