//! Mail related packets. \[0x1A\]
use super::{HelperReadWrite, PacketReadWrite};
use std::time::Duration;

// ----------------------------------------------------------------
// Mail packets
// ----------------------------------------------------------------

/// (0x1A, 0x00) Mail List Request.
///
/// (C -> S) Sent when the client wants to get a list of mail messages.
///
/// Respond with: [`crate::protocol::Packet::MailList`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1A, 0x00)]
pub struct MailListRequestPacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

/// (0x1A, 0x01) Mail List.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::MailListRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1A, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x36A1, 0xBF)]
pub struct MailListPacket {
    pub unk1: u16,
    pub unk2: u16,
    pub unk3: u16,
    pub unk4: u16,
    pub unk5: [u8; 4],
    pub unk6: u32,
    /// Player name (?).
    pub name: String,
    /// Character name (?).
    pub nickname: String,
    /// Mail headers.
    pub headers: Vec<MailHeader>,
}

/// (0x1A, 0x02) Delete Mail Request.
///
/// (C -> S) Sent when the client wants to delete mail messages.
///
/// Respond with: [`crate::protocol::Packet::DeletedMail`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1A, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xBC5F, 0x0B)]
pub struct DeleteMailRequestPacket {
    /// IDs of messages to delete.
    pub ids: Vec<MailId>,
}

/// (0x1A, 0x03) Deleted Mail.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::DeleteMailRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1A, 0x03)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x421C, 0x56)]
pub struct DeletedMailPacket {
    /// Deleted messages IDs.
    pub ids: Vec<MailId>,
    pub unk: u32,
}

/// (0x1A, 0x06) Mail Body Request.
///
/// (C -> S) Sent when the client wants to get a body of mail.
///
/// Respond with: [`crate::protocol::Packet::MailBody`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1A, 0x06)]
pub struct MailBodyRequestPacket {
    /// Message ID.
    pub id: MailId,
}

/// (0x1A, 0x07) Mail Body.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::MailBodyRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1A, 0x07)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x5913, 0x82)]
pub struct MailBodyPacket {
    /// Message ID.
    pub id: MailId,
    /// Message body.
    pub message: String,
    pub unk3: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

/// Mail ID with extra information.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct MailId {
    /// Mail ID.
    pub mail_id: u32,
    pub unk1: u32,
    pub unk2: u32,
}

/// Mail header.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct MailHeader {
    /// Mail ID.
    pub mail_id: u32,
    pub unk2: u32,
    /// Sender player ID (?).
    pub user_id: u32,
    pub unk3: [u8; 0x14],
    pub unk4: u32,
    pub unk5: u32,
    /// Mail receive timestamp.
    pub receive_time: Duration,
    pub unk6: u32,
    /// Sender name.
    #[FixedLen(0x22)]
    pub sender: String,
    /// Mail subject.
    #[FixedLen(0x2A)]
    pub subject: String,
}
