#[cfg(feature = "ngs_packets")]
use super::models::FunValue;
use super::{
    items::ItemId,
    models::{character::Character, SGValue},
    EntityType, Flags, HelperReadWrite, ObjectHeader, PacketHeader, PacketReadWrite, PacketType,
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

// 0x11, 0x00
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x00)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x5E6, 0x6B)]
pub struct SegaIDLoginPacket {
    //FIXME: fix data sizes
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub ver_id: [u8; 0x20],
    pub interfaces: Vec<NetInterface>,
    #[Seek(0x14)]
    #[FixedLen(0x90)]
    pub unk4: Vec<u8>,
    #[Seek(0x10)]
    pub unk5: [u8; 0x10],
    #[Seek(0x10)]
    pub text_lang: Language,
    pub voice_lang: Language,
    pub text_lang2: Language,
    pub lang_lang: Language,
    #[Seek(0x8)]
    #[FixedStr(0x10)]
    pub language: String,
    pub unk6: u32,
    pub unk7: u32,
    pub magic1: u32,
    pub unk8: [u8; 0x20],
    #[FixedLen(0x44)]
    pub unk9: Vec<u8>,
    #[Seek(0x104)]
    #[FixedStr(0x40)]
    pub username: AsciiString,
    #[Seek(0x20)]
    #[FixedStr(0x40)]
    pub password: AsciiString,
    #[Seek(0x4)]
    pub unk10: u32,
    #[SeekAfter(0x4)]
    pub unk11: AsciiString,
}

// 0x11, 0x01
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x8BA4, 0xB6)]
pub struct LoginResponsePacket {
    pub status: LoginStatus,
    pub error: String,
    pub player: ObjectHeader,
    #[FixedStr(0x20)]
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
    #[FixedStr(0x20)]
    pub unk28: String,
    pub unk29: u32,
    pub unk30: String,
    pub unk31: u32,
}

// 0x11, 0x03
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CharacterListPacket {
    pub characters: Vec<Character>,
    pub play_times: [u32; 30],
    pub deletion_flags: [(u32, u32); 30],
    pub transfer_flags: [(u32, u32); 30],
    pub account_accessory: u16,
    pub login_survey: u32,
    pub ad: u32,
}

//0x11, 0x04
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x04)]
pub struct StartGamePacket {
    pub char_id: u32,
    pub unk1: u32,
    pub unk2: u32,
}

// 0x11, 0x05
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x05)]
pub struct CharacterCreatePacket {
    pub character: Character,
}

// 0x11, 0x06
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x06)]
pub struct CharacterDeletionRequestPacket {
    pub char_id: u32,
}

// 0x11, 0x08
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x08)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x33D4, 0xC4)]
pub struct CharacterDeletionPacket {
    pub status: DeletionStatus,
    pub unk1: u32,
    pub unk2: Vec<ItemId>,
    pub unk3: Vec<ItemId>,
    pub unk4: Vec<ItemId>,
    pub unk5: Vec<ItemId>,
    pub unk6: Vec<ItemId>,
}

// 0x11, 0x0B
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EncryptionRequestPacket {
    pub rsa_data: Vec<u8>,
}

// 0x11, 0x0C
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EncryptionResponsePacket {
    pub data: Vec<u8>,
}

// 0x11, 0x0D
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x0D)]
pub struct ClientPingPacket {
    #[PSOTime]
    pub time: Duration,
}

// 0x11, 0x0E
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x0E)]
pub struct ClientPongPacket {
    #[PSOTime]
    pub client_time: Duration,
    #[PSOTime]
    pub server_time: Duration,
    pub unk1: u32,
}

// 0x11, 0x10
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x10)]
pub struct BlockListPacket {
    #[FixedLen(200)]
    pub blocks: Vec<BlockInfo>,
    pub unk: u32,
}

// 0x11, 0x11
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Default, Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x11)]
pub struct BlockSwitchRequestPacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u16,
    pub block_id: u16,
    pub unk4: u32,
}

