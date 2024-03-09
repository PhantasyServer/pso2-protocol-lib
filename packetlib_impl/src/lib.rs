mod packet_structs;
mod protocol_enum;
use packet_structs::{helper_deriver, packet_deriver};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use protocol_enum::protocol_deriver;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

// Be warned that most of the variable names are nonsensical.

// Internal derive.
#[proc_macro_derive(
    PacketReadWrite,
    attributes(
        Id, Seek, SeekAfter, Const_u16, Flags, PSOTime, Magic, Len_u16, Len_u32, FixedLen, OnlyOn,
        NotOn
    )
)]
pub fn packet_read_write_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    packet_deriver(&input, true).unwrap_or_else(|err| err.to_compile_error().into())
}

// Public derive.
#[proc_macro_derive(
    PacketRW,
    attributes(
        Id, Seek, SeekAfter, Const_u16, Flags, PSOTime, Magic, Len_u16, Len_u32, FixedLen, OnlyOn,
        NotOn
    )
)]
pub fn pub_packet_read_write_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    packet_deriver(&input, false).unwrap_or_else(|err| err.to_compile_error().into())
}

// Internal derive.
#[proc_macro_derive(
    HelperReadWrite,
    attributes(
        Seek,
        SeekAfter,
        Const_u16,
        PSOTime,
        Len_u16,
        Len_u32,
        FixedLen,
        Read_default,
        Skip,
        Flags,
        NoPadding,
        ManualRW,
        OnlyOn,
        NotOn,
    )
)]
pub fn helper_read_write_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    helper_deriver(&input, true).unwrap_or_else(|err| output_error(input.to_token_stream(), err))
}

// External derive.
#[proc_macro_derive(
    HelperRW,
    attributes(
        Seek,
        SeekAfter,
        Const_u16,
        PSOTime,
        Len_u16,
        Len_u32,
        FixedLen,
        Read_default,
        Skip,
        Flags,
        NoPadding,
        ManualRW,
        OnlyOn,
        NotOn,
    )
)]
pub fn pub_helper_read_write_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    helper_deriver(&input, false).unwrap_or_else(|err| output_error(input.to_token_stream(), err))
}

// Internal derive.
#[proc_macro_derive(
    ProtocolReadWrite,
    attributes(Id, Empty, Raw, Unknown, NGS, Classic, NA, JP, Vita, Category)
)]
pub fn protocol_read_write_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    protocol_deriver(&input, true).unwrap_or_else(|err| output_error(input.to_token_stream(), err))
}

// Public derive.
#[proc_macro_derive(
    ProtocolRW,
    attributes(Id, Empty, Raw, Unknown, NGS, Classic, NA, JP, Vita, Category)
)]
pub fn pub_protocol_read_write_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    protocol_deriver(&input, false).unwrap_or_else(|err| output_error(input.to_token_stream(), err))
}

fn output_error(mut input: TS2, err: syn::Error) -> TokenStream {
    input.extend(err.to_compile_error());
    input.into()
}
