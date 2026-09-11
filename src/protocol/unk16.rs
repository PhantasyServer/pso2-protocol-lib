//! Unknown \[0x16\] packets.
use super::PacketReadWrite;

// ----------------------------------------------------------------
// Unknown 0x16 packets
// ----------------------------------------------------------------

/// (0x16, 0x07) Unknown. (C -> S)
///
/// Sent by the classic client during initial login, right after the character is loaded
/// (paired with [`super::Packet::Unk182E`]). `unk1` looks like a handle/id, `unk2` is always 1.
/// Recovered from the 2026-09-10 classic spike capture; fields are not yet named.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x16, 0x07)]
#[Flags(Flags::PACKED)]
#[Magic(0, 0)]
pub struct Unk1607Packet {
    pub unk1: u32,
    pub unk2: u32,
}
