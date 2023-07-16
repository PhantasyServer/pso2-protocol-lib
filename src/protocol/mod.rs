#![allow(unused_imports)]
pub mod items;
pub mod login;
pub mod models;
pub mod objects;
pub mod questlist;
pub mod server;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use items::*;
use login::*;
use objects::*;
use packetlib_impl::{HelperReadWrite, PacketReadWrite, ProtocolReadWrite};
use questlist::*;
use server::*;
use std::{
    io::{Cursor, Read, Seek, Write},
    time::Duration,
};

use self::models::{character::Character, Position};

// Code is getting really messy.

pub(crate) trait PacketReadWrite: Sized {
    /// Read a packet from stream.
    fn read(reader: &mut (impl Read + Seek), flags: Flags) -> std::io::Result<Self>;
    /// Write a packet to a Vec.
    fn write(&self, is_ngs: bool) -> Vec<u8>;
}

pub(crate) trait HelperReadWrite: Sized {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self>;
    fn write(&self, writer: &mut impl Write) -> std::io::Result<()>;
}

#[cfg(not(feature = "proxy"))]
#[derive(Debug, Default, Clone, PartialEq, ProtocolReadWrite)]
#[non_exhaustive]
pub enum Packet {
    #[default]
    #[Empty]
    None,

    // Server packets [0x03]
    #[Id(0x03, 0x03)]
    InitialLoad,
    #[Id(0x03, 0x04)]
    LoadingScreenTransition,
    #[Id(0x03, 0x08)]
    ServerHello(ServerHelloPacket),
    #[Id(0x03, 0x0B)]
    ServerPing,
    #[Id(0x03, 0x0C)]
    ServerPong,
    #[Id(0x03, 0x23)]
    FinishLoading,
    #[Id(0x03, 0x24)]
    LoadLevel(LoadLevelPacket),
    #[Id(0x03, 0x2B)]
    UnlockControls,

    // Object related packets [0x04]
    #[Id(0x04, 0x02)]
    TeleportTransfer(TeleportTransferPacket),
    #[Id(0x04, 0x07)]
    Movement(MovementPacket),
    #[Id(0x04, 0x08)]
    MovementAction(MovementActionPacket),
    #[Id(0x04, 0x13)]
    Unk4_13(Unk4_13Packet),
    #[Id(0x04, 0x14)]
    Interact(InteractPacket),
    #[Id(0x04, 0x15)]
    SetTag(SetTagPacket),
    #[Id(0x04, 0x24)]
    Unk4_24(Unk4_24Packet),
    #[Id(0x04, 0x71)]
    MovementEnd(MovementEndPacket),

    #[Id(0x06, 0x00)]
    SetPlayerID(SetPlayerIDPacket),

    #[Id(0x07, 0x00)]
    #[Base]
    ChatMessage(ChatMessage),
    #[Id(0x07, 0x00)]
    #[NGS]
    ChatMessageNGS(ChatMessageNGS),

    // Spawn packets [0x08]
    #[Id(0x08, 0x04)]
    #[Base]
    CharacterSpawn(CharacterSpawnPacket),
    #[Id(0x08, 0x09)]
    EventSpawn(EventSpawnPacket),
    #[Id(0x08, 0x0B)]
    ObjectSpawn(ObjectSpawnPacket),
    #[Id(0x08, 0x0C)]
    NPCSpawn(NPCSpawnPacket),

    // Quest List packets [0x0B]
    #[Id(0x0B, 0x09)]
    Unk0B09(Unk0B09Packet),
    #[Id(0x0B, 0x15)]
    AvailableQuestsRequest(AvailableQuestsRequestPacket),
    #[Id(0x0B, 0x16)]
    AvailableQuests(AvailableQuestsPacket),
    #[Id(0x0B, 0x17)]
    QuestCategoryRequest(QuestCategoryRequestPacket),
    #[Id(0x0B, 0x18)]
    QuestCategory(QuestCategoryPacket),
    #[Id(0x0B, 0x30)]
    QuestCounterRequest,
    #[Id(0x0B, 0xAF)]
    Unk0BAF(Unk0BAFPacket),

