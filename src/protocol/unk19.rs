//! Unknown \[0x19\] packets.
use super::{HelperReadWrite, PacketReadWrite};
use crate::AsciiString;
use std::time::Duration;

// ----------------------------------------------------------------
// Unknown 0x19 packets
// ----------------------------------------------------------------

/// (0x19, 0x01) System Message (broadcast).
///
/// (S -> C) Sent when an important server message needs to be broadcast
/// (e.g. ship going for maintenance).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x19, 0x01)]
#[Flags(Flags::PACKED)]
#[Magic(0x78F7, 0xA2)]
pub struct SystemMessagePacket {
    /// Message to be broadcast.
    pub message: String,
    pub unk: String,
    /// Message type.
    pub msg_type: MessageType,
    pub msg_num: u32,
}

/// (0x19, 0x09) Set Lobby Event (broadcast).
///
/// (S -> C) Sent when an new lobby event has started/ended (e.g. emergency quest).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x19, 0x09)]
#[Flags(Flags::PACKED)]
#[Magic(0xA6E4, 0xFB)]
pub struct SetLobbyEventPacket {
    /// Event string ID.
    pub event_name: AsciiString,
    /// Voice line string ID.
    pub voice_line: AsciiString,
    /// Event start timestamp.
    pub start_time: Duration,
    /// Event end timestamp.
    pub end_time: Duration,
    /// Event message repeat timer in seconds.
    pub repeat_secs: u32,
    pub unk4: u64,
}

/// (0x19, 0x0F) Set Lobby Monitor Video (broadcast).
///
/// (S -> C) Sent when an new lobby video must be played or on logon.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x19, 0x0F)]
pub struct LobbyMonitorPacket {
    /// Video ID to play.
    pub video_id: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

/// System message type.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
pub enum MessageType {
    /// Admin message (must wait a few seconds before dismissal).
    AdminMessage = 1,
    /// Admin message (can be instantly dismissed).
    AdminMessageInstant,
    #[default]
    SystemMessage,
    GoldenMessage,
    EventInformationYellow,
    EventInformationGreen,
    ImportantMessage,
    PopupMessage,

    #[Read_default]
    Undefined = 0xFFFF_FFFF,
}