// 0x11, 0x13
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x13)]
pub struct BlockSwitchResponsePacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u16,
    pub block_id: u16,
    pub ip: Ipv4Addr,
    pub port: u16,
    pub unk4: u16,
    pub challenge: u32,
    pub user_id: u32,
}

// 0x11, 0x14
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, Default, PacketReadWrite)]
#[Id(0x11, 0x14)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x78B8, 0x49)]
pub struct BlockLoginPacket {
    pub player_id: u64,
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u16,
    pub unk4: u32,
    pub unk5: u32,
    pub ver_id: [u8; 0x20],
    pub interfaces: Vec<NetInterface>,
    pub challenge: u32,
    #[FixedLen(0xC4)]
    pub unk6: Vec<u8>,
    pub unk7: [u8; 0x10],
}

// 0x11, 0x1B
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
    pub premium_expiration: Duration,
    pub unk7: u32,
    pub pq_expiration: Duration,
    pub pshop_expiration: Duration,
    pub unk8: [u32; 2],
    pub expand_max_orders_expiration: Duration,
    pub unk9: [u32; 19],
    pub material_storage_expiration: Duration,
    pub ex_storage4_expiration: Duration,
    pub ex_storage5_expiration: Duration,
    pub unk10: [u32; 4],
}

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
    // also pso2es char id
    pub ac5: u32,
    pub ac6: u32,
    // also unlnked es account flag?
    pub ac7: u32,
    pub ac8: [u32; 11],
    pub fun: u32,
    pub unk4: u16,
    pub sg1: SGValue,
    pub free_sg: SGValue,
    pub sg2: [SGValue; 18],
    pub unk5: u16,
    pub unk6: [u32; 6],
    pub premium_expiration: Duration,
    pub unk7: u32,
    pub pq_expiration: Duration,
    pub pshop_expiration: Duration,
    pub unk8: [u32; 2],
    pub expand_max_orders_expiration: Duration,
    pub unk9: [u32; 19],
    pub material_storage_expiration: Duration,
    pub ex_storage4_expiration: Duration,
    pub ex_storage5_expiration: Duration,
}

// 0x11, 0x1E
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x1E)]
pub struct NicknameRequestPacket {
    #[SeekAfter(0x42)]
    pub error: u16,
}

// 0x11, 0x1D
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x1D)]
pub struct NicknameResponsePacket {
    #[FixedStr(0x10)]
    #[SeekAfter(0x20)]
    pub nickname: String,
}

// 0x11, 0x2C
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x2C)]
pub struct BlockBalancePacket {
    pub unk1: [u8; 0x20],
    #[FixedStr(0x20)]
    pub blockname: String,
    pub ip: Ipv4Addr,
    pub port: u16,
    #[FixedLen(0x11A)]
    pub unk2: Vec<u8>,
}

// 0x11, 0x2D
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x2D)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x883D, 0x9F)]
pub struct SystemInformationPacket {
    pub cpu_info: AsciiString,
    pub video_info: AsciiString,
    pub vram: u64,
    pub total_ram: u64,
    pub unk1: u32,
    pub unk2: u32,
    pub windows_version: String,
    pub window_size: AsciiString,
    pub unk3: String,
    pub unk4: String,
    pub video_driver: String,
    pub total_disk_space: u64,
    pub free_disk_space: u64,
}

// 0x11, 0x3D
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x3D)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xE418, 0x51)]
pub struct ShipListPacket {
    pub ships: Vec<ShipEntry>,
    pub timestamp: Duration,
    pub unk: u32,
}

// 0x11, 0x42
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x42)]
pub struct CreateCharacter1ResponsePacket {
    pub status: u32,
    pub unk2: u32,
    pub used_smth: u32,
    pub req_ac: u32,
}

// 0x11, 0x55
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x55)]
pub struct CreateCharacter2ResponsePacket {
    pub unk: u32,
}

