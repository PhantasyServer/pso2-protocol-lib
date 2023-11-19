use super::{models::character::Class, HelperReadWrite, ObjectHeader, PacketReadWrite};
use half::f16;

// ----------------------------------------------------------------
// Player status packets
// ----------------------------------------------------------------

// 0x06, 0x00
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x06, 0x00)]
pub struct SetPlayerIDPacket {
    pub player_id: u32,
    pub unk1: u32,
    pub unk2: u32,
}

// 0x06, 0x01
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x06, 0x01)]
pub struct DealDamagePacket {
    pub inflicter: ObjectHeader,
    pub target: ObjectHeader,
    pub attack_id: u32,
    pub unk2: u64,
    pub hitbox_id: u32,
    pub x_pos: f16,
    pub y_pos: f16,
    pub z_pos: f16,

    pub unk4: u16,
    pub unk5: u64,
    pub unk6: [u8; 0x18],
}

// 0x06, 0x05
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x06, 0x05)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct GainedEXPPacket {
    pub sender: ObjectHeader,
    #[Magic(0x7C49, 0x9E)]
    pub receivers: Vec<EXPReceiver>,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct EXPReceiver {
    pub object: ObjectHeader,
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: [u8; 6],
    pub gained: u64,
    pub total: u64,
    pub level2: u16,
    pub level: u16,
    pub class: Class,
    pub pad1: [u8; 3],
    pub gained_sub: u64,
    pub total_sub: u64,
    pub level2_sub: u16,
    pub level_sub: u16,
    pub subclass: Class,
    pub pad2: [u8; 3],
}
