//! Unknown \[0x31\] packets.
use super::{HelperReadWrite, Item, ItemId, PacketError, PacketReadWrite, PacketType};
use crate::AsciiString;

// ----------------------------------------------------------------
// Unknown 0x31 packets
// ----------------------------------------------------------------

/// (0x31, 0x02) New Title.
///
/// (S -> C) Sent in response to a request.
///
/// Response to: [`crate::protocol::Packet::NewTitlesRequest`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x31, 0x02)]
#[Flags(Flags::PACKED)]
#[Magic(0xC6AD, 0xB1)]
pub struct NewTitlesPacket {
    /// Unclaimed title IDs (i.e. new titles).
    pub new_titles_ids: Vec<u32>,
}

/// (0x31, 0x04) Title List.
///
/// (S -> C) Sent in response to a request.
///
/// Response to: [`crate::protocol::Packet::TitleListRequest`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x31, 0x04)]
#[Flags(Flags::PACKED)]
#[Magic(0xD228, 0x47)]
pub struct TitleListPacket {
    /// Title information.
    pub title_infos: Vec<TitleInfo>,
    pub unk: u32,
}

/// (0x31, 0x05) Load Title Names.
///
/// (S -> C) Sent when a client sees a title for the first time since logging in.
//
// See internal repr for real structure.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LoadTitlesPacket {
    /// Title ID - Name pairs.
    pub names: Vec<NamedTitleId>,
}

/// (0x31, 0x06) Title Condition Request.
///
/// (C -> S) Sent when a player wants to receive a title completion condition (i.e. the player
/// hovers over a title at the title counter).
///
/// Respond with: [`crate::protocol::Packet::TitleCondition`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x31, 0x06)]
pub struct GetTitleConditionPacket {
    /// Requested title ID.
    pub title_id: u32,
}

/// (0x31, 0x07) Title Condition.
///
/// (S -> C) Sent in response to a request.
///
/// Response to: [`crate::protocol::Packet::TitleConditionRequest`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x31, 0x07)]
#[Flags(Flags::PACKED)]
#[Magic(0x6361, 0x28)]
pub struct LoadTitleConditionPacket {
    /// Requested title ID.
    pub title_id: u32,
    /// Translated condition string.
    pub condition: String,
}

/// (0x31, 0x09) Play Achievements Request.
///
/// (S -> C) Sent in response to a achievement request.
///
/// Response to: [`crate::protocol::Packet::PlayAchievementsRequest`]
// See internal repr for more info.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayAchievementsResponsePacket {
    pub unk1: u32,
    /// Total enemies suppressed.
    pub total_suppressed: u32,
    /// Total quest completions.
    pub quest_completions: u32,
    /// S ranks.
    pub s_ranks: u32,
    /// A ranks.
    pub a_ranks: u32,
    /// B ranks.
    pub b_ranks: u32,
    /// C ranks.
    pub c_ranks: u32,
    /// Times incapacitated.
    pub deaths: u32,
    /// Maximum damage.
    pub max_damage: u32,
    pub unk2: Vec<u32>,
    /// Quest records.
    pub quest_records: Vec<QuestRecord>,
    /// Rare item acquisition records.
    pub rare_items: Vec<ItemId>,
    /// Boss enemy records.
    pub boss_enemies: Vec<EnemyRecord>,
    /// Rare enemy records.
    pub rare_enemies: Vec<EnemyRecord>,
    /// Titles acquired.
    pub titles_acquired: u32,
}

/// (0x31, 0x0A) Receive Title Reward Request.
///
/// (C -> S) Sent when a player wants to receive a reward for a new title.
///
/// Respond with: [`crate::protocol::Packet::AddedItem`],
/// [`crate::protocol::Packet::ReceiveTitleReward`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x31, 0x0A)]
pub struct ReceiveTitleRewardRequestPacket {
    /// Requested title ID.
    pub title_id: u32,
}

