//! PSE Burst packets. \[0x1B\]
use super::{HelperReadWrite, PacketReadWrite};

// ----------------------------------------------------------------
// Unknown 0x1B packets
// ----------------------------------------------------------------

/// (0x1B, 0x00) Start PSE (broadcast).
///
/// (S -> C) Sent by the server when a PSE is started (e.g. after an enemy is killed there
/// is a chance for a PSE). Client display is delayed.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1B, 0x00)]
pub struct PseStartPacket {
    /// ID of the PSE.
    pub pse_id: u32,
    /// Level of the PSE, zero based.
    pub level: u32,
    pub unk1: u32,
    pub unk2: u32,
}

/// (0x1B, 0x01) End PSE (broadcast).
///
/// (S -> C) Sent by the server once a PSE lapses.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1B, 0x01)]
pub struct PseEndPacket {
    pub pse_id: u32,
}

/// (0x1B, 0x04) Set PSE Level (broadcast).
///
/// (S -> C) Sent by the server when a PSE level changes. Client display is delayed.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1B, 0x04)]
pub struct SetPseLevelPacket {
    /// ID of the PSE.
    pub pse_id: u32,
    /// Level of the PSE, zero based.
    pub level: u32,
    /// If this effect triggers a PSE burst, this contains the bursts timer.
    pub pse_burst_timer: f32,
}

/// (0x1B, 0x05) PSE Burst Action (broadcast).
///
/// (S -> C) Sent by the server when any PSE burst action is performed (e.g. PSE burst is started).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x1B, 0x05)]
pub struct PseBurstActionPacket {
    /// PSE burst action ID.
    pub action: PSEBurstAction,
    /// ID of the PSE 
    pub pse_id: u32,
    pub unk3: u32,
    /// New PSE burst timer.
    pub timer: f32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u32)]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
pub enum PSEBurstAction {
    Start,
    AddTime,
    StartCrossburst,
    OneMore,

    #[default]
    #[Read_default]
    Unknown = 0xFFFF_FFFF,
}
