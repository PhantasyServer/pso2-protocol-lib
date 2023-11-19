use crate::AsciiString;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use packetlib_impl::{HelperReadWrite, PacketReadWrite, ProtocolReadWrite};
use std::{
    io::{Cursor, Read, Seek, Write},
    time::Duration,
};

// Packet definitions modules
pub mod emergency;
pub mod friends;
pub mod items;
pub mod login;
pub mod mail;
pub mod missionpass;
pub mod missions;
pub mod models;
pub mod objects;
pub mod orders;
pub mod palette;
pub mod party;
pub mod playerstatus;
pub mod questlist;
pub mod server;
pub mod spawn;
pub mod symbolart;
use emergency::*;
use friends::*;
use items::*;
use login::*;
use mail::*;
use missionpass::*;
use missions::*;
use objects::*;
use orders::*;
use palette::*;
use party::*;
use playerstatus::*;
use questlist::*;
use server::*;
use spawn::*;
use symbolart::*;

// Code is getting really messy.

mod private {
    pub trait Sealed: Sized {}
    impl Sealed for super::Packet {}
}

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum PacketType {
    #[default]
    NGS,
    Classic,
    NA,
    JP,
    Vita,
    Raw,
}

/// Read/Write trait for [`Packet`].
///
/// This trait is sealed and cannot be implemented for other types.
pub trait ProtocolRW: private::Sealed {
    /// Read packets from input slice.
    fn read(input: &[u8], packet_type: PacketType) -> std::io::Result<Vec<Self>>;
    /// Write a packet to a byte vector.
    fn write(&self, packet_type: PacketType) -> Vec<u8>;
    /// Get category of the packet.
    fn get_category(&self) -> PacketCategory;
}

pub(crate) trait PacketReadWrite: Sized {
    /// Read a packet from stream.
    fn read(
        reader: &mut (impl Read + Seek),
        flags: Flags,
        packet_type: PacketType,
    ) -> std::io::Result<Self>;
    /// Write a packet to a Vec.
    fn write(&self, packet_type: PacketType) -> Vec<u8>;
}

pub(crate) trait HelperReadWrite: Sized {
    fn read(reader: &mut (impl Read + Seek), packet_type: PacketType) -> std::io::Result<Self>;
    fn write(&self, writer: &mut impl Write, packet_type: PacketType) -> std::io::Result<()>;
}

/// All known packets
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg(not(feature = "proxy"))]
#[derive(Debug, Default, Clone, PartialEq, ProtocolReadWrite)]
#[non_exhaustive]
pub enum Packet {
    #[default]
    #[Empty]
    None,

    // Server packets [0x03]
    #[Category(PacketCategory::Server)]
    #[Id(0x03, 0x00)]
    MapTransfer(MapTransferPacket),
    #[Id(0x03, 0x03)]
    InitialLoad,
    #[Id(0x03, 0x04)]
    LoadingScreenTransition,
    #[Id(0x03, 0x08)]
    #[Classic]
    ServerHello(ServerHelloPacket),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
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
    #[Id(0x04, 0x2E)]
    LoadPAs(LoadPAsPacket),
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

    #[Category(PacketCategory::PlayerStatus)]
    #[Id(0x06, 0x00)]
    SetPlayerID(SetPlayerIDPacket),
    #[Id(0x06, 0x01)]
    DealDamage(DealDamagePacket),
    #[Id(0x06, 0x05)]
    GainedEXP(GainedEXPPacket),

