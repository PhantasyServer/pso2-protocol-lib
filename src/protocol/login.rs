//! Login related packets. \[0x11\]
#[cfg(feature = "ngs_packets")]
use super::models::FunValue;
use super::{
    items::Item,
    items::ItemId,
    models::{character::Character, SGValue},
    Flags, HelperReadWrite, ObjectHeader, ObjectType, PacketError, PacketHeader, PacketReadWrite,
    PacketType,
};
use crate::AsciiString;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    io::{Read, Seek, Write},
    net::Ipv4Addr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

// ----------------------------------------------------------------
// Login packets
// ----------------------------------------------------------------

/// (0x11, 0x00) Sega ID Login.
///
/// (C -> S) Sent when the client wants to auth using Sega ID (JP and deplatformed Global).
///
/// Respond with:
/// [`crate::protocol::Packet::LoginResponse`],
/// [`crate::protocol::Packet::UserInfo`] (if auth was successful)
///
/// Response to: [`crate::protocol::Packet::EncryptionResponse`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x00)]
#[Flags(Flags::PACKED)]
#[Magic(0x5E6, 0x6B)]
pub struct SegaIDLoginPacket {
    //FIXME: fix data sizes
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    /// Some kind of version ID.
    pub ver_id: [u8; 0x20],
    /// Clients active network interfaces.
    pub interfaces: Vec<NetInterface>,
    #[Seek(0x14)]
    #[FixedLen(0x90)]
    pub unk4: Vec<u8>,
    #[Seek(0x10)]
    pub unk5: [u8; 0x10],
    #[Seek(0x10)]
    /// Clients text language.
    pub text_lang: Language,
    /// Clients voice language.
    pub voice_lang: Language,
    /// Clients text language (?).
    pub text_lang2: Language,
    /// Clients language.
    pub lang_lang: Language,
    /// Language code (in game lang?).
    #[Seek(0x8)]
    #[FixedLen(0x10)]
    pub language: String,
    pub unk6: u32,
    pub unk7: u32,
    pub magic1: u32,
    pub unk8: [u8; 0x20],
    #[FixedLen(0x44)]
    pub unk9: Vec<u8>,
    /// Sega ID username.
    #[Seek(0x104)]
    #[FixedLen(0x40)]
    pub username: AsciiString,
    /// Sega ID password.
    #[Seek(0x20)]
    #[FixedLen(0x40)]
    pub password: AsciiString,
    #[Seek(0x4)]
    pub unk10: u32,
    #[SeekAfter(0x4)]
    pub unk11: AsciiString,
}

/// (0x11, 0x01) Login Result.
///
/// (S -> C) Sent when the client tried to auth.
///
/// Respond with (if login was successful):
/// [`crate::protocol::Packet::SystemInformation`],
/// [`crate::protocol::Packet::SettingsRequest`]
///
/// Response to:
/// [`crate::protocol::Packet::SegaIDLogin`],
/// [`crate::protocol::Packet::VitaLogin`],
/// [`crate::protocol::Packet::BlockLogin`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x01)]
#[Flags(Flags::PACKED)]
#[Magic(0x8BA4, 0xB6)]
pub struct LoginResponsePacket {
    /// Login status.
    pub status: LoginStatus,
    /// Error message (if login failed).
    pub error: String,
    /// Player object.
    pub player: ObjectHeader,
    /// Current block name.
    #[FixedLen(0x20)]
    pub blockname: String,
    pub unk1: f32,
    pub unk2: u32,
    pub level_cap: u32,
    pub level_cap2: u32,
    pub unk5: u32,
    pub unk6: f32,
    pub unk7: f32,
    pub unk8: u32,
    pub unk9: f32,
    pub unk10: f32,
    pub unk11: u32,
    pub unk12: f32,
    pub unk13: u32,
    pub unk14: [f32; 0xA],
    pub unk15: [f32; 0x15],
    pub unk16: f32,
    pub unk17: f32,
    pub unk18: [f32; 0x9],
    pub unk19: [u32; 0x2],
    pub unk20: u32,
    pub unk21: u32,
    pub unk22: [f32; 0x3],
    pub unk23: u32,
    pub unk24: f32,
    pub unk25: f32,
    pub unk26: u32,
    pub unk27: [u8; 0xC],
    #[FixedLen(0x20)]
    pub unk28: String,
    pub unk29: u32,
    pub unk30: String,
    pub unk31: u32,
}

/// (0x11, 0x03) Character List.
///
/// (S -> C) Sent in response to the request.
///
/// Response to:
/// [`crate::protocol::Packet::CharacterListRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CharacterListPacket {
    /// Available characters.
    pub characters: Vec<Character>,
    pub equiped_items: Vec<[Item; 10]>,
    /// Character play times.
    pub play_times: [u32; 30],
    /// Character deletion flags (flag, deletion timestamp).
    pub deletion_flags: [(u32, u32); 30],
    /// Character ship transfer flags.
    pub transfer_flags: [(u32, u32); 30],
    /// Account accessory flag (?).
    pub account_accessory: u16,
    /// Login survey flag.
    pub login_survey: u32,
    /// Ad flag (on global 12 star unit ad).
    pub ad: u32,
}

/// (0x11, 0x04) Start Game.
///
/// (C -> S) Sent when the client has selected the character.
///
/// Respond with:
/// [`crate::protocol::Packet::LoadingScreenTransition`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x04)]
pub struct StartGamePacket {
    /// Selected character ID.
    pub char_id: u32,
    pub unk1: u32,
    pub unk2: u32,
}

/// (0x11, 0x05) Create New Character Request.
///
/// (C -> S) Sent when the client has created a new character (i.e. in the end of the character
/// creation screen).
///
/// Respond with: [`crate::protocol::Packet::CharacterCreateResponse`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x05)]
pub struct CharacterCreatePacket {
    /// New character data.
    pub character: Character,
}

/// (0x11, 0x06) Delete Character Request.
///
/// (C -> S) Sent when the client wants to delete an existing character.
///
/// Respond with: [`crate::protocol::Packet::CharacterDeletion`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x06)]
pub struct CharacterDeletionRequestPacket {
    /// Deleted character ID.
    pub char_id: u32,
}