    // Item packets [0x0F]
    #[Id(0x0F, 0x00)]
    LoadItemAttributes(ItemAttributesPacket),
    #[Id(0x0F, 0x0D)]
    #[Base]
    LoadPlayerInventory(LoadPlayerInventoryPacket),
    #[Id(0x0F, 0x0D)]
    #[NGS]
    LoadPlayerInventoryNGS(LoadPlayerInventoryNGSPacket),
    #[Id(0x0F, 0x13)]
    #[Base]
    LoadStorages(LoadStoragesPacket),
    #[Id(0x0F, 0x13)]
    #[NGS]
    LoadStoragesNGS(LoadStoragesNGSPacket),
    #[Id(0x0F, 0x1C)]
    GetItemDescription(GetItemDescriptionPacket),
    #[Id(0x0F, 0x1D)]
    LoadItemDescription(LoadItemDescriptionPacket),
    #[Id(0x0F, 0x30)]
    LoadItem(LoadItemPacket),
    #[Id(0x0F, 0x6F)]
    AccountCapaignsRequest,
    #[Id(0x0F, 0x70)]
    AccountCapaigns(AccountCapaignsPacket),
    #[Id(0x0F, 0x71)]
    CampaignItemsRequest(CampaignItemsRequestPacket),
    #[Id(0x0F, 0x72)]
    CampaignItemList(CampaignItemListPacket),
    #[Id(0x0F, 0x73)]
    ReceiveCampaignRequest(ReceiveCampaignRequestPacket),
    #[Id(0x0F, 0x9C)]
    Unk0f9c(Unk0f9cPacket),
    #[Id(0x0F, 0xDF)]
    LoadMaterialStorage(LoadMaterialStoragePacket),
    #[Id(0x0f, 0xE0)]
    MoveToMaterialStorage(MoveToMaterialStoragePacket),
    #[Id(0x0F, 0xEF)]
    Unk0fef(Unk0fefPacket),
    #[Id(0x0F, 0xFC)]
    Unk0ffc(Unk0ffcPacket),

    #[Id(0x10, 0x03)]
    #[Base]
    RunLua(LuaPacket),

    // Login packets [0x11]
    #[Id(0x11, 0x00)]
    #[Base]
    SegaIDLogin(SegaIDLoginPacket),
    #[Id(0x11, 0x01)]
    #[Base]
    LoginResponse(LoginResponsePacket),
    #[Id(0x11, 0x02)]
    CharacterListRequest,
    #[Id(0x11, 0x03)]
    #[Base]
    CharacterListResponse(CharacterListPacket),
    #[Id(0x11, 0x04)]
    #[Base]
    StartGame(StartGamePacket),
    #[Id(0x11, 0x05)]
    #[Base]
    CharacterCreate(CharacterCreatePacket),
    #[Id(0x11, 0x0B)]
    EncryptionRequest(EncryptionRequestPacket),
    #[Id(0x11, 0x0C)]
    EncryptionResponse(EncryptionResponsePacket),
    #[Id(0x11, 0x0D)]
    ClientPing(ClientPingPacket),
    #[Id(0x11, 0x0E)]
    ClientPong(ClientPongPacket),
    #[Id(0x11, 0x1B)]
    #[NGS]
    UserInfoNGS(UserInfoNGSPacket),
    #[Id(0x11, 0x1b)]
    #[Base]
    UserInfo(UserInfoPacket),
    #[Id(0x11, 0x1E)]
    NicknameRequest(NicknameRequestPacket),
    #[Id(0x11, 0x1D)]
    NicknameResponse(NicknameResponsePacket),
    #[Id(0x11, 0x2B)]
    ClientGoodbye,
    #[Id(0x11, 0x2C)]
    #[Base]
    BlockBalance(BlockBalancePacket),
    #[Id(0x11, 0x2D)]
    SystemInformation(SystemInformationPacket),
    #[Id(0x11, 0x3D)]
    ShipList(ShipListPacket),
    #[Id(0x11, 0x41)]
    CreateCharacter1,
    #[Id(0x11, 0x42)]
    CreateCharacter1Response(CreateCharacter1ResponsePacket),
    #[Id(0x11, 0x54)]
    CreateCharacter2,
    #[Id(0x11, 0x55)]
    CreateCharacter2Response(CreateCharacter2ResponsePacket),
    #[Id(0x11, 0x63)]
    #[Base]
    VitaLogin(VitaLoginPacket),
    #[Id(0x11, 0x66)]
    SalonEntryRequest,
    #[Id(0x11, 0x67)]
    #[Base]
    SalonEntryResponse(SalonResponse),
    #[Id(0x11, 0x6B)]
    #[Base]
    SegaIDInfoRequest,
    #[Id(0x11, 0x86)]
    LoginHistoryRequest,
    #[Id(0x11, 0x87)]
    LoginHistoryResponse(LoginHistoryPacket),
    #[Id(0x11, 0xEA)]
    NicknameError(NicknameErrorPacket),
    #[Id(0x11, 0xED)]
    #[Base]
    BannerList(BannerListPacket),
    #[Id(0x11, 0xEE)]
    EmailCodeRequest(EmailCodeRequestPacket),
    #[Id(0x11, 0xFF)]
    #[Base]
    Unk1(Unk1Packet),

