use super::{ChatArea, HelperReadWrite, ObjectHeader, PacketReadWrite};

// ----------------------------------------------------------------
// Symbol Art packets
// ----------------------------------------------------------------

// 0x2F, 0x00
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x00)]
pub struct SymbolArtClientDataRequestPacket {
    pub uuid: u128,
}

// 0x2F, 0x01
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x01)]
pub struct SymbolArtDataRequestPacket {
    pub uuid: u128,
}

// 0x2F, 0x02
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SymbolArtDataPacket {
    pub uuid: u128,
    #[Magic(0x4B58, 0x76)]
    pub data: Vec<u8>,
    #[VariableStr(0x4B58, 0x76)]
    pub name: String,
}

// 0x2F, 0x03
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x03)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SymbolArtClientDataPacket {
    pub uuid: u128,
    #[Magic(0xD116, 0xC1)]
    pub data: Vec<u8>,
}

// 0x2F, 0x04
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x04)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct ChangeSymbolArtPacket {
    #[Magic(0x56D3, 0x0C)]
    pub uuids: Vec<SlottedSymbolArt>,
}

// 0x2F, 0x05
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x05)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SymbolArtResultPacket {
    pub unk1: u32,
    #[Magic(0xDC91, 0x57)]
    pub uuids: Vec<u128>,
}

// 0x2F, 0x07
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x07)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SymbolArtListPacket {
    pub object: ObjectHeader,
    pub character_id: u32,
    #[Magic(0xE80C, 0xED)]
    pub uuids: Vec<u128>,
}

// 0x2F, 0x08
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x08)]
pub struct SendSymbolArtPacket {
    pub object: ObjectHeader,
    pub uuid: u128,
    pub area: ChatArea,
    pub unk1: u8,
    pub unk2: u16,
    pub unk3: u32,
}

// 0x2F, 0x09
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x09)]
pub struct ReceiveSymbolArtPacket {
    pub object: ObjectHeader,
    pub uuid: u128,
    pub area: ChatArea,
    pub unk1: u8,
    pub unk2: u16,
    pub unk3: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
pub struct SlottedSymbolArt {
    pub uuid: u128,
    pub slot: u32,
}
