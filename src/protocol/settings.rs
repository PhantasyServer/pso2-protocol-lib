//! Settings related packets. \[0x2B\]
use super::PacketReadWrite;
use crate::AsciiString;

// ----------------------------------------------------------------
// Settings packets
// ----------------------------------------------------------------

/// (0x2B, 0x01) Save Player Settings.
///
/// (C -> S) Sent when the client wants to save the settings.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2B, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xCEF1, 0xB5)]
pub struct SaveSettingsPacket {
    /// Player settings.
    pub settings: AsciiString,
}

/// (0x2B, 0x02) Load Player Settings.
///
/// (S -> C) Sent in response to a request.
///
/// Response to: [`crate::protocol::Packet::SettingsRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2B, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x54AF, 0x100)]
pub struct LoadSettingsPacket {
    /// Player settings.
    pub settings: AsciiString,
}