    #[Id(0x19, 0x01)]
    #[Base]
    SystemMessage(SystemMessagePacket),

    //Settings packets [0x2B]
    #[Id(0x2B, 0x00)]
    SettingsRequest,
    #[Id(0x2B, 0x01)]
    SaveSettings(SaveSettingsPacket),
    #[Id(0x2B, 0x02)]
    LoadSettings(LoadSettingsPacket),

    // Symbol art packets [0x2F]
    #[Id(0x2F, 0x01)]
    SymbolArtDataRequest(SymbolArtDataRequestPacket),
    #[Id(0x2F, 0x02)]
    SymbolArtData(SymbolArtDataPacket),
    #[Id(0x2F, 0x06)]
    SymbolArtListRequest,
    #[Id(0x2F, 0x07)]
    SymbolArtList(SymbolArtListPacket),

    #[Id(0x4A, 0x00)]
    MissionListRequest,
    #[Id(0x4A, 0x01)]
    MissionList(MissionListPacket),
    #[Id(0x4A, 0x03)]
    Unk4A03(Unk4A03Packet),

    #[Id(0x4D, 0x00)]
    MissionPassInfoRequest,
    #[Id(0x4D, 0x01)]
    MissionPassInfo(MissionPassInfoPacket),
    #[Id(0x4D, 0x02)]
    MissionPassRequest,
    #[Id(0x4D, 0x03)]
    MissionPass(MissionPassPacket),

    //Other packets
    #[Unknown]
    Unknown((PacketHeader, Vec<u8>)),
}

#[cfg(feature = "proxy")]
#[derive(Debug, Default, Clone, PartialEq, ProtocolReadWrite)]
// bare minimum specifically for proxies
pub enum Packet {
    // TODO: we need to implement other "server changing" packets
    #[default]
    #[Empty]
    None,
    #[Id(0x11, 0x0B)]
    EncryptionRequest(EncryptionRequestPacket),
    #[Id(0x11, 0x0C)]
    EncryptionResponse(EncryptionResponsePacket),
    #[Id(0x11, 0x3D)]
    ShipList(ShipListPacket),
    #[Unknown]
    Unknown((PacketHeader, Vec<u8>)),
}

