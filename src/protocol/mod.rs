//! PSO2 packet definitions and protocol information.

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use packetlib_impl::{HelperReadWrite, PacketReadWrite, ProtocolReadWrite};
use std::{
    io::{Cursor, Read, Seek, Write},
    time::Duration,
};

// Packet definitions modules
pub mod chat;
pub mod emergency;
pub mod flag;
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
pub mod settings;
pub mod spawn;
pub mod symbolart;
pub mod unk10;
pub mod unk19;
pub mod unk2a;
pub mod unk34;
use chat::*;
use emergency::*;
use flag::*;
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
use settings::*;
use spawn::*;
use symbolart::*;
use unk10::*;
use unk19::*;
use unk2a::*;
use unk34::*;

// Code is getting really messy.

mod private {
    pub trait Sealed: Sized {}
    impl Sealed for super::Packet {}
    #[cfg(feature = "proxy")]
    impl Sealed for super::ProxyPacket {}
}

/// Type of the packet.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum PacketType {
    #[default]
    /// NGS packet.
    NGS,
    /// Classic packet. (i.e. NA, JP and Vita)
    Classic,
    /// NA packet.
    NA,
    /// JP packet.
    JP,
    /// Vita packet.
    Vita,
    /// Raw packet. (i.e. don't parse the packet)
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
        flags: &Flags,
        packet_type: PacketType,
    ) -> std::io::Result<Self>;
    /// Write a packet to a Vec.
    fn write(&self, packet_type: PacketType) -> std::io::Result<Vec<u8>>;
}

pub(crate) trait HelperReadWrite: Sized {
    fn read(
        reader: &mut (impl Read + Seek),
        packet_type: PacketType,
        xor: u32,
        sub: u32,
    ) -> std::io::Result<Self>;
    fn write(
        &self,
        writer: &mut impl Write,
        packet_type: PacketType,
        xor: u32,
        sub: u32,
    ) -> std::io::Result<()>;
}

/// All known packets
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, ProtocolReadWrite)]
#[non_exhaustive]
pub enum Packet {
    /// Empty packet, produces no output.
    #[default]
    #[Empty]
    None,

    // Server packets [0x03]
    #[Category(PacketCategory::Server)]
    /// (0x03, 0x00) Map Transfer.
    #[Id(0x03, 0x00)]
    MapTransfer(MapTransferPacket),
    /// (0x03, 0x03) Initial Load (?).
    ///
    /// (C -> S) Sent when the client loads for the first time in the session.
    ///
    /// Response to: [`Packet::LoadingScreenTransition`] (?).
    ///
    /// Respond with: lobby map setup.
    #[Id(0x03, 0x03)]
    InitialLoad,
    /// (0x03, 0x04) Loading Screen Transition.
    ///
    /// (S -> C) Sent when the server wants the client to display the loading screen.
    ///
    /// Response to: [`Packet::StartGame`].
    ///
    /// Respond with: [`Packet::InitialLoad`] (?, also unsure if it is only sent once).
    #[Id(0x03, 0x04)]
    LoadingScreenTransition,
    /// (0x03, 0x06) Unknown.
    #[Id(0x03, 0x06)]
    Unk0306(Unk0306Packet),
    /// (0x03, 0x08) Server Hello.
    #[Id(0x03, 0x08)]
    ServerHello(ServerHelloPacket),
    /// (0x03, 0x0B) Server Ping.
    ///
    /// (S -> C) Sent by the server periodically.
    ///
    /// Respond with: [`Packet::ServerPong`].
    #[Id(0x03, 0x0B)]
    ServerPing,
    /// (0x03, 0x0C) Server Pong.
    ///
    /// (C -> S) Sent by the client in response to the ping.
    ///
    /// Response to: [`Packet::ServerPing`].
    #[Id(0x03, 0x0C)]
    ServerPong,
    /// (0x03, 0x10) Map Loading Finished.
    #[Id(0x03, 0x10)]
    MapLoaded(MapLoadedPacket),
    /// (0x03, 0x12) Move Lobby -> Campship.
    #[Id(0x03, 0x12)]
    ToCampship(ToCampshipPacket),
    /// (0x03, 0x16) Move Campship -> Quest Level.
    #[Id(0x03, 0x16)]
    CampshipDown(CampshipDownPacket),
    /// (0x03, 0x23) Remove Loading Screen.
    ///
    /// (S -> C) Sent to remove the loading screen.
    ///
    /// Response to: [`Packet::MapLoaded`].
    #[Id(0x03, 0x23)]
    FinishLoading,
    /// (0x03, 0x24) Load Level.
    #[Id(0x03, 0x24)]
    LoadLevel(LoadLevelPacket),
    /// (0x03, 0x2B) Enable Controls.
    ///
    /// (S -> C) Sent to enable player movement.
    ///
    /// Response to: [`Packet::MapLoaded`].
    #[Id(0x03, 0x2B)]
    UnlockControls,
    /// (0x03, 0x34) Move Casino -> Lobby.
    #[Id(0x03, 0x34)]
    CasinoToLobby(CasinoToLobbyPacket),
    /// (0x03, 0x35) Move Lobby -> Casino.
    #[Id(0x03, 0x35)]
    CasinoTransport(CasinoTransportPacket),
    /// (0x03, 0x38) Move Bridge -> Lobby.
    #[Id(0x03, 0x38)]
    BridgeToLobby(BridgeToLobbyPacket),
    /// (0x03, 0x39) Move Lobby -> Bridge.
    #[Id(0x03, 0x39)]
    BridgeTransport(BridgeTransportPacket),
    /// (0x03, 0x3B) Move Cafe -> Lobby.
    #[Id(0x03, 0x3B)]
    CafeToLobby(CafeToLobbyPacket),
    /// (0x03, 0x3C) Move Lobby -> Cafe.
    #[Id(0x03, 0x3C)]
    CafeTransport(CafeTransportPacket),

