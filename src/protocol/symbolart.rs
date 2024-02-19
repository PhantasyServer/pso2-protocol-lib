use super::{HelperReadWrite, MessageChannel, ObjectHeader, PacketReadWrite};

// ----------------------------------------------------------------
// Symbol Art packets
// ----------------------------------------------------------------

// 0x2F, 0x00
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x00)]
pub struct SymbolArtClientDataRequestPacket {
    pub uuid: u128,
}

// 0x2F, 0x01
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x01)]
pub struct SymbolArtDataRequestPacket {
    pub uuid: u128,
}

// 0x2F, 0x02
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x4B58, 0x76)]
pub struct SymbolArtDataPacket {
    pub uuid: u128,
    pub data: Vec<u8>,
    pub name: String,
}

// 0x2F, 0x03
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x03)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xD116, 0xC1)]
pub struct SymbolArtClientDataPacket {
    pub uuid: u128,
    pub data: Vec<u8>,
}

// 0x2F, 0x04
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x04)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x56D3, 0x0C)]
pub struct ChangeSymbolArtPacket {
    pub uuids: Vec<SlottedSymbolArt>,
}

// 0x2F, 0x05
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x05)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xDC91, 0x57)]
pub struct SymbolArtResultPacket {
    pub unk1: u32,
    pub uuids: Vec<u128>,
}

// 0x2F, 0x07
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x07)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xE80C, 0xED)]
pub struct SymbolArtListPacket {
    pub object: ObjectHeader,
    pub character_id: u32,
    pub uuids: Vec<u128>,
}

// 0x2F, 0x08
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x08)]
#[Flags(Flags {object_related: true, ..Default::default()})]
pub struct SendSymbolArtPacket {
    pub object: ObjectHeader,
    pub uuid: u128,
    pub area: MessageChannel,
    pub unk1: u8,
    pub unk2: u16,
    pub unk3: u32,
}

// 0x2F, 0x09
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x09)]
pub struct ReceiveSymbolArtPacket {
    pub object: ObjectHeader,
    pub uuid: u128,
    pub area: MessageChannel,
    pub unk1: u8,
    pub unk2: u16,
    pub unk3: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
pub struct SlottedSymbolArt {
    pub uuid: u128,
    pub slot: u32,
}