// ----------------------------------------------------------------
// Common structures
// ----------------------------------------------------------------

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PacketHeader {
    pub id: u8,
    pub subid: u16,
    pub flag1: Flags,
    pub unk: u8,
}
impl PacketHeader {
    fn new(id: u8, subid: u16, flag1: Flags) -> Self {
        Self {
            id,
            subid,
            flag1,
            unk: 0,
        }
    }
    fn read(reader: &mut (impl Read + Seek), is_ngs: bool) -> std::io::Result<Self> {
        let (id, subid, flag1, unk) = if !is_ngs {
            let id = reader.read_u8()?;
            let subid = reader.read_u8()? as u16;
            let flag1 = Flags::read(reader)?;
            let unk = reader.read_u8()?;
            (id, subid, flag1, unk)
        } else {
            let flag1 = Flags::read(reader)?;
            let id = reader.read_u8()?;
            let subid = reader.read_u16::<LittleEndian>()?;
            let unk = 0;
            (id, subid, flag1, unk)
        };

        Ok(Self {
            id,
            subid,
            flag1,
            unk,
        })
    }
    fn write(&self, is_ngs: bool) -> Vec<u8> {
        let mut buf = vec![];
        if !is_ngs {
            buf.write_u8(self.id).unwrap();
            buf.write_u8(self.subid as u8).unwrap();
            self.flag1.write(&mut buf).unwrap();
            buf.write_u8(self.unk).unwrap();
        } else {
            self.flag1.write(&mut buf).unwrap();
            buf.write_u8(self.id).unwrap();
            buf.write_u16::<LittleEndian>(self.subid).unwrap();
            // buf.write_u8(self.flag2 as u8).unwrap();
        }
        buf
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Flags {
    pub packed: bool,
    pub flag10: bool,
    pub full_movement: bool,
    pub object_related: bool,
}
impl HelperReadWrite for Flags {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let mut flags = Self::default();
        let mut num = reader.read_u8()?;
        if num & 0x40 != 0 {
            flags.object_related = true;
            num -= 0x40;
        }
        if num & 0x20 != 0 {
            flags.full_movement = true;
            num -= 0x20;
        }
        if num & 0x10 != 0 {
            flags.flag10 = true;
            num -= 0x10;
        }
        if num & 0x4 != 0 {
            flags.packed = true;
            num -= 0x4;
        }
        if num != 0 {
            println!("Unknown flags: {num}");
        }
        Ok(flags)
    }
    fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        let mut num = 0;
        if self.packed {
            num += 0x4;
        }
        if self.flag10 {
            num += 0x10;
        }
        if self.full_movement {
            num += 0x20;
        }
        if self.object_related {
            num += 0x40;
        }
        writer.write_u8(num)?;
        Ok(())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u16)]
pub enum EntityType {
    #[default]
    Unknown = 0,
    Player = 4,
    Map = 5,
    Object = 6,
    Unk1 = 7,
    Unk2 = 22,

    #[Read_default]
    Undefined = 0xFFFF,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct ObjectHeader {
    pub id: u32,
    // #[Seek(4)]
    pub unk: u32,
    // #[SeekAfter(2)]
    pub entity_type: EntityType,
    pub unk2: u16,
}

// ----------------------------------------------------------------
// Packets
// ----------------------------------------------------------------

// 0x06, 0x00
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x06, 0x00)]
pub struct SetPlayerIDPacket {
    pub player_id: u32,
    pub unk1: u32,
    pub unk2: u32,
}

// 0x07, 0x00
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x07, 0x00)]
#[Flags(Flags {packed: true, object_related: true, ..Default::default()})]
pub struct ChatMessage {
    pub object: ObjectHeader,
    pub area: u8,
    pub unk3: u8,
    pub unk4: u16,
    #[VariableUtf16(0x9D3F, 0x44)]
    pub unk5: String,
    #[VariableUtf16(0x9D3F, 0x44)]
    pub message: String,
}

// 0x07, 0x00
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x07, 0x00)]
#[Flags(Flags {packed: true, object_related: true, ..Default::default()})]
pub struct ChatMessageNGS {
    pub object: ObjectHeader,
    pub unk2: u8,
    pub unk3: u8,
    pub unk4: u16,
    pub unk5: u16,
    pub unk6: u16,
    #[VariableUtf16(0x9D3F, 0x44)]
    pub unk7: String,
    #[VariableUtf16(0x9D3F, 0x44)]
    pub message: String,
}

