//! Player shop related packets. \[0x2D\]
use crate::fixed_types::{FixedBytes, FixedVec};

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
#[Flags(Flags::PACKED)]
#[Magic(0xD003, 0x3B)]
pub struct ProductSearchRequestPacket {
    pub unk1: u16,
    pub unk2: u8,
    pub unk3: u8,
    /// Item's ID name.
    pub item_name: String,
    pub unk5: u64,
    pub unk6: u64,
    pub unk7: FixedBytes<12>,
    pub unk8: FixedBytes<0x10>,
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
#[Flags(Flags::PACKED)]
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
#[Flags(Flags::PACKED)]
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

/// (0x2D, 0x0D) Character Search Request.
///
/// (C -> S) Sent when the client searches for a character.
///
/// Respond with: [`crate::protocol::Packet::CharacterSearchResponse`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2D, 0x0D)]
#[Flags(Flags::PACKED)]
#[Magic(0x8F2A, 0x75)]
pub struct CharacterSearchRequestPacket {
    /// Searched character name.
    pub char_name: String,
}

/// (0x2D, 0x0E) Character Search Response.
///
/// (S -> C) Sent in response to a search request.
///
/// Response to: [`crate::protocol::Packet::CharacterSearchRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2D, 0x0E)]
#[Flags(Flags::PACKED)]
#[Magic(0x14E7, 0xC0)]
pub struct CharacterSearchResponsePacket {
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u16,
    pub characters: FixedVec<0x32, CharacterSearchEntry>,
}

/// (0x2D, 0x12) Recruiting Alliances List Request.
///
/// (C -> S) Sent when the client wants the list of recruiting alliances.
///
/// Respond with: [`crate::protocol::Packet::RecruitingAlliancesResponse`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2D, 0x12)]
pub struct RecruitingAlliancesRequestPacket {
    pub unk: u64,
}

/// (0x2D, 0x13) Recruiting Alliances List Response.
///
/// (S -> C) Sent in response to a list request.
///
/// Response to: [`crate::protocol::Packet::RecruitingAlliancesRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2D, 0x13)]
#[Flags(Flags::PACKED)]
#[Magic(0xB19C, 0x38)]
pub struct RecruitingAlliancesResponsePacket {
    pub unk1: u32,
    pub unk2: u16,
    pub unk3: u16,
    pub alliances: FixedVec<0x64, RecruitingAlliance>,
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

/// Character entry in a character search results.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct CharacterSearchEntry {
    /// Player object.
    pub player: ObjectHeader,
    /// Player's username.
    pub username: String,
    /// Character's name.
    pub char_name: String,
}

/// Recruiting alliance entry.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct RecruitingAlliance {
    pub unk1: u32,
    /// Name of the alliance.
    pub alliance_name: String,
    /// Number of members.
    pub members: u8,
    /// Alliance level.
    pub level: u8,
    pub unk5: u8,
    pub unk6: u8,
    /// Alliance flag UUID.
    pub symbol_art_uuid: u128,
    /// Alliance leader comments.
    pub comment: String,
    pub unk8: u32,
}