    // Object related packets [0x04]
    #[Category(PacketCategory::Object)]
    /// (0x04, 0x02) Object Teleport Location.
    #[Id(0x04, 0x02)]
    TeleportTransfer(TeleportTransferPacket),
    /// (0x04, 0x06) Item Picked Up.
    #[Id(0x04, 0x06)]
    ItemPickedUp(ItemPickedUpPacket),
    /// (0x04, 0x07) Object Movement (broadcast).
    #[Id(0x04, 0x07)]
    Movement(MovementPacket),
    /// (0x04, 0x08) Client Movement Action.
    #[Id(0x04, 0x08)]
    MovementAction(MovementActionPacket),
    /// (0x04, 0x0F) Unknown.
    #[Id(0x04, 0x0F)]
    Unk040F(Unk040FPacket),
    /// (0x04, 0x13) Unknown.
    #[Id(0x04, 0x13)]
    Unk0413(Unk0413Packet),
    /// (0x04, 0x14) Client Interaction.
    #[Id(0x04, 0x14)]
    Interact(InteractPacket),
    /// (0x04, 0x15) Object Action or Set Object Tag (unicast or broadcast).
    #[Id(0x04, 0x15)]
    SetTag(SetTagPacket),
    /// (0x04, 0x22) Unknown.
    #[Id(0x04, 0x22)]
    Unk0422(Unk0422Packet),
    /// (0x04, 0x23) Unknown.
    #[Id(0x04, 0x23)]
    Unk0423(Unk0423Packet),
    /// (0x04, 0x24) Unknown.
    #[Id(0x04, 0x24)]
    Unk0424(Unk0424Packet),
    /// (0x04, 0x25) Unknown.
    #[Id(0x04, 0x25)]
    Unk0425(Unk0425Packet),
    /// (0x04, 0x2B) Unknown.
    #[Id(0x04, 0x2B)]
    Unk042B(Unk042BPacket),
    /// (0x04, 0x2E) Load Learned Photon Arts (broadcast).
    #[Id(0x04, 0x2E)]
    LoadPAs(LoadPAsPacket),
    /// (0x04, 0x3B) Remove Object (broadcast).
    #[Id(0x04, 0x3B)]
    RemoveObject(RemoveObjectPacket),
    /// (0x04, 0x3C) Client Action Update.
    #[Id(0x04, 0x3C)]
    ActionUpdate(ActionUpdatePacket),
    /// (0x04, 0x52) Damage Received.
    #[Id(0x04, 0x52)]
    DamageReceive(DamageReceivePacket),
    /// (0x04, 0x71) Object Movement End (broadcast).
    #[Id(0x04, 0x71)]
    MovementEnd(MovementEndPacket),
    /// (0x04, 0x75) Action End (broadcast).
    #[Id(0x04, 0x75)]
    ActionEnd(ActionEndPacket),
    /// (0x04, 0x79) Unknown.
    #[Id(0x04, 0x79)]
    Unk0479(Unk0479Packet),
    /// (0x04, 0x80) Movement Action Response (broadcast).
    #[Id(0x04, 0x80)]
    MovementActionServer(MovementActionServerPacket),
    /// (0x04, 0x81) Action Update Response (broadcast).
    #[Id(0x04, 0x81)]
    ActionUpdateServer(ActionUpdateServerPacket),
    /// (0x04, 0x86) Unknown.
    #[Id(0x04, 0x86)]
    Unk0486(Unk0486Packet),
    /// (0x04, 0xB0) Unknown.
    #[Id(0x04, 0xB0)]
    Unk04B0(Unk04B0Packet),
    /// (0x04, 0xEA) Unknown.
    #[Id(0x04, 0xEA)]
    Unk04EA(Unk04EAPacket),

