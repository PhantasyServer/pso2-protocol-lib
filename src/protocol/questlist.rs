use half::f16;

use super::{HelperReadWrite, ObjectHeader, PacketReadWrite};
use crate::AsciiString;

// ----------------------------------------------------------------
// Quests packets
// ----------------------------------------------------------------

// 0x0B, 0x06
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x06)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xB65A, 0x7D)]
pub struct StartCutscenePacket {
    pub scene_name: AsciiString,
    pub unk1: [u32; 9],
    pub unk2: Vec<ObjectHeader>,
    pub unk3: u64,
    pub unk4: u32,
    pub unk5: u8,
    pub unk6: u8,
    pub unk7: u16,
    pub unk8: AsciiString,
    pub unk9: AsciiString,
    pub unk10: u32,
    pub unk11: ObjectHeader,
}

// 0x0B, 0x09
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x09)]
pub struct Unk0B09Packet {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
}

// 0x0B, 0x13
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x13)]
pub struct Unk0B13Packet {
    pub unk1: ObjectHeader,
    pub party: ObjectHeader,
    pub unk2: u32,
    pub unk3: [u8; 8],
    pub unk4: u32,
    pub unk5: u32,
}

// 0x0B, 0x15
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x15)]
pub struct AvailableQuestsRequestPacket {
    pub unk1: u32,
}

// 0x0B, 0x16
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x16)]
pub struct AvailableQuestsPacket {
    pub unk1: u16,
    pub extreme_count: u16,
    pub unk2: u16,
    pub arks_count: u16,
    pub limited_time_count: u16,
    pub extreme_debug_count: u16,
    pub blank1_count: u16,
    pub unk3: u16,
    pub net_cafe_count: u16,
    pub warming_debug_count: u16,
    pub blank2_count: u16,
    pub advance_count: u16,
    pub expedition_count: u16,
    pub expedition_debug_count: u16,
    pub arks_debug_count: u16,
    pub unk4_count: u16,
    pub challenge_count: u16,
    pub urgent_count: u16,
    pub urgent_debug_count: u16,
    pub time_attack_count: u16,
    pub time_attack_debug_count: u16,
    pub arks_debug2_count: [u16; 9],
    pub blank3_count: u16,
    pub unk5: u16,
    pub recommended_count: u16,
    pub unk6: u16,
    pub ultimate_debug_count: u16,
    pub agp_count: u16,
    pub bonus_count: u16,
    pub unk7: u16,
    pub training_count: [u16; 10],
    pub trigger_count: u16,
    pub ridroid_count: u16,
    pub net_cafe_agp_count: u16,
    pub battle_broken_count: u16,
    pub buster_debug_count: u16,
    pub poka12_count: u16,
    pub unk8: u16,
    pub unk9: u16,
    pub buster_count: u16,
    pub hero_training_count: u16,
    pub amplified_count: u16,
    pub unk10: u16,
    pub unk11: u16,
    pub dark_blast_training_count: u16,
    pub endless_count: u16,
    pub unk12: u16,
    pub unk13: u16,
    pub phantom_training_count: u16,
    pub ais_training_count: u16,
    pub unk14: u16,
    pub damage_calc_count: u16,
    pub etoile_training_count: u16,
    pub divide_count: u16,
    // unsure
    pub stars1_count: u16,
    pub stars2_count: u16,
    pub stars3_count: u16,
    pub unk15: [u16; 2],
    #[NotOn(super::PacketType::Vita)]
    pub unk16: [u16; 2],
    pub available_types: AvailableQuestType,
    pub available_types2: AvailableQuestType2,
    #[NotOn(super::PacketType::Vita)]
    pub unk19: AvailableQuestType,
    #[NotOn(super::PacketType::Vita)]
    pub unk20: AvailableQuestType2,
    pub round_boost: u32,
    pub unk21: u32,
}

// 0x0B, 0x17
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x17)]
pub struct QuestCategoryRequestPacket {
    pub unk1: u32,
    #[SeekAfter(3)]
    pub category: QuestType,
}

// 0x0B, 0x18
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x18)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x1DB0, 0xC5)]
pub struct QuestCategoryPacket {
    pub quests: Vec<Quest>,
}

