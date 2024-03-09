use super::{HelperReadWrite, PacketReadWrite};
use std::time::Duration;

// ----------------------------------------------------------------
// Mail packets
// ----------------------------------------------------------------

// 0x1A, 0x00
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1A, 0x00)]
pub struct MailListRequestPacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

// 0x1A, 0x01
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
    pub name: String,
    pub nickname: String,
    pub headers: Vec<MailHeader>,
}

// 0x1A, 0x02
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1A, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xBC5F, 0x0B)]
pub struct DeleteMailRequestPacket {
    pub ids: Vec<MailId>,
}

// 0x1A, 0x03
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1A, 0x03)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x421C, 0x56)]
pub struct DeletedMailPacket {
    pub ids: Vec<MailId>,
    pub unk: u32,
}

// 0x1A, 0x06
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1A, 0x06)]
pub struct MailBodyRequestPacket {
    pub id: MailId,
}

// 0x1A, 0x07
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1A, 0x07)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x5913, 0x82)]
pub struct MailBodyPacket {
    pub id: MailId,
    pub message: String,
    pub unk3: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct MailId {
    pub mail_id: u32,
    pub unk1: u32,
    pub unk2: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct MailHeader {
    pub mail_id: u32,
    pub unk2: u32,
    pub user_id: u32,
    pub unk3: [u8; 0x14],
    pub unk4: u32,
    pub unk5: u32,
    pub receive_time: Duration,
    pub unk6: u32,
    #[FixedLen(0x22)]
    pub sender: String,
    #[FixedLen(0x2A)]
    pub subject: String,
}