/// (0x11, 0x07) Create New Character Response.
///
/// (S -> C) Sent in response to character creation.
///
/// Response to: [`crate::protocol::Packet::CharacterCreate`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x07)]
pub struct CharacterCreateResponsePacket {
    /// Creation result.
    pub status: CharacterCreationStatus,
    /// New character ID.
    pub char_id: u32,
}

/// (0x11, 0x08) Delete Character.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::CharacterDeletionRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x08)]
#[Flags(Flags::PACKED)]
#[Magic(0x33D4, 0xC4)]
pub struct CharacterDeletionPacket {
    /// Deletion request status.
    pub status: DeletionStatus,
    pub unk1: u32,
    pub unk2: Vec<ItemId>,
    pub unk3: Vec<ItemId>,
    pub unk4: Vec<ItemId>,
    pub unk5: Vec<ItemId>,
    pub unk6: Vec<ItemId>,
}

/// (0x11, 0x0B) Encryption Setup Request.
///
/// (C -> S) Sent when a client wants to setup encryption.
///
/// Respond with: [`crate::protocol::Packet::EncryptionResponse`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EncryptionRequestPacket {
    /// RSA encrypted data (if received using [`crate::connection::Connection`] it will be
    /// decrypted).
    pub rsa_data: Vec<u8>,
}

/// (0x11, 0x0C) Encryption Setup Response.
///
/// (S -> C) Sent in response to the request.
///
/// Respond with:
/// [`crate::protocol::Packet::SegaIDLogin`],
/// [`crate::protocol::Packet::VitaLogin`],
/// [`crate::protocol::Packet::BlockLogin`]
///
/// Response to: [`crate::protocol::Packet::EncryptionRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EncryptionResponsePacket {
    /// Encryption key.
    pub data: Vec<u8>,
}

/// (0x11, 0x0D) Client Ping.
///
/// (C -> S) Sent periodically by the client.
///
/// Respond with:
/// [`crate::protocol::Packet::ClientPong`],
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x0D)]
pub struct ClientPingPacket {
    /// Ping timestamp.
    #[PSOTime]
    pub time: Duration,
}

/// (0x11, 0x0E) Client Pong.
///
/// (S -> C) Sent by the server in response to the ping.
///
/// Response to:
/// [`crate::protocol::Packet::ClientPing`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x0E)]
pub struct ClientPongPacket {
    /// Ping timestamp.
    #[PSOTime]
    pub client_time: Duration,
    /// Pong timestamp.
    #[PSOTime]
    pub server_time: Duration,
    pub unk1: u32,
}

/// (0x11, 0x10) Block List.
///
/// (S -> C) Sent in response to the request.
///
/// Response to:
/// [`crate::protocol::Packet::BlockListRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x10)]
pub struct BlockListPacket {
    #[FixedLen(200)]
    pub blocks: Vec<BlockInfo>,
    pub unk: u32,
}

/// (0x11, 0x11) Block Switch Request.
///
/// (C -> S) Sent when the client wants to switch to a different block.
///
/// Respond with: [`crate::protocol::Packet::BlockSwitchResponse`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Default, Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x11)]
pub struct BlockSwitchRequestPacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u16,
    /// Block ID to switch to.
    pub block_id: u16,
    pub unk4: u32,
}

/// (0x11, 0x13) Block Switch Response.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::BlockSwitchRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x13)]
pub struct BlockSwitchResponsePacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u16,
    /// Block ID to switch to.
    pub block_id: u16,
    /// Block IP.
    pub ip: Ipv4Addr,
    /// Block port.
    pub port: u16,
    pub unk4: u16,
    /// Login challenge to use after switching.
    pub challenge: u32,
    /// Player ID.
    pub user_id: u32,
}

/// (0x11, 0x14) Block Login.
///
/// (C -> S) Sent when the client wants to auth after switching blocks.
///
/// Response to: [`crate::protocol::Packet::EncryptionRequest`]
///
/// Respond with: [`crate::protocol::Packet::LoginResponse`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, Default, PacketReadWrite)]
#[Id(0x11, 0x14)]
#[Flags(Flags::PACKED)]
#[Magic(0x78B8, 0x49)]
pub struct BlockLoginPacket {
    /// Player ID.
    pub player_id: u64,
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u16,
    pub unk4: u32,
    pub unk5: u32,
    /// Some kind of version ID.
    pub ver_id: [u8; 0x20],
    /// Clients active network interfaces.
    pub interfaces: Vec<NetInterface>,
    /// Login challenge (from [`crate::protocol::Packet::BlockSwitchResponse`]).
    pub challenge: u32,
    #[FixedLen(0xC4)]
    pub unk6: Vec<u8>,
    pub unk7: [u8; 0x10],
}

/// (0x11, 0x1B) User Info.
///
/// (S -> C) Sent to notify user about account status after logging in.
///
/// Response to: [`crate::protocol::Packet::LoginResponse`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x1B)]
pub struct UserInfoPacket {
    pub unk1: u32,
    pub unk2: u32,
    pub ac1: u32,
    pub unk3: u32,
    pub ac2: u32,
    pub ac3: u32,
    pub ac4: u32,
    pub ac5: u32,
    pub ac6: u32,
    pub ac7: u32,
    pub ac8: [u32; 11],
    pub fun: u32,
    pub unk4: u16,
    pub sg1: SGValue,
    pub free_sg: SGValue,
    pub sg2: [SGValue; 18],
    pub unk5: u16,
    pub unk6: [u32; 6],
    /// Premium status expiration timestamp.
    pub premium_expiration: Duration,
    pub unk7: u32,
    /// Personal quarters expiration timestamp.
    pub pq_expiration: Duration,
    /// Player shop expiration timestamp.
    pub pshop_expiration: Duration,
    pub unk8: [u32; 2],
    /// Max order expansion expiration timestamp.
    pub expand_max_orders_expiration: Duration,
    pub unk9: [u32; 19],
    /// Material storage expiration timestamp.
    pub material_storage_expiration: Duration,
    /// Extended storage 4 expiration timestamp.
    pub ex_storage4_expiration: Duration,
    /// Extended storage 5 expiration timestamp.
    pub ex_storage5_expiration: Duration,
}