/// (0x31, 0x0B) Receive Title Reward Response.
///
/// (S -> C) Sent in response to a request.
///
/// Response to: [`crate::protocol::Packet::ReceiveTitleRewardRequest`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x31, 0x0B)]
pub struct ReceiveTitleRewardPacket {
    pub unk1: u32,
    pub unk2: u32,
    /// Requested title ID.
    pub title_id: u32,
}

// ----------------------------------------------------------------
// Internal structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x31, 0x05)]
#[Flags(Flags::PACKED)]
#[Magic(0x57E6, 0x92)]
struct LoadTitlesInternal {
    title_ids: Vec<u32>,
    names: String,
    name_lens: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x31, 0x09)]
#[Flags(Flags::PACKED)]
#[Magic(0x6EDC, 0xBE)]
struct PlayAchievementsInternal {
    unk1: u32,
    /// Total enemies suppressed.
    total_suppressed: u32,
    /// Total quest completions.
    quest_completions: u32,
    /// S ranks.
    s_ranks: u32,
    /// A ranks.
    a_ranks: u32,
    /// B ranks.
    b_ranks: u32,
    /// C ranks.
    c_ranks: u32,
    /// Times incapacitated.
    deaths: u32,
    /// Maximum damage.
    max_damage: u32,
    #[FixedLen(3)]
    unk2: Vec<u32>,
    /// Quest records.
    quest_records: Vec<QuestRecord>,
    /// Rare item acquisition records.
    rare_items: Vec<ItemId>,
    /// Boss enemy records.
    boss_enemies: Vec<EnemyRecordInternal>,
    /// Rare enemy records.
    rare_enemies: Vec<EnemyRecordInternal>,
    /// List of enemy IDs.
    enemy_ids: AsciiString,
    /// Titles acquired.
    titles_acquired: u32,
}

/// Internal representation of the enemy record.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
struct EnemyRecordInternal {
    /// Offset of the name ID.
    pub name_offset: u32,
    /// Length of the name ID.
    pub name_length: u32,
    /// Level of the enemy
    pub level: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

/// Title name + ID pair.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NamedTitleId {
    /// Title ID.
    pub title_id: u32,
    /// Title name.
    pub name: String,
}

/// Title information.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct TitleInfo {
    /// Title ID.
    pub title_id: u32,
    pub title_id2: u32,
    pub unk1: u32,
    /// Reward received flag.
    pub reward_received: u32,
    /// Reward item.
    pub reward_item: Item,
}

/// Quest record entry.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct QuestRecord {
    /// Id of the quest's name.
    pub name_id: u32,
    /// Quest's difficulty.
    pub difficulty: u8,
    pub unk2: u8,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: u32,
    pub unk6: u32,
}

/// Enemy record entry.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EnemyRecord {
    /// Name ID.
    pub name: AsciiString,
    /// Level of the enemy
    pub level: u32,
}

// ----------------------------------------------------------------
// Read/Write implementations
// ----------------------------------------------------------------

impl PacketReadWrite for LoadTitlesPacket {
    fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        flags: &super::Flags,
        packet_type: PacketType,
    ) -> Result<Self, PacketError> {
        let packet = LoadTitlesInternal::read(reader, flags, packet_type).map_err(|e| {
            PacketError::CompositeFieldError {
                packet_name: "LoadTitlesPacket",
                field_name: "internal",
                error: Box::new(e),
            }
        })?;
        let mut names = packet.names.chars();
        let mut items = vec![];
        for (title_id, name_length) in packet
            .title_ids
            .into_iter()
            .zip(packet.name_lens.into_iter())
        {
            let name = names.by_ref().take(name_length as usize).collect();
            items.push(NamedTitleId { name, title_id });
        }
        Ok(Self { names: items })
    }

    fn write(&self, packet_type: PacketType) -> Result<Vec<u8>, PacketError> {
        let mut names = String::new();
        let mut name_lens = vec![];
        let mut title_ids = vec![];
        for item in self.names.iter() {
            name_lens.push(item.name.chars().count() as u8);
            names.push_str(&item.name);
            title_ids.push(item.title_id);
        }
        LoadTitlesInternal {
            title_ids,
            names,
            name_lens,
        }
        .write(packet_type)
        .map_err(|e| PacketError::CompositeFieldError {
            packet_name: "LoadTitlesPacket",
            field_name: "internal",
            error: Box::new(e),
        })
    }
}

