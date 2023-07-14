mod packet_structs;
mod protocol_enum;
use packet_structs::{helper_deriver, packet_deriver};
use proc_macro::TokenStream;
use protocol_enum::protocol_deriver;
use syn::{parse_macro_input, DeriveInput};

// Be warned that most of the variable names are nonsensical.

#[proc_macro_derive(
    PacketReadWrite,
    attributes(
        Id,
        Seek,
        SeekAfter,
        Const_u16,
        FixedAscii,
        FixedUtf16,
        VariableAscii,
        VariableUtf16,
        Flags,
        PSOTime,
        Magic,
        Len_u32,
    )
)]
pub fn packet_read_write_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    packet_deriver(&input)
        .unwrap_or_else(|err| err.to_compile_error().into())
        .into()
}

#[proc_macro_derive(
    HelperReadWrite,
    attributes(
        Seek,
        SeekAfter,
        Const_u16,
        FixedAscii,
        FixedUtf16,
        VariableAscii,
        VariableUtf16,
        PSOTime,
        Magic,
        Len_u32,
        Read_default,
    )
)]
pub fn helper_read_write_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    helper_deriver(&input)
        .unwrap_or_else(|err| err.to_compile_error().into())
        .into()
}

#[proc_macro_derive(ProtocolReadWrite, attributes(Id, Empty, Unknown, Base, NGS,))]
pub fn protocol_read_write_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    protocol_deriver(&input)
        .unwrap_or_else(|err| err.to_compile_error().into())
        .into()
}