/// (0x11, 0x1B) User Info (NGS).
///
/// (S -> C) Sent to notify user about account status after logging in.
///
/// Response to: [`crate::protocol::Packet::LoginResponse`]
#[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x1B)]
pub struct UserInfoNGSPacket {
    // i'm unsure about real types, just deriving from base version struct
    pub unk1: [u32; 22],
    pub unk2: u16,
    pub unk3: [u32; 16],
    pub fun: FunValue,
    pub unk4: [u32; 2],
    pub free_sg: SGValue,
    pub unk5: u16,
    pub unk6: [u32; 24],
    /// Premium status expiration timestamp.
    pub premium_expiration: Duration,
    pub unk7: u32,
    /// Personal quarters expiration timestamp.
    pub pq_expiration: Duration,
    /// Player shop expiration timestamp.
    pub pshop_expiration: Duration,
    pub unk8: [u32; 2],
    /// Max order expansion expiration timestamp.
    pub expand_max_orders_expiration: Duration,
    pub unk9: [u32; 19],
    /// Material storage expiration timestamp.
    pub material_storage_expiration: Duration,
    /// Extended storage 4 expiration timestamp.
    pub ex_storage4_expiration: Duration,
    /// Extended storage 5 expiration timestamp.
    pub ex_storage5_expiration: Duration,
    pub unk10: [u32; 4],
}

/// (0x11, 0x1E) Set Nickname Request.
///
/// (S -> C) Sent when a player first creates an account (using Sega ID) to set a nickname.
///
/// Following: [`crate::protocol::Packet::LoginResponse`]
///
/// Respond with: [`crate::protocol::Packet::NicknameResponse`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x1E)]
pub struct NicknameRequestPacket {
    /// Error flag.
    #[SeekAfter(0x42)]
    pub error: u16,
}

/// (0x11, 0x1D) Set Nickname Response.
///
/// (C -> S) Sent in response to the request
///
/// Response to: [`crate::protocol::Packet::NicknameRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x1D)]
pub struct NicknameResponsePacket {
    /// Desired nickname.
    #[FixedLen(0x20)]
    pub nickname: String,
}

/// (0x11, 0x2C) Block Balance.
///
/// (S -> C) Sent by the block balancer to transfer a client to a block.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x2C)]
pub struct BlockBalancePacket {
    pub unk1: [u8; 0x20],
    /// Target block name.
    #[FixedLen(0x20)]
    pub blockname: String,
    /// Target block IP.
    pub ip: Ipv4Addr,
    /// Target block port.
    pub port: u16,
    #[FixedLen(0x11A)]
    pub unk2: Vec<u8>,
}

/// (0x11, 0x2D) System Information.
///
/// (C -> S) Sent after logging in, contains information about players device.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x2D)]
#[Flags(Flags::PACKED)]
#[Magic(0x883D, 0x9F)]
pub struct SystemInformationPacket {
    /// CPU Model.
    pub cpu_info: AsciiString,
    /// GPU Model.
    pub video_info: AsciiString,
    /// Video RAM in bytes (if available).
    pub vram: u64,
    /// Total system RAM in bytes.
    pub total_ram: u64,
    pub unk1: u32,
    pub unk2: u32,
    /// OS version and build.
    pub windows_version: String,
    /// Game window size and mode.
    pub window_size: AsciiString,
    /// Audio device names.
    pub audio_devices: String,
    pub unk4: String,
    /// User video driver and DirectX version.
    pub video_driver: String,
    /// Total drive space in bytes.
    pub total_disk_space: u64,
    /// Free drive space in bytes.
    pub free_disk_space: u64,
}

/// (0x11, 0x3D) Ship List.
///
/// (S -> C) Sent by a ship list server when a client connects.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x3D)]
#[Flags(Flags::PACKED)]
#[Magic(0xE418, 0x51)]
pub struct ShipListPacket {
    /// Known ship infos.
    pub ships: Vec<ShipEntry>,
    /// Packet timestamp.
    pub timestamp: Duration,
    pub unk: u32,
}

/// (0x11, 0x42) New Character Screen Response.
///
/// (S -> C) Sent in response to a request.
///
/// Respond with: [`crate::protocol::Packet::CreateCharacter2`]
///
/// Response to: [`crate::protocol::Packet::CreateCharacter1`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x42)]
pub struct CreateCharacter1ResponsePacket {
    /// Creation status.
    pub status: u32,
    pub unk2: u32,
    pub used_smth: u32,
    /// Required AC to buy a character creation pass.
    pub req_ac: u32,
}

/// (0x11, 0x55) New Player Referral Response.
///
/// (S -> C) Sent in response to a request.
///
/// Response to: [`crate::protocol::Packet::CreateCharacter2`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x55)]
pub struct CreateCharacter2ResponsePacket {
    /// Player already referred flag.
    pub referral_flag: u32,
}

/// (0x11, 0x63) Vita Login.
///
/// (C -> S) Sent when the client wants to auth using PSN.
///
/// Respond with:
/// [`crate::protocol::Packet::LoginResponse`],
/// [`crate::protocol::Packet::UserInfo`] (if auth was successful)
///
/// Response to: [`crate::protocol::Packet::EncryptionResponse`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x63)]
#[Flags(Flags::PACKED)]
#[Magic(0xBE3F, 0x77)]
pub struct VitaLoginPacket {
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u16,
    pub unk4: u32,
    pub unk5: u32,
    pub ver_id: [u8; 0x20],
    /// Client netword interfaces.
    pub interfaces: Vec<NetInterface>,
    pub unk6: [u8; 0x10],
    #[Seek(0x4)]
    #[FixedLen(0x90)]
    pub unk7: Vec<u8>,
    #[Seek(0x10)]
    pub unk8: [u8; 0x10],
    #[Seek(0x10)]
    pub flag1: u32,
    pub flag2: u32,
    pub flag3: u32,
    pub flag4: u32,
    pub flag5: u32,
    pub flag6: u32,
    /// Language code.
    #[FixedLen(0x10)]
    pub language: String,
    pub unk9: u32,
    pub unk10: u32,
    pub magic1: u32,
    pub unk11: [u8; 0x20],
    #[FixedLen(0x44)]
    pub unk12: Vec<u8>,
    /// PSN username.
    #[Seek(0xFC)]
    #[FixedLen(0x40)]
    pub username: AsciiString,
    #[Seek(0x20)]
    #[FixedLen(0x40)]
    pub password: AsciiString,
    #[Seek(0x4)]
    pub unk13: u8,
    pub unk14: u8,
    pub unk15: u16,
    pub unk16: AsciiString,
    pub unk17: Vec<u8>,
    pub unk18: [u8; 0x10],
}