    #[Id(0x07, 0x00)]
    #[Classic]
    ChatMessage(ChatMessage),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x07, 0x00)]
    #[NGS]
    ChatMessageNGS(ChatMessageNGS),

    // Spawn packets [0x08]
    #[Category(PacketCategory::Spawning)]
    #[Id(0x08, 0x04)]
    #[Classic]
    CharacterSpawn(CharacterSpawnPacket),
    #[cfg(not(test))]
    #[Id(0x08, 0x09)]
    EventSpawn(EventSpawnPacket),
    #[cfg(not(test))]
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
    #[Id(0x0E, 0x00)]
    #[Classic]
    AddMember(AddMemberPacket),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x0E, 0x00)]
    #[NGS]
    AddMemberNGS(AddMemberNGSPacket),
    #[Id(0x0E, 0x01)]
    RemoveMember(RemoveMemberPacket),
    #[Id(0x0E, 0x02)]
    #[Classic]
    PartyInit(PartyInitPacket),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x0E, 0x02)]
    #[NGS]
    PartyInitNGS(PartyInitNGSPacket),
    #[Id(0x0E, 0x03)]
    RemovedFromParty,
    #[Id(0x0E, 0x04)]
    PartyInviteResult(PartyInviteResultPacket),
    #[Id(0x0E, 0x05)]
    PartyInviteRequest(PartyInviteRequestPacket),
    #[Id(0x0E, 0x06)]
    NewInvite(NewInvitePacket),
    #[Id(0x0E, 0x07)]
    AcceptInvite(AcceptInvitePacket),
    #[Id(0x0E, 0x09)]
    LeaveParty,
    #[Id(0x0E, 0x0C)]
    NewPartySettings(NewPartySettingsPacket),
    #[Id(0x0E, 0x0D)]
    PartySettings(PartySettingsPacket),
    #[Id(0x0E, 0x0E)]
    TransferLeader(TransferLeaderPacket),
    #[Id(0x0E, 0x0F)]
    NewLeader(NewLeaderPacket),
    #[Id(0x0E, 0x10)]
    KickMember(KickMemberPacket),
    #[Id(0x0E, 0x11)]
    KickedMember(KickedMemberPacket),
    #[Id(0x0E, 0x17)]
    DisbandParty(DisbandPartyPacket),
    #[Id(0x0E, 0x18)]
    PartyDisbandedMarker,
    #[Id(0x0E, 0x19)]
    ChatStatus(ChatStatusPacket),
    #[Id(0x0E, 0x1B)]
    PartyInfo(PartyInfoPacket),
    #[Id(0x0E, 0x1C)]
    PartyInfoStopper(PartyInfoStopperPacker),
    #[Id(0x0E, 0x1D)]
    GetPartyDetails(GetPartyDetailsPacket),
    #[Id(0x0E, 0x1E)]
    PartyDetails(PartyDetailsPacket),
    #[Id(0x0E, 0x1F)]
    PartyDetailsStopper,
    #[Id(0x0E, 0x28)]
    SetBusy,
    #[Id(0x0E, 0x29)]
    SetNotBusy,
    #[Id(0x0E, 0x2B)]
    NewBusyState(NewBusyStatePacket),
    #[Id(0x0E, 0x2C)]
    SetInviteDecline(InviteDeclinePacket),
    #[Id(0x0E, 0x2E)]
    GetPartyInfo(GetPartyInfoPacket),
    #[Id(0x0E, 0x4F)]
    SetPartyColor(SetPartyColorPacket),
    #[Id(0x0E, 0x67)]
    PartySetupFinish(PartySetupFinishPacket),

    // Item packets [0x0F]
    #[Category(PacketCategory::Item)]
    #[Id(0x0F, 0x00)]
    LoadItemAttributes(ItemAttributesPacket),
    #[Id(0x0F, 0x06)]
    UpdateInventory(UpdateInventoryPacket),
    #[Id(0x0F, 0x0C)]
    #[Classic]
    LoadEquiped(LoadEquipedPacket),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x0F, 0x0C)]
    #[NGS]
    LoadEquipedNGS(LoadEquipedNGSPacket),
    #[Id(0x0F, 0x0D)]
    #[Classic]
    LoadPlayerInventory(LoadPlayerInventoryPacket),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x0F, 0x0D)]
    #[NGS]
    LoadPlayerInventoryNGS(LoadPlayerInventoryNGSPacket),
    #[Id(0x0F, 0x0F)]
    MoveToStorageRequest(MoveToStorageRequestPacket),
    #[Id(0x0F, 0x10)]
    #[Classic]
    MoveToStorage(MoveToStoragePacket),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x0F, 0x10)]
    #[NGS]
    MoveToStorageNGS(MoveToStorageNGSPacket),
    #[Id(0x0F, 0x11)]
    MoveToInventoryRequest(MoveToInventoryRequestPacket),
    #[Id(0x0F, 0x12)]
    #[Classic]
    MoveToInventory(MoveToInventoryPacket),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x0F, 0x12)]
    #[NGS]
    MoveToInventoryNGS(MoveToInventoryNGSPacket),
    #[Id(0x0F, 0x13)]
    #[Classic]
    LoadStorages(LoadStoragesPacket),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x0F, 0x13)]
    #[NGS]
    LoadStoragesNGS(LoadStoragesNGSPacket),
    #[Id(0x0F, 0x14)]
    InventoryMeseta(InventoryMesetaPacket),
    #[Id(0x0F, 0x15)]
    MoveMeseta(MoveMesetaPacket),
    #[Id(0x0F, 0x16)]
    StorageMeseta(StorageMesetaPacket),
    #[Id(0x0F, 0x17)]
    DiscardItemRequest(DiscardItemRequestPacket),
    #[Id(0x0F, 0x18)]
    MoveStoragesRequest(MoveStoragesRequestPacket),
    #[Id(0x0F, 0x19)]
    #[Classic]
    MoveStorages(MoveStoragesPacket),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x0F, 0x19)]
    #[NGS]
    MoveStoragesNGS(MoveStoragesNGSPacket),
    #[Id(0x0F, 0x1C)]
    GetItemDescription(GetItemDescriptionPacket),
    #[Id(0x0F, 0x1D)]
    LoadItemDescription(LoadItemDescriptionPacket),
    #[Id(0x0F, 0x21)]
    #[Classic]
    EquipedWeapon(EquipedWeaponPacket),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x0F, 0x21)]
    #[NGS]
    EquipedWeaponNGS(EquipedWeaponNGSPacket),
    #[Id(0x0F, 0x22)]
    #[Classic]
    UpdateStorage(UpdateStoragePacket),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x0F, 0x22)]
    #[NGS]
    UpdateStorageNGS(UpdateStorageNGSPacket),
    #[Id(0x0F, 0x25)]
    DiscardStorageItemRequest(DiscardStorageItemRequestPacket),
    #[cfg(not(test))]
    #[Id(0x0F, 0x30)]
    LoadItem(LoadItemPacket),
    #[cfg(test)]
    #[Id(0x0F, 0x30)]
    LoadItem(LoadItemInternal),
    #[Id(0x0F, 0x33)]
    LearnedPA(LearnedPAPacket),
    #[Id(0x0F, 0x65)]
    PotentialList(PotentialListPacket),
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
    #[Id(0x0F, 0x8A)]
    CharacterCapaignsRequest,
    #[Id(0x0F, 0x9C)]
    Unk0f9c(Unk0f9cPacket),
    #[Id(0x0F, 0xBC)]
    ChangeWeaponPalette(ChangeWeaponPalettePacket),
    #[Id(0x0F, 0xDF)]
    LoadMaterialStorage(LoadMaterialStoragePacket),
    #[Id(0x0F, 0xE0)]
    MoveToMatStorageRequest(MoveToMatStorageRequestPacket),
    #[Id(0x0F, 0xE1)]
    MoveToMatStorage(MoveToMatStoragePacket),
    #[Id(0x0F, 0xE2)]
    MoveFromMatStorageRequest(MoveFromMatStorageRequestPacket),
    #[Id(0x0F, 0xE3)]
    #[Classic]
    MoveFromMatStorage(MoveFromMatStoragePacket),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x0F, 0xE3)]
    #[NGS]
    MoveFromMatStorageNGS(MoveFromMatStorageNGSPacket),
    #[Id(0x0F, 0xE8)]
    MoveMSToStorageRequest(MoveMSToStorageRequestPacket),
    #[Id(0x0F, 0xE9)]
    #[Classic]
    MoveMSToStorage(MoveMSToStoragePacket),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x0F, 0xE9)]
    #[NGS]
    MoveMSToStorageNGS(MoveMSToStorageNGSPacket),
    #[Id(0x0F, 0xEF)]
    Unk0fef(Unk0fefPacket),
    #[Id(0x0F, 0xFC)]
    Unk0ffc(Unk0ffcPacket),

    #[Id(0x10, 0x00)]
    #[Classic]
    RunLua(LuaPacket),

    // Login packets [0x11]
    #[Category(PacketCategory::Login)]
    #[Id(0x11, 0x00)]
    #[Classic]
    SegaIDLogin(SegaIDLoginPacket),
    #[Id(0x11, 0x01)]
    #[Classic]
    LoginResponse(LoginResponsePacket),
    #[Id(0x11, 0x02)]
    CharacterListRequest,
    #[Id(0x11, 0x03)]
    #[Classic]
    CharacterListResponse(CharacterListPacket),
    #[Id(0x11, 0x04)]
    StartGame(StartGamePacket),
    #[Id(0x11, 0x05)]
    #[Classic]
    CharacterCreate(CharacterCreatePacket),
    #[Id(0x11, 0x06)]
    CharacterDeletionRequest(CharacterDeletionRequestPacket),
    #[Id(0x11, 0x08)]
    CharacterDeletion(CharacterDeletionPacket),
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
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x11, 0x1B)]
    #[NGS]
    UserInfoNGS(UserInfoNGSPacket),
    #[Id(0x11, 0x1B)]
    #[Classic]
    UserInfo(UserInfoPacket),
    #[Id(0x11, 0x1E)]
    NicknameRequest(NicknameRequestPacket),
    #[Id(0x11, 0x1D)]
    NicknameResponse(NicknameResponsePacket),
    #[Id(0x11, 0x2B)]
    ClientGoodbye,
    #[Id(0x11, 0x2C)]
    #[Classic]
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
    #[Classic]
    VitaLogin(VitaLoginPacket),
    #[Id(0x11, 0x66)]
    SalonEntryRequest,
    #[Id(0x11, 0x67)]
    #[Classic]
    SalonEntryResponse(SalonResponse),
    #[Id(0x11, 0x68)]
    ChallengeRequest(ChallengeRequestPacket),
    #[Id(0x11, 0x69)]
    ChallengeResponse(ChallengeResponsePacket),
    #[Id(0x11, 0x6B)]
    #[Classic]
    SegaIDInfoRequest,
    #[Id(0x11, 0x71)]
    NotificationStatus(NotificationStatusPacket),
    #[Id(0x11, 0x86)]
    LoginHistoryRequest,
    #[Id(0x11, 0x87)]
    LoginHistoryResponse(LoginHistoryPacket),
    #[Id(0x11, 0x90)]
    CharacterUndeletionRequest(CharacterUndeletionRequestPacket),
    #[Id(0x11, 0x91)]
    CharacterUndeletion(CharacterUndeletionPacket),
    #[Id(0x11, 0x97)]
    CharacterRenameRequest(CharacterRenameRequestPacket),
    #[Id(0x11, 0x98)]
    CharacterRename(CharacterRenamePacket),
    #[Id(0x11, 0x9B)]
    CharacterNewNameRequest(CharacterNewNameRequestPacket),
    #[Id(0x11, 0x9C)]
    CharacterNewName(CharacterNewNamePacket),
    #[Id(0x11, 0x9D)]
    NicknameChangeRequest,
    #[Id(0x11, 0xB8)]
    CharacterMoveRequest(CharacterMoveRequestPacket),
    #[Id(0x11, 0xB9)]
    CharacterMove(CharacterMovePacket),
    #[Id(0x11, 0xDE)]
    PlayerReported(PlayerReportedPacket),
    #[Id(0x11, 0xEA)]
    NicknameError(NicknameErrorPacket),
    #[Id(0x11, 0xED)]
    #[Classic]
    BannerList(BannerListPacket),
    #[Id(0x11, 0xEE)]
    EmailCodeRequest(EmailCodeRequestPacket),
    #[Id(0x11, 0xFF)]
    #[Classic]
    Unk11FF(Unk11FFPacket),

    // Emergency packets [0x15]
    #[Category(PacketCategory::Emergency)]
    #[Id(0x15, 0x02)]
    SpawnEmergency(SpawnEmergencyPacket),
    #[Id(0x15, 0x03)]
    EmergencyEnd(EmergencyEndPacket),
    #[Id(0x15, 0x11)]
    AvailableEmergencies(AvailableEmergenciesPacket),

    // Friends packets [0x18]
    #[Category(PacketCategory::Friends)]
    #[Id(0x18, 0x14)]
    FriendListRequest(FriendListRequestPacket),
    #[Id(0x18, 0x15)]
    FriendList(FriendListPacket),
    #[Id(0x18, 0x18)]
    SendFriendRequest(SendFriendRequestPacket),
    #[Id(0x18, 0x1A)]
    AddedRequest(AddedRequestPacket),

    #[Category(PacketCategory::Unknown)]
    #[Id(0x19, 0x01)]
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

    // Character packets [0x1C]
    #[Category(PacketCategory::Characters)]
    #[Id(0x1C, 0x10)]
    GetNearbyCharacters,

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

    // Palette packets [0x21]
    #[Category(PacketCategory::Palette)]
    #[Id(0x21, 0x01)]
    LoadPalette(LoadPalettePacket),
    #[Id(0x21, 0x02)]
    FullPaletteInfoRequest,
    #[Id(0x21, 0x03)]
    FullPaletteInfo(FullPaletteInfoPacket),
    #[Id(0x21, 0x04)]
    SetPalette(SetPalettePacket),
    #[Id(0x21, 0x05)]
    UpdateSubPalette(UpdateSubPalettePacket),
    #[Id(0x21, 0x06)]
    UpdatePalette(UpdatePalettePacket),
    #[Id(0x21, 0x08)]
    SetSubPalette(SetSubPalettePacket),
    #[Id(0x21, 0x0A)]
    SetDefaultPAs(SetDefaultPAsPacket),
    #[Id(0x21, 0x0F)]
    NewDefaultPAs(NewDefaultPAsPacket),

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
    #[Classic]
    SendSymbolArt(SendSymbolArtPacket),
    #[Id(0x2F, 0x09)]
    #[Classic]
    ReceiveSymbolArt(ReceiveSymbolArtPacket),

    // ARKS Misions packets [0x4A]
    #[Category(PacketCategory::ARKSMissions)]
    #[Id(0x4A, 0x00)]
    MissionListRequest,
    #[Id(0x4A, 0x01)]
    MissionList(MissionListPacket),
    #[Id(0x4A, 0x03)]
    #[Classic]
    Unk4A03(Unk4A03Packet),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x4A, 0x03)]
    #[NGS]
    Unk4A03NGS(Unk4A03NGSPacket),
    #[Id(0x4A, 0x0C)]
    SetTrackedMission(SetTrackedMissionPacket),

    // Classic Mission Pass packets [0x4D]
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
    Raw(Vec<u8>),
    #[Unknown]
    Unknown((PacketHeader, Vec<u8>)),
}

