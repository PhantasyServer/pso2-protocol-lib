//! Party related packets. \[0x0E\]
use crate::{
    protocol::{models::character::Class, HelperReadWrite, ObjectHeader, PacketReadWrite},
    AsciiString,
};

use super::questlist::{Quest, QuestDifficulty, QuestType};

// ----------------------------------------------------------------
// Party packets
// ----------------------------------------------------------------

/// (0x0E, 0x00) Add New Party Member (broadcast).
///
/// (S -> C) Sent when a new player joins the party.
///
/// Follow with:
/// [`crate::protocol::Packet::SetPartyColor`] (for all players in the party),
/// [`crate::protocol::Packet::PartySetupFinish`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x00)]
#[Flags(Flags::PACKED)]
#[Magic(0xCCE7, 0x13)]
pub struct AddMemberPacket {
    /// New player object.
    pub new_member: ObjectHeader,
    /// Party color of the player.
    pub color: Color,
    #[Seek(3)]
    /// Level of the main class.
    pub level: u32,
    /// Level of the subclass.
    pub sublevel: u32,
    #[SeekAfter(3)]
    /// Class of the player.
    pub class: Class,
    /// Subclass of the player.
    pub subclass: Class,
    pub padding: [u8; 3],
    /// Nickname of the player.
    pub nickname: String,
    /// Name of the character.
    pub char_name: String,
    pub unk5: [u8; 0xC],
    pub unk6: u16,
    pub unk7: [u8; 2],
    /// HP of the player (exact reasons for 3 values are unknown).
    pub hp: [u32; 3],
    /// Map ID where the player is located.
    pub map_id: u16,
    pub unk10: [u8; 4],
    pub unk11: u16,
    pub unk12: u32,
    pub unk13: [u8; 0xC],
    pub unk14: [u32; 3],
    pub unk15: String,
    pub unk16: AsciiString,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(PacketType::NGS)]
    pub unk17: AsciiString,
}

/// (0x0E, 0x01) Remove Party Member (broadcast).
///
/// (S -> C) Sent when a player is removed from the party.
///
/// Follow with:
/// [`crate::protocol::Packet::SetPartyColor`] (for all players in the party),
/// [`crate::protocol::Packet::PartySetupFinish`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x01)]
pub struct RemoveMemberPacket {
    /// Removed player object.
    pub removed_member: ObjectHeader,
    /// Receiving player object.
    pub receiver: ObjectHeader,
}

/// (0x0E, 0x02) Init Party.
///
/// (S -> C) Sent when a new party is created or a receiver joins an existing party.
///
/// Follow with:
/// [`crate::protocol::Packet::PartySettings`],
/// [`crate::protocol::Packet::SetPartyColor`] (for all players in the party),
/// [`crate::protocol::Packet::PartySetupFinish`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x02)]
#[Flags(Flags::PACKED)]
#[Magic(0xD863, 0xA9)]
pub struct PartyInitPacket {
    /// Party object.
    pub party_object: ObjectHeader,
    /// Party leader's object.
    pub leader: ObjectHeader,
    /// Number of people in the party.
    pub people_amount: u32,
    /// Party members.
    pub entries: [PartyEntry; 4],
    pub unk2: AsciiString,
}

/// (0x0E, 0x04) Party Invite Result.
///
/// (S -> C) Sent as a result of inviting someone to the party.
///
/// Response to: [`crate::protocol::Packet::PartyInviteRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x04)]
pub struct PartyInviteResultPacket {
    pub unk1: u32,
    pub status: u32,
}

/// (0x0E, 0x05) Party Invite Request.
///
/// (C -> S) Sent when a client invites someone to the party.
///
/// Respond with: [`crate::protocol::Packet::PartyInviteResult`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x05)]
pub struct PartyInviteRequestPacket {
    /// Object of the player being invited.
    pub invitee: ObjectHeader,
}

/// (0x0E, 0x06) New Party Invite.
///
/// (S -> C) Sent when someone invites the receiver to the party.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x06)]
#[Flags(Flags::PACKED)]
#[Magic(0xEF59, 0xD5)]
pub struct NewInvitePacket {
    /// Invited party object.
    pub party_object: ObjectHeader,
    /// Object of the inviting player.
    pub inviter: ObjectHeader,
    /// Name of the party.
    pub name: String,
    /// Character name(?) of the inviting player.
    pub inviter_name: String,
    /// Accepted quest name.
    pub questname: String,
}

