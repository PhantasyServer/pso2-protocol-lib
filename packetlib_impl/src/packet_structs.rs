use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TS2};
use quote::{format_ident, quote};
use syn::{
    parse::Parse, punctuated::Punctuated, spanned::Spanned, Attribute, Data, DataEnum, DataStruct,
    Expr, Fields, GenericArgument, Ident, Lit, LitInt, MetaList, PathArguments, Token, Type,
};

pub fn packet_deriver(ast: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (id, subid) = get_packet_id(&ast.attrs)?;
    let flags = get_flags(&ast.attrs)?;

    let mut read = quote! {};
    let mut write = quote! {};

    if let Data::Struct(data) = &ast.data {
        parse_struct_field(&mut read, &mut write, data, false)?;
    }

    let gen = quote! {
        #[automatically_derived]
        impl PacketReadWrite for #name {
            fn read(reader: &mut (impl std::io::Read + std::io::Seek), flags: crate::protocol::Flags, packet_type: crate::protocol::PacketType) -> std::io::Result<Self> {
                use byteorder::{LittleEndian, ReadBytesExt};
                use crate::protocol::HelperReadWrite;
                use crate::asciistring::StringRW;
                #read
            }
            fn write(&self, packet_type: crate::protocol::PacketType) -> Vec<u8> {
                use byteorder::{LittleEndian, WriteBytesExt};
                use crate::protocol::{HelperReadWrite, Flags};
                use crate::asciistring::StringRW;
                use std::io::Write;
                let mut buf = crate::protocol::PacketHeader::new(#id, #subid, #flags).write(packet_type);
                let writer = &mut buf;
                #write
                buf
            }
        }
    };
    Ok(gen.into())
}

pub fn helper_deriver(ast: &syn::DeriveInput) -> syn::Result<TokenStream> {
    let name = &ast.ident;

    let mut read = quote! {};
    let mut write = quote! {};
    let repr_type = get_repr(&ast.attrs)?;
    let is_flags = get_flags_struct(&ast.attrs)?;
    let no_seek = get_no_seek(&ast.attrs)?;

    match &ast.data {
        Data::Struct(data) if matches!(is_flags, Some(_)) => {
            let Some(repr_type) = is_flags else {
                unreachable!()
            };
            parse_flags_struct(&mut read, &mut write, data, repr_type)?
        }
        Data::Struct(data) => parse_struct_field(&mut read, &mut write, data, no_seek)?,
        Data::Enum(data) => parse_enum_field(&mut read, &mut write, data, repr_type)?,
        _ => {}
    }

    let gen = quote! {
        #[automatically_derived]
        impl HelperReadWrite for #name {
            fn read(reader: &mut (impl std::io::Read + std::io::Seek), packet_type: crate::protocol::PacketType) -> std::io::Result<Self> {
                use byteorder::{LittleEndian, ReadBytesExt};
                use crate::asciistring::StringRW;
                #read
            }
            fn write(&self, writer: &mut impl std::io::Write, packet_type: crate::protocol::PacketType) -> std::io::Result<()> {
                use byteorder::{LittleEndian, WriteBytesExt};
                use crate::asciistring::StringRW;
                #write
                Ok(())
            }
        }
    };
    Ok(gen.into())
}