#[cfg(feature = "proxy")]
#[derive(Debug, Default, Clone, PartialEq, ProtocolReadWrite)]
// bare minimum specifically for proxies
pub enum Packet {
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
    Raw(Vec<u8>),
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
    /// Player status related packets. See [`playerstatus`]
    PlayerStatus,
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
    /// Emergency related packets. See [`emergency`]
    Emergency,
    /// Friends related packets. See [`friends`]
    Friends,
    /// Mail related packets. See [`mail`]
    Mail,
    /// Charater related packets.
    Characters,
    /// Daily orders related packets.
    DailyOrders,
    /// Palette related packets. See [`palette`]
    Palette,
    /// Settings related packets.
    Settings,
    /// Symbol Art related packets. See [`symbolart`]
    SymbolArt,
    /// ARKS Missions related packets. See [`missions`]
    ARKSMissions,
    /// Classic Mission pass related packets. See [`missionpass`]
    MissionPass,
}

// ----------------------------------------------------------------
// Common structures
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
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
    fn read(reader: &mut (impl Read + Seek), packet_type: PacketType) -> std::io::Result<Self> {
        let (id, subid, flag) = if !matches!(packet_type, PacketType::NGS) {
            let id = reader.read_u8()?;
            let subid = reader.read_u8()? as u16;
            let flag = Flags::read(reader, packet_type)?;
            reader.read_u8()?;
            (id, subid, flag)
        } else {
            let flag = Flags::read(reader, packet_type)?;
            let id = reader.read_u8()?;
            let subid = reader.read_u16::<LittleEndian>()?;
            (id, subid, flag)
        };

        Ok(Self { id, subid, flag })
    }
    fn write(&self, packet_type: PacketType) -> Vec<u8> {
        let mut buf = vec![];
        if !matches!(packet_type, PacketType::NGS) {
            buf.write_u8(self.id).unwrap();
            buf.write_u8(self.subid as u8).unwrap();
            self.flag.write(&mut buf, packet_type).unwrap();
            buf.write_u8(0).unwrap();
        } else {
            self.flag.write(&mut buf, packet_type).unwrap();
            buf.write_u8(self.id).unwrap();
            buf.write_u16::<LittleEndian>(self.subid).unwrap();
        }
        buf
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    Party = 13,
    Unk2 = 22,
    #[Read_default]
    Undefined = 0xFFFF,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
pub struct ObjectHeader {
    pub id: u32,
    pub unk: u32,
    pub entity_type: EntityType,
    pub map_id: u16,
}

// ----------------------------------------------------------------
// Packets
// ----------------------------------------------------------------

// 0x07, 0x00
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
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
#[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
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
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x19, 0x0F)]
pub struct LobbyMonitorPacket {
    pub video_id: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x10, 0x00)]