/// (0x0E, 0x07) Accept Party Invite.
///
/// (C -> S) Sent when the player accepts an invite to the party.
///
/// Respond with: [`crate::protocol::Packet::PartyInit`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x07)]
pub struct AcceptInvitePacket {
    /// Invited party object.
    pub party_object: ObjectHeader,
    /// Object of the inviting player.
    pub inviter: ObjectHeader,
}

/// (0x0E, 0x0C) Set Party Settings.
///
/// (C -> S) Sent when the player sets party settings.
///
/// Respond with: [`crate::protocol::Packet::PartySettings`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x0C)]
#[Flags(Flags::PACKED)]
#[Magic(0x11CB, 0x98)]
pub struct NewPartySettingsPacket {
    /// Name of the party.
    pub name: String,
    /// Party password.
    pub password: String,
    /// Party comments.
    pub comments: String,
    /// Accepted quest name.
    pub questname: String,
    /// Minimum acceptable level.
    pub min_level: u8,
    /// Maximum acceptable level.
    pub max_level: u8,
    /// Party playstyle.
    pub playstyle: u8,
    /// Party flags.
    pub flags: PartyFlags,
    pub unk: u64,
}

/// (0x0E, 0x0D) Party Settings (broadcast).
///
/// (S -> C) Sent when the leader sets party settings or when the player first joins the party.
///
/// Response to: [`crate::protocol::Packet::NewPartySettings`]
///
/// Following: [`crate::protocol::Packet::PartyInit`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x0D)]
#[Flags(Flags::PACKED)]
#[Magic(0x9789, 0xE3)]
pub struct PartySettingsPacket {
    /// Name of the party.
    pub name: String,
    /// Party password.
    pub password: String,
    /// Party comments.
    pub comments: String,
    /// Minimum acceptable level.
    pub min_level: u8,
    /// Maximum acceptable level.
    pub max_level: u8,
    /// Party playstyle.
    pub playstyle: u8,
    /// Party flags.
    pub flags: PartyFlags,
    pub unk: u64,
}

/// (0x0E, 0x0E) Transfer Party Leadership.
///
/// (C -> S) Sent when the player transfers party leadership.
///
/// Respond with: [`crate::protocol::Packet::NewLeader`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x0E)]
pub struct TransferLeaderPacket {
    /// Object of the new leader.
    pub target: ObjectHeader,
}

/// (0x0E, 0x0F) New Party Leader (broadcast).
///
/// (S -> C) Sent when the party has a new leader.
///
/// Response to: [`crate::protocol::Packet::TransferLeader`]
///
/// Following: [`crate::protocol::Packet::RemoveMember`] (?)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x0F)]
pub struct NewLeaderPacket {
    /// Object of the new leader.
    pub leader: ObjectHeader,
}

/// (0x0E, 0x10) Kick Party Member.
///
/// (C -> S) Sent when the player kicks a party member.
///
/// Respond with: [`crate::protocol::Packet::KickedMember`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x10)]
pub struct KickMemberPacket {
    /// Member to be kicked.
    pub member: ObjectHeader,
}

/// (0x0E, 0x11) Party Member Kicked (broadcast).
///
/// (S -> C) Sent when a party member is kicked (including the receiver).
///
/// Response to: [`crate::protocol::Packet::KickMember`]
///
/// Follow with: [`crate::protocol::Packet::RemovedFromParty`] (if being kicked)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x11)]
pub struct KickedMemberPacket {
    /// Member who was kicked.
    pub member: ObjectHeader,
}

/// (0x0E, 0x17) Disband Party Request.
///
/// (C -> S) Sent when a player wants to disband the party.
///
/// Respond with: [`crate::protocol::Packet::PartyDisbandedMarker`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x17)]
pub struct DisbandPartyPacket {
    /// Disbanded party object.
    pub party: ObjectHeader,
}

/// (0x0E, 0x19) Set Chat Status (broadcast).
///
/// (bidirectional) Sent when a player enters or leaves the chat.
///
/// Respond with: [`crate::protocol::Packet::ChatStatus`] (S -> C)
///
/// Response to: [`crate::protocol::Packet::ChatStatus`] (C -> S)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x19)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct ChatStatusPacket {
    /// Object of the player (not set for C -> S).
    pub object: ObjectHeader,
    /// Chat status.
    pub status: u32,
}

/// (0x0E, 0x1A) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x1A)]
pub struct Unk0E1APacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