    // Player status packets [0x06]
    #[Category(PacketCategory::PlayerStatus)]
    #[Id(0x06, 0x00)]
    SetPlayerID(SetPlayerIDPacket),
    #[Id(0x06, 0x01)]
    DealDamage(DealDamagePacket),
    #[Id(0x06, 0x05)]
    GainedEXP(GainedEXPPacket),

    // Chat packets [0x07]
    #[Category(PacketCategory::Chat)]
    /// (0x07, 0x00) Chat Message.
    #[Id(0x07, 0x00)]
    ChatMessage(ChatMessage),

    // Spawn packets [0x08]
    #[Category(PacketCategory::Spawning)]
    #[Id(0x08, 0x04)]
    #[Classic]
    CharacterSpawn(CharacterSpawnPacket),
    // temporarily commented out for the server
    // #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x08, 0x04)]
    #[NGS]
    CharacterSpawnNGS(CharacterSpawnNGSPacket),
    #[Id(0x08, 0x05)]
    TransporterSpawn(TransporterSpawnPacket),
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
    #[Id(0x0B, 0x06)]
    StartCutscene(StartCutscenePacket),
    #[Id(0x0B, 0x09)]
    Unk0B09(Unk0B09Packet),
    #[Id(0x0B, 0x13)]
    Unk0B13(Unk0B13Packet),
    #[Id(0x0B, 0x15)]
    AvailableQuestsRequest(AvailableQuestsRequestPacket),
    #[Id(0x0B, 0x16)]
    AvailableQuests(AvailableQuestsPacket),
    #[Id(0x0B, 0x17)]
    QuestCategoryRequest(QuestCategoryRequestPacket),
    #[Id(0x0B, 0x18)]
    QuestCategory(QuestCategoryPacket),
    #[Id(0x0B, 0x19)]
    QuestDifficultyRequest(QuestDifficultyRequestPacket),
    #[Id(0x0B, 0x1A)]
    QuestDifficulty(QuestDifficultyPacket),
    #[Id(0x0B, 0x1B)]
    QuestCategoryStopper,
    #[Id(0x0B, 0x1C)]
    QuestDifficultyStopper,
    #[Id(0x0B, 0x1F)]
    SetQuestPoints(SetQuestPointsPacket),
    #[Id(0x0B, 0x20)]
    AcceptQuest(AcceptQuestPacket),
    #[Id(0x0B, 0x28)]
    QuestPointsAdded(QuestPointsAddedPacket),
    #[Id(0x0B, 0x2F)]
    AcceptQuestOther(AcceptQuestOtherPacket),
    #[Id(0x0B, 0x30)]
    QuestCounterRequest,
    #[Id(0x0B, 0x62)]
    EQARKSLevel(EQARKSLevelPacket),
    #[Id(0x0B, 0xAF)]
    Unk0BAF(Unk0BAFPacket),
    #[Id(0x0B, 0xD0)]
    Unk0BD0(Unk0BD0Packet),