// 0x0B, 0x19
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x19)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xA36E, 0x10)]
pub struct QuestDifficultyRequestPacket {
    pub quests: Vec<ObjectHeader>,
}

// 0x0B, 0x1A
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x1A)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x292C, 0x5B)]
pub struct QuestDifficultyPacket {
    pub quests: Vec<QuestDifficulty>,
}

// 0x0B, 0x1F
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x1F)]
pub struct SetQuestPointsPacket {
    pub unk1: ObjectHeader,
    pub party: ObjectHeader,
    pub total: u32,
    pub gained: u32,
}

// 0x0B, 0x20
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x20)]
pub struct AcceptQuestPacket {
    pub quest_obj: ObjectHeader,
    pub diff: u16,
    pub unk1: u16,
    pub unk2: [u32; 7],
}

// 0x0B, 0x28
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x28)]
pub struct QuestPointsAddedPacket {
    pub added: u32,
    pub x: f16,
    pub y: f16,
    #[SeekAfter(2)]
    pub z: f16,
}

// 0x0B, 0x2F
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x2F)]
pub struct AcceptQuestOtherPacket {
    pub quest_obj: ObjectHeader,
    pub diff: u16,
    pub unk1: u16,
    pub unk2: [u32; 7],
}

// 0x0B, 0x62
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x62)]
pub struct EQARKSLevelPacket {
    pub unk1: u32,
}

// 0x0B, 0xAF
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0xAF)]
pub struct Unk0BAFPacket {
    pub unk1: u32,
    pub unk2: u32,
}

// 0x0B, 0xD0
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0xD0)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x3E03, 0xC2)]
pub struct Unk0BD0Packet {
    #[FixedLen(0x23)]
    pub unk1: Vec<u32>,
    pub unk2: Vec<u32>,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Quest {
    #[FixedStr(0x20)]
    pub date: AsciiString,
    pub quest_obj: ObjectHeader,
    pub name_id: u32,
    pub unk3: [u32; 27],
    pub unk4: u16,
    pub unk5: u8,
    pub unk6: u8,
    pub unk7: [u32; 20],
    pub unk8: [u16; 3],
    pub length: u8,
    pub party_type: PartyType,
    pub difficulties: QuestDifficultyType,
    pub difficulties_completed: QuestDifficultyType,
    pub unk9: u8,
    pub req_level: u8,
    pub sub_class_req_level: u8,
    pub enemy_level: u8,
    pub unk10: u8,
    pub quest_type: QuestType,
    pub unk11: [u8; 6],
    pub unk12: u16,
    pub unk13: [u32; 2],
    pub unk14: u16,
    pub unk15: [u8; 2],
    // pub unk15: [QuestThing; 16],
    #[FixedLen(0x320)]
    pub unk16: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u8)]
