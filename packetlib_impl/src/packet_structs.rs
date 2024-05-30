use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TS2};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::Parse, punctuated::Punctuated, spanned::Spanned, Attribute, Data, DataEnum, DataStruct,
    Expr, Fields, GenericArgument, Ident, Lit, LitInt, MetaList, PathArguments, Token, Type,
};

pub fn packet_deriver(ast: &syn::DeriveInput, is_internal: bool) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let (id, subid) = get_packet_id(&ast.attrs)?;
    let xor_sub = get_magic(&ast.attrs)?;
    let flags = get_flags(&ast.attrs)?;
    if flags.to_string().contains("PACKED") && xor_sub.is_none() {
        return Err(syn::Error::new(ast.ident.span(), "No magic provided"));
    }
    let (xor, sub) = xor_sub.unwrap_or((0, 0));

    let crate_location = if is_internal {
        quote! {crate}
    } else {
        quote! {pso2packetlib}
    };

    let mut read = quote! {};
    let mut write = quote! {};

    if let Data::Struct(data) = &ast.data {
        parse_struct_field(&mut read, &mut write, data, false)?;
    }

    let code = quote! {
        #[automatically_derived]
        impl #crate_location::protocol::PacketReadWrite for #name {
            fn read(
                reader: &mut (impl std::io::Read + std::io::Seek),
                flags: &#crate_location::protocol::Flags,
                packet_type: #crate_location::protocol::PacketType
            ) -> Result<Self, #crate_location::protocol::PacketError> {
                use #crate_location::derive_reexports::*;
                use #crate_location::protocol::PacketError as Error;
                let packet_name = stringify!(#name);

                let (xor, sub) = (#xor, #sub);
                #read
            }
            fn write(
                &self,
                packet_type: #crate_location::protocol::PacketType
            ) -> Result<Vec<u8>, #crate_location::protocol::PacketError> {
                use #crate_location::derive_reexports::*;
                use #crate_location::protocol::PacketError as Error;
                let packet_name = stringify!(#name);

                let mut buf = PacketHeader::new(#id, #subid, #flags).write(packet_type);
                let writer = &mut buf;
                let (xor, sub) = (#xor, #sub);
                #write
                Ok(buf)
            }
        }
    };
    Ok(code.into())
}

pub fn helper_deriver(ast: &syn::DeriveInput, is_internal: bool) -> syn::Result<TokenStream> {
    let name = &ast.ident;

    let mut read = quote! {};
    let mut write = quote! {};
    let repr_type = get_repr(&ast.attrs)?;
    let is_flags = get_flags_struct(&ast.attrs)?;
    let is_bitflags = get_bitflags_struct(&ast.attrs)?;
    let no_seek = get_no_seek(&ast.attrs);

    let crate_location = if is_internal {
        quote! {crate}
    } else {
        quote! {pso2packetlib}
    };

    match &ast.data {
        Data::Struct(_) if is_bitflags.is_some() => {
            let Some(repr_type) = is_bitflags else {
                unreachable!()
            };
            parse_bitflags(&mut read, &mut write, repr_type)?
        }
        Data::Struct(data) if is_flags.is_some() => {
            let Some(repr_type) = is_flags else {
                unreachable!()
            };
            parse_flags_struct(&mut read, &mut write, data, repr_type)?
        }
        Data::Struct(data) => parse_struct_field(&mut read, &mut write, data, no_seek)?,
        Data::Enum(data) => parse_enum(&mut read, &mut write, data, repr_type)?,
        _ => {}
    }

    let gen = quote! {
        #[automatically_derived]
        impl #crate_location::protocol::HelperReadWrite for #name {
            fn read(
                reader: &mut (impl std::io::Read + std::io::Seek),
                packet_type: #crate_location::protocol::PacketType,
                xor: u32,
                sub: u32
            ) -> Result<Self, #crate_location::protocol::PacketError> {
                use #crate_location::derive_reexports::*;
                use #crate_location::protocol::PacketError as Error;
                let packet_name = stringify!(#name);

                #read
            }
            fn write(
                &self,
                writer: &mut impl std::io::Write,
                packet_type: #crate_location::protocol::PacketType,
                xor: u32,
                sub: u32
            ) -> Result<(), #crate_location::protocol::PacketError> {
                use #crate_location::derive_reexports::*;
                use #crate_location::protocol::PacketError as Error;
                let packet_name = stringify!(#name);

                #write
                Ok(())
            }
        }
    };
    Ok(gen.into())
}