/// (0x11, 0x65) Full Block List.
///
/// (S -> C) Sent when the client need the information about all blocks (e.g. as part of friend
/// list).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x65)]
pub struct AllBlocksListPacket {
    /// All blocks.
    #[FixedLen(200)]
    pub blocks: Vec<BlockInfo>,
    pub unk: u32,
}

/// (0x11, 0x67) Salon Entry Response.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::SalonEntryRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x67)]
#[Flags(Flags::PACKED)]
#[Magic(0xD536, 0xA4)]
pub struct SalonResponse {
    /// Available edit pass time.
    pub reedit_time: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: Vec<SalonThing1>,
    pub unk6: Vec<SalonThing2>,
    pub unk7: u32,
}

/// (0x11, 0x68) Anticheat Challenge Request.
///
/// (S -> C) Sent periodically by the server to check anticheat status.
///
/// Respond with: [`crate::protocol::Packet::ChallengeResponse`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x68)]
#[Flags(Flags::PACKED)]
#[Magic(0x5AF4, 0xEF)]
pub struct ChallengeRequestPacket {
    /// Challenge data.
    pub data: Vec<u8>,
}

/// (0x11, 0x69) Anticheat Challenge Response.
///
/// (C -> S) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::ChallengeRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x69)]
#[Flags(Flags::PACKED)]
#[Magic(0xE0B1, 0x3A)]
pub struct ChallengeResponsePacket {
    /// Response data.
    pub data: Vec<u8>,
}

/// (0x11, 0x6F) Unknown
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x6F)]
#[Flags(Flags::PACKED)]
#[Magic(0x0323, 0xFD)]
pub struct Unk116FPacket {
    pub unk1: String,
    pub unk2: u32,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(PacketType::NGS)]
    pub unk3: u32,
}

/// (0x11, 0x71) Notification Status.
///
/// (S -> C) Sent when a client has unchecked notifications.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x71)]
pub struct NotificationStatusPacket {
    /// Number of new mails.
    pub new_mail: u32,
    /// Number of unclaimed characted campaigns.
    pub char_campaigns: u32,
    /// Number of unclaimed account campaigns.
    pub campaigns: u32,
    pub unk3: u32,
}

/// (0x11, 0x87) Login History.
///
/// (S -> C) Sent in response to a request.
///
/// Response to: [`crate::protocol::Packet::LoginHistoryRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x87)]
#[Flags(Flags::PACKED)]
#[Magic(0x8CEB, 0x8)]
pub struct LoginHistoryPacket {
    /// List of login attempts (max 50).
    pub attempts: Vec<LoginAttempt>,
}

/// (0x11, 0x8B) 2nd Password Operation Request.
///
/// (C -> S) Sent when a client wants to do something with a 2nd password (i.e set a 2nd password
/// or unlock shops).
///
/// Respond with: [`crate::protocol::Packet::SecondPwdOperation`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x8B)]
pub struct SecondPwdOperationRequestPacket {
    // 0 - unlock
    // 1 - set new pwd
    pub operation_type: u32,
    #[FixedLen(0x10)]
    pub password: AsciiString,
}

/// (0x11, 0x8C) 2nd Password Operation.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::SecondPwdOperationRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x8C)]
#[Flags(Flags::PACKED)]
#[Magic(0x29A0, 0x7F)]
pub struct SecondPwdOperationPacket {
    pub unk1: u32,
    pub unk2: u8,
    /// 2nd password set flag.
    pub is_set: u8,
    /// 2nd password entered correctly flag.
    pub is_unlocked: u16,
    pub unk5: u32,
    pub unk: String,
}

/// (0x11, 0x90) Character Undeletion Request.
///
/// (C -> S) Sent when a client wants to cancel a character deletion.
///
/// Respond with: [`crate::protocol::Packet::CharacterUndeletion`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x90)]
pub struct CharacterUndeletionRequestPacket {
    /// Character ID to cancel deletion.
    pub char_id: u32,
}

/// (0x11, 0x91) Character Undeletion.
///
/// (S -> C) Sent in response to a request.
///
/// Response to: [`crate::protocol::Packet::CharacterUndeletionRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x91)]
pub struct CharacterUndeletionPacket {
    /// Undeletion status.
    pub status: UndeletionStatus,
}

/// (0x11, 0x97) Character Rename Rights Request.
///
/// (C -> S) Sent when a client wants to get character renaming rights (i.e. clicked on "rename character").
///
/// Respond with: [`crate::protocol::Packet::CharacterRename`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x97)]
pub struct CharacterRenameRequestPacket {
    /// Character ID for renaming.
    pub char_id: u32,
}

/// (0x11, 0x98) Character Rename Rights Response.
///
/// (S -> C) Sent in response to a request.
///
/// Respond with: [`crate::protocol::Packet::CharacterNewNameRequest`] (if permitted)
///
/// Response to: [`crate::protocol::Packet::CharacterRenameRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x98)]
pub struct CharacterRenamePacket {
    /// Renaming availability status.
    pub status: RenameRequestStatus,
    /// AC price for a rename pass.
    pub ac_price: u32,
    /// Rename cooldown expiry timestamp (?).
    pub cooldown_expires: u32,
    /// Seconds untime rename cooldown expires.
    pub cooldown_secs: u32,
}

/// (0x11, 0x9B) Set New Character Name Request.
///
/// (C -> S) Sent when a client wants to set a new character name.
///
/// Respond with: [`crate::protocol::Packet::CharacterNewName`]
///
/// Response to: [`crate::protocol::Packet::CharacterRename`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x9B)]
pub struct CharacterNewNameRequestPacket {
    /// Character ID for renaming.
    pub char_id: u32,
    /// New character name.
    #[FixedLen(0x10)]
    pub name: String,
}