    // Party packets [0x0E]
    #[Category(PacketCategory::Party)]
    #[Id(0x0E, 0x00)]
    AddMember(AddMemberPacket),
    #[Id(0x0E, 0x01)]
    RemoveMember(RemoveMemberPacket),
    #[Id(0x0E, 0x02)]
    PartyInit(PartyInitPacket),
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
    #[Id(0x0E, 0x1A)]
    Unk0E1A(Unk0E1APacket),
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
    #[Id(0x0E, 0x21)]
    Unk0E21(Unk0E21Packet),
    #[Id(0x0E, 0x25)]
    SetQuestInfo(SetQuestInfoPacket),
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
    #[Id(0x0E, 0x31)]
    SetPartyQuest(SetPartyQuestPacket),
    #[Id(0x0E, 0x4F)]
    SetPartyColor(SetPartyColorPacket),
    #[Id(0x0E, 0x52)]
    Unk0E52(Unk0E52Packet),
    #[Id(0x0E, 0x67)]
    PartySetupFinish(PartySetupFinishPacket),

    // Item packets [0x0F]
    #[Category(PacketCategory::Item)]
    #[Id(0x0F, 0x00)]
    LoadItemAttributes(ItemAttributesPacket),
    #[Id(0x0F, 0x01)]
    ItemPickupRequest(ItemPickupRequestPacket),
    #[Id(0x0F, 0x02)]
    ItemPickupResponse(ItemPickupResponsePacket),
    #[Id(0x0F, 0x04)]
    NewItemDrop(NewItemDropPacket),
    #[Id(0x0F, 0x05)]
    AddedItem(AddedItemPacket),
    #[Id(0x0F, 0x06)]
    UpdateInventory(UpdateInventoryPacket),
    #[Id(0x0F, 0x0C)]
    LoadEquiped(LoadEquipedPacket),
    #[Id(0x0F, 0x0D)]
    LoadPlayerInventory(LoadPlayerInventoryPacket),
    #[Id(0x0F, 0x0F)]
    MoveToStorageRequest(MoveToStorageRequestPacket),
    #[Id(0x0F, 0x10)]
    MoveToStorage(MoveToStoragePacket),
    #[Id(0x0F, 0x11)]
    MoveToInventoryRequest(MoveToInventoryRequestPacket),
    #[Id(0x0F, 0x12)]
    MoveToInventory(MoveToInventoryPacket),
    #[Id(0x0F, 0x13)]
    LoadStorages(LoadStoragesPacket),
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
    MoveStorages(MoveStoragesPacket),
    #[Id(0x0F, 0x1C)]
    GetItemDescription(GetItemDescriptionPacket),
    #[Id(0x0F, 0x1D)]
    LoadItemDescription(LoadItemDescriptionPacket),
    #[Id(0x0F, 0x21)]
    EquipedWeapon(EquipedWeaponPacket),
    #[Id(0x0F, 0x22)]
    UpdateStorage(UpdateStoragePacket),
    #[Id(0x0F, 0x25)]
    DiscardStorageItemRequest(DiscardStorageItemRequestPacket),
    #[Id(0x0F, 0x30)]
    LoadItem(LoadItemPacket),
    #[Id(0x0F, 0x33)]
    LearnedPA(LearnedPAPacket),
    #[Id(0x0F, 0x5B)]
    Unk0F5B,
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
    Unk0F9C(Unk0F9CPacket),
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
    MoveFromMatStorage(MoveFromMatStoragePacket),
    #[Id(0x0F, 0xE8)]
    MoveMSToStorageRequest(MoveMSToStorageRequestPacket),
    #[Id(0x0F, 0xE9)]
    MoveMSToStorage(MoveMSToStoragePacket),
    #[Id(0x0F, 0xEF)]
    Unk0FEF(Unk0FEFPacket),
    #[Id(0x0F, 0xFC)]
    Unk0FFC(Unk0FFCPacket),

