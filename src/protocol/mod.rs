#![allow(unused_imports)]
pub mod items;
pub mod login;
pub mod mail;
pub mod models;
pub mod objects;
pub mod orders;
pub mod party;
pub mod questlist;
pub mod server;
pub mod spawn;
pub mod symbolart;
use crate::AsciiString;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use half::f16;
use items::*;
use login::*;
use mail::*;
use models::{character::Character, Position};
use objects::*;
use orders::*;
use packetlib_impl::{HelperReadWrite, PacketReadWrite, ProtocolReadWrite};
use party::*;
use questlist::*;
use server::*;
use spawn::*;
use std::{
    io::{Cursor, Read, Seek, Write},
    time::Duration,
};
use symbolart::*;

// Code is getting really messy.

mod private {
    pub trait Sealed: Sized {}
    impl Sealed for crate::protocol::Packet {}
}

/// Read/Write trait for [`Packet`].
///
/// This trait is sealed and cannot be implemented for other types.
pub trait ProtocolRW: private::Sealed {
    /// Read packets from input slice.
    fn read(input: &[u8], is_ngs: bool) -> std::io::Result<Vec<Self>>;
    /// Write a packet to a byte vector.
    fn write(&self, is_ngs: bool) -> Vec<u8>;
    /// Get category of the packet.
    fn get_category(&self) -> PacketCategory;
}

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

/// All known packets
#[cfg(not(feature = "proxy"))]
#[derive(Debug, Default, Clone, PartialEq, ProtocolReadWrite)]
#[non_exhaustive]
pub enum Packet {
    #[default]
    #[Empty]
    None,

    // Server packets [0x03]
    #[Category(PacketCategory::Server)]
    #[Id(0x03, 0x03)]
    InitialLoad,
    #[Id(0x03, 0x04)]
    LoadingScreenTransition,
    #[Id(0x03, 0x08)]
    #[Base]
    ServerHello(ServerHelloPacket),
    #[Id(0x03, 0x08)]
    #[NGS]
    ServerHelloNGS(ServerHelloNGSPacket),
    #[Id(0x03, 0x0B)]
    ServerPing,
    #[Id(0x03, 0x0C)]
    ServerPong,
    #[Id(0x03, 0x10)]
    MapLoaded(MapLoadedPacket),
    #[Id(0x03, 0x23)]
    FinishLoading,
    #[Id(0x03, 0x24)]
    LoadLevel(LoadLevelPacket),
    #[Id(0x03, 0x2B)]
    UnlockControls,

    // Object related packets [0x04]
    #[Category(PacketCategory::Object)]
    #[Id(0x04, 0x02)]
    TeleportTransfer(TeleportTransferPacket),
    // this fails the tests
    #[cfg(not(test))]
    #[Id(0x04, 0x07)]
    Movement(MovementPacket),
    #[Id(0x04, 0x08)]
    MovementAction(MovementActionPacket),
    #[Id(0x04, 0x13)]
    Unk0413(Unk0413Packet),
    #[Id(0x04, 0x14)]
    Interact(InteractPacket),
    #[Id(0x04, 0x15)]
    SetTag(SetTagPacket),
    #[Id(0x04, 0x22)]
    Unk0422(Unk0422Packet),
    #[Id(0x04, 0x23)]
    Unk0423(Unk0423Packet),
    #[Id(0x04, 0x24)]
    Unk0424(Unk0424Packet),
    #[Id(0x04, 0x2B)]
    Unk042B(Unk042BPacket),
    #[Id(0x04, 0x3B)]
    RemoveObject(RemoveObjectPacket),
    #[Id(0x04, 0x3C)]
    ActionUpdate(ActionUpdatePacket),
    #[Id(0x04, 0x52)]
    DamageReceive(DamageReceivePacket),
    #[Id(0x04, 0x71)]
    MovementEnd(MovementEndPacket),
    #[Id(0x04, 0x75)]
    ActionEnd(ActionEndPacket),
    #[Id(0x04, 0x80)]
    MovementActionServer(MovementActionServerPacket),
    #[Id(0x04, 0x81)]
    ActionUpdateServer(ActionUpdateServerPacket),

