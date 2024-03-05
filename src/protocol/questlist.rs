//! Quest list related packets. \[0x0B\]
use super::{HelperReadWrite, ObjectHeader, PacketReadWrite};
use crate::AsciiString;
use half::f16;

// ----------------------------------------------------------------
// Quests packets
// ----------------------------------------------------------------

/// (0x0B, 0x06) Start Cutscene.
///
/// (S -> C) Sent in order to start a cutscene.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x06)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xB65A, 0x7D)]
pub struct StartCutscenePacket {
    /// Name of the cutscene.
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

/// (0x0B, 0x09) Unknown.
///
/// (C -> S)
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

/// (0x0B, 0x13) Unknown.
///
/// (S -> C)
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

/// (0x0B, 0x15) Available Quests Request.
///
/// (C -> S) Sent when the client wants to display quest category list
/// (i.e. interacts with the quest counter).
///
/// Respond with: (0x0B, 0xF1), [`crate::protocol::Packet::AvailableQuests`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x15)]
pub struct AvailableQuestsRequestPacket {
    pub unk1: u32,
}

/// (0x0B, 0x16) Available Quests Response.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::AvailableQuestsRequest`]
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
    /// Round boost active flag.
    pub round_boost: u32,
    pub unk21: u32,
}

/// (0x0B, 0x17) Quest Category List Request
///
/// (C -> S) Sent when the client requests the list of quests in a certain category.
///
/// Respond with: [`crate::protocol::Packet::QuestCategory`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x17)]
pub struct QuestCategoryRequestPacket {
    pub unk1: u32,
    /// Requested category.
    #[SeekAfter(3)]
    pub category: QuestType,
}

/// (0x0B, 0x18) Quest Category List Response
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::QuestCategoryRequest`]
///
/// Follow with: [`crate::protocol::Packet::QuestCategory`] (if there are more quests),
/// [`crate::protocol::Packet::QuestCategoryStopper`] (if all quests are sent)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x18)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x1DB0, 0xC5)]
pub struct QuestCategoryPacket {
    /// List of quests in a requested category.
    pub quests: Vec<Quest>,
}

/// (0x0B, 0x19) Quest Difficulty List Request
///
/// (C -> S) Sent when the client requests the difficulties of certain quests.
///
/// Respond with: [`crate::protocol::Packet::QuestDifficulty`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x19)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xA36E, 0x10)]
pub struct QuestDifficultyRequestPacket {
    /// List of object of requested quests.
    pub quests: Vec<ObjectHeader>,
}

/// (0x0B, 0x1A) Quest Difficulty List Response
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::QuestDifficultyRequest`]
///
/// Follow with: [`crate::protocol::Packet::QuestDifficulty`] (if there are more quests),
/// [`crate::protocol::Packet::QuestDifficultyStopper`] (if all quests are sent)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x1A)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x292C, 0x5B)]
pub struct QuestDifficultyPacket {
    /// List of difficulties for requested quests.
    pub quests: Vec<QuestDifficulty>,
}

/// (0x0B, 0x1F) Set Quest Points. (broadcast)
///
/// (S -> C) Sent when quest points are changed (usually due to an emergency).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x1F)]
pub struct SetQuestPointsPacket {
    pub unk1: ObjectHeader,
    /// Party receiving the points.
    pub party: ObjectHeader,
    /// Total amount of points.
    pub total: u32,
    /// Gained amount of points (may be zero).
    pub gained: u32,
}

/// (0x0B, 0x20) Accept Quest.
///
/// (C -> S) Sent when the client accepts a quest.
/// When this packet is sent is currently unknown.
///
/// Respond with: setup quest.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x20)]
pub struct AcceptQuestPacket {
    /// Selected quest object.
    pub quest_obj: ObjectHeader,
    /// Selected difficulty.
    pub diff: u16,
    pub unk1: u16,
    pub unk2: [u32; 7],
}

