use super::{HelperReadWrite, ObjectHeader, PacketReadWrite};
use crate::AsciiString;

// ----------------------------------------------------------------
// Party packets
// ----------------------------------------------------------------

// 0x0E, 0x02
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct PartyInitPacket {
    pub unk1: [u8; 0xC],
    pub receiver: ObjectHeader,
    pub people_amount: u32,
    pub entries: [PartyEntryNGS; 4],
    #[VariableStr(0xD863, 0xA9)]
    pub unk2: AsciiString,
}

#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct PartyInitNGSPacket {
    pub unk1: [u8; 0xC],
    pub receiver: ObjectHeader,
    pub people_amount: u32,
    pub entries: [PartyEntryNGS; 4],
    #[VariableStr(0xD863, 0xA9)]
    pub unk2: AsciiString,
}

// 0x0E, 0x2B
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x2B)]
pub struct Unk0E2BPacket {
    pub object: ObjectHeader,
    pub unk1: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[derive(Debug, Clone, Default, PartialEq, HelperReadWrite)]
pub struct PartyEntry {
    pub id: ObjectHeader,
    #[VariableStr(0xD863, 0xA9)]
    pub name: String,
    #[VariableStr(0xD863, 0xA9)]
    pub nickname: String,
    pub level: u8,
    pub unk1: [u8; 11],
    pub unk2: [u32; 4],
    pub unk3: [u16; 2],
    pub unk4: [u8; 0xC],
    pub unk5: [u32; 3],
    #[VariableStr(0xD863, 0xA9)]
    pub unk6: String,
    #[VariableStr(0xD863, 0xA9)]
    pub unk7: AsciiString,
    pub unk8: u8,
    pub unk9: [u8; 3],
}

#[derive(Debug, Clone, Default, PartialEq, HelperReadWrite)]
pub struct PartyEntryNGS {
    pub id: ObjectHeader,
    #[VariableStr(0xD863, 0xA9)]
    pub name: String,
    #[VariableStr(0xD863, 0xA9)]
    pub nickname: String,
    pub level: u8,
    pub unk1: [u8; 11],
    pub unk2: [u32; 4],
    pub unk3: [u16; 2],
    pub unk4: [u8; 0xC],
    pub unk5: [u32; 3],
    #[VariableStr(0xD863, 0xA9)]
    pub unk6: String,
    #[VariableStr(0xD863, 0xA9)]
    pub unk7: AsciiString,
    pub unk8: u8,
    pub unk9: [u8; 3],
    // i'm unsure about what type of string this is
    #[VariableStr(0xD863, 0xA9)]
    pub unk10: String,
}