    #[Category(PacketCategory::Unknown)]
    #[Id(0x06, 0x00)]
    SetPlayerID(SetPlayerIDPacket),
    #[Id(0x06, 0x01)]
    DealDamage(DealDamagePacket),

    #[Id(0x07, 0x00)]
    #[Base]
    ChatMessage(ChatMessage),
    #[Id(0x07, 0x00)]
    #[NGS]
    ChatMessageNGS(ChatMessageNGS),

    // Spawn packets [0x08]
    #[Category(PacketCategory::Spawning)]
    #[Id(0x08, 0x04)]
    #[Base]
    CharacterSpawn(CharacterSpawnPacket),
    #[Id(0x08, 0x09)]
    EventSpawn(EventSpawnPacket),
    #[Id(0x08, 0x0B)]
    ObjectSpawn(ObjectSpawnPacket),
    #[Id(0x08, 0x0D)]
    EnemySpawn(EnemySpawnPacket),
    #[Id(0x08, 0x0C)]
    NPCSpawn(NPCSpawnPacket),

    // Quest List packets [0x0B]
    #[Category(PacketCategory::QuestList)]
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

    // Party packets [0x0E]
    #[Category(PacketCategory::Party)]
    #[Id(0x0E, 0x02)]
    #[Base]
    PartyInit(PartyInitPacket),
    #[Id(0x0E, 0x02)]
    #[NGS]
    PartyInitNGS(PartyInitNGSPacket),
    #[Id(0x0E, 0x0C)]
    NewPartySettings(NewPartySettingsPacket),
    #[Id(0x0E, 0x0D)]
    PartySettings(PartySettingsPacket),
    #[Id(0x0E, 0x19)]
    Unk0E19(Unk0E19Packet),
    #[Id(0x0E, 0x2B)]
    Unk0E2B(Unk0E2BPacket),
    #[Id(0x0E, 0x2C)]
    SetInviteDecline(InviteDeclinePacket),

    // Item packets [0x0F]
    #[Category(PacketCategory::Item)]
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
    #[cfg(not(test))]
    #[Id(0x0F, 0x30)]
    LoadItem(LoadItemPacket),
    #[cfg(test)]
    #[Id(0x0F, 0x30)]
    LoadItem(LoadItemInternal),
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
    #[Id(0x0F, 0xE0)]
    MoveToMaterialStorage(MoveToMaterialStoragePacket),
    #[Id(0x0F, 0xEF)]
    Unk0fef(Unk0fefPacket),
    #[Id(0x0F, 0xFC)]
    Unk0ffc(Unk0ffcPacket),

    #[Id(0x10, 0x00)]
    #[Base]
    RunLua(LuaPacket),

    // Login packets [0x11]
    #[Category(PacketCategory::Login)]
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
    #[Id(0x11, 0x0F)]
    BlockListRequest,
    #[Id(0x11, 0x10)]
    BlockList(BlockListPacket),
    #[Id(0x11, 0x1B)]
    #[NGS]
    UserInfoNGS(UserInfoNGSPacket),
    #[Id(0x11, 0x1B)]
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
    #[Id(0x11, 0x71)]
    NotificationStatus(NotificationStatusPacket),
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
    Unk11FF(Unk11FFPacket),

    #[Category(PacketCategory::Unknown)]
    #[Id(0x19, 0x01)]
    #[Base]
    SystemMessage(SystemMessagePacket),
    #[Id(0x19, 0x0F)]
    LobbyMonitor(LobbyMonitorPacket),

    // Mail packets [0x1A]
    #[Category(PacketCategory::Mail)]
    #[Id(0x1A, 0x00)]
    MailListRequest(MailListRequestPacket),
    #[Id(0x1A, 0x01)]
    MailList(MailListPacket),
    #[Id(0x1A, 0x02)]
    DeleteMailRequest(DeleteMailRequestPacket),
    #[Id(0x1A, 0x03)]
    DeletedMail(DeletedMailPacket),
    #[Id(0x1A, 0x06)]
    MailBodyRequest(MailBodyRequestPacket),
    #[Id(0x1A, 0x07)]
    MailBody(MailBodyPacket),
    #[Id(0x1A, 0x0D)]
    NewMailMarker,

