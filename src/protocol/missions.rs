use super::{HelperReadWrite, PacketReadWrite, PacketType};

// ----------------------------------------------------------------
// ARKS Missions packets
// ----------------------------------------------------------------

// 0x4A, 0x01
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x4A, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xC691, 0x47)]
pub struct MissionListPacket {
    pub unk1: u32,
    pub missions: Vec<Mission>,
    pub daily_update: u32,
    pub weekly_update: u32,
    pub tier_update: u32,
}

// 0x4A, 0x03
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

// 0x4A, 0x0C
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x4A, 0x0C)]
pub struct SetTrackedMissionPacket {
    pub id: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Mission {
    /*5 - main
    1 - daily
    2 - weekly
    7 - tier */
    pub mission_type: u32,
    pub start_date: u32,
    pub end_date: u32,
    pub unk4: u32,
    pub unk5: u32,
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
    #[OnlyOn(PacketType::NGS)]
    #[FixedLen(0x28)]
    pub unk2: Vec<u32>,
}
