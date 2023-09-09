use super::{
    models::{character::Character, Position},
    EntityType, HelperReadWrite, ObjectHeader, PacketReadWrite,
};
use crate::AsciiString;

// ----------------------------------------------------------------
// Spawn packets
// ----------------------------------------------------------------

//0x08, 0x04
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
    pub character: Character,
    pub is_global: bool,
    #[FixedStr(0x20)]
    pub unk9: String, // title?
    pub unk10: u32,
    pub unk11: u32, // gmflag?
    #[FixedStr(0x10)]
    pub nickname: String,
    pub unk12: [u8; 0x40],
}

// 0x08, 0x09
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
    #[VariableStr(0x9FCD, 0xE7)]
    pub unk13: AsciiString,
}

// 0x08, 0x0D
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x0D)]
#[Flags(Flags {packed: true, ..Default::default()})]
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
    #[VariableStr(0x258B, 0x32)]
    pub unk10: AsciiString,
    pub unk11: u8,
    pub unk12: u8,
    pub unk13: u16,
    pub unk14: [u8; 0xC],
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum CharacterSpawnType {
    #[default]
    Myself = 47,
    Other = 39,

    #[Read_default]
    Undefined = 0xFFFF_FFFF,
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
                unk2: 0,
                entity_type: EntityType::Player,
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
            character: Character::default(),
            is_global: true,
            unk9: String::new(),
            unk10: 0,
            unk11: 0,
            nickname: String::new(),
            unk12: [0u8; 0x40],
        }
    }
}