fn parse_enum(
    read: &mut TS2,
    write: &mut TS2,
    data: &DataEnum,
    repr_type: Size,
) -> syn::Result<()> {
    let mut default_token = quote! {};
    let mut match_expr = quote! {};
    let mut discriminant = match repr_type {
        Size::U8 => {
            read.extend(quote! {let num = reader.read_u8()});
            write.extend(quote! {writer.write_u8(*self as _)});
            Discriminant::U8(0)
        }
        Size::U16 => {
            read.extend(quote! {let num = reader.read_u16::<LittleEndian>()});
            write.extend(quote! {writer.write_u16::<LittleEndian>(*self as _)});
            Discriminant::U16(0)
        }
        Size::U32 => {
            read.extend(quote! {let num = reader.read_u32::<LittleEndian>()});
            write.extend(quote! {writer.write_u32::<LittleEndian>(*self as _)});
            Discriminant::U32(0)
        }
        Size::U64 => {
            read.extend(quote! {let num = reader.read_u64::<LittleEndian>()});
            write.extend(quote! {writer.write_u64::<LittleEndian>(*self as _)});
            Discriminant::U64(0)
        }
        Size::U128 => {
            read.extend(quote! {let num = reader.read_u128::<LittleEndian>()});
            write.extend(quote! {writer.write_u128::<LittleEndian>(*self as _)});
            Discriminant::U128(0)
        }
    };
    read.extend(quote! {.map_err(|e| Error::ValueError{
            packet_name,
            error: e,
        })?;
    });
    write.extend(quote! {.map_err(|e| Error::ValueError{
            packet_name,
            error: e,
        })?;
    });

    for variant in &data.variants {
        let variant_name = &variant.ident;
        let mut settings = Settings::default();

        if let Some((_, Expr::Lit(x))) = &variant.discriminant {
            let Lit::Int(int) = &x.lit else {
                return Err(syn::Error::new(x.span(), "Expected integer literal"));
            };
            match &mut discriminant {
                Discriminant::U8(d) => *d = int.base10_parse()?,
                Discriminant::U16(d) => *d = int.base10_parse()?,
                Discriminant::U32(d) => *d = int.base10_parse()?,
                Discriminant::U64(d) => *d = int.base10_parse()?,
                Discriminant::U128(d) => *d = int.base10_parse()?,
            }
        }

        for attr in &variant.attrs {
            let syn::Meta::Path(path) = &attr.meta else {
                continue;
            };
            let attribute_name = path.get_ident().unwrap().to_string();
            get_attrs(
                &mut settings,
                &attribute_name,
                None,
                &mut quote! {},
                &mut quote! {},
            )?;
        }

        if settings.is_default {
            default_token = quote! {_ => Self::#variant_name,};
            discriminant.increase();
            continue;
        }

        match_expr.extend(quote! {#discriminant => Self::#variant_name,});
        discriminant.increase();
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
    let mut discriminant;
    write.extend(quote! {let mut num = 0;});

    let mut write_after = match repr {
        Size::U8 => {
            read.extend(quote! {let num = reader.read_u8()});
            discriminant = Discriminant::U8(1);
            quote! {writer.write_u8(num)}
        }
        Size::U16 => {
            read.extend(quote! {let num = reader.read_u16::<LittleEndian>()});
            discriminant = Discriminant::U16(1);
            quote! {writer.write_u16::<LittleEndian>(num)}
        }
        Size::U32 => {
            read.extend(quote! {let num = reader.read_u32::<LittleEndian>()});
            discriminant = Discriminant::U32(1);
            quote! {writer.write_u32::<LittleEndian>(num)}
        }
        Size::U64 => {
            read.extend(quote! {let num = reader.read_u64::<LittleEndian>()});
            discriminant = Discriminant::U64(1);
            quote! {writer.write_u64::<LittleEndian>(num)}
        }
        Size::U128 => {
            read.extend(quote! {let num = reader.read_u128::<LittleEndian>()});
            discriminant = Discriminant::U128(1);
            quote! {writer.write_u128::<LittleEndian>(num)}
        }
    };
    read.extend(quote! {.map_err(|e| Error::ValueError{
            packet_name,
            error: e,
        })?;
    });
    write_after.extend(quote! {.map_err(|e| Error::ValueError{
            packet_name,
            error: e,
        })?;
    });

    for field in data.fields.iter() {
        let field_name = field.ident.as_ref().unwrap();
        return_token.extend(quote! {#field_name,});

        for attr in &field.attrs {
            let syn::Meta::Path(path) = &attr.meta else {
                continue;
            };
            let attribute_name = path.get_ident().unwrap().to_string();
            if attribute_name == "Skip" {
                discriminant.skip_flag();
            }
        }

        read.extend(quote! {
            let #field_name = if num & #discriminant != 0 {
                true
            } else {
                false
            };
        });
        write.extend(quote! {
            if self.#field_name {
                num += #discriminant;
            }
        });
        discriminant.skip_flag();
    }

    read.extend(quote! {Ok(Self{#return_token})});
    write.extend(write_after);
    Ok(())
}

fn parse_bitflags(read: &mut TS2, write: &mut TS2, repr: Size) -> syn::Result<()> {
    match repr {
        Size::U8 => {
            read.extend(quote! {let num = reader.read_u8()});
            write.extend(quote! {writer.write_u8(self.bits())});
        }
        Size::U16 => {
            read.extend(quote! {let num = reader.read_u16::<LittleEndian>()});
            write.extend(quote! {writer.write_u16::<LittleEndian>(self.bits())});
        }
        Size::U32 => {
            read.extend(quote! {let num = reader.read_u32::<LittleEndian>()});
            write.extend(quote! {writer.write_u32::<LittleEndian>(self.bits())});
        }
        Size::U64 => {
            read.extend(quote! {let num = reader.read_u64::<LittleEndian>()});
            write.extend(quote! {writer.write_u64::<LittleEndian>(self.bits())});
        }
        Size::U128 => {
            read.extend(quote! {let num = reader.read_u128::<LittleEndian>()});
            write.extend(quote! {writer.write_u128::<LittleEndian>(self.bits())});
        }
    };
    read.extend(quote! {.map_err(|e| Error::ValueError{
            packet_name,
            error: e,
        })?;
    });
    write.extend(quote! {.map_err(|e| Error::ValueError{
            packet_name,
            error: e,
        })?;
    });

    read.extend(quote! {Ok(Self::from_bits_truncate(num))});
    Ok(())
}

fn parse_struct_field(
    read: &mut TS2,
    write: &mut TS2,
    data: &DataStruct,
    no_seek: bool,
) -> syn::Result<()> {
    let mut return_token = quote! {};

    // unnamed struct
    if let Fields::Unnamed(fileds) = &data.fields {
        for (id, field) in fileds.unnamed.iter().enumerate() {
            let field_name = format_ident!("unnamed_{}", id);
            return_token.extend(quote! {#field_name,});

            let id = syn::Index::from(id);
            write.extend(quote! { let #field_name = self.#id;});

            parse_field_type(
                &field.ty,
                read,
                write,
                &field_name,
                &Settings::default(),
                false,
                no_seek,
            )?;
        }
        read.extend(quote! {Ok(Self(#return_token))});
        return Ok(());
    }

    for field in &data.fields {
        let field_name = field.ident.as_ref().unwrap();
        return_token.extend(quote! {#field_name,});

        let mut settings = Settings::default();

        for attr in &field.attrs {
            match &attr.meta {
                syn::Meta::NameValue(_) => {}
                syn::Meta::Path(path) => {
                    let attribute_name = path.get_ident().unwrap().to_string();
                    get_attrs(&mut settings, &attribute_name, None, read, write)?;
                }
                syn::Meta::List(list) => {
                    let attribute_name = list.path.get_ident().unwrap().to_string();
                    get_attrs(&mut settings, &attribute_name, Some(list), read, write)?;
                }
            }
        }

        let mut tmp_read = quote! {};
        let mut tmp_write = quote! {};

        parse_field_type(
            &field.ty,
            &mut tmp_read,
            &mut tmp_write,
            field_name,
            &settings,
            true,
            no_seek,
        )?;

        if let Some(data) = settings.only_on {
            read.extend(quote! {let #field_name = if matches!(packet_type, #data) {
                #tmp_read
                #field_name
            } else {
                Default::default()
            };});
            write.extend(quote! {if matches!(packet_type, #data) {
                #tmp_write
            }});
        } else if let Some(data) = settings.not_on {
            read.extend(quote! {let #field_name = if !matches!(packet_type, #data) {
                #tmp_read
                #field_name
            } else {
                Default::default()
            };});
            write.extend(quote! {if !matches!(packet_type, #data) {
                #tmp_write
            }});
        } else {
            read.extend(tmp_read);
            write.extend(tmp_write)
        }

        if settings.seek_after != 0 {
            let seek_after = settings.seek_after;
            read.extend(quote! {reader.seek(std::io::SeekFrom::Current(#seek_after))
                .map_err(|e| Error::PaddingError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: e,
                })?;
            });
            write.extend(quote! {writer.write_all(&[0u8; #seek_after as usize])
                .map_err(|e| Error::PaddingError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: e,
                })?;
            });
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
    not_on: Option<TS2>,
    fixed_len: u32,
    len_size: Option<Size>,
    manual_rw: Option<(TS2, TS2)>,
}

fn get_attrs(
    set: &mut Settings,
    string: &str,
    list: Option<&MetaList>,
    read: &mut TS2,
    write: &mut TS2,
) -> syn::Result<()> {
    match string {
        "Read_default" => set.is_default = true,
        "PSOTime" => set.is_psotime = true,
        "Skip" => set.to_skip = true,
        "OnlyOn" => {
            let Some(attrs) = list.map(|l| l.tokens.clone()) else {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "Invalid syntax \nPerhaps you ment OnlyOn(..)?",
                ));
            };
            set.only_on = Some(attrs);
        }
        "NotOn" => {
            let Some(attrs) = list.map(|l| l.tokens.clone()) else {
                return Err(syn::Error::new(
                    Span::call_site(),
                    "Invalid syntax \nPerhaps you ment NotOn(..)?",
                ));
            };
            set.not_on = Some(attrs);
        }
        "ManualRW" => {
            let attrs: FnList = list.unwrap().parse_args()?;
            set.manual_rw = Some((
                attrs.fields[0].clone().into_token_stream(),
                attrs.fields[1].clone().into_token_stream(),
            ));
        }
        "Seek" => {
            let amount: i64 = list.unwrap().parse_args::<LitInt>()?.base10_parse()?;
            read.extend(quote! {reader.seek(std::io::SeekFrom::Current(#amount))
                .map_err(|e| Error::PaddingError{
                    packet_name,
                    field_name: "unknown",
                    error: e,
                })?;
            });
            write.extend(quote! {writer.write_all(&[0u8; #amount as usize])
                .map_err(|e| Error::PaddingError{
                    packet_name,
                    field_name: "unknown",
                    error: e,
                })?;
            });
        }
        "SeekAfter" => {
            set.seek_after = list.unwrap().parse_args::<LitInt>()?.base10_parse()?;
        }
        "FixedLen" => {
            set.fixed_len = list.unwrap().parse_args::<LitInt>()?.base10_parse()?;
            set.str_type = StringType::Fixed(set.fixed_len as u64);
        }
        "Const_u16" => {
            let num: u16 = list.unwrap().parse_args::<LitInt>()?.base10_parse()?;
            read.extend(quote! {reader.seek(std::io::SeekFrom::Current(2))
                .map_err(|e| Error::ConstantError{
                    packet_name,
                    const_val: #num as _,
                    error: e,
                })?;
            });
            write.extend(quote! {writer.write_u16::<LittleEndian>(#num)
                .map_err(|e| Error::ConstantError{
                    packet_name,
                    const_val: #num as _,
                    error: e,
                })?;
            });
        }
        "Len_u16" => {
            set.len_size = Some(Size::U16);
        }
        "Len_u32" => {
            set.len_size = Some(Size::U32);
        }
        _ => {}
    }
    Ok(())
}

fn parse_field_type(
    in_type: &Type,
    read: &mut TS2,
    write: &mut TS2,
    field_name: &Ident,
    set: &Settings,
    is_first: bool,
    no_seek: bool,
) -> syn::Result<()> {
    match in_type {
        Type::Path(path) => {
            let type_name_segment = path.path.segments.last().unwrap();
            let type_name = type_name_segment.ident.to_string();
            if !type_name.contains("Vec") {
                let (type_read, type_write) =
                    type_read_write(type_name, field_name, set, is_first)?;
                read.extend(type_read);
                write.extend(type_write);
                return Ok(());
            }

            // assume type is Vec<T>
            let PathArguments::AngleBracketed(args) = &type_name_segment.arguments else {
                return Ok(());
            };
            let GenericArgument::Type(inner_type) = &args.args[0] else {
                return Ok(());
            };
            let mut tmp_read = quote! {};
            let mut tmp_write = quote! {};
            let tmp_name = format_ident!("vec_{}_value", field_name);
            parse_field_type(
                inner_type,
                &mut tmp_read,
                &mut tmp_write,
                &tmp_name,
                set,
                false,
                no_seek,
            )?;

            let read_padding = if no_seek {
                quote! {}
            } else {
                quote! { reader.seek(std::io::SeekFrom::Current((len.next_multiple_of(4) - len) as i64))
                    .map_err(|e| Error::PaddingError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: e,
                    })?;
                }
            };
            let write_padding = if no_seek {
                quote! {}
            } else {
                quote! { writer.write_all(&vec![0u8; len.next_multiple_of(4) - len])
                    .map_err(|e| Error::PaddingError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: e,
                    })?;
                }
            };

            let mut read_len = if let Some(size) = &set.len_size {
                match size {
                    Size::U8 => quote! { reader.read_u8() },
                    Size::U16 => quote! { reader.read_u16::<LittleEndian>() },
                    Size::U32 => quote! { reader.read_u32::<LittleEndian>() },
                    Size::U64 => quote! { reader.read_u64::<LittleEndian>() },
                    Size::U128 => quote! { reader.read_u128::<LittleEndian>() },
                }
            } else {
                quote! { read_magic(reader, sub, xor)}
            };
            read_len.extend(quote! {
                .map_err(|e| Error::FieldLengthError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: e,
                })?;
            });

            let mut write_len = if let Some(size) = &set.len_size {
                match size {
                    Size::U8 => {
                        quote! { writer.write_u8(self.#field_name.len() as _) }
                    }
                    Size::U16 => {
                        quote! { writer.write_u16::<LittleEndian>(self.#field_name.len() as _) }
                    }
                    Size::U32 => {
                        quote! { writer.write_u32::<LittleEndian>(self.#field_name.len() as _) }
                    }
                    Size::U64 => {
                        quote! { writer.write_u64::<LittleEndian>(self.#field_name.len() as _) }
                    }
                    Size::U128 => {
                        quote! { writer.write_u128::<LittleEndian>(self.#field_name.len() as _) }
                    }
                }
            } else {
                quote! { writer.write_u32::<LittleEndian>(write_magic(self.#field_name.len() as _, sub, xor)) }
            };
            write_len.extend(quote! {
                .map_err(|e| Error::FieldLengthError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: e,
                })?;
            });

            if set.fixed_len == 0 {
                read.extend(quote! {
                    let len = #read_len;
                    let mut #field_name = vec![];
                    let seek1 = reader.seek(std::io::SeekFrom::Current(0))
                        .map_err(|e| Error::PaddingError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                    for _ in 0..len {
                        #tmp_read;
                        #field_name.push(#tmp_name);
                    }
                    let seek2 = reader.seek(std::io::SeekFrom::Current(0))
                        .map_err(|e| Error::PaddingError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                    let len = (seek2 - seek1) as usize;
                    #read_padding
                });
                write.extend(quote! {
                    #write_len;
                    let mut tmp_buf = vec![];
                    {
                        let writer = &mut tmp_buf;
                        for #tmp_name in &self.#field_name {
                            #tmp_write;
                        }
                    }
                    writer.write_all(&tmp_buf)
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                    let len = tmp_buf.len();
                    #write_padding
                });
            } else {
                let len = set.fixed_len;
                read.extend(quote! {
                    let mut #field_name = vec![];
                    let seek1 = reader.seek(std::io::SeekFrom::Current(0))
                        .map_err(|e| Error::PaddingError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                    for _ in 0..#len {
                        #tmp_read
                        #field_name.push(#tmp_name);
                    }
                    let seek2 = reader.seek(std::io::SeekFrom::Current(0))
                        .map_err(|e| Error::PaddingError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                    let len = (seek2 - seek1) as usize;
                    #read_padding
                });
                write.extend(quote! {
                    let mut tmp_buf = vec![];
                    {
                        let writer = &mut tmp_buf;
                        for #tmp_name in self.#field_name.iter().chain(std::iter::repeat(&Default::default())).take(#len as _) {
                            #tmp_write
                        }
                    };
                    writer.write_all(&tmp_buf)
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                    let len = tmp_buf.len();
                    #write_padding
                });
            }
        }
        Type::Array(arr) => {
            let inner_type = arr.elem.as_ref();
            let len = &arr.len;
            let mut tmp_read = quote! {};
            let mut tmp_write = quote! {};
            let tmp_name = format_ident!("array_{}_value", field_name);
            parse_field_type(
                inner_type,
                &mut tmp_read,
                &mut tmp_write,
                &tmp_name,
                set,
                false,
                no_seek,
            )?;
            if set.manual_rw.is_some() {
                read.extend(quote! {
                    #tmp_read
                    let #field_name = #tmp_name;
                });
                write.extend(quote! {
                    let #tmp_name = &self.#field_name;
                    #tmp_write
                });
            } else if tmp_read.to_string().contains("read_u8()") {
                read.extend(quote! {
                    let mut #field_name = [Default::default(); #len];
                    reader.read_exact(&mut #field_name)
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e
                        })?;
                });
                write.extend(quote! {
                    writer.write_all(&self.#field_name)
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e
                        })?;
                });
            } else {
                read.extend(quote! {
                    let mut #field_name = vec![];
                    for i in 0..#len {
                        #tmp_read
                        #field_name.push(#tmp_name);
                    }
                    let #field_name = #field_name.try_into().unwrap();
                });
                write.extend(quote! {
                    for #tmp_name in &self.#field_name {
                        #tmp_write
                    }
                });
            }
        }
        _ => {}
    }
    Ok(())
}

fn type_read_write(
    full_type_path: String,
    field_name: &Ident,
    set: &Settings,
    is_self: bool,
) -> syn::Result<(TS2, TS2)> {
    let mut read = quote! {};
    let mut write = quote! {};

    let write_name = if is_self {
        quote! {self.#field_name}
    } else {
        quote! {#field_name}
    };

    if let Some((read_fn, write_fn)) = &set.manual_rw {
        read.extend(
            quote! { let #field_name = #read_fn(reader, packet_type, xor, sub)
                .map_err(|e| Error::CompositeFieldError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: Box::new(e)
                })?;
            },
        );
        write.extend(quote! {
            #write_fn(&#write_name, writer, packet_type, xor, sub)
                .map_err(|e| Error::CompositeFieldError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: Box::new(e)
                })?;
        });
        return Ok((read, write));
    }

    let type_str = full_type_path.split("::").last().unwrap();

    match type_str {
        "u8" => {
            read.extend(quote! {let #field_name = reader.read_u8()
                .map_err(|e| Error::FieldError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: e,
                })?;
            });
            write.extend(quote! {writer.write_u8(#write_name.clone())
                .map_err(|e| Error::FieldError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: e,
                })?;
            });
        }
        "i8" => {
            read.extend(quote! {let #field_name = reader.read_i8()
                .map_err(|e| Error::FieldError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: e,
                })?;
            });
            write.extend(quote! {writer.write_i8(#write_name.clone())
                .map_err(|e| Error::FieldError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: e,
                })?;
            });
        }
        "u16" => {
            read.extend(quote! {let #field_name = reader.read_u16::<LittleEndian>()
                .map_err(|e| Error::FieldError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: e,
                })?;
            });
            write.extend(
                quote! {writer.write_u16::<LittleEndian>(#write_name.clone())
                    .map_err(|e| Error::FieldError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: e,
                    })?;
                },
            );
        }
        "i16" => {
            read.extend(quote! {let #field_name = reader.read_i16::<LittleEndian>()
                .map_err(|e| Error::FieldError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: e,
                })?;
            });
            write.extend(
                quote! {writer.write_i16::<LittleEndian>(#write_name.clone())
                    .map_err(|e| Error::FieldError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: e,
                    })?;
                },
            );
        }
        "u32" => {
            read.extend(quote! {let #field_name = reader.read_u32::<LittleEndian>()
                .map_err(|e| Error::FieldError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: e,
                })?;
            });
            write.extend(
                quote! {writer.write_u32::<LittleEndian>(#write_name.clone())
                    .map_err(|e| Error::FieldError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: e,
                    })?;
                },
            );
        }
        "i32" => {
            read.extend(quote! {let #field_name = reader.read_i32::<LittleEndian>()
                .map_err(|e| Error::FieldError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: e,
                })?;
            });
            write.extend(
                quote! {writer.write_i32::<LittleEndian>(#write_name.clone())
                    .map_err(|e| Error::FieldError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: e,
                    })?;
                },
            );
        }
        "u64" => {
            read.extend(quote! {let #field_name = reader.read_u64::<LittleEndian>()
                .map_err(|e| Error::FieldError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: e,
                })?;
            });
            write.extend(
                quote! {writer.write_u64::<LittleEndian>(#write_name.clone())
                    .map_err(|e| Error::FieldError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: e,
                    })?;
                },
            );
        }
        "i64" => {
            read.extend(quote! {let #field_name = reader.read_i64::<LittleEndian>()
                .map_err(|e| Error::FieldError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: e,
                })?;
            });
            write.extend(
                quote! {writer.write_i64::<LittleEndian>(#write_name.clone())
                    .map_err(|e| Error::FieldError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: e,
                    })?;
                },
            );
        }
        "u128" => {
            read.extend(quote! {let #field_name = reader.read_u128::<LittleEndian>()
                .map_err(|e| Error::FieldError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: e,
                })?;
            });
            write.extend(
                quote! {writer.write_u128::<LittleEndian>(#write_name.clone())
                    .map_err(|e| Error::FieldError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: e,
                    })?;
                },
            );
        }
        "i128" => {
            read.extend(quote! {let #field_name = reader.read_i128::<LittleEndian>()
                .map_err(|e| Error::FieldError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: e,
                })?;
            });
            write.extend(
                quote! {writer.write_i128::<LittleEndian>(#write_name.clone())
                    .map_err(|e| Error::FieldError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: e,
                    })?;
                },
            );
        }
        "f16" => {
            read.extend(
                quote! {let #field_name = f16::from_bits(reader.read_u16::<LittleEndian>()
                .map_err(|e| Error::FieldError{
                    packet_name,
                    field_name: stringify!(#field_name),
                    error: e,
                })?);
                },
            );
            write.extend(
                quote! {writer.write_u16::<LittleEndian>(#write_name.clone().to_bits())
                    .map_err(|e| Error::FieldError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: e,
                    })?;
                },
            );
        }
        "f32" => {
            read.extend(quote! {let #field_name = reader.read_f32::<LittleEndian>()
                    .map_err(|e| Error::FieldError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: e,
                    })?;
            });
            write.extend(
                quote! {writer.write_f32::<LittleEndian>(#write_name.clone())
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                },
            );
        }
        "f64" => {
            read.extend(quote! {let #field_name = reader.read_f64::<LittleEndian>()
                    .map_err(|e| Error::FieldError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: e,
                    })?;
            });
            write.extend(
                quote! {writer.write_f64::<LittleEndian>(#write_name.clone())
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                },
            );
        }
        "Ipv4Addr" => {
            read.extend(quote! {
                let mut ip_buf = [0u8; 4];
                reader.read_exact(&mut ip_buf)
                    .map_err(|e| Error::FieldError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: e,
                    })?;

                let #field_name = std::net::Ipv4Addr::from(ip_buf);
            });
            write.extend(quote! {
                writer.write_all(&#write_name.octets())
                    .map_err(|e| Error::FieldError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: e,
                    })?;
            });
        }
        "Duration" => {
            if set.is_psotime {
                const WIN_FT_TIME_TO_TIMESTAMP: u64 = 0x0295_E964_8864; 
                read.extend(
                    quote! {let #field_name = std::time::Duration::from_millis(reader.read_u64::<LittleEndian>()
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })? - #WIN_FT_TIME_TO_TIMESTAMP);
                    },
                );
                write.extend(
                    quote! {writer.write_u64::<LittleEndian>(#write_name.as_millis() as u64 + #WIN_FT_TIME_TO_TIMESTAMP)
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                    },
                );
            } else {
                read.extend(
                    quote! {let #field_name = core::time::Duration::from_secs(reader.read_u32::<LittleEndian>()
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })? 
                        as u64);
                    }
                );
                write.extend(
                    quote! {writer.write_u32::<LittleEndian>(#write_name.as_secs() as u32)
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                    },
                );
            }
        }
        "String" => match set.str_type {
            StringType::Unknown => {
                read.extend(quote! {let #field_name = String::read_variable(reader, sub, xor)
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                });
                write.extend(quote! {writer.write_all(&#write_name.write_variable(sub, xor))
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                });
            }
            StringType::Fixed(len) => {
                read.extend(quote! {let #field_name = String::read(reader, #len)
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                });
                write.extend(quote! {writer.write_all(&#write_name.write(#len as usize))
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                });
            }
        },
        "AsciiString" => match set.str_type {
            StringType::Unknown => {
                read.extend(
                    quote! {let #field_name = AsciiString::read_variable(reader, sub, xor)
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                    },
                );
                write.extend(quote! {writer.write_all(&#write_name.write_variable(sub, xor))
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                });
            }
            StringType::Fixed(len) => {
                read.extend(quote! {let #field_name = AsciiString::read(reader, #len)
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                });
                write.extend(quote! {writer.write_all(&#write_name.write(#len as usize))
                        .map_err(|e| Error::FieldError{
                            packet_name,
                            field_name: stringify!(#field_name),
                            error: e,
                        })?;
                });
            }
        },
        _ => {
            let out_type = Ident::new(&full_type_path, Span::call_site());
            read.extend(quote! {let #field_name = <#out_type as HelperReadWrite>::read(reader, packet_type, xor, sub)
                .map_err(|e| {
                    Error::CompositeFieldError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: Box::new(e),
                    }
                })?;
            });
            write.extend(quote! {#write_name.write(writer, packet_type, xor, sub)
                .map_err(|e| {
                    Error::CompositeFieldError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: Box::new(e),
                    }
                })?;
            });
        }
    }
    Ok((read, write))
}

fn get_packet_id(attrs: &[Attribute]) -> syn::Result<(u8, u16)> {
    let Some(attr) = attrs.iter().find(|a| a.path().is_ident("Id")) else {
        return Err(syn::Error::new(Span::call_site(), "No Id defined"));
    };
    let syn::Meta::List(list) = &attr.meta else {
        return Err(syn::Error::new(
            attr.span(),
            "Invalid syntax \nPerhaps you ment Id(id, subid)?",
        ));
    };

    let attrs: AttributeList = list.parse_args()?;
    if attrs.fields.len() != 2 {
        return Err(syn::Error::new(attr.span(), "Invalid number of arguments"));
    }
    let id = attrs.fields[0].base10_parse()?;
    let subid = attrs.fields[1].base10_parse()?;
    Ok((id, subid))
}

fn get_flags(attrs: &[Attribute]) -> syn::Result<TS2> {
    let Some(attr) = attrs.iter().find(|a| a.path().is_ident("Flags")) else {
        return Ok(quote! {Flags::default()});
    };
    let syn::Meta::List(list) = &attr.meta else {
        return Err(syn::Error::new(
            attr.span(),
            "Invalid syntax \nPerhaps you ment Flags(..)?",
        ));
    };

    let attrs = &list.tokens;
    Ok(quote! {#attrs})
}

fn get_repr(attrs: &[Attribute]) -> syn::Result<Size> {
    let Some(attr) = attrs.iter().find(|a| a.path().is_ident("repr")) else {
        return Ok(Size::U8);
    };
    let syn::Meta::List(list) = &attr.meta else {
        return Err(syn::Error::new(
            attr.span(),
            "Invalid syntax \nPerhaps you ment BitFlags(u*)?",
        ));
    };
    Ok(match list.tokens.to_string().as_str() {
        "u8" => Size::U8,
        "u16" => Size::U16,
        "u32" => Size::U32,
        "u64" => Size::U64,
        _ => return Err(syn::Error::new(list.span(), "Unsupported repr")),
    })
}

fn get_magic(attrs: &[Attribute]) -> syn::Result<Option<(u32, u32)>> {
    let Some(attr) = attrs.iter().find(|a| a.path().is_ident("Magic")) else {
        return Ok(None);
    };
    let syn::Meta::List(list) = &attr.meta else {
        return Err(syn::Error::new(
            attr.span(),
            "Invalid syntax \nPerhaps you ment Magic(xor, sub)?",
        ));
    };

    let attrs: AttributeList = list.parse_args()?;
    if attrs.fields.len() != 2 {
        return Err(syn::Error::new(attr.span(), "Invalid number of arguments"));
    }
    let xor = attrs.fields[0].base10_parse()?;
    let sub = attrs.fields[1].base10_parse()?;
    Ok(Some((xor, sub)))
}

fn get_bitflags_struct(attrs: &[Attribute]) -> syn::Result<Option<Size>> {
    let Some(attr) = attrs.iter().find(|a| a.path().is_ident("BitFlags")) else {
        return Ok(None);
    };
    let syn::Meta::List(list) = &attr.meta else {
        return Err(syn::Error::new(
            attr.span(),
            "Invalid syntax \nPerhaps you ment BitFlags(u*)?",
        ));
    };
    Ok(match list.tokens.to_string().as_str() {
        "u8" => Some(Size::U8),
        "u16" => Some(Size::U16),
        "u32" => Some(Size::U32),
        "u64" => Some(Size::U64),
        "u128" => Some(Size::U128),
        _ => None,
    })
}

fn get_flags_struct(attrs: &[Attribute]) -> syn::Result<Option<Size>> {
    let Some(attr) = attrs.iter().find(|a| a.path().is_ident("Flags")) else {
        return Ok(None);
    };
    let syn::Meta::List(list) = &attr.meta else {
        return Err(syn::Error::new(
            attr.span(),
            "Invalid syntax \nPerhaps you ment Flags(u*)?",
        ));
    };
    Ok(match list.tokens.to_string().as_str() {
        "u8" => Some(Size::U8),
        "u16" => Some(Size::U16),
        "u32" => Some(Size::U32),
        "u64" => Some(Size::U64),
        "u128" => Some(Size::U128),
        _ => None,
    })
}

fn get_no_seek(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| a.path().is_ident("NoPadding"))
}

enum Size {
    U8,
    U16,
    U32,
    U64,
    U128,
}

#[derive(Default)]
enum StringType {
    #[default]
    Unknown,
    // len
    Fixed(u64),
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

struct FnList {
    fields: Punctuated<Ident, Token![,]>,
}

impl Parse for FnList {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            fields: Punctuated::parse_separated_nonempty(input)?,
        })
    }
}

#[derive(Debug)]
enum Discriminant {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
}

impl ToTokens for Discriminant {
    fn to_tokens(&self, tokens: &mut TS2) {
        match self {
            Discriminant::U8(x) => x.to_tokens(tokens),
            Discriminant::U16(x) => x.to_tokens(tokens),
            Discriminant::U32(x) => x.to_tokens(tokens),
            Discriminant::U64(x) => x.to_tokens(tokens),
            Discriminant::U128(x) => x.to_tokens(tokens),
        }
    }
}

impl Discriminant {
    fn increase(&mut self) {
        match self {
            Discriminant::U8(x) => *x = x.overflowing_add(1).0,
            Discriminant::U16(x) => *x = x.overflowing_add(1).0,
            Discriminant::U32(x) => *x = x.overflowing_add(1).0,
            Discriminant::U64(x) => *x = x.overflowing_add(1).0,
            Discriminant::U128(x) => *x = x.overflowing_add(1).0,
        }
    }
    fn skip_flag(&mut self) {
        match self {
            Discriminant::U8(x) => *x <<= 1,
            Discriminant::U16(x) => *x <<= 1,
            Discriminant::U32(x) => *x <<= 1,
            Discriminant::U64(x) => *x <<= 1,
            Discriminant::U128(x) => *x <<= 1,
        }
    }
}