/// (0x11, 0x9C) Set New Character Name.
///
/// (S -> C) Sent in response to a request.
///
/// Response to: [`crate::protocol::Packet::CharacterNewNameRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x9C)]
pub struct CharacterNewNamePacket {
    /// Renaming status.
    pub status: NewNameStatus,
    /// Character ID for renaming.
    pub char_id: u32,
    /// New character name.
    #[FixedLen(0x10)]
    pub name: String,
}

/// (0x11, 0xAF) Unknown
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xAF)]
pub struct Unk11AFPacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
}

/// (0x11, 0xB0) Unknown
///
/// (C -> S)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xB0)]
pub struct Unk11B0Packet {
    pub unk1: u32,
    pub unk2: u32,
}

/// (0x11, 0xB8) Character Ship Transfer Rights Request.
///
/// (C -> S) Sent when a client wants to get ship transfer rights (i.e. clicked on
/// "transfer ship").
///
/// Respond with: [`crate::protocol::Packet::CharacterMove`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xB8)]
pub struct CharacterMoveRequestPacket {
    /// Character ID to move.
    pub char_id: u32,
    pub unk1: u32,
}

/// (0x11, 0xB9) Character Ship Transfer Rights.
///
/// (S -> C) Sent in response to a request.
///
/// Respond with: [`crate::protocol::Packet::CharacterMove`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xB9)]
pub struct CharacterMovePacket {
    pub status: u32,
    pub ac_price: u32,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
}

/// (0x11, 0xD7) Unknown
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xD7)]
pub struct Unk11D7Packet {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
}

/// (0x11, 0xDE) Player Report Request
///
/// (C -> S) Sent when a player has reported other player.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xDE)]
#[Flags(Flags::PACKED)]
#[Magic(0x60, 0x8F)]
pub struct PlayerReportedPacket {
    /// Reported player ID.
    pub targed_id: u32,
    /// Report reason.
    pub reason: u8,
    /// Report message.
    #[Seek(3)]
    pub msg: String,
}

/// (0x11, 0xEA) Prohibited Nickname Entered.
///
/// (C -> S) Sent when a player has entered a prohibited nickname (e.g. "sega").
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xEA)]
#[Flags(Flags::PACKED)]
#[Magic(0x4544, 0x14)]
pub struct NicknameErrorPacket {
    pub unk1: u32,
    /// Entered nickname.
    pub nickname: String,
}

/// (0x11, 0xED) Banner List.
///
/// (S -> C) Sent when a player has logged in.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xED)]
#[Flags(Flags::PACKED)]
#[Magic(0xD67D, 0xF5)]
pub struct BannerListPacket {
    /// Banner names (semicolon delimited).
    pub banners: AsciiString,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(PacketType::NGS)]
    pub unk1: AsciiString,
    /// News URLs (semicolon delimited).
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(PacketType::NGS)]
    pub unk2: AsciiString,
}

/// (0x11, 0xEE) Email 2FA Code Request.
///
/// (S -> C) Sent to verify a login when a suspicious one has occured.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xEE)]
#[Flags(Flags::PACKED)]
#[Magic(0x5C3B, 0x40)]
pub struct EmailCodeRequestPacket {
    pub unk1: u32,
    /// Message displayed in the box.
    pub message: String,
}

/// (0x11, 0xFF) Unknown.
///
/// (? -> ?)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xFF)]
#[Flags(Flags::PACKED)]
#[Magic(0x3DD3, 0x3D)]
pub struct Unk11FFPacket {
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: String,
    pub unk6: [u8; 0xC],
    #[FixedLen(0x40)]
    pub unk7: Vec<u8>,
    pub unk8: [u8; 0x20],
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

/// Client network interface.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct NetInterface {
    /// Interface status.
    pub state: u32,
    /// Interface MAC address.
    #[FixedLen(0x18)]
    pub mac: AsciiString,
}

/// Ship information.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct ShipEntry {
    /// Ship numerical ID.
    pub id: u32,
    /// Ship string ID (in form of "ShipXX").
    #[FixedLen(0x10)]
    pub name: String,
    /// Ship IP (ignored by the client).
    pub ip: Ipv4Addr,
    /// Ship status.
    #[Seek(4)]
    pub status: ShipStatus,
    /// Ship order.
    #[SeekAfter(4)]
    pub order: u16,
}

/// Ship status.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
pub enum ShipStatus {
    /// Status unknown or ship is under maintenance.
    #[default]
    Unknown,
    /// Ship is online.
    Online,
    /// Ship is busy.
    Busy,
    /// Ship is full.
    Full,
    /// Ship is offline.
    Offline,

    #[Read_default]
    Undefined = 0xFFFF,
}

/// Login attempt in login history.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct LoginAttempt {
    /// IP of the login.
    pub ip: Ipv4Addr,
    /// Login status.
    pub status: LoginResult,
    /// Login timestamp.
    pub timestamp: Duration,
    pub unk: u32,
}

/// Login history result.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum LoginResult {
    /// Login was successful.
    #[default]
    Successful,
    /// Email 2FA was passed.
    EmailConfirmed,
    /// Login failed.
    LoginError,
    /// Email 2FA was failed.
    EmailAuthError,
    /// Email 2FA was sent.
    AuthEmailSent,
    /// One time password error.
    OTPError,
    /// Ship was in maintenance during logon.
    InMaintenance,
    /// Generic error occured.
    GenericError,

    #[Read_default]
    Undefined = 0xFFFF_FFFF,
}

/// Login status.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum LoginStatus {
    /// Login was successful.
    #[default]
    Success,
    /// Login failed.
    Failure,

    #[Read_default]
    Undefined = 0xFFFF_FFFF,
}

/// Information about a block.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct BlockInfo {
    pub unk1: u32,
    pub unk2: u8,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: u8,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u16,
    /// Block ID.
    pub block_id: u16,
    /// Block name.
    #[FixedLen(0x20)]
    pub blockname: String,
    /// Block IP.
    pub ip: Ipv4Addr,
    /// Block port.
    pub port: u16,
    pub unk10: u16,
    pub unk11: u16,
    pub unk12: [u16; 3],
    /// Block fullness (between 0 and 1).
    pub cur_capacity: f32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct SalonThing1 {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct SalonThing2 {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
    pub unk14: u32,
}

/// Game languages.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum Language {
    #[default]
    #[Read_default]
    Japanese,
    English,
}

