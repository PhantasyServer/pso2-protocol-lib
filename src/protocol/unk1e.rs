//! Unknown \[0x1E\] packets.
use super::PacketReadWrite;

// ----------------------------------------------------------------
// Unknown 0x1E packets
// ----------------------------------------------------------------

/// (0x1E, 0x0C) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1E, 0x0C)]
pub struct Unk1E0CPacket {
    pub unk: u32,
}
