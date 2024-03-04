use super::{items::Item, ObjectHeader, PacketReadWrite};

// ----------------------------------------------------------------
// Player shop packets
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
#[Flags(Flags {packed: true, ..Default::default()})]
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
    #[FixedLen(0x100)]
    pub unk13: Vec<u8>,
    pub unk14: u32,
    // 0x174 on classic
    #[FixedLen(0x300)]
    pub character_data: Vec<u8>,
    /// Item used as a weapon.
    pub weapon: Item,
    /// Other items (e.g. camos, outfit).
    #[FixedLen(0x9)]
    pub other_items: Vec<Item>,
    pub unk18: u32,
    pub unk19: u32,
}
