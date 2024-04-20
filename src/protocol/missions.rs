//! ARKS Missions related packets. \[0x4A\]
use super::{HelperReadWrite, PacketReadWrite};

// ----------------------------------------------------------------
// ARKS Missions packets
// ----------------------------------------------------------------

/// (0x4A, 0x01) ARKS Mission List.
///
/// (S -> C) Sent in response to the request.
///
/// Respond with: [`crate::protocol::Packet::MissionListRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x4A, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xC691, 0x47)]
pub struct MissionListPacket {
    pub unk1: u32,
    /// List of missions.
    pub missions: Vec<Mission>,
    /// Timestamp of daily missions update.
    pub daily_update: u32,
    /// Timestamp of weekly missions update.
    pub weekly_update: u32,
    /// Timestamp of tier missions update.
    pub tier_update: u32,
}

/// (0x4A, 0x03) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x4A, 0x03)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xD20D, 0xDD)]
pub struct Unk4A03Packet {
    pub unk1: u32,
    pub unk2: Vec<Mission>,
    pub unk3: Vec<u32>,
    pub unk4: Vec<Unk2Struct>,
    pub unk5: u32,
}

/// (0x4A, 0x0C) Set Tracked Mission Request.
///
/// (C -> S) Sent when the client wants to set the currently tracked mission.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x4A, 0x0C)]
pub struct SetTrackedMissionPacket {
    /// Mission ID or [`u32::MAX`] if no mission is selected.
    pub id: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

/// ARKS Mission definition.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Mission {
    /*5 - main
    1 - daily
    2 - weekly
    7 - tier */
    /// Mission type.
    pub mission_type: u32,
    /// Mission start timestamp.
    pub start_date: u32,
    /// Mission end timestamp.
    pub end_date: u32,
    /// Mission ID.
    pub id: u32,
    pub unk5: u32,
    /// Last completion timestamp.
    pub completion_date: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
    pub unk14: u32,
    pub unk15: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk2Struct {
    #[FixedLen(0x40)]
    pub unk: Vec<u32>,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(super::PacketType::NGS)]
    #[FixedLen(0x28)]
    pub unk2: Vec<u32>,
}
