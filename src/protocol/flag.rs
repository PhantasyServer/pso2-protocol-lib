use super::{HelperReadWrite, ObjectHeader, PacketReadWrite};
use crate::AsciiString;

// ----------------------------------------------------------------
// Flag packets
// ----------------------------------------------------------------

// 0x23, 0x02
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x02)]
pub struct SetFlagPacket {
    pub flag_type: FlagType,
    pub id: u32,
    pub value: u32,
}

// 0x23, 0x04
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x04)]
pub struct ServerSetFlagPacket {
    pub flag_type: FlagType,
    pub id: u32,
    pub value: u32,
    pub unk: u32,
}

// 0x23, 0x05
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x05)]
pub struct ServerSetParamPacket {
    pub param_type: FlagType,
    pub id: u32,
    pub value: u32,
}

// 0x23, 0x06
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x06)]
pub struct AccountFlagsPacket {
    #[FixedLen(0x400)]
    pub flags: Vec<u8>,
    #[FixedLen(0x100)]
    pub params: Vec<u32>,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(super::PacketType::NGS)]
    #[FixedLen(0x400)]
    pub unk: Vec<u8>,
}

// 0x23, 0x07
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x07)]
pub struct CharacterFlagsPacket {
    #[FixedLen(0xC00)]
    pub flags: Vec<u8>,
    #[FixedLen(0x100)]
    pub params: Vec<u32>,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(super::PacketType::NGS)]
    #[FixedLen(0xF40)]
    pub unk: Vec<u8>,
}

// 0x23, 0x0A
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x0A)]
pub struct CutsceneEndPacket {
    #[FixedStr(0x20)]
    pub skit_name: AsciiString,
    pub emergency_obj: ObjectHeader,
    pub unk2: u32,
    pub unk3: u32,
}

// 0x23, 0x0B
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x0B)]
pub struct SkitItemAddRequestPacket {
    #[FixedStr(0x20)]
    pub skit_name: AsciiString,
    pub unk: u32,
}

// 0x23, 0x0C
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x0C)]
pub struct SkitItemAddResponsePacket {
    #[FixedStr(0x20)]
    pub skit_name: AsciiString,
    pub unk: u32,
}

// 0x23, 0x0D
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x0D)]
pub struct Unk230DPacket {
    pub unk: u32,
}

// 0x23, 0x0E
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x0E)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xAC40, 0x99)]
pub struct Unk230EPacket {
    pub unk: Vec<Unk230EThing>,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
pub struct Unk230EThing {
    pub unk1: u16,
    pub unk2: u16,
    pub unk3: u32,
    pub unk4: ObjectHeader,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum FlagType {
    #[default]
    #[Read_default]
    Account,
    Character,
}
