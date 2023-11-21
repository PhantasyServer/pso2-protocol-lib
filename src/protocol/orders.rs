use super::{HelperReadWrite, ObjectHeader, PacketReadWrite};
use crate::AsciiString;

// ----------------------------------------------------------------
// Client order packets
// ----------------------------------------------------------------

// 0x1F, 0x01
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

// 0x1F, 0x02
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1F, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x70B2, 0x9E)]
pub struct OrderListRequestPacket {
    pub unk1: u32,
    pub source: AsciiString,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
}

// 0x1F, 0x03
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x1F, 0x03)]
pub struct OrderListPacket {
    pub user: ObjectHeader,
    #[FixedLen(100)]
    pub orders: Vec<ClientOrder>,
    pub unk1: u32,
    pub unk2: u32,
}

// 0x1F, 0x08
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x1F, 0x08)]
pub struct TakenOrdersPacket {
    pub user: ObjectHeader,
    #[FixedLen(50)]
    pub orders: Vec<ClientOrder>,
    #[FixedLen(50)]
    pub statues: Vec<OrderStatus>,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Copy, Clone, Default, PartialEq, HelperReadWrite)]
pub struct ClientOrder {
    pub unk1: u32,
    pub id: u32,
    pub status: u32,
    pub finish_date: u32,
}

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