/// Deletion request status.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum DeletionStatus {
    /// Character has items which prevent deletion.
    #[default]
    #[Read_default]
    UndeletableItems,
    /// Character has been scheduled for deletion.
    Success,
}

/// Undeletion request status.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum UndeletionStatus {
    /// Character was already deleted.
    #[default]
    #[Read_default]
    AlreadyDeleted,
    /// Character deletion canceled.
    Success,
}

/// Rename rights status.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum RenameRequestStatus {
    /// Renaming allowed.
    #[default]
    Allowed,
    /// Renaming permit needed.
    PermitNeeded,
    /// Renaming privileges suspended.
    PrivilegesSuspended,
    /// System error has occurred.
    #[Read_default]
    SystemError,
    /// Renaming cooldown hasn't yet expired.
    TooEarly,
}

/// Renaming status.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum NewNameStatus {
    /// Renaming was successful.
    #[default]
    #[Read_default]
    Success,
    /// Renaming failed.
    Failure,
}

/// Character creation status.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum CharacterCreationStatus {
    /// Character has been successfully created.
    #[default]
    #[Read_default]
    Success,
    /// Displays an empty error message.
    EmptyError,
    /// Character limit reached.
    LimitReached,
    /// Not enough AC to create a character.
    NoAC,
    /// Generic system error message.
    SystemError,
}

// ----------------------------------------------------------------
// Read/Write implementations
// ----------------------------------------------------------------

