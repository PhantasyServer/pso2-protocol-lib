//! Unknown \[0x1C\] (alliance/character) packets.
use super::PacketReadWrite;

// ----------------------------------------------------------------
// Unknown 0x1C packets
// ----------------------------------------------------------------

/// (0x1C, 0x1E) Unknown. (C -> S)
///
/// Sent by the classic client during lobby load, just before [`super::Packet::Unk1C46`].
/// Recovered from the 2026-09-10 classic spike capture; fields are not yet named.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x1C, 0x1E)]
pub struct Unk1C1EPacket {
    pub unk1: u32,
    pub unk2: u32,
}

/// (0x1C, 0x46) Unknown. (C -> S)
///
/// Sent by the classic client during lobby load, immediately after [`super::Packet::Unk1C1E`].
/// Recovered from the 2026-09-10 classic spike capture; fields are not yet named.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x1C, 0x46)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct Unk1C46Packet {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}