/// (0x0E, 0x1B) Party Info.
///
/// (S -> C) Sent when a player requests party info.
///
/// Response to: [`crate::protocol::Packet::GetPartyInfo`]
///
/// Follow with: [`crate::protocol::Packet::PartyInfo`] (if more infos are available),
/// [`crate::protocol::Packet::PartyInfoStopper`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x1B)]
#[Flags(Flags::PACKED)]
#[Magic(0xE7E8, 0xFF)]
pub struct PartyInfoPacket {
    /// Number of populated party infos.
    pub num_of_infos: u32,
    /// Party infos.
    pub infos: [PartyInfo; 10],
}

/// (0x0E, 0x1C) Party Info Stopper.
///
/// (S -> C) Sent when no more party infos are available.
///
/// Following: [`crate::protocol::Packet::PartyInfo`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x1C)]
pub struct PartyInfoStopperPacker {
    pub unk: u32,
}

/// (0x0E, 0x1D) Party Details Request.
///
/// (C -> S) Sent when the client requests party details.
///
/// Respond with: [`crate::protocol::Packet::PartyDetails`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x1D)]
#[Flags(Flags::PACKED)]
#[Magic(0xF364, 0x95)]
pub struct GetPartyDetailsPacket {
    /// Requested party objects.
    pub parties: Vec<ObjectHeader>,
}

/// (0x0E, 0x1E) Party Details.
///
/// (S -> C) Sent when a player requests party details.
///
/// Response to: [`crate::protocol::Packet::GetPartyDetails`]
///
/// Follow with: [`crate::protocol::Packet::PartyDetails`] (if more infos are available),
/// [`crate::protocol::Packet::PartyDetailsStopper`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x1E)]
#[Flags(Flags::PACKED)]
#[Magic(0x7921, 0xE0)]
pub struct PartyDetailsPacket {
    /// Number of populated party details.
    pub num_of_details: u32,
    /// Party details.
    #[FixedLen(0x0C)]
    pub details: Vec<PartyDetails>,
}

/// (0x0E, 0x21) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x21)]
#[Flags(Flags::PACKED)]
#[Magic(0x0A5A, 0xC1)]
pub struct Unk0E21Packet {
    pub people_amount: u32,
    pub entries: [PartyEntry; 4],
}

/// (0x0E, 0x25) Set Quest Info.
///
/// (S -> C) Sent when a player accepts a quest (not always, causes currently unknown).
///
/// Response to: [`crate::protocol::Packet::AcceptQuest`]
///
/// Follow with: [`crate::protocol::Packet::SetPartyQuest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x25)]
pub struct SetQuestInfoPacket {
    /// Name ID of the quest.
    pub name: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u16,
    pub unk4: u16,
    /// Player who accepted the quest.
    pub player: ObjectHeader,
    pub unk5: [u32; 5],
    pub unk6: u8,
    pub unk7: u8,
    pub unk8: u8,
    /// Quest difficulty.
    pub diff: u8,
    /// Quest type.
    pub quest_type: QuestType,
}

/// (0x0E, 0x2B) New Busy State (broadcast).
///
/// (S -> C) Sent when a player changes their busy state.
///
/// Response to: [`crate::protocol::Packet::SetBusy`],
/// [`crate::protocol::Packet::SetNotBusy`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x2B)]
pub struct NewBusyStatePacket {
    /// Object of the player.
    pub object: ObjectHeader,
    /// New busy state.
    pub state: BusyState,
}

/// (0x0E, 0x2C) Set Invite Decline.
///
/// (C -> S) Sent when a player changes their invite decline state.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x2C)]
pub struct InviteDeclinePacket {
    /// New decline status.
    pub decline_status: RejectStatus,
}

/// (0x0E, 0x2E) Party Info Request.
///
/// (C -> S) Sent when the client requests party info.
///
/// Respond with: [`crate::protocol::Packet::PartyInfo`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x2E)]
#[Flags(Flags::PACKED)]
#[Magic(0xD4FC, 0x92)]
pub struct GetPartyInfoPacket {
    /// Requested party objects.
    pub parties: Vec<ObjectHeader>,
}

/// (0x0E, 0x31) Set Party Quest.
///
/// (S -> C) Sent when the client accepts a quest (not always, causes currently unknown).
///
/// Following: [`crate::protocol::Packet::SetQuestInfo`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x31)]
pub struct SetPartyQuestPacket {
    pub name: u32,
    pub difficulty: u32,
    #[SeekAfter(3)]
    pub quest_type: QuestType,
    pub quest_def: Quest,
    pub quest_diffs: QuestDifficulty,
    pub player: ObjectHeader,
    pub unk1: u16,
    pub unk2: u16,
}