// 0x11, 0x63
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x63)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xBE3F, 0x77)]
pub struct VitaLoginPacket {
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u16,
    pub unk4: u32,
    pub unk5: u32,
    pub ver_id: [u8; 0x20],
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
    #[FixedStr(0x10)]
    pub language: String,
    pub unk9: u32,
    pub unk10: u32,
    pub magic1: u32,
    pub unk11: [u8; 0x20],
    #[FixedLen(0x44)]
    pub unk12: Vec<u8>,
    #[Seek(0xFC)]
    #[FixedStr(0x40)]
    pub username: AsciiString,
    #[Seek(0x20)]
    #[FixedStr(0x40)]
    pub password: AsciiString,
    #[Seek(0x4)]
    pub unk13: u8,
    pub unk14: u8,
    pub unk15: u16,
    pub unk16: AsciiString,
    pub unk17: Vec<u8>,
    pub unk18: [u8; 0x10],
}

// 0x11, 0x65
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x65)]
pub struct AllBlocksListPacket {
    #[FixedLen(200)]
    pub blocks: Vec<BlockInfo>,
    pub unk: u32,
}

// 0x11, 0x67
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x67)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xD536, 0xA4)]
pub struct SalonResponse {
    pub reedit_time: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: Vec<SalonThing1>,
    pub unk6: Vec<SalonThing2>,
    pub unk7: u32,
}

// 0x11, 0x68
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x68)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x5AF4, 0xEF)]
pub struct ChallengeRequestPacket {
    pub data: Vec<u8>,
}

// 0x11, 0x69
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x69)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xE0B1, 0x3A)]
pub struct ChallengeResponsePacket {
    pub data: Vec<u8>,
}

// 0x11, 0x6F
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x6F)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x0323, 0xFD)]
pub struct Unk116FPacket {
    pub unk1: String,
    pub unk2: u32,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(PacketType::NGS)]
    pub unk3: u32,
}

// 0x11, 0x71
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x71)]
pub struct NotificationStatusPacket {
    pub new_mail: u32,
    pub char_campaigns: u32,
    pub campaigns: u32,
    pub unk3: u32,
}

// 0x11, 0x87
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x87)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x8CEB, 0x8)]
pub struct LoginHistoryPacket {
    pub attempts: Vec<LoginAttempt>,
}

// 0x11, 0x90
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x90)]
pub struct CharacterUndeletionRequestPacket {
    pub char_id: u32,
}

// 0x11, 0x91
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x91)]
pub struct CharacterUndeletionPacket {
    pub status: UndeletionStatus,
}

// 0x11, 0x97
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x97)]
pub struct CharacterRenameRequestPacket {
    pub char_id: u32,
}

// 0x11, 0x98
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x98)]
pub struct CharacterRenamePacket {
    pub status: RenameRequestStatus,
    pub ac_price: u32,
    pub cooldown_expires: u32,
    pub cooldown_secs: u32,
}

// 0x11, 0x9B
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x9B)]
pub struct CharacterNewNameRequestPacket {
    pub char_id: u32,
    #[FixedStr(0x10)]
    pub name: String,
}

// 0x11, 0x9C
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0x9C)]
pub struct CharacterNewNamePacket {
    pub status: NewNameStatus,
    pub char_id: u32,
    #[FixedStr(0x10)]
    pub name: String,
}

// 0x11, 0xAF
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

// 0x11, 0xB0
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xB0)]
pub struct Unk11B0Packet {
    pub unk1: u32,
    pub unk2: u32,
}

// 0x11, 0xB8
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xB8)]
pub struct CharacterMoveRequestPacket {
    pub char_id: u32,
    pub unk1: u32,
}

// 0x11, 0xB9
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

// 0x11, 0xD7
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
// 0x11, 0xDE
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xDE)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x60, 0x8F)]
pub struct PlayerReportedPacket {
    pub targed_id: u32,
    pub reason: u8,
    #[Seek(3)]
    pub msg: String,
}