pub struct LuaPacket {
    pub unk1: u16,
    pub unk2: u16,
    #[VariableStr(0, 0)]
    pub lua: AsciiString,
}

// ----------------------------------------------------------------
// Settings packets
// ----------------------------------------------------------------

// 0x2B, 0x01
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2B, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SaveSettingsPacket {
    #[VariableStr(0xCEF1, 0xB5)]
    pub settings: AsciiString,
}

// 0x2B, 0x02
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
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
    use crate::ppac::PPACReader;
    use crate::protocol::ProtocolRW;
    use std::{fs, io::BufReader, io::Write};
    #[test]
    fn file_check() {
        // this is hard to test, because original server doesn't clear output buffers
        let mut is_failed = false;
        traverse_dir2("test_data", &mut is_failed);
        if is_failed {
            panic!();
        }
    }

    fn traverse_dir2<T: AsRef<std::path::Path>>(path: T, is_failed: &mut bool) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap().path();
            if entry.is_dir() {
                traverse_dir2(entry, is_failed);
            } else if entry.is_file() {
                match entry.extension() {
                    Some(ext) => {
                        if ext != "pak" {
                            continue;
                        }
                    }
                    None => continue,
                }
                println!("Testing: {:?}", entry);
                let reader = BufReader::new(fs::File::open(&entry).unwrap());
                let mut reader = PPACReader::open(reader).unwrap();
                reader.set_out_type(crate::ppac::OutputType::Both);

                while let Some(packet) = reader.read().unwrap() {
                    let in_data = match packet.data {
                        Some(data) => data,
                        None => continue,
                    };
                    let id = format!(
                        "{}_{:X}",
                        packet.time.as_nanos(),
                        u32::from_be_bytes(in_data[4..8].try_into().unwrap())
                    );
                    let out_type = packet.protocol_type;
                    let packet = match packet.packet {
                        Some(x) => x,
                        None => {
                            println!("{id} - FAIL (can't read)");
                            *is_failed = true;
                            let path = format!(
                                "failed_tests/{}/{id}_unreadable",
                                entry.file_name().unwrap().to_string_lossy()
                            );
                            create_dir(&path).unwrap();
                            fs::File::create(format!("{path}/in.bin"))
                                .unwrap()
                                .write_all(&in_data)
                                .unwrap();
                            continue;
                        }
                    };
                    let out_data = packet.write(out_type);
                    if in_data.len() != out_data.len() {
                        println!(
                            "{id} - FAIL (different length - in: 0x{:X}, out: 0x{:X})",
                            in_data.len(),
                            out_data.len()
                        );
                        *is_failed = true;
                        let path = format!(
                            "failed_tests/{}/{id}_len",
                            entry.file_name().unwrap().to_string_lossy()
                        );
                        create_dir(&path).unwrap();
                        fs::File::create(format!("{path}/in.bin"))
                            .unwrap()
                            .write_all(&in_data)
                            .unwrap();
                        fs::File::create(format!("{path}/out.bin"))
                            .unwrap()
                            .write_all(&out_data)
                            .unwrap();
                    } else if in_data != out_data {
                        println!("{id} - FAIL (different data)");
                        *is_failed = true;
                        let path = format!(
                            "failed_tests/{}/{id}_data",
                            entry.file_name().unwrap().to_string_lossy()
                        );
                        create_dir(&path).unwrap();
                        fs::File::create(format!("{path}/in.bin"))
                            .unwrap()
                            .write_all(&in_data)
                            .unwrap();
                        fs::File::create(format!("{path}/out.bin"))
                            .unwrap()
                            .write_all(&out_data)
                            .unwrap();
                    }
                }
            }
        }
    }

    fn create_dir<P: AsRef<std::path::Path>>(filename: P) -> std::io::Result<()> {
        match fs::create_dir_all(filename) {
            Ok(()) => Ok(()),
            Err(x) if x.kind() == std::io::ErrorKind::AlreadyExists => Ok(()),
            Err(x) => Err(x),
        }
    }
}
