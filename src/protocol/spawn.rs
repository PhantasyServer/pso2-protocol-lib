use super::{
    models::{character::Character, Position},
    HelperReadWrite, ObjectHeader, ObjectType, PacketReadWrite,
};
use crate::AsciiString;

// ----------------------------------------------------------------
// Spawn packets
// ----------------------------------------------------------------

//0x08, 0x04
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x04)]
pub struct CharacterSpawnPacket {
    // unsure about real structure
    pub player_obj: ObjectHeader,
    pub position: Position,
    pub unk1: u16, // padding?
    #[FixedStr(0x20)]
    pub unk2: AsciiString,
    pub unk3: u16,
    pub unk4: u16,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub is_me: CharacterSpawnType,
    pub unk9: u8,
    pub unk10: u16,
    pub character: Character,
    pub unk11: u32,
    pub gm_flag: u32,
    #[FixedStr(0x10)]
    pub nickname: String,
    pub unk12_1: [u8; 0x20],
    #[SeekAfter(0x60)]
    pub unk12_2: [u8; 0x20],
}

// #[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite, Default)]
#[Id(0x08, 0x04)]
pub struct CharacterSpawnNGSPacket {
    // unsure about real structure
    pub player_obj: ObjectHeader,
    pub position: Position,
    pub unk1: u16, // padding?
    #[FixedStr(0x20)]
    pub unk2: AsciiString,
    pub unk3: u16,
    pub unk4: u16,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub is_me: CharacterSpawnType,
    pub unk9: u8,
    pub unk10: u16,
    #[FixedLen(0x63C)]
    pub character: Vec<u8>,
    pub unk11: u32,
    pub gm_flag: u32,
    #[FixedStr(0x10)]
    pub nickname: String,
    pub unk12_1: [u8; 0x20],
    #[SeekAfter(0x60)]
    pub unk12_2: [u8; 0x20],
}

// 0x08, 0x05
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x05)]
pub struct TransporterSpawnPacket {
    pub object: ObjectHeader,
    pub position: Position,
    pub unk1: u16,
    #[FixedStr(0x20)]
    pub name: AsciiString,
    pub unk2: u32,
    pub unk3: u16,
    pub unk4: u16,
    pub unk5: u16,
    pub unk6: u16,
    pub unk7: u32,
    pub unk8: u32,
}

// 0x08, 0x09
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x09)]
pub struct EventSpawnPacket {
    pub object: ObjectHeader,
    pub position: Position,
    pub unk1: u16,
    #[FixedStr(0x20)]
    pub name: AsciiString,
    pub unk3: u32,
    pub unk4: [u8; 0xC],
    pub unk5: u16,
    pub unk6: u16,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
    pub unk14: u32,
    pub flags: u32,
    #[Len_u32]
    pub data: Vec<u32>,
}

// 0x08, 0x0B
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x0B)]
pub struct ObjectSpawnPacket {
    pub object: ObjectHeader,
    pub position: Position,
    pub unk1: u16,
    #[FixedStr(0x20)]
    pub name: AsciiString,
    pub unk2: [u32; 5],
    pub flags: u32,
    #[Len_u32]
    pub data: Vec<u32>,
}

// 0x08, 0x0C
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x0C)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x9FCD, 0xE7)]
pub struct NPCSpawnPacket {
    pub object: ObjectHeader,
    pub position: Position,
    pub unk1: u16,
    #[FixedStr(0x20)]
    pub name: AsciiString,
    pub unk2: u32,
    pub unk3: [u8; 0xC],
    pub unk4: u16,
    pub unk5: u16,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: AsciiString,
}

// 0x08, 0x0D
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x0D)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x258B, 0x32)]
pub struct EnemySpawnPacket {
    pub object: ObjectHeader,
    pub position: Position,
    pub unk1: u16,
    #[FixedStr(0x20)]
    pub name: AsciiString,
    pub unk2: u32,
    pub hp: u32,
    pub unk4: u32,
    pub level: u32,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u16,
    pub unk8: u16,
    pub unk9: [u32; 16],
    pub unk10: AsciiString,
    pub unk11: u8,
    pub unk12: u8,
    pub unk13: u16,
    pub unk14: [u8; 0xC],
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u8)]
pub enum CharacterSpawnType {
    #[default]
    Myself = 47,
    Other = 39,

    #[Read_default]
    Undefined = 0xFF,
}

// ----------------------------------------------------------------
// Default implementations
// ----------------------------------------------------------------

impl Default for CharacterSpawnPacket {
    fn default() -> Self {
        Self {
            player_obj: ObjectHeader {
                id: 0,
                unk: 0,
                map_id: 0,
                entity_type: ObjectType::Player,
            },
            position: Position {
                rot_x: half::f16::from_bits(0),
                rot_y: half::f16::from_bits(15360),
                rot_z: half::f16::from_bits(0),
                rot_w: half::f16::from_bits(0),
                pos_x: half::f16::from_bits(14892),
                pos_y: half::f16::from_bits(0),
                pos_z: half::f16::from_bits(22589),
            },
            unk1: 0,
            unk2: "Character".into(),
            unk3: 1,
            unk4: 0,
            unk5: 602,
            unk6: 1,
            unk7: 53,
            unk8: 0,
            is_me: CharacterSpawnType::Myself,
            unk9: 0,
            unk10: 0,
            character: Character::default(),
            unk11: 0,
            gm_flag: 0,
            nickname: String::new(),
            unk12_1: [0u8; 0x20],
            unk12_2: [0u8; 0x20],
        }
    }
}