// 0x11, 0xEA
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xEA)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x4544, 0x14)]
pub struct NicknameErrorPacket {
    pub unk1: u32,
    pub nickname: String,
}

// 0x11, 0xED
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xED)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xD67D, 0xF5)]
pub struct BannerListPacket {
    pub banners: AsciiString,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(PacketType::NGS)]
    pub unk1: AsciiString,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(PacketType::NGS)]
    pub unk2: AsciiString,
}

// 0x11, 0xEE
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xEE)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x5C3B, 0x40)]
pub struct EmailCodeRequestPacket {
    pub unk1: u32,
    pub message: String,
}

// 0x11, 0xFF
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x11, 0xFF)]
#[Flags(Flags {packed: true, ..Default::default()})]
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct NetInterface {
    pub state: u32,
    #[FixedStr(0x18)]
    pub mac: AsciiString,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct ShipEntry {
    pub id: u32,
    #[FixedStr(0x10)]
    pub name: String,
    pub ip: Ipv4Addr,
    #[Seek(4)]
    pub status: ShipStatus,
    #[SeekAfter(4)]
    pub order: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
pub enum ShipStatus {
    #[default]
    Unknown,
    Online,
    Busy,
    Full,
    Offline,

    #[Read_default]
    Undefined = 0xFFFF,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct LoginAttempt {
    pub ip: Ipv4Addr,
    pub status: LoginResult,
    pub timestamp: Duration,
    pub unk: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum LoginResult {
    #[default]
    Successful,
    EmailConfirmed,
    LoginError,
    EmailAuthError,
    AuthEmailSent,
    OTPError,
    InMaintenance,
    GenericError,

    #[Read_default]
    Undefined = 0xFFFF_FFFF,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum LoginStatus {
    #[default]
    Success,
    Failure,

    #[Read_default]
    Undefined = 0xFFFF_FFFF,
}

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
    pub block_id: u16,
    #[FixedStr(0x20)]
    pub blockname: String,
    pub ip: Ipv4Addr,
    pub port: u16,
    pub unk10: u16,
    pub unk11: u16,
    pub unk12: [u16; 3],
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum Language {
    #[default]
    #[Read_default]
    Japanese,
    English,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum DeletionStatus {
    #[default]
    #[Read_default]
    UndeletableItems,
    Success,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum UndeletionStatus {
    #[default]
    #[Read_default]
    AlreadyDeleted,
    Success,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum RenameRequestStatus {
    #[default]
    Allowed,
    PermitNeeded,
    PrivilegesSuspended,
    #[Read_default]
    SystemError,
    TooEarly,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum NewNameStatus {
    #[default]
    #[Read_default]
    Success,
    Failure,
}

// ----------------------------------------------------------------
// Read/Write implementations
// ----------------------------------------------------------------

impl PacketReadWrite for CharacterListPacket {
    fn read(
        reader: &mut (impl Read + Seek),
        _: &Flags,
        packet_type: PacketType,
    ) -> std::io::Result<Self> {
        let char_amount = reader.read_u32::<LittleEndian>()?.clamp(0, 30);
        reader.seek(std::io::SeekFrom::Current(4))?;
        let mut characters = vec![];
        for i in 0..30 {
            reader.seek(std::io::SeekFrom::Current(4))?;
            let character = Character::read(reader, packet_type, 0, 0)?;
            if i < char_amount {
                characters.push(character);
            }
        }
        // ???
        reader.seek(std::io::SeekFrom::Current(0x41A4))?;
        let mut play_times = [0u32; 30];
        for item in &mut play_times {
            *item = reader.read_u32::<LittleEndian>()?;
        }
        reader.seek(std::io::SeekFrom::Current(32))?;
        let mut deletion_flags = [(0u32, 0u32); 30];
        for item in &mut deletion_flags {
            item.0 = reader.read_u32::<LittleEndian>()?;
            item.1 = reader.read_u32::<LittleEndian>()?;
        }
        let mut transfer_flags = [(0u32, 0u32); 30];
        for item in &mut transfer_flags {
            item.0 = reader.read_u32::<LittleEndian>()?;
            item.1 = reader.read_u32::<LittleEndian>()?;
        }
        let account_accessory = reader.read_u16::<LittleEndian>()?;
        reader.seek(std::io::SeekFrom::Current(6))?;
        let login_survey = reader.read_u32::<LittleEndian>()?;
        let ad = reader.read_u32::<LittleEndian>()?;

        Ok(Self {
            characters,
            play_times,
            deletion_flags,
            transfer_flags,
            account_accessory,
            login_survey,
            ad,
        })
    }
    fn write(&self, packet_type: PacketType) -> std::io::Result<Vec<u8>> {
        let mut buf = PacketHeader::new(0x11, 0x03, Flags::default()).write(packet_type);
        buf.write_u32::<LittleEndian>((self.characters.len() as u32).clamp(0, 30))?;
        buf.write_u32::<LittleEndian>(0)?;

        let mut characters = &self.characters;
        let default_character = vec![Character::default()];
        if characters.is_empty() {
            characters = &default_character;
        }

        for character in characters.iter().cycle().take(30) {
            buf.write_u32::<LittleEndian>(0)?;
            character.write(&mut buf, packet_type, 0, 0)?;
        }
        // ???
        for _ in 0..0x41A4 {
            buf.write_u8(0)?;
        }
        for i in 0..30 {
            buf.write_u32::<LittleEndian>(self.play_times[i])?;
        }
        // ???
        for _ in 0..32 {
            buf.write_u8(0)?;
        }
        for i in 0..30 {
            // deletion flag
            buf.write_u32::<LittleEndian>(self.deletion_flags[i].0)?;
            // timestamp
            buf.write_u32::<LittleEndian>(self.deletion_flags[i].1)?;
        }
        for i in 0..30 {
            // transfer flag
            buf.write_u32::<LittleEndian>(self.transfer_flags[i].0)?;
            // ??? prob target ship
            buf.write_u32::<LittleEndian>(self.transfer_flags[i].1)?;
        }
        buf.write_u16::<LittleEndian>(self.account_accessory)?;
        // ???
        buf.write_all(&[0u8; 6])?;
        buf.write_u32::<LittleEndian>(self.login_survey)?;
        buf.write_u32::<LittleEndian>(self.ad)?;
        // ???
        buf.write_u32::<LittleEndian>(0x00_00_00_00)?;
        // ???
        buf.write_u32::<LittleEndian>(0x00_00_00_00)?;
        Ok(buf)
    }
}

impl PacketReadWrite for EncryptionRequestPacket {
    fn read(reader: &mut impl Read, _: &Flags, _: PacketType) -> std::io::Result<Self> {
        let mut rsa_data = vec![];
        reader.read_to_end(&mut rsa_data)?;
        let mut tmp_data = vec![];
        let mut iter = rsa_data.into_iter().rev().skip(4);
        if let Some(x) = iter.find(|x| *x != 0x00) {
            tmp_data.push(x);
            tmp_data.extend(iter);
        }
        Ok(Self { rsa_data: tmp_data })
    }
    fn write(&self, packet_type: PacketType) -> std::io::Result<Vec<u8>> {
        let mut buf = PacketHeader::new(0x11, 0x0B, Flags::default()).write(packet_type);
        let mut data = self.rsa_data.clone();
        data.reverse();
        data.resize(0x104, 0);
        buf.extend(data);
        Ok(buf)
    }
}

impl PacketReadWrite for EncryptionResponsePacket {
    fn read(reader: &mut impl Read, _: &Flags, _: PacketType) -> std::io::Result<Self> {
        let mut data = vec![];
        reader.read_to_end(&mut data)?;

        Ok(Self { data })
    }
    fn write(&self, packet_type: PacketType) -> std::io::Result<Vec<u8>> {
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
                entity_type: EntityType::Player,
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