    // Daily order packets [0x1F]
    #[Category(PacketCategory::DailyOrders)]
    #[Id(0x1F, 0x01)]
    TakenOrdersRequest(TakenOrdersRequestPacket),
    #[Id(0x1F, 0x02)]
    OrderListRequest(OrderListRequestPacket),
    #[Id(0x1F, 0x03)]
    OrderList(OrderListPacket),
    #[Id(0x1F, 0x08)]
    TakenOrders(TakenOrdersPacket),

    // Settings packets [0x2B]
    #[Category(PacketCategory::Settings)]
    #[Id(0x2B, 0x00)]
    SettingsRequest,
    #[Id(0x2B, 0x01)]
    SaveSettings(SaveSettingsPacket),
    #[Id(0x2B, 0x02)]
    LoadSettings(LoadSettingsPacket),

    // Symbol art packets [0x2F]
    #[Category(PacketCategory::SymbolArt)]
    #[Id(0x2F, 0x00)]
    SymbolArtClientDataRequest(SymbolArtClientDataRequestPacket),
    #[Id(0x2F, 0x01)]
    SymbolArtDataRequest(SymbolArtDataRequestPacket),
    #[Id(0x2F, 0x02)]
    SymbolArtData(SymbolArtDataPacket),
    #[Id(0x2F, 0x03)]
    SymbolArtClientData(SymbolArtClientDataPacket),
    #[Id(0x2F, 0x04)]
    ChangeSymbolArt(ChangeSymbolArtPacket),
    #[Id(0x2F, 0x05)]
    SymbolArtResult(SymbolArtResultPacket),
    #[Id(0x2F, 0x06)]
    SymbolArtListRequest,
    #[Id(0x2F, 0x07)]
    SymbolArtList(SymbolArtListPacket),
    #[Id(0x2F, 0x08)]
    #[Base]
    SendSymbolArt(SendSymbolArtPacket),
    #[Id(0x2F, 0x09)]
    #[Base]
    ReceiveSymbolArt(ReceiveSymbolArtPacket),

    // ARKS Misions packets [0x4A]
    #[Category(PacketCategory::ARKSMissions)]
    #[Id(0x4A, 0x00)]
    MissionListRequest,
    #[Id(0x4A, 0x01)]
    MissionList(MissionListPacket),
    #[Id(0x4A, 0x03)]
    Unk4A03(Unk4A03Packet),

