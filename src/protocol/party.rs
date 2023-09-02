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
    pub entries: [PartyEntry; 4],
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

// 0x0E, 0x0C
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x0C)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct NewPartySettingsPacket {
    #[VariableStr(0x11CB, 0x98)]
    pub name: String,
    #[VariableStr(0x11CB, 0x98)]
    pub password: String,
    #[VariableStr(0x11CB, 0x98)]
    pub comments: String,
    #[VariableStr(0x11CB, 0x98)]
    pub questname: String,
    pub min_level: u8,
    pub max_level: u8,
    pub playstyle: u8,
    pub flags: PartyFlags,
    pub unk: u64,
}

// 0x0E, 0x0D
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x0D)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct PartySettingsPacket {
    #[VariableStr(0x9789, 0xe3)]
    pub name: String,
    #[VariableStr(0x9789, 0xe3)]
    pub password: String,
    #[VariableStr(0x9789, 0xe3)]
    pub comments: String,
    pub min_level: u8,
    pub max_level: u8,
    pub playstyle: u8,
    pub flags: PartyFlags,
    pub unk: u64,
}

// 0x0E, 0x19
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x19)]
#[Flags(Flags {object_related: true, ..Default::default()})]
pub struct Unk0E19Packet {
    pub object: ObjectHeader,
    pub unk: u32,
}

// 0x0E, 0x2B
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x2B)]
pub struct Unk0E2BPacket {
    pub object: ObjectHeader,
    pub unk: u32,
}

// 0x0E, 0x2C
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x2C)]
pub struct InviteDeclinePacket {
    pub decline_status: u32,
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
    #[VariableStr(0xD863, 0xA9)]
    pub unk10: String,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[Flags(u8)]
pub struct PartyFlags {
    pub friends_only: bool,
    pub alliance_only: bool,
    pub limit_others: bool,
    pub unk4: bool,
    pub unk5: bool,
    pub unk6: bool,
    pub vc_focus: bool,
    pub unk8: bool,
}