impl PacketReadWrite for CharacterListPacket {
    fn read(
        reader: &mut (impl Read + Seek),
        _: &Flags,
        packet_type: PacketType,
    ) -> Result<Self, PacketError> {
        let char_amount = reader
            .read_u32::<LittleEndian>()
            .map_err(|e| PacketError::FieldLengthError {
                packet_name: "CharacterListPacket",
                field_name: "characters",
                error: e,
            })?
            .clamp(0, 30);
        reader
            .seek(std::io::SeekFrom::Current(4))
            .map_err(|e| PacketError::PaddingError {
                packet_name: "CharacterListPacket",
                field_name: "characters",
                error: e,
            })?;
        let mut characters = vec![];
        for i in 0..30 {
            reader
                .seek(std::io::SeekFrom::Current(4))
                .map_err(|e| PacketError::PaddingError {
                    packet_name: "CharacterListPacket",
                    field_name: "vec_characters_value",
                    error: e,
                })?;
            let character = Character::read(reader, packet_type, 0, 0).map_err(|e| {
                PacketError::CompositeFieldError {
                    packet_name: "CharacterListPacket",
                    field_name: "vec_characters_value",
                    error: Box::new(e),
                }
            })?;
            if i < char_amount {
                characters.push(character);
            }
        }
        // ???
        reader
            .seek(std::io::SeekFrom::Current(0x4))
            .map_err(|e| PacketError::FieldError {
                packet_name: "CharacterListPacket",
                field_name: "undefined",
                error: e,
            })?;
        let mut equiped_items = vec![];
        // items
        for i in 0..30 {
            let mut items: [Item; 10] = Default::default();
            for item in &mut items {
                *item = Item::read(reader, packet_type, 0, 0).map_err(|e| {
                    PacketError::CompositeFieldError {
                        packet_name: "CharacterListPacket",
                        field_name: "vec_equiped_items_value",
                        error: Box::new(e),
                    }
                })?;
            }
            if i < char_amount {
                equiped_items.push(items);
            }
        }
        let mut play_times = [0u32; 30];
        for item in &mut play_times {
            *item = reader
                .read_u32::<LittleEndian>()
                .map_err(|e| PacketError::FieldError {
                    packet_name: "CharacterListPacket",
                    field_name: "array_play_times_value",
                    error: e,
                })?;
        }
        reader
            .seek(std::io::SeekFrom::Current(32))
            .map_err(|e| PacketError::PaddingError {
                packet_name: "CharacterListPacket",
                field_name: "deletion_flags",
                error: e,
            })?;
        let mut deletion_flags = [(0u32, 0u32); 30];
        for item in &mut deletion_flags {
            item.0 = reader
                .read_u32::<LittleEndian>()
                .map_err(|e| PacketError::FieldError {
                    packet_name: "CharacterListPacket",
                    field_name: "array_deletion_flags_0_value",
                    error: e,
                })?;
            item.1 = reader
                .read_u32::<LittleEndian>()
                .map_err(|e| PacketError::FieldError {
                    packet_name: "CharacterListPacket",
                    field_name: "array_deletion_flags_1_value",
                    error: e,
                })?;
        }
        let mut transfer_flags = [(0u32, 0u32); 30];
        for item in &mut transfer_flags {
            item.0 = reader
                .read_u32::<LittleEndian>()
                .map_err(|e| PacketError::FieldError {
                    packet_name: "CharacterListPacket",
                    field_name: "array_transfer_flags_0_value",
                    error: e,
                })?;
            item.1 = reader
                .read_u32::<LittleEndian>()
                .map_err(|e| PacketError::FieldError {
                    packet_name: "CharacterListPacket",
                    field_name: "array_transfer_flags_1_value",
                    error: e,
                })?;
        }
        let account_accessory =
            reader
                .read_u16::<LittleEndian>()
                .map_err(|e| PacketError::FieldError {
                    packet_name: "CharacterListPacket",
                    field_name: "account_accessory",
                    error: e,
                })?;
        reader
            .seek(std::io::SeekFrom::Current(6))
            .map_err(|e| PacketError::PaddingError {
                packet_name: "CharacterListPacket",
                field_name: "login_survey",
                error: e,
            })?;
        let login_survey =
            reader
                .read_u32::<LittleEndian>()
                .map_err(|e| PacketError::FieldError {
                    packet_name: "CharacterListPacket",
                    field_name: "login_survey",
                    error: e,
                })?;
        let ad = reader
            .read_u32::<LittleEndian>()
            .map_err(|e| PacketError::FieldError {
                packet_name: "CharacterListPacket",
                field_name: "ad",
                error: e,
            })?;

        Ok(Self {
            characters,
            equiped_items,
            play_times,
            deletion_flags,
            transfer_flags,
            account_accessory,
            login_survey,
            ad,
        })
    }
    fn write(&self, packet_type: PacketType) -> Result<Vec<u8>, PacketError> {
        let mut buf = PacketHeader::new(0x11, 0x03, Flags::default()).write(packet_type);
        buf.write_u32::<LittleEndian>((self.characters.len() as u32).clamp(0, 30))
            .map_err(|e| PacketError::FieldLengthError {
                packet_name: "CharacterListPacket",
                field_name: "characters",
                error: e,
            })?;
        buf.write_u32::<LittleEndian>(0)
            .map_err(|e| PacketError::PaddingError {
                packet_name: "CharacterListPacket",
                field_name: "characters",
                error: e,
            })?;

        let characters = &self.characters;

        for character in characters
            .iter()
            .chain([Character::default()].iter())
            .cycle()
            .take(30)
        {
            buf.write_u32::<LittleEndian>(0)
                .map_err(|e| PacketError::PaddingError {
                    packet_name: "CharacterListPacket",
                    field_name: "vec_characters_value",
                    error: e,
                })?;
            character.write(&mut buf, packet_type, 0, 0).map_err(|e| {
                PacketError::CompositeFieldError {
                    packet_name: "CharacterListPacket",
                    field_name: "vec_characters_value",
                    error: Box::new(e),
                }
            })?;
        }
        // ???
        buf.write_u32::<LittleEndian>(0)
            .map_err(|e| PacketError::FieldError {
                packet_name: "CharacterListPacket",
                field_name: "undefined",
                error: e,
            })?;
        for equiped_items in self
            .equiped_items
            .iter()
            .chain([Default::default()].iter())
            .cycle()
            .take(30)
        {
            for item in equiped_items {
                item.write(&mut buf, packet_type, 0, 0).map_err(|e| {
                    PacketError::CompositeFieldError {
                        packet_name: "CharacterListPacket",
                        field_name: "vec_equiped_items_value",
                        error: Box::new(e),
                    }
                })?;
            }
        }
        for i in 0..30 {
            buf.write_u32::<LittleEndian>(self.play_times[i])
                .map_err(|e| PacketError::FieldError {
                    packet_name: "CharacterListPacket",
                    field_name: "array_play_times_value",
                    error: e,
                })?;
        }
        // ???
        for _ in 0..32 {
            buf.write_u8(0).map_err(|e| PacketError::PaddingError {
                packet_name: "CharacterListPacket",
                field_name: "deletion_flags",
                error: e,
            })?;
        }
        for i in 0..30 {
            // deletion flag
            buf.write_u32::<LittleEndian>(self.deletion_flags[i].0)
                .map_err(|e| PacketError::FieldError {
                    packet_name: "CharacterListPacket",
                    field_name: "array_deletion_flags_0_value",
                    error: e,
                })?;
            // timestamp
            buf.write_u32::<LittleEndian>(self.deletion_flags[i].1)
                .map_err(|e| PacketError::FieldError {
                    packet_name: "CharacterListPacket",
                    field_name: "array_deletion_flags_1_value",
                    error: e,
                })?;
        }
        for i in 0..30 {
            // transfer flag
            buf.write_u32::<LittleEndian>(self.transfer_flags[i].0)
                .map_err(|e| PacketError::FieldError {
                    packet_name: "CharacterListPacket",
                    field_name: "array_transfer_flags_0_value",
                    error: e,
                })?;
            // ??? prob target ship
            buf.write_u32::<LittleEndian>(self.transfer_flags[i].1)
                .map_err(|e| PacketError::FieldError {
                    packet_name: "CharacterListPacket",
                    field_name: "array_transfer_flags_1_value",
                    error: e,
                })?;
        }
        buf.write_u16::<LittleEndian>(self.account_accessory)
            .map_err(|e| PacketError::FieldError {
                packet_name: "CharacterListPacket",
                field_name: "account_accessory",
                error: e,
            })?;
        // ???
        buf.write_all(&[0u8; 6])
            .map_err(|e| PacketError::PaddingError {
                packet_name: "CharacterListPacket",
                field_name: "login_survey",
                error: e,
            })?;
        buf.write_u32::<LittleEndian>(self.login_survey)
            .map_err(|e| PacketError::FieldError {
                packet_name: "CharacterListPacket",
                field_name: "login_survey",
                error: e,
            })?;
        buf.write_u32::<LittleEndian>(self.ad)
            .map_err(|e| PacketError::FieldError {
                packet_name: "CharacterListPacket",
                field_name: "ad",
                error: e,
            })?;
        // ???
        buf.write_u32::<LittleEndian>(0x00_00_00_00)
            .map_err(|e| PacketError::FieldError {
                packet_name: "CharacterListPacket",
                field_name: "undefined",
                error: e,
            })?;

        // ???
        buf.write_u32::<LittleEndian>(0x00_00_00_00)
            .map_err(|e| PacketError::FieldError {
                packet_name: "CharacterListPacket",
                field_name: "undefined",
                error: e,
            })?;

        Ok(buf)
    }
}

impl PacketReadWrite for EncryptionRequestPacket {
    fn read(reader: &mut impl Read, _: &Flags, _: PacketType) -> Result<Self, PacketError> {
        let mut rsa_data = vec![];
        reader
            .read_to_end(&mut rsa_data)
            .map_err(|e| PacketError::FieldError {
                packet_name: "EncryptionRequestPacket",
                field_name: "rsa_data",
                error: e,
            })?;
        let mut tmp_data = vec![];
        let mut iter = rsa_data.into_iter().rev().skip(4);
        if let Some(x) = iter.find(|x| *x != 0x00) {
            tmp_data.push(x);
            tmp_data.extend(iter);
        }
        Ok(Self { rsa_data: tmp_data })
    }
    fn write(&self, packet_type: PacketType) -> Result<Vec<u8>, PacketError> {
        let mut buf = PacketHeader::new(0x11, 0x0B, Flags::default()).write(packet_type);
        let mut data = self.rsa_data.clone();
        data.reverse();
        data.resize(0x104, 0);
        buf.extend(data);
        Ok(buf)
    }
}

