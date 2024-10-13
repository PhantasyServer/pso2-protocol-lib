//! Classic Mission Pass related packets. \[0x4D\]
use crate::fixed_types::FixedVec;
use super::{items::Item, HelperReadWrite, PacketReadWrite};

// ----------------------------------------------------------------
// Classic mission pass packets
// ----------------------------------------------------------------

/// (0x4D, 0x01) Mission Pass Info.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::MissionPassInfoRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x4D, 0x01)]
pub struct MissionPassInfoPacket {
    pub unk: FixedVec<0x2F, u32>,
}

/// (0x4D, 0x03) Mission Pass.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::MissionPassRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x4D, 0x03)]
#[Flags(Flags::PACKED)]
#[Magic(0xB0C, 0x35)]
pub struct MissionPassPacket {
    pub unk1: u32,
    /// Ongoing season ID.
    pub cur_season_id: u32,
    /// Ongoing season name.
    pub cur_season: String,
    /// Stars required to advance to the next tier.
    pub stars_per_tier: u32,
    /// Regular tier count.
    pub tiers: u32,
    /// Overrun tier count.
    pub overrun_tiers: u32,
    /// Total tier count.
    pub total_tiers: u32,
    /// Pass start timestamp.
    pub start_date: u32,
    /// Pass end timestamp.
    pub end_date: u32,
    /// Catchup period start timestamp.
    pub catchup_start: u32,
    pub unk11: u32,
    /// Banner ID for the ongoing season.
    pub cur_banner: String,
    /// SG price per tier.
    pub price_per_tier: u32,
    /// SG price for gold pass.
    pub gold_pass_price: u32,
    /// Mission Pass rewards.
    pub cur_items: Vec<MissionPassItem>,
    /// Previous season ID.
    pub last_season_id: u32,
    /// Previous season name.
    pub last_season: String,
    /// Stars required to advance to the next tier (previous season).
    pub last_stars_per_tier: u32,
    /// Regular tier count (previous season).
    pub last_tiers: u32,
    /// Overrun tier count (previous season).
    pub last_overrun_tiers: u32,
    /// Total tier count (previous season).
    pub last_total_tiers: u32,
    /// Pass start timestamp (previous season).
    pub last_start_date: u32,
    /// Pass end timestamp (previous season).
    pub last_end_date: u32,
    /// Catchup period start timestamp (previous season).
    pub last_catchup_start: u32,
    /// Catchup period end timestamp (previous season).
    pub last_catchup_end: u32,
    /// Banner ID for the previous season.
    pub last_banner: String,
    /// SG price per tier (previous season).
    pub last_price_per_tier: u32,
    /// SG price for gold pass (previous season).
    pub last_gold_pass_price: u32,
    /// Mission Pass rewards (previous season).
    pub last_items: Vec<MissionPassItem>,
    pub unk30: u32,
    pub unk31: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

/// Item in the mission pass.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct MissionPassItem {
    /// Reward ID (?).
    pub id: u32,
    /// Reward tier.
    pub tier: u32,
    /// Gold pass only flag.
    pub is_gold: u32,
    pub unk4: u32,
    /// Group ID.
    pub group: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    /// Item data.
    pub item: Item,
}