pub enum PartyType {
    #[default]
    #[Read_default]
    Solo,
    SingleParty,
    MultiParty,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct QuestDifficulty {
    #[FixedStr(0x20)]
    pub date: AsciiString,
    pub quest_obj: ObjectHeader,
    pub name_id: u32,
    pub planet: u8,
    pub area: u8,
    pub unk1: u8,
    pub unk2: u8,
    pub diffs: [QuestDifficultyEntry; 8],
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct QuestDifficultyEntry {
    pub req_level: u8,
    pub sub_class_req_level: u8,
    pub monster_level: u8,
    pub unk1: u8,
    pub ability_adj: u32,
    pub dmg_limit: u32,
    pub time_limit: u32,
    pub time_limit2: u32,
    pub supp_target: u32,
    pub unk2: u32,
    pub enemy1: u32,
    pub unk3: u32,
    pub enemy2: u32,
    pub unk4: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct QuestThing {
    pub unk1: [u32; 2],
    pub unk2: [u8; 2],
    pub unk3: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[Flags(u64)]
pub struct AvailableQuestType {
    #[Skip]
    // 0x02
    pub extreme: bool,
    // unsure
    pub storyep1: bool,
    pub arks: bool,
    pub limited_time: bool,
    pub extreme_debug: bool,
    pub blank1: bool,
    // unsure
    pub storyep2: bool,

    pub net_cafe: bool,
    pub warming_debug: bool,
    pub blank2: bool,
    pub advance: bool,
    pub expedition: bool,
    pub free_debug: bool,
    pub arks_debug: bool,
    // unsure
    pub story_debug: bool,

    pub challenge: bool,
    pub urgent: bool,
    pub urgent_debug: bool,
    pub time_attack: bool,
    pub time_debug: bool,
    pub arks_debug2: bool,
    pub arks_debug3: bool,
    pub arks_debug4: bool,

    pub arks_debug5: bool,
    pub arks_debug6: bool,
    pub arks_debug7: bool,
    pub arks_debug8: bool,
    pub arks_debug9: bool,
    pub arks_debug10: bool,
    pub blank3: bool,
    // unsure
    pub storyep3: bool,

    pub recommended: bool,
    pub ultimate: bool,
    pub ultimate_debug: bool,
    pub agp: bool,
    pub bonus: bool,
    // storyep4?
    pub unk1: bool,
    pub standard_training: bool,
    pub hunter_training: bool,

    pub ranger_training: bool,
    pub force_training: bool,
    pub fighter_training: bool,
    pub gunner_training: bool,
    pub techter_training: bool,
    pub braver_training: bool,
    pub bouncer_training: bool,
    pub summoner_training: bool,

    // if set the client auto selects this category(48)
    pub auto_accept: bool,
    pub ridroid: bool,
    pub net_cafe_agp: bool,
    pub battle_broken: bool,
    pub buster_debug: bool,
    pub poka12: bool,
    pub unk2: bool,
    pub unk3: bool,

    pub buster: bool,
    pub hero_training: bool,
    pub amplified: bool,
    pub unk4: bool,
    pub unk5: bool,
    pub dark_blast_training: bool,
    pub endless: bool,
    pub unk6: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[Flags(u64)]
pub struct AvailableQuestType2 {
    pub blank4: bool,
    pub phantom_training: bool,
    pub ais_training: bool,
    pub unk1: bool,
    pub damage_calc: bool,
    pub etoile_training: bool,
    pub divide: bool,
    pub stars1: bool,

    pub stars2: bool,
    pub stars3: bool,
    pub stars4: bool,
    pub stars5: bool,
    pub stars6: bool,
    pub unk2: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u8)]
pub enum QuestType {
    #[default]
    #[Read_default]
    Unk0,
    Extreme,
    ARKS = 3,
    LimitedTime,
    ExtremeDebug,
    Blank1,
    NetCafe = 8,
    WarmingDebug,
    Blank2,
    Advance,
    Expedition,
    FreeDebug,
    ArksDebug,
    Challenge = 16,
    Urgent,
    UrgentDebug,
    TimeAttack,
    TimeDebug,
    ArksDebug2,
    ArksDebug3,
    ArksDebug4,
    ArksDebug5,
    ArksDebug6,
    ArksDebug7,
    ArksDebug8,
    ArksDebug9,
    ArksDebug10,
    Blank3,
    Recommended = 32,
    Ultimate,
    UltimateDebug,
    AGP,
    Bonus,
    StandardTraining,
    HunterTraining,
    RangerTraining,
    ForceTraining,
    FighterTraining,
    GunnerTraining,
    TechterTraining,
    BraverTraining,
    BouncerTraining,
    SummonerTraining,
    AutoAccept,
    Ridroid,
    CafeAGP,
    BattleBroken,
    BusterDebug,
    Poka12,
    StoryEP1 = 55,
    Buster,
    HeroTraining,
    Amplified,
    DarkBlastTraining = 61,
    Endless,
    Blank4 = 64,
    PhantomTraining,
    AISTraining,
    DamageCalculation = 68,
    EtoileTraining,
    Divide,
    Stars1,
    Stars2,
    Stars3,
    Stars4,
    Stars5,
    Stars6,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[Flags(u8)]
pub struct QuestDifficultyType {
    pub normal: bool,
    pub hard: bool,
    pub very_hard: bool,
    pub super_hard: bool,
    pub ex_hard: bool,
    pub ultra_hard: bool,
    pub unnamed1: bool,
    pub unnamed2: bool,
}
