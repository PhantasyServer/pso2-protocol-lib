use super::{HelperReadWrite, PacketReadWrite};
use crate::AsciiString;
use std::time::Duration;

// ----------------------------------------------------------------
// Unknown 0x19 packets
// ----------------------------------------------------------------

// 0x19, 0x01
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x19, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x78F7, 0xA2)]
pub struct SystemMessagePacket {
    pub message: String,
    pub unk: String,
    pub msg_type: MessageType,
    pub msg_num: u32,
}

// 0x19, 0x09
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x19, 0x09)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xA6E4, 0xFB)]
pub struct SetLobbyEventPacket {
    pub event_name: AsciiString,
    pub voice_line: AsciiString,
    pub start_time: Duration,
    pub end_time: Duration,
    //repeat secs?
    pub unk3: u32,
    pub unk4: u64,
}

// 0x19, 0x0F
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x19, 0x0F)]
pub struct LobbyMonitorPacket {
    pub video_id: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u32)]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
pub enum MessageType {
    AdminMessage = 1,
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
