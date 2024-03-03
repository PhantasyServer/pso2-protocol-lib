use super::{items::Item, HelperReadWrite, ObjectHeader, PacketReadWrite};

// ----------------------------------------------------------------
// Player shop packets
// ----------------------------------------------------------------

/// (0x2D, 0x02) Player Shop Item Search Request.
///
/// (C -> S) Sent when the client wants to get item listings for selected item.
///
/// Respond with: [`crate::protocol::Packet::ProductSearchResponse`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2D, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xD003, 0x3B)]
pub struct ProductSearchRequestPacket {
    pub unk1: u16,
    pub unk2: u8,
    pub unk3: u8,
    /// Item's ID name.
    pub item_name: String,
    pub unk5: u64,
    pub unk6: u64,
    #[FixedLen(12)]
    pub unk7: Vec<u8>,
    #[FixedLen(0x10)]
    pub unk8: Vec<u8>,
}

/// (0x2D, 0x03) Player Shop Item Search Response.
///
/// (S -> C) Contains currently available listings of an item.
///
/// Response to: [`crate::protocol::Packet::ProductSearchRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2D, 0x03)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x55C1, 0x86)]
pub struct ProductSearchResponsePacket {
    /// All listings of the queried item.
    pub items: Vec<SoldItem>,
}

/// (0x2D, 0x0B) Player Shop Details Request.
///
/// (C -> S) Sent when the client wants the details of a player's shop.
///
/// Respond with: [`crate::protocol::Packet::PlayerShopDetailsResponse`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2D, 0x0B)]
pub struct PlayerShopDetailsRequestPacket {
    /// Queried shop's owner.
    pub owner: ObjectHeader,
}

/// (0x2D, 0x0C) Player Shop Details Response.
///
/// (S -> C) Sent in response to the details request.
///
/// Response to: [`crate::protocol::Packet::PlayerShopDetailsRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2D, 0x0C)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x096C, 0x2A)]
pub struct PlayerShopDetailsResponsePacket {
    /// Queried shop's owner.
    pub owner: ObjectHeader,
    /// Player's character name. (?)
    pub char_name: String,
    /// Player's username. (?)
    pub username: String,
    /// Shop's advertisement.
    pub ad: String,
    /// Symbol Art's UUID (if any).
    pub symbol_art_uuid: u128,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

/// Item listed in a player shop.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct SoldItem {
    /// Player that is selling the item.
    pub seller: ObjectHeader,
    /// Item's UUID.
    pub uuid: u64,
    /// Item data.
    pub item: Item,
    /// Amount of items sold.
    pub amount: u32,
    /// Price of an item.
    pub price: u64,
}
