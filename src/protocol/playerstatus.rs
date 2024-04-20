//! Player status packets \[0x06\]
use super::{models::character::Class, HelperReadWrite, ObjectHeader, PacketReadWrite};
use half::f16;

// ----------------------------------------------------------------
// Player status packets
// ----------------------------------------------------------------

/// (0x06, 0x00) Set Player ID.
///
/// (S -> C) Sent during map loading.
///
/// Following: [`crate::protocol::Packet::LoadLevel`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x06, 0x00)]
pub struct SetPlayerIDPacket {
    pub player_id: u32,
    pub unk1: u32,
    pub unk2: u32,
}

/// (0x06, 0x01) Deal damage to an object.
///
/// (C -> S) Sent when the client wants to deal damage.
///
/// Respond with: [`crate::protocol::Packet::DamageReceive`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x06, 0x01)]
pub struct DealDamagePacket {
    /// Object that inflicted the damage.
    pub inflicter: ObjectHeader,
    /// Object that received the damage.
    pub target: ObjectHeader,
    pub attack_id: u32,
    pub unk2: u64,
    /// Hitbox ID (?).
    pub hitbox_id: u32,
    /// Hit x position.
    pub x_pos: f16,
    /// Hit y position.
    pub y_pos: f16,
    /// Hit z position.
    pub z_pos: f16,

    pub unk4: u16,
    pub unk5: u64,
    pub unk6: [u8; 0x18],
}

/// (0x06, 0x05) EXP Gained. (broadcast)
///
/// (S -> C) Sent when the players earn EXP.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x06, 0x05)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x7C49, 0x9E)]
pub struct GainedEXPPacket {
    /// Packet receiver.
    pub sender: ObjectHeader,
    /// All players that gained EXP.
    pub receivers: Vec<EXPReceiver>,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

/// Information about EXP receiving player.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct EXPReceiver {
    /// Player that received EXP.
    pub object: ObjectHeader,
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: [u8; 6],
    /// How much EXP did the main class earn.
    pub gained: u64,
    /// Main class total EXP.
    pub total: u64,
    /// New sublevel(?) of the main class.
    pub level2: u16,
    /// New level of the main class.
    pub level: u16,
    /// Main class.
    pub class: Class,
    pub pad1: [u8; 3],
    /// How much EXP did the subclass earn.
    pub gained_sub: u64,
    /// Subclass total EXP.
    pub total_sub: u64,
    /// New sublevel(?) of the subclass.
    pub level2_sub: u16,
    /// New level of the subclass.
    pub level_sub: u16,
    /// Subclass.
    pub subclass: Class,
    pub pad2: [u8; 3],
}