    // Base Mission Pass packets [0x4D]
    #[Category(PacketCategory::MissionPass)]
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

/// Known packet categories
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum PacketCategory {
    #[default]
    /// Category is unspecified or packet is unknown
    Unknown,
    /// Server related packets. See [`server`]
    Server,
    /// Object related packets. See [`objects`]
    Object,
    /// Spawning related packets. See [`spawn`]
    Spawning,
    /// Quest list related packets. See [`questlist`]
    QuestList,
    /// Party related packets. See [`party`]
    Party,
    /// Item related packets. See [`items`]
    Item,
    /// Login related packets. See [`login`]
    Login,
    /// Mail related packets. See [`mail`]
    Mail,
    /// Daily orders related packets.
    DailyOrders,
    /// Settings related packets.
    Settings,
    /// Symbol Art related packets. See [`symbolart`]
    SymbolArt,
    /// ARKS Missions related packets.
    ARKSMissions,
    /// Classic Mission pass related packets.
    MissionPass,
}

// ----------------------------------------------------------------
// Common structures
// ----------------------------------------------------------------

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PacketHeader {
    pub id: u8,
    pub subid: u16,
    pub flag: Flags,
}
impl PacketHeader {
    fn new(id: u8, subid: u16, flag: Flags) -> Self {
        Self { id, subid, flag }
    }
    fn read(reader: &mut (impl Read + Seek), is_ngs: bool) -> std::io::Result<Self> {
        let (id, subid, flag) = if !is_ngs {
            let id = reader.read_u8()?;
            let subid = reader.read_u8()? as u16;
            let flag = Flags::read(reader)?;
            reader.read_u8()?;
            (id, subid, flag)
        } else {
            let flag = Flags::read(reader)?;
            let id = reader.read_u8()?;
            let subid = reader.read_u16::<LittleEndian>()?;
            (id, subid, flag)
        };

        Ok(Self { id, subid, flag })
    }
    fn write(&self, is_ngs: bool) -> Vec<u8> {
        let mut buf = vec![];
        if !is_ngs {
            buf.write_u8(self.id).unwrap();
            buf.write_u8(self.subid as u8).unwrap();
            self.flag.write(&mut buf).unwrap();
            buf.write_u8(0).unwrap();
        } else {
            self.flag.write(&mut buf).unwrap();
            buf.write_u8(self.id).unwrap();
            buf.write_u16::<LittleEndian>(self.subid).unwrap();
        }
        buf
    }
}

#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[Flags(u8)]
pub struct Flags {
    #[Skip]
    #[Skip]
    // 0x04
    pub packed: bool,
    #[Skip]
    // 0x10
    pub flag10: bool,
    pub full_movement: bool,
    pub object_related: bool,
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
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
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

// 0x06, 0x01
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x06, 0x01)]
pub struct DealDamagePacket {
    pub inflicter: ObjectHeader,
    pub target: ObjectHeader,
    pub attack_id: u32,
    pub unk2: u64,
    pub hitbox_id: u32,
    pub x_pos: f16,
    pub y_pos: f16,
    pub z_pos: f16,

    pub unk4: u16,
    pub unk5: u64,
    pub unk6: [u8; 0x18],
}

// 0x07, 0x00
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x07, 0x00)]
#[Flags(Flags {packed: true, object_related: true, ..Default::default()})]
pub struct ChatMessage {
    pub object: ObjectHeader,
    pub area: ChatArea,
    pub unk3: u8,
    pub unk4: u16,
    #[VariableStr(0x9D3F, 0x44)]
    pub unk5: String,
    #[VariableStr(0x9D3F, 0x44)]
    pub message: String,
}

// 0x07, 0x00
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x07, 0x00)]
#[Flags(Flags {packed: true, object_related: true, ..Default::default()})]
pub struct ChatMessageNGS {
    pub object: ObjectHeader,
    pub unk2: ChatArea,
    pub unk3: u8,
    pub unk4: u16,
    pub unk5: u16,
    pub unk6: u16,
    #[VariableStr(0x9D3F, 0x44)]
    pub unk7: String,
    #[VariableStr(0x9D3F, 0x44)]
    pub message: String,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
pub enum ChatArea {
    #[default]
    Map,
    Party,
    // the following is only speculation
    Alliance,
    Whisper,
    Group,

