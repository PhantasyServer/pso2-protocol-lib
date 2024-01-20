mod packet_structs;
mod protocol_enum;
use packet_structs::{helper_deriver, packet_deriver};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TS2;
use protocol_enum::protocol_deriver;
use quote::ToTokens;
use syn::{parse_macro_input, DeriveInput};

// Be warned that most of the variable names are nonsensical.

#[proc_macro_derive(
    PacketReadWrite,
    attributes(
        Id, Seek, SeekAfter, Const_u16, FixedStr, Flags, PSOTime, Magic, Len_u32, Len_u16, OnlyOn,
        FixedLen, NotOn
    )
)]
pub fn packet_read_write_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    packet_deriver(&input).unwrap_or_else(|err| err.to_compile_error().into())
}

#[proc_macro_derive(
    HelperReadWrite,
    attributes(
        Seek,
        SeekAfter,
        Const_u16,
        FixedStr,
        PSOTime,
        Magic,
        Len_u32,
        Len_u16,
        Read_default,
        Skip,
        Flags,
        OnlyOn,
        FixedLen,
        NoPadding,
        NotOn,
        ManualRW,
    )
)]
pub fn helper_read_write_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    helper_deriver(&input).unwrap_or_else(|err| output_error(input.to_token_stream(), err))
}

#[proc_macro_derive(
    ProtocolReadWrite,
    attributes(Id, Empty, Unknown, NGS, Classic, NA, JP, Vita, Category)
)]
pub fn protocol_read_write_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    protocol_deriver(&input).unwrap_or_else(|err| output_error(input.to_token_stream(), err))
}

fn output_error(mut input: TS2, err: syn::Error) -> TokenStream {
    input.extend(err.to_compile_error());
    input.into()
}
