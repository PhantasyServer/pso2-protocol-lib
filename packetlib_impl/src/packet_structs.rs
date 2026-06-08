use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TS2};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::Parse, punctuated::Punctuated, spanned::Spanned, Attribute, Data, DataEnum, DataStruct,
    Expr, Fields, Ident, Lit, LitInt, MetaList, Token, Type,
};

pub fn packet_deriver(ast: &syn::DeriveInput, is_internal: bool) -> syn::Result<TokenStream> {
    let name = &ast.ident;
    let mut set = ContainerSettings::default();
    get_attr_iter(
        &mut set,
        &ast.attrs,
        &mut quote! {},
        &mut quote! {},
        get_container_attrs,
    )?;

    let Some((id, subid)) = set.id else {
        return Err(syn::Error::new(ast.ident.span(), "No packet id provided"));
    };
    let xor_sub = set.magic;
    let flags = set.flags.unwrap_or_else(|| quote! {Flags::default()});
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
        parse_struct_field(&mut read, &mut write, data)?;
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
            ) -> Vec<u8> {
                use #crate_location::derive_reexports::*;
                use #crate_location::protocol::PacketError as Error;
                let packet_name = stringify!(#name);

                let mut buf = PacketHeader::new(#id, #subid, #flags).write(packet_type);
                let writer = &mut buf;
                let (xor, sub) = (#xor, #sub);
                #write
                buf
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
    let mut set = ContainerHelperSettings::default();
    get_attr_iter(
        &mut set,
        &ast.attrs,
        &mut quote! {},
        &mut quote! {},
        get_helper_container_attrs,
    )?;
    let is_bitflags = set.bitflags_ty;

    let crate_location = if is_internal {
        quote! {crate}
    } else {
        quote! {pso2packetlib}
    };

    match &ast.data {
        Data::Struct(_) if is_bitflags => parse_bitflags(&mut read, &mut write)?,
        Data::Struct(data) => parse_struct_field(&mut read, &mut write, data)?,
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
                writer: &mut Vec<u8>,
                packet_type: #crate_location::protocol::PacketType,
                xor: u32,
                sub: u32
            ) {
                use #crate_location::derive_reexports::*;
                use #crate_location::protocol::PacketError as Error;
                let packet_name = stringify!(#name);

                #write
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
            write.extend(quote! {writer.extend_from_slice(&(*self as u8).to_le_bytes());});
            Discriminant::U8(0)
        }
        Size::U16 => {
            read.extend(quote! {let num = reader.read_u16::<LittleEndian>()});
            write.extend(quote! {writer.extend_from_slice(&(*self as u16).to_le_bytes());});
            Discriminant::U16(0)
        }
        Size::U32 => {
            read.extend(quote! {let num = reader.read_u32::<LittleEndian>()});
            write.extend(quote! {writer.extend_from_slice(&(*self as u32).to_le_bytes());});
            Discriminant::U32(0)
        }
        Size::U64 => {
            read.extend(quote! {let num = reader.read_u64::<LittleEndian>()});
            write.extend(quote! {writer.extend_from_slice(&(*self as u64).to_le_bytes());});
            Discriminant::U64(0)
        }
        Size::U128 => {
            read.extend(quote! {let num = reader.read_u128::<LittleEndian>()});
            write.extend(quote! {writer.extend_from_slice(&(*self as u128).to_le_bytes());});
            Discriminant::U128(0)
        }
    };
    read.extend(quote! {.map_err(|e| Error::ValueError{
            packet_name,
            error: e,
        })?;
    });

    for variant in &data.variants {
        let variant_name = &variant.ident;
        let mut settings = EnumSettings::default();

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

        get_attr_iter(&mut settings, &variant.attrs, read, write, get_enum_attrs)?;

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

fn parse_bitflags(read: &mut TS2, write: &mut TS2) -> syn::Result<()> {
    read.extend(quote! {Ok(Self::from_bits_truncate(HelperReadWrite::read(reader, packet_type, xor, sub)?))});
    write.extend(quote! {writer.extend_from_slice(&self.bits().to_le_bytes());});

    Ok(())
}

fn parse_struct_field(read: &mut TS2, write: &mut TS2, data: &DataStruct) -> syn::Result<()> {
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
            )?;
        }
        read.extend(quote! {Ok(Self(#return_token))});
        return Ok(());
    }

    for field in &data.fields {
        let field_name = field.ident.as_ref().unwrap();
        return_token.extend(quote! {#field_name,});

        let mut settings = Settings::default();

        get_attr_iter(&mut settings, &field.attrs, read, write, get_attrs)?;

        let mut tmp_read = quote! {};
        let mut tmp_write = quote! {};

        parse_field_type(
            &field.ty,
            &mut tmp_read,
            &mut tmp_write,
            field_name,
            &settings,
            true,
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
            write.extend(quote! {writer.extend_from_slice(&[0u8; #seek_after as usize]);});
        }
    }
    read.extend(quote! {Ok(Self{#return_token})});
    Ok(())
}

fn get_attr_iter<S>(
    set: &mut S,
    attrs: &[syn::Attribute],
    read: &mut TS2,
    write: &mut TS2,
    cb: fn(&mut S, &str, Option<&MetaList>, Span, &mut TS2, &mut TS2) -> syn::Result<()>,
) -> syn::Result<()> {
    for attr in attrs {
        match &attr.meta {
            syn::Meta::NameValue(_) => {}
            syn::Meta::Path(path) => {
                let string = path.get_ident().unwrap().to_string();
                get_attr_stub(set, &string, None, path.span(), read, write, cb)?;
            }
            syn::Meta::List(list) => {
                let string = list.path.get_ident().unwrap().to_string();
                get_attr_stub(set, &string, Some(list), list.span(), read, write, cb)?;
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
    read: &mut TS2,
    write: &mut TS2,
    cb: fn(&mut S, &str, Option<&MetaList>, Span, &mut TS2, &mut TS2) -> syn::Result<()>,
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
                let span = path.span();
                cb(set, &string, None, span, read, write)?;
            }
            syn::Meta::List(list) => {
                let string = list.path.get_ident().unwrap().to_string();
                let span = list.path.span();
                cb(set, &string, Some(list), span, read, write)?;
            }
        }
    }
    Ok(())
}

#[derive(Default)]
struct Settings {
    seek_after: i64,
    only_on: Option<TS2>,
    not_on: Option<TS2>,
    manual_rw: Option<(TS2, TS2)>,
}

#[derive(Default)]
struct ContainerSettings {
    id: Option<(u8, u16)>,
    flags: Option<TS2>,
    magic: Option<(u32, u32)>,
}

#[derive(Default)]
struct ContainerHelperSettings {
    bitflags_ty: bool,
}

#[derive(Default)]
struct EnumSettings {
    is_default: bool,
}

fn get_attrs(
    set: &mut Settings,
    string: &str,
    list: Option<&MetaList>,
    span: Span,
    read: &mut TS2,
    write: &mut TS2,
) -> syn::Result<()> {
    match string {
        "only_on" => {
            let Some(attrs) = list.map(|l| l.tokens.clone()) else {
                return Err(syn::Error::new(
                    span,
                    "Invalid syntax \nPerhaps you ment only_on(..)?",
                ));
            };
            set.only_on = Some(attrs);
        }
        "not_on" => {
            let Some(attrs) = list.map(|l| l.tokens.clone()) else {
                return Err(syn::Error::new(
                    span,
                    "Invalid syntax \nPerhaps you ment not_on(..)?",
                ));
            };
            set.not_on = Some(attrs);
        }
        "manual_rw" => {
            let attrs: FnList = list.unwrap().parse_args()?;
            set.manual_rw = Some((
                attrs.fields[0].clone().into_token_stream(),
                attrs.fields[1].clone().into_token_stream(),
            ));
        }
        "seek" => {
            let amount: i64 = list.unwrap().parse_args::<LitInt>()?.base10_parse()?;
            read.extend(quote! {reader.seek(std::io::SeekFrom::Current(#amount))
                .map_err(|e| Error::PaddingError{
                    packet_name,
                    field_name: "unknown",
                    error: e,
                })?;
            });
            write.extend(quote! {writer.extend_from_slice(&[0u8; #amount as usize]);});
        }
        "seek_after" => {
            set.seek_after = list.unwrap().parse_args::<LitInt>()?.base10_parse()?;
        }
        "const_u16" => {
            let num: u16 = list.unwrap().parse_args::<LitInt>()?.base10_parse()?;
            read.extend(quote! {reader.seek(std::io::SeekFrom::Current(2))
                .map_err(|e| Error::ConstantError{
                    packet_name,
                    const_val: #num as _,
                    error: e,
                })?;
            });
            write.extend(quote! {writer.extend_from_slice(&#num.to_le_bytes());});
        }
        _ => return Err(syn::Error::new(span, "Unknown attribute")),
    }
    Ok(())
}

fn get_enum_attrs(
    set: &mut EnumSettings,
    string: &str,
    _: Option<&MetaList>,
    span: Span,
    _: &mut TS2,
    _: &mut TS2,
) -> syn::Result<()> {
    match string {
        "read_default" => set.is_default = true,
        _ => return Err(syn::Error::new(span, "Unknown attribute")),
    }
    Ok(())
}

fn get_container_attrs(
    set: &mut ContainerSettings,
    string: &str,
    list: Option<&MetaList>,
    span: Span,
    _: &mut TS2,
    _: &mut TS2,
) -> syn::Result<()> {
    let Some(list) = list else {
        return Err(syn::Error::new(span, "Invalid syntax"));
    };
    match string {
        "id" => {
            let attrs: AttributeList = list.parse_args()?;

            if attrs.fields.len() != 2 {
                return Err(syn::Error::new(span, "Invalid number of arguments"));
            }
            let id = attrs.fields[0].base10_parse()?;
            let subid = attrs.fields[1].base10_parse()?;
            set.id = Some((id, subid));
        }
        "flags" => {
            let attrs = &list.tokens;
            set.flags = Some(quote! {#attrs});
        }
        "magic" => {
            let attrs: AttributeList = list.parse_args()?;
            if attrs.fields.len() != 2 {
                return Err(syn::Error::new(span, "Invalid number of arguments"));
            }
            let xor = attrs.fields[0].base10_parse()?;
            let sub = attrs.fields[1].base10_parse()?;
            set.magic = Some((xor, sub));
        }
        _ => return Err(syn::Error::new(span, "Unknown attribute")),
    }
    Ok(())
}

fn get_helper_container_attrs(
    set: &mut ContainerHelperSettings,
    string: &str,
    _: Option<&MetaList>,
    span: Span,
    _: &mut TS2,
    _: &mut TS2,
) -> syn::Result<()> {
    match string {
        "bitflags" => set.bitflags_ty = true,
        _ => return Err(syn::Error::new(span, "Unknown attribute")),
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
) -> syn::Result<()> {
    let (type_read, type_write) = type_read_write(
        in_type.to_token_stream().to_string(),
        field_name,
        set,
        is_first,
    )?;
    read.extend(type_read);
    write.extend(type_write);
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
            #write_fn(&#write_name, writer, packet_type, xor, sub);
        });
        return Ok((read, write));
    }

    let out_type = TS2::from_str(&full_type_path)?;
    read.extend(quote! {let #field_name = <#out_type as HelperReadWrite>::read(reader, packet_type, xor, sub)
                .map_err(|e| {
                    Error::CompositeFieldError{
                        packet_name,
                        field_name: stringify!(#field_name),
                        error: Box::new(e),
                    }
                })?;
            });
    write.extend(quote! {#write_name.write(writer, packet_type, xor, sub);});
    Ok((read, write))
}

fn get_repr(attrs: &[Attribute]) -> syn::Result<Size> {
    let Some(attr) = attrs.iter().find(|a| a.path().is_ident("repr")) else {
        return Ok(Size::U8);
    };
    let syn::Meta::List(list) = &attr.meta else {
        return Err(syn::Error::new(
            attr.span(),
            "Invalid syntax \nPerhaps you ment repr(u*)?",
        ));
    };
    Ok(Size::from_string(&list.tokens.to_string())
        .ok_or_else(|| syn::Error::new(list.span(), "Unsupported repr"))?)
}

enum Size {
    U8,
    U16,
    U32,
    U64,
    U128,
}

impl Size {
    fn from_string(str: &str) -> Option<Self> {
        match str {
            "u8" => Some(Self::U8),
            "u16" => Some(Self::U16),
            "u32" => Some(Self::U32),
            "u64" => Some(Self::U64),
            "u128" => Some(Self::U128),
            _ => None,
        }
    }
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
}
