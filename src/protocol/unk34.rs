//! Unknown \[0x34\] packets.
use super::{items::Item, HelperReadWrite, PacketReadWrite};

// ----------------------------------------------------------------
// Unknown 0x34 packets
// ----------------------------------------------------------------

/// (0x34, 0x35) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x34, 0x35)]
#[Flags(Flags::PACKED)]
#[Magic(0xA475, 0x100)]
pub struct Unk3435Packet {
    pub unk1: u32,
    pub unk2: Vec<Unk3435_1>,
}

/// (0x34, 0x5C) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x34, 0x5C)]
pub struct Unk345CPacket {
    pub unk: u32,
}

/// (0x34, 0x71) Player Shop Top Items List Response.
///
/// (S -> C) Sent in response to a list request.
///
/// Response to: [`crate::protocol::Packet::PlayerShopListRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x34, 0x71)]
#[Flags(Flags::PACKED)]
#[Magic(0xFCE8, 0x9B)]
pub struct PlayerShopListResponsePacket {
    pub unk1: u32,
    pub items: Vec<TopItem>,
    pub unk2: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk3435_1 {
    #[FixedLen(0xC)]
    pub unk: Vec<u8>,
}

/// Item listed in a player shop top items list.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct TopItem {
    /// Item data.
    pub item: Item,
    pub unk1: u8,
    /// Item's rank.
    pub rank: u8,
    pub unk2: u16,
    /// Amount of items sold.
    pub amount: u32,
    /// Max price of an item.
    pub max_price: u64,
    /// Average price of an item.
    pub average_price: u64,
}
