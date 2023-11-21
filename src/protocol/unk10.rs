use super::PacketReadWrite;
use crate::AsciiString;

// ----------------------------------------------------------------
// Unknown 0x10 packets
// ----------------------------------------------------------------

// 0x10, 0x00
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x10, 0x00)]
pub struct LuaPacket {
    pub unk1: u16,
    pub unk2: u16,
    pub lua: AsciiString,
}

// 0x10, 0x03
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x10, 0x03)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xD975, 0x2F)]
pub struct Unk1003Packet {
    pub unk1: u16,
    pub unk2: u16,
    pub unk3: AsciiString,
}