    // Unknown 0x10 packets [0x10]
    #[Category(PacketCategory::Unk10)]
    #[Id(0x10, 0x00)]
    #[Classic]
    RunLua(LuaPacket),
    #[Id(0x10, 0x03)]
    Unk1003(Unk1003Packet),

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
    #[Id(0x11, 0x11)]
    BlockSwitchRequest(BlockSwitchRequestPacket),
    #[Id(0x11, 0x13)]
    #[Classic]
    BlockSwitchResponse(BlockSwitchResponsePacket),
    #[Id(0x11, 0x14)]
    #[Classic]
    BlockLogin(BlockLoginPacket),
    #[Id(0x11, 0x1B)]
    #[Classic]
    UserInfo(UserInfoPacket),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x11, 0x1B)]
    #[NGS]
    UserInfoNGS(UserInfoNGSPacket),
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
    #[Id(0x11, 0x65)]
    AllBlocksList(AllBlocksListPacket),
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
    #[Id(0x11, 0x6F)]
    Unk116F(Unk116FPacket),
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
    #[Id(0x11, 0xAF)]
    Unk11AF(Unk11AFPacket),
    #[Id(0x11, 0xB0)]
    Unk11B0(Unk11B0Packet),
    #[Id(0x11, 0xB8)]
    CharacterMoveRequest(CharacterMoveRequestPacket),
    #[Id(0x11, 0xB9)]
    CharacterMove(CharacterMovePacket),
    #[Id(0x11, 0xD7)]
    Unk11D7(Unk11D7Packet),
    #[Id(0x11, 0xDE)]
    PlayerReported(PlayerReportedPacket),
    #[Id(0x11, 0xEA)]
    NicknameError(NicknameErrorPacket),
    #[Id(0x11, 0xED)]
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
    #[Id(0x15, 0x05)]
    EmergencyProgress(EmergencyProgressPacket),
    #[Id(0x15, 0x08)]
    Unk1508(Unk1508Packet),
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

    // Unknown 0x19 packets [0x19]
    #[Category(PacketCategory::Unk19)]
    #[Id(0x19, 0x01)]
    SystemMessage(SystemMessagePacket),
    #[Id(0x19, 0x04)]
    Unk1904,
    #[Id(0x19, 0x06)]
    Unk1906,
    #[Id(0x19, 0x09)]
    SetLobbyEvent(SetLobbyEventPacket),
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

    // Flag packets [0x23]
    #[Category(PacketCategory::Flag)]
    #[Id(0x23, 0x02)]
    SetFlag(SetFlagPacket),
    #[Id(0x23, 0x04)]
    ServerSetFlag(ServerSetFlagPacket),
    #[Id(0x23, 0x05)]
    ServerSetParam(ServerSetParamPacket),
    #[Id(0x23, 0x06)]
    AccountFlags(AccountFlagsPacket),
    #[Id(0x23, 0x07)]
    CharacterFlags(CharacterFlagsPacket),
    #[Id(0x23, 0x0A)]
    CutsceneEnd(CutsceneEndPacket),
    #[Id(0x23, 0x0B)]
    SkitItemAddRequest(SkitItemAddRequestPacket),
    #[Id(0x23, 0x0C)]
    SkitItemAddResponse(SkitItemAddResponsePacket),
    #[Id(0x23, 0x0D)]
    Unk230D(Unk230DPacket),
    #[Id(0x23, 0x0E)]
    Unk230E(Unk230EPacket),
    #[Id(0x23, 0x10)]
    Unk2310,

    // Unknown 0x2A packets [0x2A]
    #[Category(PacketCategory::Unk2A)]
    #[Id(0x2A, 0x08)]
    Unk2A08(Unk2A08Packet),

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

    // Unknown 0x34 packets [0x34]
    #[Category(PacketCategory::Unk34)]
    #[Id(0x34, 0x35)]
    Unk3435(Unk3435Packet),
    #[Id(0x34, 0x5C)]
    Unk345C(Unk345CPacket),

    // ARKS Misions packets [0x4A]
    #[Category(PacketCategory::ARKSMissions)]
    #[Id(0x4A, 0x00)]
    MissionListRequest,
    #[Id(0x4A, 0x01)]
    MissionList(MissionListPacket),
    #[Id(0x4A, 0x03)]
    Unk4A03(Unk4A03Packet),
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
    /// Raw packet with header.
    #[Unknown]
    Raw(Vec<u8>),
    /// Unknown packet.
    #[Unknown]
    Unknown((PacketHeader, Vec<u8>)),
}