//0x08, 0x04
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x04)]
pub struct CharacterSpawnPacket {
    // unsure about real structure
    pub player_obj: ObjectHeader,
    pub position: Position,
    pub unk1: u16, // padding?
    #[FixedAscii(0x20)]
    pub unk2: String,
    pub unk3: u16,
    pub unk4: u16,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub is_me: u32, //47 - me, 39 - otherwise
    pub character: Character,
    pub is_global: bool,
    #[FixedUtf16(0x20)]
    pub unk9: String, // title?
    pub unk10: u32,
    pub unk11: u32, // gmflag?
    #[FixedUtf16(0x10)]
    pub nickname: String,
    pub unk12: [u8; 0x40],
}
impl Default for CharacterSpawnPacket {
    fn default() -> Self {
        Self {
            player_obj: ObjectHeader {
                id: 0,
                unk: 0,
                unk2: 0,
                entity_type: EntityType::Player,
            },
            position: Position {
                rot_x: half::f16::from_bits(0),
                rot_y: half::f16::from_bits(15360),
                rot_z: half::f16::from_bits(0),
                rot_w: half::f16::from_bits(0),
                pos_x: half::f16::from_bits(14892),
                pos_y: half::f16::from_bits(0),
                pos_z: half::f16::from_bits(22589),
            },
            unk1: 0,
            unk2: "Character".to_string(),
            unk3: 1,
            unk4: 0,
            unk5: 602,
            unk6: 1,
            unk7: 53,
            unk8: 0,
            is_me: 47,
            character: Character::default(),
            is_global: true,
            unk9: String::new(),
            unk10: 0,
            unk11: 0,
            nickname: String::new(),
            unk12: [0u8; 0x40],
        }
    }
}

// 0x08, 0x09
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x09)]
pub struct EventSpawnPacket {
    pub object: ObjectHeader,
    pub position: Position,
    pub unk1: u16,
    #[FixedAscii(0x20)]
    pub name: String,
    pub unk2: u32,
    pub unk3: [u8; 0xC],
    pub unk4: u16,
    pub unk5: u16,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
    pub flags: u32,
    #[Len_u32]
    pub data: Vec<u32>,
}

// 0x08, 0x0B
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x0B)]
pub struct ObjectSpawnPacket {
    pub object: ObjectHeader,
    pub position: Position,
    pub unk1: u16,
    #[FixedAscii(0x34)]
    pub name: String,
    pub flags: u32,
    #[Len_u32]
    pub data: Vec<u32>,
}

// 0x08, 0x0C
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x0C)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct NPCSpawnPacket {
    pub object: ObjectHeader,
    pub position: Position,
    pub unk1: u16,
    #[FixedAscii(0x20)]
    pub name: String,
    pub unk2: u32,
    pub unk3: [u8; 0xC],
    pub unk4: u16,
    pub unk5: u16,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    #[VariableAscii(0x9FCD, 0xE7)]
    pub unk13: String,
}

// 0x19, 0x01
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x19, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SystemMessagePacket {
    #[VariableUtf16(0x78F7, 0xA2)]
    pub message: String,
    #[VariableUtf16(0x78F7, 0xA2)]
    pub unk: String,
    pub msg_type: MessageType,
    pub msg_num: u32,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
pub enum MessageType {
    AdminMessage = 1,
    AdminMessageInstant,
    #[default]
    SystemMessage,
    GoldenMessage,
    EventInformationYellow,
    EventInformationGreen,
    ImportantMessage,
    PopupMessage,

    #[Read_default]
    Undefined = 0xFFFF_FFFF,
}

// 0x2F, 0x01
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x01)]
pub struct SymbolArtDataRequestPacket {
    pub uuid: u128,
}

