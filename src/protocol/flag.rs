//! Flag related packets. \[0x23\]
use super::{HelperReadWrite, ObjectHeader, PacketReadWrite};
use crate::fixed_types::{FixedAsciiString, FixedBytes, FixedVec};

// ----------------------------------------------------------------
// Flag packets
// ----------------------------------------------------------------

/// (0x23, 0x02) Set Flag.
///
/// (C -> S) Sent when a client sets any flag.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x02)]
pub struct SetFlagPacket {
    /// Flag type.
    pub flag_type: FlagType,
    /// Flag ID.
    pub id: u32,
    /// Flag value.
    pub value: u32,
}

/// (0x23, 0x04) Server Set Flag.
///
/// (S -> C) Sent when a server sets any flag for a client.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x04)]
pub struct ServerSetFlagPacket {
    /// Flag type.
    pub flag_type: FlagType,
    /// Flag ID.
    pub id: u32,
    /// Flag value.
    pub value: u32,
    pub unk: u32,
}

/// (0x23, 0x05) Server Set Parameter.
///
/// (S -> C) Sent when a server sets any flag parameter for a client.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x05)]
pub struct ServerSetParamPacket {
    /// Parameter type.
    pub param_type: FlagType,
    /// Parameter ID.
    pub id: u32,
    /// Parameter value.
    pub value: u32,
}

/// (0x23, 0x06) Load Account Flags.
///
/// (S -> C) Sent when a client starts the game.
///
/// Response to: [`crate::protocol::Packet::StartGame`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x06)]
pub struct AccountFlagsPacket {
    /// Account flags.
    pub flags: FixedBytes<0x400>,
    /// Account parameters.
    pub params: FixedVec<0x100, u32>,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(super::PacketType::NGS)]
    pub unk: FixedBytes<0x400>,
}

/// (0x23, 0x07) Load Character Flags.
///
/// (S -> C) Sent when a client starts the game.
///
/// Response to: [`crate::protocol::Packet::StartGame`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x07)]
pub struct CharacterFlagsPacket {
    /// Character flags.
    pub flags: FixedBytes<0xC00>,
    /// Character parameters.
    pub params: FixedVec<0x100, u32>,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(super::PacketType::NGS)]
    pub unk: FixedBytes<0xF40>,
}

/// (0x23, 0x0A) Cutscene Ended.
///
/// (C -> S) Sent when a cutscene ends.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x0A)]
pub struct CutsceneEndPacket {
    /// Cuscene ID.
    pub skit_name: FixedAsciiString<0x20>,
    /// Emergency object (if related).
    pub emergency_obj: ObjectHeader,
    pub unk2: u32,
    pub unk3: u32,
}

/// (0x23, 0x0B) Skit Item Add Request.
///
/// (C -> S) Sent when a client wants to receive an item for a skit (cutscene) or to notify the
/// server of some event.
///
/// Respond with: [`crate::protocol::Packet::SkitItemAddResponse`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x0B)]
pub struct SkitItemAddRequestPacket {
    /// Skit ID.
    pub skit_name: FixedAsciiString<0x20>,
    pub unk: u32,
}

/// (0x23, 0x0C) Skit Item Add Response.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::SkitItemAddRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x0C)]
pub struct SkitItemAddResponsePacket {
    /// Skit ID.
    pub skit_name: FixedAsciiString<0x20>,
    pub unk: u32,
}

/// (0x23, 0x0D) Unknown
///
/// (C -> S)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x0D)]
pub struct Unk230DPacket {
    pub unk: u32,
}

/// (0x23, 0x0E) Unknown
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x0E)]
#[Flags(Flags::PACKED)]
#[Magic(0xAC40, 0x99)]
pub struct Unk230EPacket {
    pub unk: Vec<Unk230EThing>,
}

/// (0x23, 0x15) Unknown
///
/// (S -> C)
///
/// Response to: [`crate::protocol::Packet::StartGame`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x15)]
pub struct Unk2315Packet {
    pub unk: FixedBytes<0x1800>,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(super::PacketType::NGS)]
    pub unk2: FixedBytes<0x1E80>,
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

/// Flag type.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum FlagType {
    /// Flag is account related.
    #[default]
    #[Read_default]
    Account,
    /// Flag is character related.
    Character,
}
