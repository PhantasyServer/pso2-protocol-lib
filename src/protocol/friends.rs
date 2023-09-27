use super::{HelperReadWrite, PacketReadWrite};
use std::time::Duration;

// ----------------------------------------------------------------
// Friend packets
// ----------------------------------------------------------------

// 0x18, 0x14
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x18, 0x14)]
pub struct FriendListRequestPacket {
    pub unk: u32,
}

// 0x18, 0x15
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x18, 0x15)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct FriendListPacket {
    pub unk1: u32,
    pub unk2: u16,
    pub unk3: u16,
    #[Magic(0x2E1E, 0x63)]
    pub friends: Vec<FriendListEntry>,
    #[VariableStr(0x2E1E, 0x63)]
    pub nickname: String,
}

// 0x18, 0x18
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x18, 0x18)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SendFriendRequestPacket {
    pub id: u32,
    #[Seek(4)]
    #[VariableStr(0xBF57, 0x44)]
    pub msg: String,
}

// 0x18, 0x1A
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x18, 0x1A)]
pub struct AddedRequestPacket {
    pub unk1: u32,
    pub sender_id: u32,
    #[Seek(4)]
    pub target_id: u32,
    #[Seek(4)]
    #[FixedStr(0x22)]
    pub sender_nickname: String,
    #[FixedStr(0x22)]
    pub target_nickname: String,
    #[FixedStr(0x80)]
    pub msg: String,
    pub send_time: Duration,
    #[Seek(0x88)]
    pub unk2: u8,
    pub unk3: u8,
    #[SeekAfter(0x91)]
    pub unk4: u8,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, HelperReadWrite)]
pub struct FriendListEntry {
    pub id: u32,
    #[Seek(4)]
    #[FixedStr(0x20)]
    pub nickname: String,
    #[Seek(4)]
    #[FixedStr(0x10)]
    pub char_name: String,
    #[Seek(4)]
    pub flags: FriendFlags,
    pub unk2: u8,
    pub unk3: u8,
    pub unk4: u8,
    #[Seek(4)]
    pub unk5: u32,
    #[Seek(8)]
    pub blockid: u32,
    pub location: FriendLocation,
    pub unk6: u16,
    pub unk7: u32,
    #[FixedStr(0x18)]
    pub alliance_name: String,
    #[Seek(0x8)]
    pub unk8: Duration,
    pub unk9: Duration,
    #[Seek(0x38)]
    pub unk10: u32,
    #[Seek(4)]
    pub unk11: u8,
    pub unk12: u8,
    #[Seek(2)]
    #[SeekAfter(4)]
    pub unk13: Duration,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Copy, Clone, Default, PartialEq, HelperReadWrite)]
#[Flags(u8)]
pub struct FriendFlags {
    pub is_online: bool,
    #[Skip]
    #[Skip]
    pub login_notif: bool,
    #[Skip]
    pub no_recent_logins: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
#[derive(Debug, Copy, Clone, Default, PartialEq, HelperReadWrite)]
pub enum FriendLocation {
    #[default]
    Lobby,
    Quest,
    PersonalQ,
    AllianceQ,
    Casino,
    ChallengerLobby,
    Bridge,
    FrancasCafe,
    BattleLobby,

    #[Read_default]
    Unknown = 0xFFFF,
}