// 0x2F, 0x02
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SymbolArtDataPacket {
    pub uuid: u128,
    #[Magic(0x4B58, 0x76)]
    pub data: Vec<u8>,
    #[VariableUtf16(0x4B58, 0x76)]
    pub name: String,
}

// 0x2F, 0x07
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x07)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SymbolArtListPacket {
    pub object: ObjectHeader,
    pub unk1: u32,
    #[Magic(0xE80C, 0xED)]
    pub uuids: Vec<u128>,
}

// 0x4A, 0x01
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x4A, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct MissionListPacket {
    pub unk1: u32,
    #[Magic(0xC691, 0x47)]
    pub missions: Vec<Mission>,
    pub daily_update: u32,
    pub weekly_update: u32,
    pub tier_update: u32,
}
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
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

// 0x4A, 0x03
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x4A, 0x03)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct Unk4A03Packet {
    pub unk1: u32,
    #[Magic(0xD20D, 0xDD)]
    pub unk2: Vec<Mission>,
    #[Magic(0xD20D, 0xDD)]
    pub unk3: Vec<u32>,
    #[Magic(0xD20D, 0xDD)]
    pub unk4: Vec<Unk2Struct>,
    pub unk5: u32,
}
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct Unk2Struct {
    pub unk1: [u32; 0x40],
}

// 0x4D, 0x01
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x4D, 0x01)]
pub struct MissionPassInfoPacket {
    pub unk1: [u32; 47],
}

// 0x4D, 0x03
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x4D, 0x03)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct MissionPassPacket {
    pub unk1: u32,
    pub unk2: u32,
    #[VariableUtf16(0xB0C, 0x35)]
    pub cur_season: String,
    pub stars_to_next_tier: u32,
    pub tiers: u32,
    pub overrun_tiers: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub end_date: u32,
    pub unk10: u32,
    pub unk11: u32,
    #[VariableUtf16(0xB0C, 0x35)]
    pub unk12: String,
    pub price_per_tier: u32,
    pub gold_pass_price: u32,
    #[Magic(0xB0C, 0x35)]
    pub unk15: Vec<MissionPassItem>,
    pub unk16: u32,
    #[VariableUtf16(0xB0C, 0x35)]
    pub last_season: String,
    pub unk18: u32,
    pub unk19: u32,
    pub unk20: u32,
    pub unk21: u32,
    pub unk22: u32,
    pub unk23: u32,
    pub unk24: u32,
    pub unk25: u32,
    #[VariableUtf16(0xB0C, 0x35)]
    pub unk26: String,
    pub unk27: u32,
    pub unk28: u32,
    #[Magic(0xB0C, 0x35)]
    pub unk29: Vec<MissionPassItem>,
    pub unk30: u32,
    pub unk31: u32,
}
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct MissionPassItem {
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
    pub unk15: u32,
    pub unk16: u32,
    pub unk17: u32,
    pub unk18: u32,
    pub unk19: u32,
    pub unk20: u32,
    pub unk21: u32,
    pub unk22: u32,
}

#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x10, 0x03)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct LuaPacket {
    pub unk1: u16,
    pub unk2: u16,
    #[VariableAscii(0xD975, 0x2F)]
    pub lua: String,
}

// ----------------------------------------------------------------
// Settings packets
// ----------------------------------------------------------------

// 0x2B, 0x01
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2B, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SaveSettingsPacket {
    #[VariableAscii(0xCEF1, 0xB5)]
    pub settings: String,
}

// 0x2B, 0x02
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2B, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct LoadSettingsPacket {
    #[VariableAscii(0x54AF, 0x100)]
    pub settings: String,
}