/// (0x0B, 0x28) Add Quest Points. (broadcast)
///
/// (S -> C) Sent when quest points are increase (usually due to killing an enemy).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x28)]
pub struct QuestPointsAddedPacket {
    /// Gained amount of points (may be zero).
    pub added: u32,
    /// X position of the number.
    pub x: f16,
    /// Y position of the number.
    pub y: f16,
    /// Z position of the number.
    #[SeekAfter(2)]
    pub z: f16,
}

/// (0x0B, 0x2F) Accept Quest. (alternative)
///
/// (C -> S) Sent when the client accepts a quest.
/// When this packet is sent is currently unknown.
///
/// Respond with: setup quest.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x2F)]
pub struct AcceptQuestOtherPacket {
    /// Selected quest object.
    pub quest_obj: ObjectHeader,
    /// Selected difficulty.
    pub diff: u16,
    pub unk1: u16,
    pub unk2: [u32; 7],
}

/// (0x0B, 0x62) Set EQ ARKS Level. (broadcast)
///
/// (S -> C) Sent when the EQ's ARKS Level is changed.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x62)]
pub struct EQARKSLevelPacket {
    /// New level.
    pub level: u32,
}

/// (0x0B, 0xAF) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0xAF)]
pub struct Unk0BAFPacket {
    pub unk1: u32,
    pub unk2: u32,
}

/// (0x0B, 0xD0) Unknown.
///
/// (S -> C)
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

/// Information about a quest.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Quest {
    /// Quest date.
    #[FixedStr(0x20)]
    pub date: AsciiString,
    /// Quest object.
    pub quest_obj: ObjectHeader,
    /// ID of the quest name.
    pub name_id: u32,
    pub unk3: [u32; 27],
    pub unk4: u16,
    pub unk5: u8,
    pub unk6: u8,
    pub unk7: [u32; 20],
    pub unk8: [u16; 3],
    /// Length of the quest.
    pub length: u8,
    /// Party type of the quest.
    pub party_type: PartyType,
    /// Available difficulties.
    pub difficulties: QuestDifficultyType,
    /// Completed difficulties.
    pub difficulties_completed: QuestDifficultyType,
    pub unk9: u8,
    /// Required level.
    pub req_level: u8,
    /// Required sub level.
    pub sub_class_req_level: u8,
    /// Enemy level.
    pub enemy_level: u8,
    pub unk10: u8,
    /// Type of the quest.
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

/// Amount of parties that can join a quest.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u8)]
pub enum PartyType {
    /// Only one player can join.
    #[default]
    #[Read_default]
    Solo,
    /// Only one party can join (up to 4 players).
    SingleParty,
    /// Multiple parties can join (up to 12 players).
    MultiParty,
}

/// Information about a quest difficulty.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct QuestDifficulty {
    /// Quest date.
    #[FixedStr(0x20)]
    pub date: AsciiString,
    /// Quest object.
    pub quest_obj: ObjectHeader,
    /// ID of the quest name.
    pub name_id: u32,
    /// Planet ID.
    pub planet: u8,
    /// Area ID.
    pub area: u8,
    pub unk1: u8,
    pub unk2: u8,
    /// Difficulty list.
    pub diffs: [QuestDifficultyEntry; 8],
}

/// Quest difficulty entry.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct QuestDifficultyEntry {
    /// Required level.
    pub req_level: u8,
    /// Required sub level.
    pub sub_class_req_level: u8,
    /// Enemy level.
    pub monster_level: u8,
    pub unk1: u8,
    pub ability_adj: u32,
    /// Damage limit.
    pub dmg_limit: u32,
    /// Time limit.
    pub time_limit: u32,
    /// Time limit.
    pub time_limit2: u32,
    /// Suppression target ID.
    pub supp_target: u32,
    pub unk2: u32,
    /// Other enemy 1.
    pub enemy1: u32,
    pub unk3: u32,
    /// Other enemy 2.
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

/// Available quest types flags.
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

/// Available quest types flags (continuation).
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

/// Type of the quest.
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

/// Available quest difficulties.
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
