//! Chat related packets. \[0x07\]
use super::{HelperReadWrite, ObjectHeader, PacketReadWrite};

// ----------------------------------------------------------------
// Chat packets
// ----------------------------------------------------------------

/// (0x07, 0x00) Chat Message.
///
/// (Bidirectional) Sent when players send chat messages.
///
/// Response to: [`crate::protocol::Packet::ChatMessage`] (C->S)
/// Respond with: [`crate::protocol::Packet::ChatMessage`] (S->C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x07, 0x00)]
#[Flags(Flags::PACKED | Flags::OBJECT_RELATED)]
#[Magic(0x9D3F, 0x44)]
pub struct ChatMessage {
    /// Sender of the message.
    pub object: ObjectHeader,
    /// Message channel.
    pub channel: MessageChannel,
    pub unk3: u8,
    pub unk4: u16,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(super::PacketType::NGS)]
    pub unk5: u16,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(super::PacketType::NGS)]
    pub unk6: u16,
    pub unk7: String,
    /// Message.
    pub message: String,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

/// Possible message channels.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
pub enum MessageChannel {
    #[default]
    /// Map channel.
    Map,
    /// Party channel.
    Party,
    // the following is only speculation
    /// Alliance channel. (?)
    Alliance,
    /// Whisper channel. (?)
    Whisper,
    /// Group channel. (?)
    Group,

    #[Read_default]
    Undefined = 0xFF,
}