#[cfg(feature = "proxy")]
#[cfg_attr(docsrs, doc(cfg(feature = "proxy")))]
#[derive(Debug, Default, Clone, PartialEq, ProtocolReadWrite)]
/// Minimal packet definitions for proxies
pub enum ProxyPacket {
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
    /// Chat related packets. See [`chat`]
    Chat,
    /// Spawning related packets. See [`spawn`]
    Spawning,
    /// Quest list related packets. See [`questlist`]
    QuestList,
    /// Party related packets. See [`party`]
    Party,
    /// Item related packets. See [`items`]
    Item,
    /// Unknown 0x10 packets. See [`unk10`]
    Unk10,
    /// Login related packets. See [`login`]
    Login,
    /// Emergency related packets. See [`emergency`]
    Emergency,
    /// Friends related packets. See [`friends`]
    Friends,
    /// Unknown 0x19 packets. See [`unk19`]
    Unk19,
    /// Mail related packets. See [`mail`]
    Mail,
    /// Charater related packets.
    Characters,
    /// Daily orders related packets. See [`orders`]
    DailyOrders,
    /// Palette related packets. See [`palette`]
    Palette,
    /// Flag packets. See [`flag`]
    Flag,
    /// Unknown 0x2A packets. See [`unk2a`]
    Unk2A,
    /// Settings related packets. See [`settings`]
    Settings,
    /// Symbol Art related packets. See [`symbolart`]
    SymbolArt,
    /// Unknown 0x34 packets. See [`unk34`]
    Unk34,
    /// ARKS Missions related packets. See [`missions`]
    ARKSMissions,
    /// Classic Mission pass related packets. See [`missionpass`]
    MissionPass,
}

// ----------------------------------------------------------------
// Common structures
// ----------------------------------------------------------------

/// Packet header.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct PacketHeader {
    /// Id (category) of the packet.
    pub id: u8,
    /// Subid (id in the category) of the packet.
    pub subid: u16,
    /// Packet flags.
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
            let flag = Flags::read(reader, packet_type, 0, 0)?;
            reader.read_u8()?;
            (id, subid, flag)
        } else {
            let flag = Flags::read(reader, packet_type, 0, 0)?;
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
            self.flag.write(&mut buf, packet_type, 0, 0).unwrap();
            buf.write_u8(0).unwrap();
        } else {
            self.flag.write(&mut buf, packet_type, 0, 0).unwrap();
            buf.write_u8(self.id).unwrap();
            buf.write_u16::<LittleEndian>(self.subid).unwrap();
        }
        buf
    }
}

/// Packet flags.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[Flags(u8)]
pub struct Flags {
    #[Skip]
    #[Skip]
    // 0x04
    /// Set when the packet contains variable length data.
    pub packed: bool,
    #[Skip]
    // 0x10
    pub flag10: bool,
    /// Set when the [`Packet::Movement`] has all fields set.
    pub full_movement: bool,
    /// Set for all (?) of (0x04) packets.
    pub object_related: bool,
}

/// Known object types.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u16)]
pub enum ObjectType {
    #[default]
    Unknown = 0,
    /// Player object.
    Player = 4,
    /// Zone object.
    Map = 5,
    /// Most of the objects and NPCs.
    Object = 6,
    /// Some destructable objects (e.g. some trees).
    StaticObject = 7,
    /// Quest object.
    Quest = 11,
    /// Party object.
    Party = 13,
    Unk10 = 16,
    /// Non-playable partners.
    APC = 22,
    #[Read_default]
    Undefined = 0xFFFF,
}

/// Information about the targeted object.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
pub struct ObjectHeader {
    /// Id of the object.
    pub id: u32,
    pub unk: u32,
    /// Type of the object.
    pub entity_type: ObjectType,
    /// Zone id of the object. Not set for players.
    pub map_id: u16,
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

    use super::Packet;
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
                            println!("{entry:?}, {id} - FAIL (can't read)");
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
                    if matches!(
                        packet,
                        Packet::Movement(..)
                            | Packet::LoadItem(..)
                            | Packet::EventSpawn(..)
                            | Packet::ObjectSpawn(..)
                    ) {
                        continue;
                    }
                    let out_data = packet.write(out_type);
                    if in_data.len() != out_data.len() {
                        println!(
                            "{entry:?}, {id} - FAIL (different length - in: 0x{:X}, out: 0x{:X})",
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
                        println!("{entry:?}, {id} - FAIL (different data)");
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
