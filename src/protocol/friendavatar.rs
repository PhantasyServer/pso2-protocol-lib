//! Friend avatar related packets. \[0x26\]
use crate::fixed_types::{FixedBytes, FixedVec};

use super::{items::Item, ObjectHeader, PacketReadWrite};

// ----------------------------------------------------------------
// Friend Avatar packets
// ----------------------------------------------------------------

/// (0x26, 0x00) Friend Avatar Data Request.
///
/// (C -> S) Sent when the client wants to get information about oneself's friend avatar.
///
/// Respond with: [`crate::protocol::Packet::FriendAvatarDataResponse`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x26, 0x00)]
pub struct FriendAvatarDataRequestPacket {
    pub unk1: u32,
    pub unk2: u32,
}

/// (0x26, 0x08) Friend Avatar Data Response.
///
/// (S -> C) Sent in response to the data request.
///
/// Response to: [`crate::protocol::Packet::FriendAvatarDataRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x26, 0x08)]
#[Flags(Flags::PACKED)]
#[Magic(0xC2CD, 0x2F)]
pub struct FriendAvatarDataResponsePacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: ObjectHeader,
    pub unk7: ObjectHeader,
    /// Name of the character
    pub name: String,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: FixedBytes<0x100>,
    pub unk14: u32,
    // 0x174 on classic
    pub character_data: FixedBytes<0x300>,
    /// Item used as a weapon.
    pub weapon: Item,
    /// Other items (e.g. camos, outfit).
    pub other_items: FixedVec<9, Item>,
    pub unk18: u32,
    pub unk19: u32,
}