impl PacketReadWrite for PlayAchievementsResponsePacket {
    fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        flags: &super::Flags,
        packet_type: super::PacketType,
    ) -> Result<Self, PacketError> {
        let packet = PlayAchievementsInternal::read(reader, flags, packet_type).map_err(|e| {
            PacketError::CompositeFieldError {
                packet_name: "PlayAchievementsResponsePacket",
                field_name: "internal",
                error: Box::new(e),
            }
        })?;
        let mut names = packet.enemy_ids.chars();
        let mut boss_enemies = vec![];
        let mut rare_enemies = vec![];
        for record in packet.boss_enemies {
            let name = AsciiString::from_string_unchecked(
                names.by_ref().take(record.name_length as usize).collect(),
            );
            boss_enemies.push(EnemyRecord {
                name,
                level: record.level,
            });
        }
        for record in packet.rare_enemies {
            let name = AsciiString::from_string_unchecked(
                names.by_ref().take(record.name_length as usize).collect(),
            );
            rare_enemies.push(EnemyRecord {
                name,
                level: record.level,
            });
        }
        Ok(Self {
            unk1: packet.unk1,
            total_suppressed: packet.total_suppressed,
            quest_completions: packet.quest_completions,
            s_ranks: packet.s_ranks,
            a_ranks: packet.a_ranks,
            b_ranks: packet.b_ranks,
            c_ranks: packet.c_ranks,
            deaths: packet.deaths,
            max_damage: packet.max_damage,
            unk2: packet.unk2,
            quest_records: packet.quest_records,
            rare_items: packet.rare_items,
            boss_enemies,
            rare_enemies,
            titles_acquired: packet.titles_acquired,
        })
    }

    fn write(&self, packet_type: super::PacketType) -> Result<Vec<u8>, PacketError> {
        let mut names = String::new();
        let mut total_len = 0;
        let mut boss_enemies = vec![];
        let mut rare_enemies = vec![];
        for item in self.boss_enemies.iter() {
            let len = item.name.len() as u32;
            names.push_str(&item.name);
            boss_enemies.push(EnemyRecordInternal {
                name_offset: total_len,
                name_length: len,
                level: item.level,
            });
            total_len += len;
        }
        for item in self.rare_enemies.iter() {
            let len = item.name.len() as u32;
            names.push_str(&item.name);
            rare_enemies.push(EnemyRecordInternal {
                name_offset: total_len,
                name_length: len,
                level: item.level,
            });
            total_len += len;
        }
        PlayAchievementsInternal {
            unk1: self.unk1,
            total_suppressed: self.total_suppressed,
            quest_completions: self.quest_completions,
            s_ranks: self.s_ranks,
            a_ranks: self.a_ranks,
            b_ranks: self.b_ranks,
            c_ranks: self.c_ranks,
            deaths: self.deaths,
            max_damage: self.max_damage,
            unk2: self.unk2.clone(),
            quest_records: self.quest_records.clone(),
            rare_items: self.rare_items.clone(),
            boss_enemies,
            rare_enemies,
            enemy_ids: AsciiString::from_string_unchecked(names),
            titles_acquired: self.titles_acquired,
        }
        .write(packet_type)
        .map_err(|e| PacketError::CompositeFieldError {
            packet_name: "PlayAchievementsResponsePacket",
            field_name: "internal",
            error: Box::new(e),
        })
    }
}
