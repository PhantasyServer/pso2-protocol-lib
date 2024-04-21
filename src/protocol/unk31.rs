//! Unknown \[0x31\] packets.
use super::{HelperReadWrite, ItemId, PacketError, PacketReadWrite};
use crate::AsciiString;

// ----------------------------------------------------------------
// Unknown 0x31 packets
// ----------------------------------------------------------------

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

// ----------------------------------------------------------------
// Internal structs
// ----------------------------------------------------------------

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

impl PacketReadWrite for PlayAchievementsResponsePacket {
    fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        flags: &super::Flags,
        packet_type: super::PacketType,
    ) -> Result<Self, PacketError> {
        let packet = PlayAchievementsInternal::read(reader, flags, packet_type)?;
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
    }
}
