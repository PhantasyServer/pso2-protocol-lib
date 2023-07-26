use super::{HelperReadWrite, ObjectHeader, PacketReadWrite};
use crate::AsciiString;

// ----------------------------------------------------------------
// Client order packets
// ----------------------------------------------------------------

// 0x1F, 0x01
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1F, 0x01)]
pub struct TakenOrdersRequestPacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
}

// 0x1F, 0x02
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1F, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct OrderListRequestPacket {
    pub unk1: u32,
    #[VariableStr(0x70B2, 0x9E)]
    pub source: AsciiString,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
}

// 0x1F, 0x03
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x1F, 0x03)]
pub struct OrderListPacket {
    pub user: ObjectHeader,
    pub orders: [ClientOrder; 100],
    pub unk1: u32,
    pub unk2: u32,
}

// 0x1F, 0x08
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x1F, 0x08)]
pub struct TakenOrdersPacket {
    pub user: ObjectHeader,
    pub orders: [ClientOrder; 50],
    pub statues: [OrderStatus; 50],
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[derive(Debug, Copy, Clone, Default, PartialEq, HelperReadWrite)]
pub struct ClientOrder {
    pub unk1: u32,
    pub id: u32,
    pub status: u32,
    pub finish_date: u32,
}

#[derive(Debug, Copy, Clone, Default, PartialEq, HelperReadWrite)]
pub struct OrderStatus {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
}
// ----------------------------------------------------------------
// Default implementations
// ----------------------------------------------------------------

impl Default for OrderListPacket {
    fn default() -> Self {
        Self {
            user: Default::default(),
            orders: [Default::default(); 100],
            unk1: 0,
            unk2: 0,
        }
    }
}