impl PacketReadWrite for EncryptionResponsePacket {
    fn read(reader: &mut impl Read, _: &Flags, _: PacketType) -> Result<Self, PacketError> {
        let mut data = vec![];
        reader
            .read_to_end(&mut data)
            .map_err(|e| PacketError::FieldError {
                packet_name: "EncryptionResponsePacket",
                field_name: "data",
                error: e,
            })?;

        Ok(Self { data })
    }
    fn write(&self, packet_type: PacketType) -> Result<Vec<u8>, PacketError> {
        let mut buf = PacketHeader::new(0x11, 0x0C, Flags::default()).write(packet_type);
        buf.extend(self.data.iter());
        Ok(buf)
    }
}

// ----------------------------------------------------------------
// Default implementations
// ----------------------------------------------------------------

impl Default for SegaIDLoginPacket {
    fn default() -> Self {
        Self {
            unk1: 0,
            unk2: 9,
            unk3: 0,
            ver_id: [0u8; 0x20],
            interfaces: vec![],
            unk4: [0u8; 0x90].into(),
            unk5: [0u8; 0x10],
            text_lang: Language::Japanese,
            voice_lang: Language::Japanese,
            text_lang2: Language::Japanese,
            lang_lang: Language::Japanese,
            language: String::new(),
            unk6: 7,
            unk7: 7,
            magic1: 0x0419,
            unk8: [0u8; 0x20],
            unk9: [0u8; 0x44].into(),
            username: Default::default(),
            password: Default::default(),
            unk10: 512,
            unk11: Default::default(),
        }
    }
}

impl Default for LoginResponsePacket {
    fn default() -> Self {
        Self {
            status: LoginStatus::Success,
            error: String::new(),
            player: ObjectHeader {
                id: 0,
                unk: 0,
                map_id: 0,
                entity_type: ObjectType::Player,
            },
            blockname: String::new(),
            unk1: 70.0,
            unk2: 32767,
            level_cap: 100,
            level_cap2: 100,
            unk5: 1,
            unk6: 10.0,
            unk7: 5.0,
            unk8: 0,
            unk9: 2.0,
            unk10: 75.0,
            unk11: 70,
            unk12: 25.0,
            unk13: 1,
            unk14: [100.0; 0xA],
            unk15: [100.0; 0x15],
            unk16: 450.0,
            unk17: 100.0,
            unk18: [100.0; 0x9],
            unk19: [0; 0x2],
            unk20: 15,
            unk21: 5,
            unk22: [15.0; 0x3],
            unk23: 0,
            unk24: 3000.0,
            unk25: 1000.0,
            unk26: 0,
            unk27: [0; 0xC],
            unk28: String::new(),
            unk29: 0,
            unk30: String::new(),
            unk31: 0,
        }
    }
}

impl Default for ClientPingPacket {
    fn default() -> Self {
        Self {
            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
        }
    }
}

impl Default for ClientPongPacket {
    fn default() -> Self {
        Self {
            client_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            server_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            unk1: 0,
        }
    }
}

impl Default for BlockBalancePacket {
    fn default() -> Self {
        Self {
            unk1: [0u8; 0x20],
            blockname: String::new(),
            ip: Ipv4Addr::UNSPECIFIED,
            port: 0,
            unk2: [0u8; 0x11A].into(),
        }
    }
}

impl Default for ShipListPacket {
    fn default() -> Self {
        Self {
            ships: vec![],
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            unk: 0,
        }
    }
}

impl Default for VitaLoginPacket {
    fn default() -> Self {
        Self {
            unk1: 0,
            unk2: 0,
            unk3: 0,
            unk4: 9,
            unk5: 0,
            ver_id: [0u8; 0x20],
            interfaces: vec![],
            unk6: [0u8; 0x10],
            unk7: [0u8; 0x90].into(),
            unk8: [0u8; 0x10],
            flag1: 0,
            flag2: 0,
            flag3: 0,
            flag4: 0,
            flag5: 0,
            flag6: 0,
            language: String::new(),
            unk9: 0,
            unk10: 0,
            magic1: 0,
            unk11: [0u8; 0x20],
            unk12: [0u8; 0x44].into(),
            username: Default::default(),
            password: Default::default(),
            unk13: 0,
            unk14: 2,
            unk15: 0,
            unk16: Default::default(),
            unk17: vec![],
            unk18: [0u8; 0x10],
        }
    }
}

impl Default for NicknameErrorPacket {
    fn default() -> Self {
        Self {
            unk1: 2,
            nickname: String::new(),
        }
    }
}

impl Default for Unk11FFPacket {
    fn default() -> Self {
        Self {
            unk1: 0,
            unk2: 0,
            unk3: 0,
            unk4: 0,
            unk5: String::new(),
            unk6: [0; 0xC],
            unk7: [0; 0x40].into(),
            unk8: [0; 0x20],
        }
    }
}

impl Default for ShipEntry {
    fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            ip: Ipv4Addr::UNSPECIFIED,
            status: ShipStatus::Unknown,
            order: 0,
        }
    }
}

impl Default for LoginAttempt {
    fn default() -> Self {
        Self {
            ip: Ipv4Addr::UNSPECIFIED,
            status: LoginResult::Successful,
            timestamp: Duration::new(0, 0),
            unk: 9,
        }
    }
}

impl Default for BlockInfo {
    fn default() -> Self {
        Self {
            unk1: 0,
            unk2: 0,
            unk3: 0,
            unk4: 0,
            unk5: 0,
            unk6: 0,
            unk7: 0,
            unk8: 0,
            block_id: 0,
            blockname: String::new(),
            ip: Ipv4Addr::UNSPECIFIED,
            port: 0,
            unk10: 0,
            unk11: 0,
            unk12: [0; 3],
            cur_capacity: 0.0,
        }
    }
}
impl Default for BlockSwitchResponsePacket {
    fn default() -> Self {
        Self {
            unk1: 0,
            unk2: 0,
            unk3: 0,
            block_id: 0,
            ip: Ipv4Addr::UNSPECIFIED,
            port: 0,
            unk4: 0,
            challenge: 0,
            user_id: 0,
        }
    }
}