/// (0x0E, 0x4F) Set In Party Status.
///
/// (S -> C) Sent when the player joins or leaves the party.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x4F)]
pub struct SetPartyColorPacket {
    /// Target player object.
    pub target: ObjectHeader,
    pub unk: [u32; 3],
    /// New in party status.
    pub in_party: u32,
}

/// (0x0E, 0x52) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x52)]
pub struct Unk0E52Packet {
    pub unk1: u32,
    pub unk2: u32,
}

/// (0x0E, 0x67) Party Setup Finish.
///
/// (S -> C) Sent when all of the party's info is sent.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x67)]
pub struct PartySetupFinishPacket {
    pub unk: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

/// Player entry in party.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, HelperReadWrite)]
pub struct PartyEntry {
    /// Object of the player.
    pub id: ObjectHeader,
    /// Player's nickname.
    pub nickname: String,
    /// Character name.
    pub char_name: String,
    /// Level of the main class.
    pub level: u8,
    /// Level of the subclass.
    pub sublevel: u8,
    /// Class of the player.
    pub class: Class,
    /// Subclass of the player.
    pub subclass: Class,
    /// Party color of the player.
    pub color: Color,
    pub unk1: [u8; 7],
    pub unk2: u32,
    /// HP of the player (exact reasons for 3 values are unknown).
    pub hp: [u32; 3],
    /// Map ID where the player is located.
    pub map_id: u16,
    pub unk3: u16,
    pub unk4: [u8; 0xC],
    pub unk5: [u32; 3],
    pub unk6: String,
    #[OnlyOn(PacketType::Vita)]
    pub unk10: String,
    pub unk7: AsciiString,
    /// Player's text language.
    pub lang: ShortLanguage,
    pub unk9: [u8; 3],
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(PacketType::NGS)]
    pub unk11: String,
}

/// Player party colors.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
#[repr(u8)]
pub enum Color {
    #[default]
    #[Read_default]
    Red,
    Green,
    Yellow,
    Blue,
}

/// Text language of the player.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
#[repr(u8)]
pub enum ShortLanguage {
    #[default]
    Japanese,
    English,

    #[Read_default]
    Unknown,
}

bitflags::bitflags! {
    /// Party flags.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
    #[BitFlags(u8)]
    pub struct PartyFlags: u8 {
        /// Is the party only for friends.
        const FRIENDS_ONLY = 1 << 0;
        /// Is the party only for alliance members.
        const ALLIANCE_ONLY = 1 << 1;
        /// Limit multiplayer requests from other parties.
        const LIMIT_OTHERS = 1 << 2;
        /// Is the party only for a single run.
        const SINGLE_RUN = 1 << 3;
        /// Is the party actively looking for members.
        const OPEN = 1 << 4;
        /// Is the party voice chat focused.
        const VC_FOCUS = 1 << 6;

        const _ = !0;
    }

}

/// Party info.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, HelperReadWrite)]
pub struct PartyInfo {
    pub unk1: [u8; 0xC],
    /// Party object.
    pub party_object: ObjectHeader,
    /// Name of the party.
    pub name: String,
    pub unk2: [u8; 9],
    pub unk3: [u8; 3],
    pub unk4: u32,
    /// Time when the player was invited.
    pub invite_time: u32,
    pub unk6: u32,
}

/// Party invite status.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum RejectStatus {
    #[default]
    #[Read_default]
    Allow,
    Reject,
}

/// Party details.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, HelperReadWrite)]
pub struct PartyDetails {
    /// Party description.
    pub party_desc: String,
    /// Party object.
    pub party_id: ObjectHeader,
    pub unk3: [u8; 0x10],
    pub unk4: u64,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u16,
    pub unk8: u16,
    pub unk9: [u8; 12],
    /// Members of the party.
    pub unk10: [PartyMember; 4],
}

/// Party member (short variant).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, HelperReadWrite)]
pub struct PartyMember {
    /// Character name.
    pub char_name: String,
    /// Player's nickname.
    pub nickname: String,
    /// Object of the player.
    pub id: ObjectHeader,
    /// Class of the player.
    pub class: Class,
    /// Subclass of the player.
    pub subclass: Class,
    /// Level of the main class.
    pub level: u8,
    /// Level of the subclass.
    pub sublevel: u8,
    pub unk4: [u8; 5],
    pub unk5: [u8; 3],
}

/// Player busy state.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum BusyState {
    #[default]
    #[Read_default]
    NotBusy,
    Busy,
}
