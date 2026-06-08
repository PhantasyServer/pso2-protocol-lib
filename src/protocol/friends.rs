//! Friend related packets. \[0x18\]
use crate::fixed_types::FixedString;

use super::{HelperReadWrite, PacketReadWrite};
use std::time::Duration;

// ----------------------------------------------------------------
// Friend packets
// ----------------------------------------------------------------

/// (0x18, 0x14) Friend List Request.
///
/// (C -> S) Sent when a client wants a friend list.
///
/// Respond with: [`crate::protocol::Packet::FriendList`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[pso2packet(id(0x18, 0x14))]
pub struct FriendListRequestPacket {
    pub unk: u32,
}

/// (0x18, 0x15) Friend List.
///
/// (S -> C) Sent in response to a request.
///
/// Response to: [`crate::protocol::Packet::FriendListRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[pso2packet(id(0x18, 0x15))]
#[pso2packet(flags(Flags::PACKED))]
#[pso2packet(magic(0x2E1E, 0x63))]
pub struct FriendListPacket {
    pub unk1: u32,
    pub unk2: u16,
    pub unk3: u16,
    /// Players friends.
    pub friends: Vec<FriendListEntry>,
    /// Player nickname.
    pub nickname: String,
}

/// (0x18, 0x18) Send Friend Request.
///
/// (C -> S) Sent when a client has sent a friend request to another player.
///
/// Respond with: [`crate::protocol::Packet::AddedRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[pso2packet(id(0x18, 0x18))]
#[pso2packet(flags(Flags::PACKED))]
#[pso2packet(magic(0xBF57, 0x44))]
pub struct SendFriendRequestPacket {
    /// Target player ID.
    pub id: u32,
    /// Request message.
    #[pso2packet(seek(4))]
    pub msg: String,
}

/// (0x18, 0x1A) Friend Request Sent.
///
/// (S -> C) Sent in response to a request.
///
/// Response to: [`crate::protocol::Packet::SendFriendRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[pso2packet(id(0x18, 0x1A))]
pub struct AddedRequestPacket {
    pub unk1: u32,
    /// Sender player ID.
    pub sender_id: u32,
    /// Target player ID.
    #[pso2packet(seek(4))]
    pub target_id: u32,
    /// Sender player nickname.
    #[pso2packet(seek(4))]
    pub sender_nickname: FixedString<0x22>,
    /// Target player nickname.
    pub target_nickname: FixedString<0x22>,
    /// Request message.
    pub msg: FixedString<0x80>,
    /// Request send timestamp.
    pub send_time: Duration,
    #[pso2packet(seek(0x88))]
    pub unk2: u8,
    pub unk3: u8,
    #[pso2packet(seek_after(0x91))]
    pub unk4: u8,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

/// Friend entry in friend list.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, HelperReadWrite)]
pub struct FriendListEntry {
    /// Player ID.
    pub id: u32,
    /// Player nickname.
    #[pso2packet(seek(4))]
    pub nickname: FixedString<0x20>,
    /// Player character name (if logged in).
    #[pso2packet(seek(4))]
    pub char_name: FixedString<0x10>,
    /// Friend flags.
    #[pso2packet(seek(4))]
    pub flags: FriendFlags,
    pub unk2: u8,
    pub unk3: u8,
    pub unk4: u8,
    #[pso2packet(seek(4))]
    pub unk5: u32,
    /// Player current block ID.
    #[pso2packet(seek(8))]
    pub blockid: u32,
    /// Player current location.
    pub location: FriendLocation,
    pub unk6: u16,
    pub unk7: u32,
    /// Player alliance name.
    pub alliance_name: FixedString<0x18>,
    #[pso2packet(seek(0x8))]
    pub unk8: Duration,
    pub unk9: Duration,
    #[pso2packet(seek(0x38))]
    pub unk10: u32,
    #[pso2packet(seek(4))]
    pub unk11: u8,
    pub unk12: u8,
    #[pso2packet(seek(2))]
    #[pso2packet(seek_after(4))]
    pub unk13: Duration,
}

bitflags::bitflags! {
    /// Friend flags.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[derive(Debug, Copy, Clone, Default, PartialEq, HelperReadWrite)]
    #[pso2packet(bitflags)]
    pub struct FriendFlags: u8 {
        /// Is the friend online.
        const IS_ONLINE = 1 << 0;
        /// Are login notifications enabled for this friend.
        const LOGIN_NOTIF = 1 << 3;
        /// Did the player not log in for a while.
        const NO_RECENT_LOGINS = 1 << 5;

        const _ = !0;
    }
}

/// Friend map location.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
#[derive(Debug, Copy, Clone, Default, PartialEq, HelperReadWrite)]
pub enum FriendLocation {
    /// Player is in the lobby.
    #[default]
    Lobby,
    /// Player is in the quest map.
    Quest,
    /// Player is in the personal quarters.
    PersonalQ,
    /// Player is in the alliance quarters.
    AllianceQ,
    /// Player is in the casino.
    Casino,
    /// Player is in the challenger lobby.
    ChallengerLobby,
    /// Player is in the ARKS bridge.
    Bridge,
    /// Player is in the cafe.
    FrancasCafe,
    /// Player is in the battle lobby.
    BattleLobby,

    #[pso2packet(read_default)]
    Unknown = 0xFFFF,
}
