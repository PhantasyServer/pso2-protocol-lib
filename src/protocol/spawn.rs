//! Spawn packets \[0x08\]
use super::{
    models::{character::Character, Position},
    HelperReadWrite, ObjectHeader, ObjectType, PacketReadWrite,
};
use crate::AsciiString;

// ----------------------------------------------------------------
// Spawn packets
// ----------------------------------------------------------------

/// (0x08, 0x04) Spawn Character. (broadcast)
///
/// (S -> C) Sent when a new character is spawned.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x04)]
pub struct CharacterSpawnPacket {
    // unsure about real structure
    /// Spawned character's player object.
    pub player_obj: ObjectHeader,
    /// Object position.
    pub position: Position,
    pub unk1: u16, // padding?
    /// Always `Character`. (?)
    #[FixedLen(0x20)]
    pub unk2: AsciiString,
    pub unk3: u16,
    pub unk4: u16,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    /// Character spawn type.
    pub spawn_type: CharacterSpawnType,
    pub unk9: u8,
    pub unk10: u16,
    /// Character data.
    pub character: Character,
    pub unk11: u32,
    /// Set to `1` if the player is a GM.
    pub gm_flag: u32,
    /// Player's nickname.
    #[FixedLen(0x10)]
    pub nickname: String,
    #[SeekAfter(0x60)]
    #[FixedLen(0x40)]
    pub unk12: Vec<u8>,
}

/// (0x08, 0x04) Spawn Character. (broadcast) (NGS)
///
/// (S -> C) Sent when a new character is spawned.
#[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite, Default)]
#[Id(0x08, 0x04)]
pub struct CharacterSpawnNGSPacket {
    // unsure about real structure
    /// Spawned character's player object.
    pub player_obj: ObjectHeader,
    /// Object position.
    pub position: Position,
    pub unk1: u16, // padding?
    /// Always `Character`. (?)
    #[FixedLen(0x20)]
    pub unk2: AsciiString,
    pub unk3: u16,
    pub unk4: u16,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    /// Character spawn type.
    pub spawn_type: CharacterSpawnType,
    pub unk9: u8,
    pub unk10: u16,
    /// Character data.
    #[FixedLen(0x63C)]
    pub character: Vec<u8>,
    pub unk11: u32,
    pub gm_flag: u32,
    /// Player's nickname.
    #[FixedLen(0x10)]
    pub nickname: String,
    #[FixedLen(0x40)]
    pub unk12: Vec<u8>,
    #[SeekAfter(0x60)]
    pub unk13: u64,
}

/// (0x08, 0x05) Spawn Transporter.
///
/// (S -> C) Sent to spawn a new transporter. (only campship telepool?)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x05)]
pub struct TransporterSpawnPacket {
    /// Spawned object header.
    pub object: ObjectHeader,
    /// Spawned object position.
    pub position: Position,
    pub unk1: u16,
    /// Object name.
    #[FixedLen(0x20)]
    pub name: AsciiString,
    pub unk2: u32,
    pub unk3: u16,
    pub unk4: u16,
    pub unk5: u16,
    pub unk6: u16,
    pub unk7: u32,
    pub unk8: u32,
}

/// (0x08, 0x09) Spawn Event.
///
/// (S -> C) Sent to spawn a new event (e.g. camera lock).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x09)]
pub struct EventSpawnPacket {
    /// Spawned event header.
    pub object: ObjectHeader,
    /// Spawned event position.
    pub position: Position,
    pub unk1: u16,
    /// Event name.
    #[FixedLen(0x20)]
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
    /// Event data.
    #[Len_u32]
    pub data: Vec<u32>,
}

/// (0x08, 0x0B) Spawn Object.
///
/// (S -> C) Sent to spawn a new object.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x0B)]
pub struct ObjectSpawnPacket {
    /// Spawned object header.
    pub object: ObjectHeader,
    /// Spawned object position.
    pub position: Position,
    pub unk1: u16,
    /// Object name.
    #[FixedLen(0x20)]
    pub name: AsciiString,
    pub unk2: [u32; 5],
    /// Object flags. (?)
    pub flags: u32,
    /// Object data.
    #[Len_u32]
    pub data: Vec<u32>,
}

/// (0x08, 0x0C) Spawn NPC.
///
/// (S -> C) Sent to spawn a new NPC.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x0C)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x9FCD, 0xE7)]
pub struct NPCSpawnPacket {
    /// Spawned NPC object.
    pub object: ObjectHeader,
    /// Spawned NPC position.
    pub position: Position,
    pub unk1: u16,
    /// NPC name.
    #[FixedLen(0x20)]
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

/// (0x08, 0x0D) Spawn Enemy.
///
/// (S -> C) Sent when a new enemy is spawned.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x0D)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x258B, 0x32)]
pub struct EnemySpawnPacket {
    /// Spawned enemy object.
    pub object: ObjectHeader,
    /// Spawned enemy position.
    pub position: Position,
    pub unk1: u16,
    /// Enemy name.
    #[FixedLen(0x20)]
    pub name: AsciiString,
    pub unk2: u32,
    /// Enemy HP.
    pub hp: u32,
    pub unk4: u32,
    /// Enemy level.
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

/// Character spawn type.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u8)]
pub enum CharacterSpawnType {
    /// Spawned character is not related to the receiver.
    Other = 0x27,
    /// Spawned character is related to the receiver.
    #[default]
    Myself = 0x2F,

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
            spawn_type: CharacterSpawnType::Myself,
            unk9: 0,
            unk10: 0,
            character: Character::default(),
            unk11: 0,
            gm_flag: 0,
            nickname: String::new(),
            unk12: vec![0; 0x40],
        }
    }
}
