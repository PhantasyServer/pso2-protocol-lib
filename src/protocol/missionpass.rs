use super::{items::Item, HelperReadWrite, PacketReadWrite};

// ----------------------------------------------------------------
// Classic mission pass packets
// ----------------------------------------------------------------

// 0x4D, 0x01
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x4D, 0x01)]
pub struct MissionPassInfoPacket {
    #[FixedLen(0x2F)]
    pub unk: Vec<u32>,
}

// 0x4D, 0x03
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x4D, 0x03)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct MissionPassPacket {
    pub unk1: u32,
    pub cur_season_id: u32,
    #[VariableStr(0xB0C, 0x35)]
    pub cur_season: String,
    pub stars_per_tier: u32,
    pub tiers: u32,
    pub overrun_tiers: u32,
    pub total_tiers: u32,
    pub start_date: u32,
    pub end_date: u32,
    pub catchup_start: u32,
    pub unk11: u32,
    #[VariableStr(0xB0C, 0x35)]
    pub cur_banner: String,
    pub price_per_tier: u32,
    pub gold_pass_price: u32,
    #[Magic(0xB0C, 0x35)]
    pub cur_items: Vec<MissionPassItem>,
    pub last_season_id: u32,
    #[VariableStr(0xB0C, 0x35)]
    pub last_season: String,
    pub last_stars_per_tier: u32,
    pub last_tiers: u32,
    pub last_overrun_tiers: u32,
    pub last_total_tiers: u32,
    pub last_start_date: u32,
    pub last_end_date: u32,
    pub last_catchup_start: u32,
    pub last_catchup_end: u32,
    #[VariableStr(0xB0C, 0x35)]
    pub last_banner: String,
    pub last_price_per_tier: u32,
    pub last_gold_pass_price: u32,
    #[Magic(0xB0C, 0x35)]
    pub last_items: Vec<MissionPassItem>,
    pub unk30: u32,
    pub unk31: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct MissionPassItem {
    pub id: u32,
    pub tier: u32,
    pub is_gold: u32,
    pub unk4: u32,
    pub group: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub item: Item,
}