// ----------------------------------------------------------------
// Utils
// ----------------------------------------------------------------
fn read_magic(reader: &mut impl Read, sub: u32, xor: u32) -> std::io::Result<u32> {
    let num = reader.read_u32::<LittleEndian>()?;
    Ok((num ^ xor) - sub)
}
fn write_magic(num: u32, sub: u32, xor: u32) -> u32 {
    (num + sub) ^ xor
}
fn read_utf16(reader: &mut impl Read, len: u64) -> String {
    let len = len * 2;
    let mut buf = vec![];
    reader.take(len).read_to_end(&mut buf).unwrap();
    let buf = match buf.len() % 2 {
        0 => &buf,
        _ => &buf[..buf.len() - 1],
    };
    let mut words = vec![];
    for word in buf.chunks(2) {
        words.push(u16::from_le_bytes(word.try_into().unwrap()))
    }
    let mut string = String::from_utf16_lossy(&words);
    if let Some(x) = string.find('\0') {
        string.replace_range(x.., "");
    }
    string
}
fn read_variable_utf16(reader: &mut impl Read, sub: u32, xor: u32) -> String {
    let magic_len = read_magic(reader, sub, xor).unwrap() as u64;
    if magic_len == 0 {
        return String::new();
    }
    let len = magic_len;
    let padding = magic_len & 1;
    read_utf16(reader, len + padding)
}
fn write_utf16(string: &str, len: usize) -> Vec<u8> {
    let mut buf = vec![];
    let string = string
        .chars()
        .take(len - 1)
        .chain("\0".chars().cycle())
        .take(len)
        .collect::<String>();
    for word in string.encode_utf16() {
        buf.extend(word.to_le_bytes())
    }
    buf
}
fn write_variable_utf16(string: &str, sub: u32, xor: u32) -> Vec<u8> {
    let mut buf = vec![];
    if string.is_empty() {
        buf.write_u32::<LittleEndian>(write_magic(0, sub, xor))
            .unwrap();
        return buf;
    }
    let length = string.len() + 1;
    let padding = length & 1;
    buf.write_u32::<LittleEndian>(write_magic(length as u32, sub, xor))
        .unwrap();
    buf.write_all(&write_utf16(string, length + padding))
        .unwrap();
    buf
}
fn read_utf8(reader: &mut impl Read, len: u64) -> String {
    let mut buf = vec![];
    reader.take(len).read_to_end(&mut buf).unwrap();
    let mut string = String::from_utf8_lossy(&buf).to_string();
    if let Some(x) = string.find('\0') {
        string.replace_range(x.., "");
    }
    string
}
fn read_variable_utf8(reader: &mut impl Read, sub: u32, xor: u32) -> String {
    let magic_len = read_magic(reader, sub, xor).unwrap() as u64;
    if magic_len == 0 {
        return String::new();
    }
    let len = magic_len - 1;
    let padding = 4 - (len & 3);
    read_utf8(reader, len + padding)
}
fn write_utf8(string: &str, len: usize) -> Vec<u8> {
    let string = string
        .chars()
        .filter(char::is_ascii)
        .take(len - 1)
        .chain("\0".chars().cycle())
        .take(len)
        .collect::<String>();
    let mut buf = vec![];
    buf.extend(string.bytes());
    buf
}
fn write_variable_utf8(string: &str, sub: u32, xor: u32) -> Vec<u8> {
    let mut buf = vec![];
    if string.is_empty() {
        buf.write_u32::<LittleEndian>(write_magic(0, sub, xor))
            .unwrap();
        return buf;
    }
    let length = string.len();
    let padding = 4 - (length & 3);
    buf.write_u32::<LittleEndian>(write_magic(length as u32 + 1, sub, xor))
        .unwrap();
    buf.write_all(&write_utf8(string, length + padding))
        .unwrap();
    buf
}
fn psotime_to_duration(timestamp: u64) -> Duration {
    const UNIX_TIME: u64 = 0x0029_5E96_4886_4000;
    let timestamp = timestamp * 1000;
    Duration::from_micros(timestamp - UNIX_TIME)
}
fn duration_to_psotime(time: Duration) -> u64 {
    const UNIX_TIME: u64 = 0x0029_5E96_4886_4000;
    let timestamp = time.as_micros() as u64 + UNIX_TIME;
    timestamp / 1000
}
