//! Daily order related packets. \[0x1F\]
use super::{HelperReadWrite, ObjectHeader, PacketReadWrite};
use crate::AsciiString;

// ----------------------------------------------------------------
// Client order packets
// ----------------------------------------------------------------

/// (0x1F, 0x01) Taken Daily Orders Request.
///
/// (C -> S) Sent when a player wants to get a list of taken daily orders.
///
/// Respond with: [`crate::protocol::Packet::TakenOrders`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1F, 0x01)]
pub struct TakenOrdersRequestPacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
}

/// (0x1F, 0x02) Daily Orders Request.
///
/// (C -> S) Sent when a player wants to get a list of daily orders.
///
/// Respond with: [`crate::protocol::Packet::OrderList`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1F, 0x02)]
#[Flags(Flags::PACKED)]
#[Magic(0x70B2, 0x9E)]
pub struct OrderListRequestPacket {
    pub unk1: u32,
    /// Daily order list source ID.
    pub source: AsciiString,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
}

/// (0x1F, 0x03) Taken Daily Orders.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::OrderListRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x1F, 0x03)]
pub struct OrderListPacket {
    /// Player object.
    pub user: ObjectHeader,
    /// Daily orders.
    #[FixedLen(100)]
    pub orders: Vec<ClientOrder>,
    pub unk1: u32,
    pub unk2: u32,
}

/// (0x1F, 0x08) Taken Daily Orders.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::TakenOrdersRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x1F, 0x08)]
pub struct TakenOrdersPacket {
    /// Player object.
    pub user: ObjectHeader,
    /// Taken orders.
    #[FixedLen(50)]
    pub orders: Vec<ClientOrder>,
    /// Taken orders status.
    #[FixedLen(50)]
    pub statues: Vec<OrderStatus>,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

/// Daily order definition.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Copy, Clone, Default, PartialEq, HelperReadWrite)]
pub struct ClientOrder {
    pub unk1: u32,
    /// Order ID.
    pub id: u32,
    /// Order status.
    pub status: u32,
    /// Last order finish date.
    pub finish_date: u32,
}

/// Taken daily order status.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Copy, Clone, Default, PartialEq, HelperReadWrite)]
pub struct OrderStatus {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
}