fn parse_enum_field(
    read: &mut TS2,
    write: &mut TS2,
    data: &DataEnum,
    repr_type: Size,
) -> syn::Result<()> {
    let mut default_token = quote! {};
    let mut match_expr = quote! {};
    match repr_type {
        Size::U8 => {
            read.extend(quote! {let num = reader.read_u8()? as u32;});
            write.extend(quote! {writer.write_u8(*self as u8)?;});
        }
        Size::U16 => {
            read.extend(quote! {let num = reader.read_u16::<LittleEndian>()? as u32;});
            write.extend(quote! {writer.write_u16::<LittleEndian>(*self as u16)?;});
        }
        Size::U32 => {
            read.extend(quote! {let num = reader.read_u32::<LittleEndian>()?;});
            write.extend(quote! {writer.write_u32::<LittleEndian>(*self as u32)?;});
        }
    }
    let mut discriminant: u32 = 0;
    for variant in &data.variants {
        let name = &variant.ident;
        let mut settings = Settings::default();
        if let Some((_, Expr::Lit(x))) = &variant.discriminant {
            if let Lit::Int(x) = &x.lit {
                discriminant = x.base10_parse()?;
            }
        }
        for attr in &variant.attrs {
            match &attr.meta {
                syn::Meta::NameValue(_) => {}
                syn::Meta::Path(path) => {
                    let string = path.get_ident().unwrap().to_string();
                    get_attrs(
                        &mut settings,
                        &string,
                        None,
                        &mut quote! {},
                        &mut quote! {},
                        name,
                    )?;
                }
                syn::Meta::List(_) => {}
            }
        }
        if settings.is_default {
            default_token = quote! {_ => Self::#name,};
            discriminant = discriminant.overflowing_add(1).0;
            continue;
        }
        match_expr.extend(quote! {#discriminant => Self::#name,});
        discriminant = discriminant.overflowing_add(1).0;
    }
    read.extend(quote! {Ok(match num {
        #match_expr
        #default_token
    })});
    Ok(())
}

fn parse_flags_struct(
    read: &mut TS2,
    write: &mut TS2,
    data: &DataStruct,
    repr: Size,
) -> syn::Result<()> {
    let mut return_token = quote! {};
    let mut discriminant = 1u64;
    write.extend(quote! {let mut num = 0;});
    let write_after = match repr {
        Size::U8 => {
            read.extend(quote! {let num = reader.read_u8()? as u64;});
            quote! {writer.write_u8(num as u8)?;}
        }
        Size::U16 => {
            read.extend(quote! {let num = reader.read_u16::<LittleEndian>()? as u64;});
            quote! {writer.write_u16::<LittleEndian>(num as u16)?;}
        }
        Size::U32 => {
            read.extend(quote! {let num = reader.read_u32::<LittleEndian>()? as u64;});
            quote! {writer.write_u32::<LittleEndian>(num as u32)?;}
        }
    };
    for field in data.fields.iter() {
        let name = field.ident.as_ref().unwrap();
        return_token.extend(quote! {#name,});

        for attr in &field.attrs {
            match &attr.meta {
                syn::Meta::NameValue(_) => {}
                syn::Meta::Path(path) => {
                    let string = path.get_ident().unwrap().to_string();
                    if string == "Skip" {
                        discriminant <<= 1;
                    }
                }
                syn::Meta::List(_) => {}
            }
        }

        read.extend(quote! {
            let mut #name = false;
            if num & #discriminant != 0 {
                #name = true;
            }
        });
        write.extend(quote! {
            if self.#name {
                num += #discriminant;
            }
        });
        discriminant <<= 1;
    }
    read.extend(quote! {Ok(Self{#return_token})});
    write.extend(write_after);
    Ok(())
}

fn parse_struct_field(
    read: &mut TS2,
    write: &mut TS2,
    data: &DataStruct,
    no_seek: bool,
) -> syn::Result<()> {
    let mut return_token = quote! {};
    if let Fields::Unnamed(fileds) = &data.fields {
        let mut writer_names = quote! {};
        let mut tmp_write = quote! {};
        for (id, field) in fileds.unnamed.iter().enumerate() {
            let varname = format_ident!("temp_{}", id);
            return_token.extend(quote! {#varname,});
            let id = syn::Index::from(id);
            writer_names.extend(quote! { let #varname = self.#id;});
            check_syn_type(
                &field.ty,
                read,
                &mut tmp_write,
                &varname,
                &Settings::default(),
                false,
                no_seek,
            )?;
        }
        read.extend(quote! {Ok(Self(#return_token))});
        write.extend(quote! {
            #writer_names
            #tmp_write
        });
        return Ok(());
    }
    for field in data.fields.iter() {
        let name = field.ident.as_ref().unwrap();
        return_token.extend(quote! {#name,});

        if name.to_string() == "is_global" {
            read.extend(quote! {let is_global = false;});
            continue;
        }

        let mut settings = Settings::default();

        for attr in &field.attrs {
            match &attr.meta {
                syn::Meta::NameValue(_) => {}
                syn::Meta::Path(path) => {
                    let string = path.get_ident().unwrap().to_string();
                    get_attrs(&mut settings, &string, None, read, write, name)?;
                }
                syn::Meta::List(list) => {
                    let string = list.path.get_ident().unwrap().to_string();
                    get_attrs(&mut settings, &string, Some(&list), read, write, name)?;
                }
            }
        }
        let mut tmp_read = quote! {};
        let mut tmp_write = quote! {};
        check_syn_type(
            &field.ty,
            &mut tmp_read,
            &mut tmp_write,
            name,
            &settings,
            true,
            no_seek,
        )?;
        if let Some(data) = settings.only_on {
            read.extend(quote! {let #name = if matches!(packet_type, #data) {
                #tmp_read
                #name
            } else {
                Default::default()
            };});
            write.extend(quote! {if matches!(packet_type, #data) {
                #tmp_write
            }});
        } else {
            read.extend(tmp_read);
            write.extend(tmp_write)
        }
        if settings.seek_after != 0 {
            let seek_after = settings.seek_after;
            read.extend(quote! {reader.seek(std::io::SeekFrom::Current(#seek_after))?;});
            write.extend(quote! {writer.write_all(&[0u8; #seek_after as usize]).unwrap();});
        }
    }
    read.extend(quote! {Ok(Self{#return_token})});
    Ok(())
}

#[derive(Default)]
struct Settings {
    is_psotime: bool,
    seek_after: i64,
    str_type: StringType,
    is_default: bool,
    to_skip: bool,
    only_on: Option<TS2>,
    fixed_len: u32,
}

fn get_attrs(
    set: &mut Settings,
    string: &str,
    list: Option<&MetaList>,
    read: &mut TS2,
    write: &mut TS2,
    name: &Ident,
) -> syn::Result<()> {
    match string {
        "Read_default" => set.is_default = true,
        "PSOTime" => set.is_psotime = true,
        "Skip" => set.to_skip = true,
        "OnlyOn" => {
            let attrs = match list {
                Some(x) => &x.tokens,
                None => {
                    return Err(syn::Error::new(
                        Span::call_site(),
                        "Invalid syntax \nPerhaps you ment OnlyOn(..)?",
                    ))
                }
            };
            set.only_on = Some(attrs.clone());
        }
        "Seek" => {
            let amount: LitInt = list.unwrap().parse_args()?;
            let amount: i64 = amount.base10_parse()?;
            read.extend(quote! {reader.seek(std::io::SeekFrom::Current(#amount))?;});
            write.extend(quote! {writer.write_all(&[0u8; #amount as usize]).unwrap();});
        }
        "SeekAfter" => {
            let amount: LitInt = list.unwrap().parse_args()?;
            set.seek_after = amount.base10_parse()?;
        }
        "FixedLen" => {
            let amount: LitInt = list.unwrap().parse_args()?;
            set.fixed_len = amount.base10_parse()?;
        }
        "Const_u16" => {
            let num: LitInt = list.unwrap().parse_args()?;
            let num: u16 = num.base10_parse()?;
            read.extend(quote! {reader.seek(std::io::SeekFrom::Current(2))?;});
            write.extend(quote! {writer.write_u16::<LittleEndian>(#num).unwrap();});
        }
        "FixedStr" => {
            let len: LitInt = list.unwrap().parse_args()?;
            let len = len.base10_parse()?;
            set.str_type = StringType::Fixed(len);
        }
        "VariableStr" => {
            let attrs: AttributeList = list.unwrap().parse_args()?;
            if attrs.fields.len() != 2 {
                return Err(syn::Error::new(list.span(), "Invalid number of arguments"));
            }
            let magic = attrs.fields[0].base10_parse()?;
            let sub = attrs.fields[1].base10_parse()?;
            set.str_type = StringType::Variable(magic, sub);
        }
        "Magic" => {
            let attrs: AttributeList = list.unwrap().parse_args()?;
            if attrs.fields.len() != 2 {
                return Err(syn::Error::new(list.span(), "Invalid number of arguments"));
            }
            let magic: u32 = attrs.fields[0].base10_parse()?;
            let sub: u32 = attrs.fields[1].base10_parse()?;
            read.extend(
                quote! {let len = crate::protocol::read_magic(reader, #sub, #magic)? as usize;},
            );
            write.extend(quote! {
                writer.write_u32::<LittleEndian>(crate::protocol::write_magic(self.#name.len() as u32, #sub, #magic))
                .unwrap();
            });
        }
        "Len_u32" => {
            read.extend(quote! { let len = reader.read_u32::<LittleEndian>()?; });
            write.extend(quote! {
                writer.write_u32::<LittleEndian>(self.#name.len() as u32)
                .unwrap();
            });
        }
        "Len_u16" => {
            read.extend(quote! { let len = reader.read_u16::<LittleEndian>()?; });
            write.extend(quote! {
                writer.write_u16::<LittleEndian>(self.#name.len() as u16)
                .unwrap();
            });
        }
        _ => {}
    }
    Ok(())
}

fn check_syn_type(
    in_type: &Type,
    read: &mut TS2,
    write: &mut TS2,
    name: &Ident,
    set: &Settings,
    is_first: bool,
    no_seek: bool,
) -> syn::Result<()> {
    match in_type {
        Type::Path(path) => match path.path.get_ident() {
            Some(identity) => {
                let string = identity.to_string();
                let (in_read, in_write) =
                    check_code_type(string, name, set, path.span(), is_first)?;
                read.extend(in_read);
                write.extend(in_write);
            }
            None => {
                let segment = &path.path.segments[0];
                if segment.ident.to_string() == "Vec" {
                    if let PathArguments::AngleBracketed(args) = &segment.arguments {
                        if let GenericArgument::Type(x) = &args.args[0] {
                            let mut tmp_read = quote! {};
                            let mut tmp_write = quote! {};
                            let tmp_name = Ident::new("tmp", Span::call_site());
                            check_syn_type(
                                x,
                                &mut tmp_read,
                                &mut tmp_write,
                                &tmp_name,
                                set,
                                false,
                                no_seek,
                            )?;

                            let seek_pad = if no_seek {
                                quote! {}
                            } else if tmp_read.to_string().contains("read_u8()") {
                                quote! { reader.seek(std::io::SeekFrom::Current((((len + 4 - 1) & (usize::MAX ^ (4 - 1))) - len) as i64))?; }
                            } else if tmp_read.to_string().contains("read_u16()") {
                                quote! { reader.seek(std::io::SeekFrom::Current(((((len * 2) + 4 - 1) & (usize::MAX ^ (4 - 1))) - (len * 2)) as i64))?; }
                            } else {
                                quote! {}
                            };
                            let write_pad = if no_seek {
                                quote! {}
                            } else if tmp_read.to_string().contains("read_u8()") {
                                quote! { writer.write_all(&vec![0u8; ((len + 4 - 1) & (usize::MAX ^ (4 - 1))) - len]).unwrap(); }
                            } else if tmp_read.to_string().contains("read_u16()") {
                                quote! { writer.write_all(&vec![0u8; (((len * 2) + 4 - 1) & (usize::MAX ^ (4 - 1))) - (len * 2)]).unwrap(); }
                            } else {
                                quote! {}
                            };

                            if set.fixed_len == 0 {
                                read.extend(quote! {
                                    let mut #name = vec![];
                                    for _ in 0..len {
                                        #tmp_read
                                        #name.push(#tmp_name);
                                    }
                                    #seek_pad
                                });
                                write.extend(quote! {
                                    let len = self.#name.len();
                                    for #tmp_name in &self.#name {
                                        #tmp_write
                                    }
                                    #write_pad
                                });
                            } else {
                                let len = set.fixed_len;
                                read.extend(quote! {
                                    let mut #name = vec![];
                                    let len = #len as usize;
                                    for _ in 0..len {
                                        #tmp_read
                                        #name.push(#tmp_name);
                                    }
                                    #seek_pad
                                });
                                write.extend(quote! {
                                    let len = #len as usize;
                                    let def_thing = vec![Default::default()];
                                    for #tmp_name in self.#name.iter().chain(def_thing.iter().cycle()).take(len) {
                                        #tmp_write
                                    }
                                    #write_pad
                                });
                            }
                        }
                    }
                }
            }
        },
        Type::Array(arr) => {
            let in_type = arr.elem.as_ref();
            let len = &arr.len;
            let mut tmp_read = quote! {};
            let mut tmp_write = quote! {};
            let tmp_name = Ident::new("tmp", Span::call_site());
            check_syn_type(
                in_type,
                &mut tmp_read,
                &mut tmp_write,
                &tmp_name,
                set,
                false,
                no_seek,
            )?;
            if tmp_read.to_string().contains("read_u8()") {
                read.extend(quote! {
                    let mut #name = [Default::default(); #len];
                    reader.read_exact(&mut #name)?;
                });
                write.extend(quote! {
                    writer.write_all(&self.#name).unwrap();
                });
            } else {
                read.extend(quote! {
                    let mut #name = vec![];
                    for i in 0..#len {
                        #tmp_read
                        #name.push(#tmp_name);
                    }
                    let #name = #name.try_into().unwrap();
                });
                write.extend(quote! {
                    for #tmp_name in &self.#name {
                        #tmp_write
                    }
                });
            }
        }
        _ => {}
    }
    Ok(())
}

fn check_code_type(
    string: String,
    name: &Ident,
    set: &Settings,
    span: Span,
    is_first: bool,
) -> syn::Result<(TS2, TS2)> {
    let mut read = quote! {};
    let mut write = quote! {};

    let write_name = if is_first {
        quote! {self.#name}
    } else {
        quote! {#name}
    };

    match string.as_str() {
        "u8" => {
            read.extend(quote! {let #name = reader.read_u8()?;});
            write.extend(quote! {writer.write_u8(#write_name.clone()).unwrap();});
        }
        "i8" => {
            read.extend(quote! {let #name = reader.read_i8()?;});
            write.extend(quote! {writer.write_i8(#write_name.clone()).unwrap();});
        }
        "u16" => {
            read.extend(quote! {let #name = reader.read_u16::<LittleEndian>()?;});
            write.extend(quote! {writer.write_u16::<LittleEndian>(#write_name.clone()).unwrap();});
        }
        "i16" => {
            read.extend(quote! {let #name = reader.read_i16::<LittleEndian>()?;});
            write.extend(quote! {writer.write_i16::<LittleEndian>(#write_name.clone()).unwrap();});
        }
        "u32" => {
            read.extend(quote! {let #name = reader.read_u32::<LittleEndian>()?;});
            write.extend(quote! {writer.write_u32::<LittleEndian>(#write_name.clone()).unwrap();});
        }
        "i32" => {
            read.extend(quote! {let #name = reader.read_i32::<LittleEndian>()?;});
            write.extend(quote! {writer.write_i32::<LittleEndian>(#write_name.clone()).unwrap();});
        }
        "u64" => {
            read.extend(quote! {let #name = reader.read_u64::<LittleEndian>()?;});
            write.extend(quote! {writer.write_u64::<LittleEndian>(#write_name.clone()).unwrap();});
        }
        "i64" => {
            read.extend(quote! {let #name = reader.read_i64::<LittleEndian>()?;});
            write.extend(quote! {writer.write_i64::<LittleEndian>(#write_name.clone()).unwrap();});
        }
        "u128" => {
            read.extend(quote! {let #name = reader.read_u128::<LittleEndian>()?;});
            write.extend(quote! {writer.write_u128::<LittleEndian>(#write_name.clone()).unwrap();});
        }
        "i128" => {
            read.extend(quote! {let #name = reader.read_i128::<LittleEndian>()?;});
            write.extend(quote! {writer.write_i128::<LittleEndian>(#write_name.clone()).unwrap();});
        }
        "f16" => {
            read.extend(
                quote! {let #name = half::f16::from_bits(reader.read_u16::<LittleEndian>()?);},
            );
            write.extend(
                quote! {writer.write_u16::<LittleEndian>(#write_name.clone().to_bits()).unwrap();},
            );
        }
        "f32" => {
            read.extend(quote! {let #name = reader.read_f32::<LittleEndian>()?;});
            write.extend(quote! {writer.write_f32::<LittleEndian>(#write_name.clone()).unwrap();});
        }
        "f64" => {
            read.extend(quote! {let #name = reader.read_f64::<LittleEndian>()?;});
            write.extend(quote! {writer.write_f64::<LittleEndian>(#write_name.clone()).unwrap();});
        }
        "Ipv4Addr" => {
            read.extend(quote! {
                let mut ip_buf = [0u8; 4];
                reader.read_exact(&mut ip_buf)?;
                let #name = Ipv4Addr::from(ip_buf);
            });
            write.extend(quote! {
                writer.write_all(&#write_name.octets()).unwrap();
            })
        }
        "Duration" => {
            if set.is_psotime {
                read.extend(
                    quote! {let #name = crate::protocol::psotime_to_duration(reader.read_u64::<LittleEndian>()?);},
                );
                write.extend(
                    quote! {writer.write_u64::<LittleEndian>(crate::protocol::duration_to_psotime(#write_name))
                    .unwrap();},
                );
            } else {
                read.extend(quote! {let #name = Duration::from_secs(reader.read_u32::<LittleEndian>()? as u64);});
                write.extend(
                    quote! {writer.write_u32::<LittleEndian>(#write_name.as_secs() as u32)
                    .unwrap();},
                );
            }
        }
        "String" => match set.str_type {
            StringType::Unknown => return Err(syn::Error::new(span, "Unknown string type")),
            StringType::Fixed(len) => {
                read.extend(quote! {let #name = String::read(reader, #len)?;});
                write
                    .extend(quote! {writer.write_all(&#write_name.write(#len as usize)).unwrap();});
            }
            StringType::Variable(magic, sub) => {
                read.extend(quote! {let #name = String::read_variable(reader, #sub, #magic)?;});
                write.extend(
                    quote! {writer.write_all(&#write_name.write_variable(#sub, #magic))
                    .unwrap();},
                );
            }
        },
        "AsciiString" => match set.str_type {
            StringType::Fixed(len) => {
                read.extend(quote! {let #name = crate::AsciiString::read(reader, #len)?;});
                write
                    .extend(quote! {writer.write_all(&#write_name.write(#len as usize)).unwrap();});
            }
            StringType::Variable(magic, sub) => {
                read.extend(
                    quote! {let #name = crate::AsciiString::read_variable(reader, #sub, #magic)?;},
                );
                write.extend(
                    quote! {writer.write_all(&#write_name.write_variable(#sub, #magic))
                    .unwrap();},
                );
            }
            _ => return Err(syn::Error::new(span, "Unknown string type")),
        },
        _ => {
            let out_type = Ident::new(&string, Span::call_site());
            read.extend(quote! {let #name = #out_type::read(reader, packet_type)?;});
            write.extend(quote! {#write_name.write(writer, packet_type).unwrap();});
        }
    }
    Ok((read, write))
}

fn get_packet_id(attrs: &Vec<Attribute>) -> syn::Result<(u8, u16)> {
    for attr in attrs.iter() {
        if !attr.path().is_ident("Id") {
            continue;
        }
        match &attr.meta {
            syn::Meta::NameValue(_) => {}
            syn::Meta::Path(_) => {
                return Err(syn::Error::new(
                    attr.span(),
                    "Invalid syntax \nPerhaps you ment Id(..)?",
                ));
            }
            syn::Meta::List(list) => {
                let attrs: AttributeList = list.parse_args()?;
                if attrs.fields.len() != 2 {
                    return Err(syn::Error::new(attr.span(), "Invalid number of arguments"));
                }
                let id = attrs.fields[0].base10_parse()?;
                let subid = attrs.fields[1].base10_parse()?;
                return Ok((id, subid));
            }
        }
    }
    return Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "No Id defined",
    ));
}

fn get_flags(attrs: &Vec<Attribute>) -> syn::Result<TS2> {
    for attr in attrs.iter() {
        if !attr.path().is_ident("Flags") {
            continue;
        }
        match &attr.meta {
            syn::Meta::NameValue(_) => {}
            syn::Meta::Path(_) => {
                return Err(syn::Error::new(
                    attr.span(),
                    "Invalid syntax \nPerhaps you ment Flags(..)?",
                ));
            }
            syn::Meta::List(list) => {
                let attrs = &list.tokens;
                return Ok(quote! {#attrs});
            }
        }
    }
    return Ok(quote! {Flags::default()});
}

fn get_repr(attrs: &Vec<Attribute>) -> syn::Result<Size> {
    for attr in attrs.iter() {
        if !attr.path().is_ident("repr") {
            continue;
        }
        match &attr.meta {
            syn::Meta::NameValue(_) => {}
            syn::Meta::Path(_) => {}
            syn::Meta::List(x) => {
                return Ok(match x.tokens.to_string().as_str() {
                    "u8" => Size::U8,
                    "u16" => Size::U16,
                    "u32" => Size::U32,
                    _ => Size::U8,
                })
            }
        }
    }
    return Ok(Size::U8);
}

fn get_flags_struct(attrs: &Vec<Attribute>) -> syn::Result<Option<Size>> {
    for attr in attrs.iter() {
        if !attr.path().is_ident("Flags") {
            continue;
        }
        match &attr.meta {
            syn::Meta::NameValue(_) => {}
            syn::Meta::Path(_) => {
                return Err(syn::Error::new(
                    attr.span(),
                    "Invalid syntax \nPerhaps you ment Flags(..)?",
                ));
            }
            syn::Meta::List(x) => {
                return Ok(match x.tokens.to_string().as_str() {
                    "u8" => Some(Size::U8),
                    "u16" => Some(Size::U16),
                    "u32" => Some(Size::U32),
                    _ => None,
                })
            }
        }
    }
    return Ok(None);
}

fn get_no_seek(attrs: &Vec<Attribute>) -> syn::Result<bool> {
    for attr in attrs.iter() {
        if !attr.path().is_ident("NoPadding") {
            continue;
        }
        return Ok(true);
    }
    return Ok(false);
}

#[derive(Default)]
enum Size {
    #[default]
    U8,
    U16,
    U32,
}

#[derive(Default)]
enum StringType {
    #[default]
    Unknown,
    // len
    Fixed(u64),
    // magic, sub
    Variable(u32, u32),
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