    #[Read_default]
    Undefined = 0xFF,
}

// 0x19, 0x01
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x19, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SystemMessagePacket {
    #[VariableStr(0x78F7, 0xA2)]
    pub message: String,
    #[VariableStr(0x78F7, 0xA2)]
    pub unk: String,
    pub msg_type: MessageType,
    pub msg_num: u32,
}

// 0x19, 0x0F
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x19, 0x0F)]
pub struct LobbyMonitorPacket {
    pub video_id: u32,
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
    #[VariableStr(0xB0C, 0x35)]
    pub cur_season: String,
    pub stars_to_next_tier: u32,
    pub tiers: u32,
    pub overrun_tiers: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub end_date: u32,
    pub unk10: u32,
    pub unk11: u32,
    #[VariableStr(0xB0C, 0x35)]
    pub unk12: String,
    pub price_per_tier: u32,
    pub gold_pass_price: u32,
    #[Magic(0xB0C, 0x35)]
    pub unk15: Vec<MissionPassItem>,
    pub unk16: u32,
    #[VariableStr(0xB0C, 0x35)]
    pub last_season: String,
    pub unk18: u32,
    pub unk19: u32,
    pub unk20: u32,
    pub unk21: u32,
    pub unk22: u32,
    pub unk23: u32,
    pub unk24: u32,
    pub unk25: u32,
    #[VariableStr(0xB0C, 0x35)]
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
    pub tier: u32,
    pub is_gold: u32,
    pub unk4: u32,
    pub group: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: ItemId,
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
#[Id(0x10, 0x00)]
pub struct LuaPacket {
    pub unk1: u16,
    pub unk2: u16,
    // #[VariableStr(0xD975, 0x2F)]
    #[VariableStr(0, 0)]
    pub lua: AsciiString,
}

// ----------------------------------------------------------------
// Settings packets
// ----------------------------------------------------------------

// 0x2B, 0x01
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2B, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SaveSettingsPacket {
    #[VariableStr(0xCEF1, 0xB5)]
    pub settings: AsciiString,
}

// 0x2B, 0x02
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2B, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct LoadSettingsPacket {
    #[VariableStr(0x54AF, 0x100)]
    pub settings: AsciiString,
}

// ----------------------------------------------------------------
// Utils
// ----------------------------------------------------------------
pub(crate) fn read_magic(reader: &mut impl Read, sub: u32, xor: u32) -> std::io::Result<u32> {
    let num = reader.read_u32::<LittleEndian>()?;
    Ok((num ^ xor) - sub)
}
pub(crate) fn write_magic(num: u32, sub: u32, xor: u32) -> u32 {
    (num + sub) ^ xor
}
fn psotime_to_duration(timestamp: u64) -> Duration {
    const UNIX_TIME: u64 = 0x0295_E964_8864;
    Duration::from_millis(timestamp - UNIX_TIME)
}
fn duration_to_psotime(time: Duration) -> u64 {
    const UNIX_TIME: u64 = 0x0295_E964_8864;
    time.as_millis() as u64 + UNIX_TIME
}

// ----------------------------------------------------------------
// Tests
// ----------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::{fs, io::Write, path::PathBuf};

    use crate::protocol::{Packet, ProtocolRW};
    #[test]
    fn file_check() {
        // this is hard to test, because original server doesn't clear output buffers
        let mut failed_paths = vec![];
        traverse_dir("test_data/ngs", true, &mut failed_paths);
        traverse_dir("test_data/base", false, &mut failed_paths);
        if !failed_paths.is_empty() {
            println!("Fails:");
            for item in failed_paths {
                println!("{:?}", item);
            }
            panic!();
        }
    }

    fn traverse_dir<T: AsRef<std::path::Path>>(
        path: T,
        is_ngs: bool,
        failed_paths: &mut Vec<PathBuf>,
    ) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap().path();
            if entry.is_dir() {
                traverse_dir(entry, is_ngs, failed_paths);
            } else if entry.is_file() {
                print!("Testing: {:?} - ", entry);
                let in_data = fs::read(&entry).unwrap();
                let packet = Packet::read(&in_data, is_ngs).unwrap();
                let out_data = packet[0].write(is_ngs);
                if in_data.len() != out_data.len() {
                    println!(
                        "FAIL (different length - in: 0x{:X}, out: 0x{:X})",
                        in_data.len(),
                        out_data.len()
                    );
                    fs::File::create(format!(
                        "failed_tests/len_{}",
                        entry.file_name().unwrap().to_string_lossy()
                    ))
                    .unwrap()
                    .write_all(&out_data)
                    .unwrap();

                    failed_paths.push(entry.to_path_buf());
                } else if in_data != out_data {
                    println!("FAIL (different data)");
                    let _ = fs::create_dir("failed_tests");
                    fs::File::create(format!(
                        "failed_tests/data_{}",
                        entry.file_name().unwrap().to_string_lossy()
                    ))
                    .unwrap()
                    .write_all(&out_data)
                    .unwrap();
                    failed_paths.push(entry.to_path_buf());
                } else {
                    println!("PASS")
                }
            }
        }
    }
}
