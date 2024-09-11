//! PSO2 packet definitions and protocol information.

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use pso2packetlib_impl::{HelperReadWrite, PacketReadWrite, ProtocolReadWrite};
use std::io::{Read, Seek};

// Packet traits
mod traits;
pub use traits::*;

// Packet definitions modules
pub mod chat;
pub mod emergency;
pub mod flag;
pub mod friendavatar;
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
pub mod playershop;
pub mod playerstatus;
pub mod questlist;
pub mod server;
pub mod settings;
pub mod spawn;
pub mod symbolart;
pub mod unk10;
pub mod unk19;
pub mod unk1e;
pub mod unk2a;
pub mod unk31;
pub mod unk34;
use chat::*;
use emergency::*;
use flag::*;
use friendavatar::*;
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
use playershop::*;
use playerstatus::*;
use questlist::*;
use server::*;
use settings::*;
use spawn::*;
use symbolart::*;
use unk10::*;
use unk19::*;
use unk1e::*;
use unk2a::*;
use unk31::*;
use unk34::*;

// Code is getting really messy.

/// Error type returned by packet parsing operations.
#[derive(Debug, thiserror::Error)]
pub enum PacketError {
    /// Failed to read or write packet field.
    #[error("failed to read/write field {field_name} from {packet_name}: {error}")]
    FieldError {
        packet_name: &'static str,
        field_name: &'static str,
        #[source]
        error: std::io::Error,
    },
    /// Failed to read or write flags or enum value.
    #[error("failed to read/write value of {packet_name}: {error}")]
    ValueError {
        packet_name: &'static str,
        #[source]
        error: std::io::Error,
    },
    /// Failed to read or write variable packet field length.
    #[error("failed to read/write length of field {field_name} from {packet_name}: {error}")]
    FieldLengthError {
        packet_name: &'static str,
        field_name: &'static str,
        #[source]
        error: std::io::Error,
    },
    /// Failed to read or write packet field (i.e. field implementing [`HelperReadWrite`]).
    #[error("failed to read/write field {field_name} from {packet_name}: {error}")]
    CompositeFieldError {
        packet_name: &'static str,
        field_name: &'static str,
        #[source]
        error: Box<Self>,
    },
    /// Failed to add padding to packet field.
    #[error("failed to pad field {field_name} from {packet_name}: {error}")]
    PaddingError {
        packet_name: &'static str,
        field_name: &'static str,
        #[source]
        error: std::io::Error,
    },
    /// Failed to read/write constant value.
    #[error("failed to read/write constant {const_val} from {packet_name}: {error}")]
    ConstantError {
        packet_name: &'static str,
        const_val: u64,
        #[source]
        error: std::io::Error,
    },
    /// Failed to read/write length of [`Packet`].
    #[error("failed to read/write length for Packet: {error}")]
    PacketLengthError {
        #[source]
        error: std::io::Error,
    },
    // #[error(transparent)]
    // Io(#[from] std::io::Error),
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
    /// (0x04, 0x06) Despawn Object.
    #[Id(0x04, 0x06)]
    DespawnObject(DespawnObjectPacket),
    /// (0x04, 0x07) Object Movement. (broadcast)
    #[Id(0x04, 0x07)]
    Movement(MovementPacket),
    /// (0x04, 0x08) Client Movement Action.
    #[Id(0x04, 0x08)]
    MovementAction(MovementActionPacket),
    /// (0x04, 0x0F) Enemy Killed (map broadcast).
    #[Id(0x04, 0x0F)]
    EnemyKilled(EnemyKilledPacket),
    /// (0x04, 0x13) Unknown.
    #[Id(0x04, 0x13)]
    Unk0413(Unk0413Packet),
    /// (0x04, 0x14) Client Interaction.
    #[Id(0x04, 0x14)]
    Interact(InteractPacket),
    /// (0x04, 0x15) Object Action or Set Object Tag. (unicast or broadcast)
    #[Id(0x04, 0x15)]
    SetTag(SetTagPacket),
    /// (0x04, 0x20) Change Class Request.
    #[Id(0x04, 0x20)]
    ChangeClassRequest(ChangeClassRequestPacket),
    /// (0x04, 0x21) Change Class Response.
    #[Id(0x04, 0x21)]
    ChangeClass(ChangeClassPacket),
    /// (0x04, 0x22) Enemy Action. (map broadcast)
    #[Id(0x04, 0x22)]
    EnemyAction(EnemyActionPacket),
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
    /// (0x04, 0x2C) Unknown.
    #[Id(0x04, 0x2C)]
    Unk042C(Unk042CPacket),
    /// (0x04, 0x2E) Load Learned Photon Arts. (broadcast)
    #[Id(0x04, 0x2E)]
    LoadPAs(LoadPAsPacket),
    /// (0x04, 0x3B) Despawn Player. (broadcast)
    #[Id(0x04, 0x3B)]
    DespawnPlayer(DespawnPlayerPacket),
    /// (0x04, 0x3C) Client Action Update.
    #[Id(0x04, 0x3C)]
    ActionUpdate(ActionUpdatePacket),
    /// (0x04, 0x52) Damage Received.
    #[Id(0x04, 0x52)]
    DamageReceive(DamageReceivePacket),
    /// (0x04, 0x5F) Set Title Request.
    #[Id(0x04, 0x5F)]
    SetTitleRequest(SetTitleRequestPacket),
    /// (0x04, 0x71) Object Movement End. (broadcast)
    #[Id(0x04, 0x71)]
    MovementEnd(MovementEndPacket),
    /// (0x04, 0x72) Set Title. (broadcast)
    #[Id(0x04, 0x72)]
    SetTitle(SetTitlePacket),
    /// (0x04, 0x75) Action End. (broadcast)
    #[Id(0x04, 0x75)]
    ActionEnd(ActionEndPacket),
    /// (0x04, 0x79) Unknown.
    #[Id(0x04, 0x79)]
    Unk0479(Unk0479Packet),
    /// (0x04, 0x80) Movement Action Response. (broadcast)
    #[Id(0x04, 0x80)]
    MovementActionServer(MovementActionServerPacket),
    /// (0x04, 0x81) Action Update Response. (broadcast)
    #[Id(0x04, 0x81)]
    ActionUpdateServer(ActionUpdateServerPacket),
    /// (0x04, 0x86) Unknown.
    #[Id(0x04, 0x86)]
    Unk0486(Unk0486Packet),
    /// (0x04, 0xB0) Unknown.
    #[Id(0x04, 0xB0)]
    Unk04B0(Unk04B0Packet),
    /// (0x04, 0xBA) Unknown.
    #[Id(0x04, 0xBA)]
    Unk04BA(Unk04BAPacket),
    /// (0x04, 0xEA) Unknown.
    #[Id(0x04, 0xEA)]
    Unk04EA(Unk04EAPacket),

    // Player status packets [0x06]
    #[Category(PacketCategory::PlayerStatus)]
    /// (0x06, 0x00) Set Player ID.
    #[Id(0x06, 0x00)]
    SetPlayerID(SetPlayerIDPacket),
    /// (0x06, 0x01) Deal damage to an object.
    #[Id(0x06, 0x01)]
    DealDamage(DealDamagePacket),
    /// (0x06, 0x05) EXP Gained. (broadcast)
    #[Id(0x06, 0x05)]
    GainedEXP(GainedEXPPacket),

    // Chat packets [0x07]
    #[Category(PacketCategory::Chat)]
    /// (0x07, 0x00) Chat Message.
    #[Id(0x07, 0x00)]
    ChatMessage(ChatMessage),

    // Spawn packets [0x08]
    #[Category(PacketCategory::Spawning)]
    /// (0x08, 0x04) Spawn Character. (broadcast)
    #[Id(0x08, 0x04)]
    #[Classic]
    CharacterSpawn(CharacterSpawnPacket),
    /// (0x08, 0x04) Spawn Character. (broadcast) (NGS)
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x08, 0x04)]
    #[NGS]
    CharacterSpawnNGS(CharacterSpawnNGSPacket),
    /// (0x08, 0x05) Spawn Transporter.
    #[Id(0x08, 0x05)]
    TransporterSpawn(TransporterSpawnPacket),
    /// (0x08, 0x09) Spawn Event.
    #[Id(0x08, 0x09)]
    EventSpawn(EventSpawnPacket),
    /// (0x08, 0x0B) Spawn Object.
    #[Id(0x08, 0x0B)]
    ObjectSpawn(ObjectSpawnPacket),
    /// (0x08, 0x0C) Spawn NPC.
    #[Id(0x08, 0x0C)]
    NPCSpawn(NPCSpawnPacket),
    /// (0x08, 0x0D) Spawn Enemy.
    #[Id(0x08, 0x0D)]
    EnemySpawn(EnemySpawnPacket),

    // Quest List packets [0x0B]
    #[Category(PacketCategory::QuestList)]
    /// (0x0B, 0x06) Start Cutscene.
    #[Id(0x0B, 0x06)]
    StartCutscene(StartCutscenePacket),
    /// (0x0B, 0x09) Minimap Reveal Chunk Request.
    #[Id(0x0B, 0x09)]
    MinimapRevealRequest(MinimapRevealRequestPacket),
    /// (0x0B, 0x13) Minimap Reveal.
    #[Id(0x0B, 0x13)]
    MinimapReveal(MinimapRevealPacket),
    /// (0x0B, 0x15) Available Quests Request.
    #[Id(0x0B, 0x15)]
    AvailableQuestsRequest(AvailableQuestsRequestPacket),
    /// (0x0B, 0x16) Available Quests Response.
    #[Id(0x0B, 0x16)]
    AvailableQuests(AvailableQuestsPacket),
    /// (0x0B, 0x17) Quest Category List Request
    #[Id(0x0B, 0x17)]
    QuestCategoryRequest(QuestCategoryRequestPacket),
    /// (0x0B, 0x18) Quest Category List Response
    #[Id(0x0B, 0x18)]
    QuestCategory(QuestCategoryPacket),
    /// (0x0B, 0x19) Quest Difficulty List Request
    #[Id(0x0B, 0x19)]
    QuestDifficultyRequest(QuestDifficultyRequestPacket),
    /// (0x0B, 0x1A) Quest Difficulty List Response
    #[Id(0x0B, 0x1A)]
    QuestDifficulty(QuestDifficultyPacket),
    /// (0x0B, 0x1B) Quest Category List Stopper.
    ///
    /// (S -> C) Sent when all quests are sent.
    ///
    /// Following: [`crate::protocol::Packet::QuestCategory`]
    #[Id(0x0B, 0x1B)]
    QuestCategoryStopper,
    /// (0x0B, 0x1C) Quest Difficulty Stopper.
    ///
    /// (S -> C) Sent when all quest difficulties are sent.
    ///
    /// Following: [`crate::protocol::Packet::QuestDifficulty`]
    #[Id(0x0B, 0x1C)]
    QuestDifficultyStopper,
    /// (0x0B, 0x1F) Set Quest Points. (broadcast)
    #[Id(0x0B, 0x1F)]
    SetQuestPoints(SetQuestPointsPacket),
    /// (0x0B, 0x20) Accept Quest.
    #[Id(0x0B, 0x20)]
    AcceptQuest(AcceptQuestPacket),
    /// (0x0B, 0x28) Add Quest Points. (broadcast)
    #[Id(0x0B, 0x28)]
    QuestPointsAdded(QuestPointsAddedPacket),
    /// (0x0B, 0x2F) Accept Quest. (alternative)
    #[Id(0x0B, 0x2F)]
    AcceptQuestOther(AcceptQuestOtherPacket),
    /// (0x0B, 0x30) Quest Counter Request.
    ///
    /// (C -> S) Sent when the client interacts with the quest counter.
    ///
    /// Respond with: (0x0B, 0x22)
    #[Id(0x0B, 0x30)]
    QuestCounterRequest,
    /// (0x0B, 0x62) Set EQ ARKS Level. (broadcast)
    #[Id(0x0B, 0x62)]
    EQARKSLevel(EQARKSLevelPacket),
    /// (0x0B, 0xAF) Unknown.
    #[Id(0x0B, 0xAF)]
    Unk0BAF(Unk0BAFPacket),
    /// (0x0B, 0xCD) Accept Story Quest.
    #[Id(0x0B, 0xCD)]
    AcceptStoryQuest(AcceptStoryQuestPacket),
    /// (0x0B, 0xD0) Unknown.
    #[Id(0x0B, 0xD0)]
    Unk0BD0(Unk0BD0Packet),
    /// (0x0B, 0xD4) Unknown.
    #[Id(0x0B, 0xD4)]
    Unk0BD4(Unk0BD4Packet),
    /// (0x0B, 0xF1) Unknown.
    #[Id(0x0B, 0xF1)]
    Unk0BF1(Unk0BF1Packet),

    // Party packets [0x0E]
    #[Category(PacketCategory::Party)]
    /// (0x0E, 0x00) Add New Party Member (broadcast).
    #[Id(0x0E, 0x00)]
    AddMember(AddMemberPacket),
    /// (0x0E, 0x01) Remove Party Member (broadcast).
    #[Id(0x0E, 0x01)]
    RemoveMember(RemoveMemberPacket),
    /// (0x0E, 0x02) Init Party.
    #[Id(0x0E, 0x02)]
    PartyInit(PartyInitPacket),
    /// (0x0E, 0x03) Removed From Party.
    ///
    /// (S -> C) Sent when a receiver is removed from the party.
    ///
    /// Follow with: [`Packet::PartyInit`]
    #[Id(0x0E, 0x03)]
    RemovedFromParty,
    /// (0x0E, 0x04) Party Invite Result.
    #[Id(0x0E, 0x04)]
    PartyInviteResult(PartyInviteResultPacket),
    /// (0x0E, 0x05) Party Invite Request.
    #[Id(0x0E, 0x05)]
    PartyInviteRequest(PartyInviteRequestPacket),
    /// (0x0E, 0x06) New Party Invite.
    #[Id(0x0E, 0x06)]
    NewInvite(NewInvitePacket),
    /// (0x0E, 0x07) Accept Party Invite.
    #[Id(0x0E, 0x07)]
    AcceptInvite(AcceptInvitePacket),
    /// (0x0E, 0x09) Leave Party.
    ///
    /// (C -> S) Sent when the player leaves the party.
    ///
    /// Respond with: [`Packet::RemovedFromParty`]
    #[Id(0x0E, 0x09)]
    LeaveParty,
    /// (0x0E, 0x0C) Set Party Settings.
    #[Id(0x0E, 0x0C)]
    NewPartySettings(NewPartySettingsPacket),
    /// (0x0E, 0x0D) Party Settings (broadcast).
    #[Id(0x0E, 0x0D)]
    PartySettings(PartySettingsPacket),
    /// (0x0E, 0x0E) Transfer Party Leadership.
    #[Id(0x0E, 0x0E)]
    TransferLeader(TransferLeaderPacket),
    /// (0x0E, 0x0F) New Party Leader (broadcast).
    #[Id(0x0E, 0x0F)]
    NewLeader(NewLeaderPacket),
    /// (0x0E, 0x10) Kick Party Member.
    #[Id(0x0E, 0x10)]
    KickMember(KickMemberPacket),
    /// (0x0E, 0x11) Party Member Kicked (broadcast).
    #[Id(0x0E, 0x11)]
    KickedMember(KickedMemberPacket),
    /// (0x0E, 0x17) Disband Party Request.
    #[Id(0x0E, 0x17)]
    DisbandParty(DisbandPartyPacket),
    /// (0x0E, 0x18) Party Disbanded (broadcast).
    ///
    /// (S -> C) Sent when the party is disbanded.
    ///
    /// Response to: [`Packet::DisbandParty`]
    ///
    /// Follow with: [`Packet::PartyInit`],
    /// [`Packet::SetPartyColor`]
    #[Id(0x0E, 0x18)]
    PartyDisbandedMarker,
    /// (0x0E, 0x19) Set Chat Status (broadcast).
    #[Id(0x0E, 0x19)]
    ChatStatus(ChatStatusPacket),
    /// (0x0E, 0x1A) Unknown.
    #[Id(0x0E, 0x1A)]
    Unk0E1A(Unk0E1APacket),
    /// (0x0E, 0x1B) Party Info.
    #[Id(0x0E, 0x1B)]
    PartyInfo(PartyInfoPacket),
    /// (0x0E, 0x1C) Pary Info Stopper.
    #[Id(0x0E, 0x1C)]
    PartyInfoStopper(PartyInfoStopperPacker),
    /// (0x0E, 0x1D) Party Details Request.
    #[Id(0x0E, 0x1D)]
    GetPartyDetails(GetPartyDetailsPacket),
    /// (0x0E, 0x1E) Party Details.
    #[Id(0x0E, 0x1E)]
    PartyDetails(PartyDetailsPacket),
    /// (0x0E, 0x1F) Party Details Stopper.
    ///
    /// (S -> C) Sent when no more party details are available.
    ///
    /// Following: [`crate::protocol::Packet::PartyDetails`]
    #[Id(0x0E, 0x1F)]
    PartyDetailsStopper,
    /// (0x0E, 0x21) Unknown.
    #[Id(0x0E, 0x21)]
    Unk0E21(Unk0E21Packet),
    /// (0x0E, 0x25) Set Quest Info.
    #[Id(0x0E, 0x25)]
    SetQuestInfo(SetQuestInfoPacket),
    /// (0x0E, 0x28) Player Is Busy.
    ///
    /// (C -> S) Sent when the player is busy (e.g. interacting with an NPC).
    ///
    /// Respond with: [`crate::protocol::Packet::NewBusyState`] (except for sender)
    #[Id(0x0E, 0x28)]
    SetBusy,
    /// (0x0E, 0x29) Player Is Not Busy.
    ///
    /// (C -> S) Sent when the player is no longer busy.
    ///
    /// Respond with: [`crate::protocol::Packet::NewBusyState`] (except for sender)
    #[Id(0x0E, 0x29)]
    SetNotBusy,
    /// (0x0E, 0x2B) New Busy State (broadcast).
    #[Id(0x0E, 0x2B)]
    NewBusyState(NewBusyStatePacket),
    /// (0x0E, 0x2C) Set Invite Decline.
    #[Id(0x0E, 0x2C)]
    SetInviteDecline(InviteDeclinePacket),
    /// (0x0E, 0x2E) Party Info Request.
    #[Id(0x0E, 0x2E)]
    GetPartyInfo(GetPartyInfoPacket),
    /// (0x0E, 0x31) Set Party Quest.
    #[Id(0x0E, 0x31)]
    SetPartyQuest(SetPartyQuestPacket),
    /// (0x0E, 0x4F) Set In Party Status.
    #[Id(0x0E, 0x4F)]
    SetPartyColor(SetPartyColorPacket),
    /// (0x0E, 0x52) Unknown.
    #[Id(0x0E, 0x52)]
    Unk0E52(Unk0E52Packet),
    /// (0x0E, 0x67) Party Setup Finish.
    #[Id(0x0E, 0x67)]
    PartySetupFinish(PartySetupFinishPacket),

    // Item packets [0x0F]
    #[Category(PacketCategory::Item)]
    /// (0x0F, 0x00) Item Attribute Data.
    #[Id(0x0F, 0x00)]
    LoadItemAttributes(ItemAttributesPacket),
    /// (0x0F, 0x01) Item Pickup Request.
    #[Id(0x0F, 0x01)]
    ItemPickupRequest(ItemPickupRequestPacket),
    /// (0x0F, 0x02) Item Pickup Response.
    #[Id(0x0F, 0x02)]
    ItemPickupResponse(ItemPickupResponsePacket),
    /// (0x0F, 0x04) New Item Drop.
    #[Id(0x0F, 0x04)]
    NewItemDrop(NewItemDropPacket),
    /// (0x0F, 0x05) Add Item To Inventory.
    #[Id(0x0F, 0x05)]
    AddedItem(AddedItemPacket),
    /// (0x0F, 0x06) Update Inventory.
    #[Id(0x0F, 0x06)]
    UpdateInventory(UpdateInventoryPacket),
    /// (0x0F, 0x08) Equip Item Request.
    #[Id(0x0F, 0x08)]
    EquipItemRequest(EquipItemRequestPacket),
    /// (0x0F, 0x09) Equip Item. (broadcast?)
    #[Id(0x0F, 0x09)]
    EquipItem(EquipItemPacket),
    /// (0x0F, 0x0A) Unequip Item Request.
    #[Id(0x0F, 0x0A)]
    UnequipItemRequest(UnequipItemRequestPacket),
    /// (0x0F, 0x0B) Unequip Item. (broadcast?)
    #[Id(0x0F, 0x0B)]
    UnequipItem(UnequipItemPacket),
    /// (0x0F, 0x0C) Load Player's Equipment (broadcast).
    #[Id(0x0F, 0x0C)]
    LoadEquiped(LoadEquipedPacket),
    /// (0x0F, 0x0D) Load Player's Inventory.
    #[Id(0x0F, 0x0D)]
    LoadPlayerInventory(LoadPlayerInventoryPacket),
    /// (0x0F, 0x0F) Move Items From Inventory To Storage Request.
    #[Id(0x0F, 0x0F)]
    MoveToStorageRequest(MoveToStorageRequestPacket),
    /// (0x0F, 0x10) Move Items From Inventory To Storage.
    #[Id(0x0F, 0x10)]
    MoveToStorage(MoveToStoragePacket),
    /// (0x0F, 0x11) Move Items From Storage To Inventory Request.
    #[Id(0x0F, 0x11)]
    MoveToInventoryRequest(MoveToInventoryRequestPacket),
    /// (0x0F, 0x12) Move Items From Storage To Inventory.
    #[Id(0x0F, 0x12)]
    MoveToInventory(MoveToInventoryPacket),
    /// (0x0F, 0x13) Load Player's Storages.
    #[Id(0x0F, 0x13)]
    LoadStorages(LoadStoragesPacket),
    /// (0x0F, 0x14) New Inventory Meseta Amount.
    #[Id(0x0F, 0x14)]
    InventoryMeseta(InventoryMesetaPacket),
    /// (0x0F, 0x15) Move Meseta Request.
    #[Id(0x0F, 0x15)]
    MoveMeseta(MoveMesetaPacket),
    /// (0x0F, 0x16) New Storage Meseta Amount.
    #[Id(0x0F, 0x16)]
    StorageMeseta(StorageMesetaPacket),
    /// (0x0F, 0x17) Discard Item Request.
    #[Id(0x0F, 0x17)]
    DiscardItemRequest(DiscardItemRequestPacket),
    /// (0x0F, 0x18) Move Items Between Storages Request.
    #[Id(0x0F, 0x18)]
    MoveStoragesRequest(MoveStoragesRequestPacket),
    /// (0x0F, 0x19) Move Items Between Storages.
    #[Id(0x0F, 0x19)]
    MoveStorages(MoveStoragesPacket),
    /// (0x0F, 0x1C) Get Item Description.
    #[Id(0x0F, 0x1C)]
    GetItemDescription(GetItemDescriptionPacket),
    /// (0x0F, 0x1D) Load Item Description.
    #[Id(0x0F, 0x1D)]
    LoadItemDescription(LoadItemDescriptionPacket),
    /// (0x0F, 0x21) Change Equiped Weapon (broadcast).
    #[Id(0x0F, 0x21)]
    EquipedWeapon(EquipedWeaponPacket),
    /// (0x0F, 0x22) Update Storage.
    #[Id(0x0F, 0x22)]
    UpdateStorage(UpdateStoragePacket),
    /// (0x0F, 0x25) Discard Storage Item Request.
    #[Id(0x0F, 0x25)]
    DiscardStorageItemRequest(DiscardStorageItemRequestPacket),
    /// (0x0F, 0x2B) Unknown.
    #[Id(0x0F, 0x2B)]
    Unk0F2B(Unk0F2BPacket),
    /// (0x0F, 0x30) Load Item Name.
    #[Id(0x0F, 0x30)]
    LoadItem(LoadItemPacket),
    /// (0x0F, 0x33) Learn Photon Art.
    #[Id(0x0F, 0x33)]
    LearnedPA(LearnedPAPacket),
    /// (0x0F, 0x5B) Unknown.
    ///
    /// (S -> C)
    #[Id(0x0F, 0x5B)]
    Unk0F5B,
    /// (0x0F, 0x65) Weapon Potential List.
    #[Id(0x0F, 0x65)]
    PotentialList(PotentialListPacket),
    /// (0x0F, 0x6F) Account Campaign List Request.
    ///
    /// (C -> S) Sent when a player has requested a list of available account campaigns.
    ///
    /// Respond with: [Packet::AccountCampaigns]
    #[Id(0x0F, 0x6F)]
    AccountCampaignsRequest,
    /// (0x0F, 0x70) Account Campaign List.
    #[Id(0x0F, 0x70)]
    AccountCampaigns(AccountCampaignsPacket),
    /// (0x0F, 0x71) Campaign Item List Request.
    #[Id(0x0F, 0x71)]
    CampaignItemsRequest(CampaignItemsRequestPacket),
    /// (0x0F, 0x72) Campaign Item List.
    #[Id(0x0F, 0x72)]
    CampaignItemList(CampaignItemListPacket),
    /// (0x0F, 0x73) Receive Campaign Rewards Request.
    #[Id(0x0F, 0x73)]
    ReceiveCampaignRequest(ReceiveCampaignRequestPacket),
    /// (0x0F, 0x8A) Character Campaign List Request.
    ///
    /// (C -> S) Sent when a player has requested a list of available character campaigns.
    ///
    /// Respond with: (0x0F, 0x8B)
    #[Id(0x0F, 0x8A)]
    CharacterCampaignsRequest,
    /// (0x0F, 0x9C) Unknown.
    #[Id(0x0F, 0x9C)]
    Unk0F9C(Unk0F9CPacket),
    /// (0x0F, 0xBC) Change Player's Current Palette (broadcast).
    #[Id(0x0F, 0xBC)]
    ChangeWeaponPalette(ChangeWeaponPalettePacket),
    /// (0x0F, 0xDF) Load Player's Material Storage.
    #[Id(0x0F, 0xDF)]
    LoadMaterialStorage(LoadMaterialStoragePacket),
    /// (0x0F, 0xE0) Move Item From Inventory To Material Storage Request.
    #[Id(0x0F, 0xE0)]
    MoveToMatStorageRequest(MoveToMatStorageRequestPacket),
    /// (0x0F, 0xE1) Move Item From Inventory To Material Storage.
    #[Id(0x0F, 0xE1)]
    MoveToMatStorage(MoveToMatStoragePacket),
    /// (0x0F, 0xE2) Move Item From Material Storage To Inventory Request.
    #[Id(0x0F, 0xE2)]
    MoveFromMatStorageRequest(MoveFromMatStorageRequestPacket),
    /// (0x0F, 0xE3) Move Item From Material Storage To Inventory.
    #[Id(0x0F, 0xE3)]
    MoveFromMatStorage(MoveFromMatStoragePacket),
    /// (0x0F, 0xE8) Move Item From Material Storage To Storage Request.
    #[Id(0x0F, 0xE8)]
    MoveMSToStorageRequest(MoveMSToStorageRequestPacket),
    /// (0x0F, 0xE9) Move Item From Material Storage To Storage.
    #[Id(0x0F, 0xE9)]
    MoveMSToStorage(MoveMSToStoragePacket),
    /// (0x0F, 0xEF) Unknown.
    #[Id(0x0F, 0xEF)]
    Unk0FEF(Unk0FEFPacket),
    /// (0x0F, 0xFC) Unknown.
    #[Id(0x0F, 0xFC)]
    Unk0FFC(Unk0FFCPacket),

    // Unknown 0x10 packets [0x10]
    #[Category(PacketCategory::Unk10)]
    /// (0x10, 0x00) Run Lua.
    #[Id(0x10, 0x00)]
    #[Classic]
    RunLua(LuaPacket),
    /// (0x10, 0x03) Unknown.
    #[Id(0x10, 0x03)]
    Unk1003(Unk1003Packet),

    // Login packets [0x11]
    #[Category(PacketCategory::Login)]
    /// (0x11, 0x00) Sega ID Login.
    #[Id(0x11, 0x00)]
    #[Classic]
    SegaIDLogin(SegaIDLoginPacket),
    /// (0x11, 0x01) Login Result.
    #[Id(0x11, 0x01)]
    #[Classic]
    LoginResponse(LoginResponsePacket),
    /// (0x11, 0x02) Character List Request.
    ///
    /// (C -> S) Sent when the client wants to get the list of available characters.
    ///
    /// Respond with:
    /// [`Packet::CharacterListResponse`]
    #[Id(0x11, 0x02)]
    CharacterListRequest,
    /// (0x11, 0x03) Character List.
    #[Id(0x11, 0x03)]
    #[Classic]
    CharacterListResponse(CharacterListPacket),
    /// (0x11, 0x04) Start Game.
    #[Id(0x11, 0x04)]
    StartGame(StartGamePacket),
    /// (0x11, 0x05) Create New Character Request.
    #[Id(0x11, 0x05)]
    #[Classic]
    CharacterCreate(CharacterCreatePacket),
    /// (0x11, 0x06) Delete Character Request.
    #[Id(0x11, 0x06)]
    CharacterDeletionRequest(CharacterDeletionRequestPacket),
    /// (0x11, 0x07) Create New Character Response.
    #[Id(0x11, 0x07)]
    CharacterCreateResponse(CharacterCreateResponsePacket),
    /// (0x11, 0x08) Delete Character.
    #[Id(0x11, 0x08)]
    CharacterDeletion(CharacterDeletionPacket),
    /// (0x11, 0x0B) Encryption Setup Request.
    #[Id(0x11, 0x0B)]
    EncryptionRequest(EncryptionRequestPacket),
    /// (0x11, 0x0C) Encryption Setup Response.
    #[Id(0x11, 0x0C)]
    EncryptionResponse(EncryptionResponsePacket),
    /// (0x11, 0x0D) Client Ping.
    #[Id(0x11, 0x0D)]
    ClientPing(ClientPingPacket),
    /// (0x11, 0x0E) Client Pong.
    #[Id(0x11, 0x0E)]
    ClientPong(ClientPongPacket),
    /// (0x11, 0x0F) Block List Request.
    ///
    /// (C -> S) Sent when the cilent wants a list of transferable blocks (i.e when the client
    /// interacts with the block transporter).
    ///
    /// Respond with:
    /// [`Packet::BlockList`],
    #[Id(0x11, 0x0F)]
    BlockListRequest,
    /// (0x11, 0x10) Block List.
    #[Id(0x11, 0x10)]
    BlockList(BlockListPacket),
    /// (0x11, 0x11) Block Switch Request.
    #[Id(0x11, 0x11)]
    BlockSwitchRequest(BlockSwitchRequestPacket),
    /// (0x11, 0x13) Block Switch Response.
    #[Id(0x11, 0x13)]
    #[Classic]
    BlockSwitchResponse(BlockSwitchResponsePacket),
    /// (0x11, 0x14) Block Login.
    #[Id(0x11, 0x14)]
    #[Classic]
    BlockLogin(BlockLoginPacket),
    /// (0x11, 0x1B) User Info.
    #[Id(0x11, 0x1B)]
    #[Classic]
    UserInfo(UserInfoPacket),
    /// (0x11, 0x1B) User Info (NGS).
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[Id(0x11, 0x1B)]
    #[NGS]
    UserInfoNGS(UserInfoNGSPacket),
    /// (0x11, 0x1E) Set Nickname Request.
    #[Id(0x11, 0x1E)]
    NicknameRequest(NicknameRequestPacket),
    /// (0x11, 0x1D) Set Nickname Response.
    #[Id(0x11, 0x1D)]
    NicknameResponse(NicknameResponsePacket),
    /// (0x11, 0x2B) Client Good Bye.
    ///
    /// (C -> S) Sent when the client is ready to disconnect.
    #[Id(0x11, 0x2B)]
    ClientGoodbye,
    /// (0x11, 0x2C) Block Balance.
    #[Id(0x11, 0x2C)]
    #[Classic]
    BlockBalance(BlockBalancePacket),
    /// (0x11, 0x2D) System Information.
    #[Id(0x11, 0x2D)]
    SystemInformation(SystemInformationPacket),
    /// (0x11, 0x3D) Ship List.
    #[Id(0x11, 0x3D)]
    ShipList(ShipListPacket),
    /// (0x11, 0x41) New Character Screen Request.
    ///
    /// (C -> S) Sent when a client wants to go to a new character screen to create a new character.
    ///
    /// Respond with: [`Packet::CreateCharacter1Response`]
    #[Id(0x11, 0x41)]
    CreateCharacter1,
    /// (0x11, 0x42) New Character Screen Response.
    #[Id(0x11, 0x42)]
    CreateCharacter1Response(CreateCharacter1ResponsePacket),
    /// (0x11, 0x54) New Player Referral Request.
    ///
    /// (C -> S) Sent when a client wants know if it is possible to be referred by another player
    /// (part of creating a new character).
    ///
    /// Respond with: [`Packet::CreateCharacter2Response`]
    #[Id(0x11, 0x54)]
    CreateCharacter2,
    /// (0x11, 0x55) New Player Referral Response.
    #[Id(0x11, 0x55)]
    CreateCharacter2Response(CreateCharacter2ResponsePacket),
    /// (0x11, 0x63) Vita Login.
    #[Id(0x11, 0x63)]
    #[Classic]
    VitaLogin(VitaLoginPacket),
    /// (0x11, 0x65) Full Block List Request.
    ///
    /// (C -> S) Sent when a client wants a list of all the blocks in the server
    ///
    /// Respond with: [`Packet::AllBlocksList`]
    #[Id(0x11, 0x64)]
    AllBlocksListRequest,
    /// (0x11, 0x65) Full Block List.
    #[Id(0x11, 0x65)]
    AllBlocksList(AllBlocksListPacket),
    /// (0x11, 0x66) Salon Entry Request.
    ///
    /// (C -> S) Sent when a client wants enter a salon.
    ///
    /// Respond with: [`Packet::SalonEntryResponse`]
    #[Id(0x11, 0x66)]
    SalonEntryRequest,
    /// (0x11, 0x67) Salon Entry Response.
    #[Id(0x11, 0x67)]
    #[Classic]
    SalonEntryResponse(SalonResponse),
    /// (0x11, 0x68) Anticheat Challenge Request.
    #[Id(0x11, 0x68)]
    ChallengeRequest(ChallengeRequestPacket),
    /// (0x11, 0x69) Anticheat Challenge Response.
    #[Id(0x11, 0x69)]
    ChallengeResponse(ChallengeResponsePacket),
    /// (0x11, 0x6B) Linked Sega ID Info Request.
    ///
    /// (C -> S) Sent when a client wants information about linked Sega ID (Vita only option?).
    ///
    /// Respond with: (0x11, 0x6C)
    #[Id(0x11, 0x6B)]
    #[Classic]
    SegaIDInfoRequest,
    /// (0x11, 0x6F) Unknown.
    #[Id(0x11, 0x6F)]
    Unk116F(Unk116FPacket),
    /// (0x11, 0x71) Notification Status.
    #[Id(0x11, 0x71)]
    NotificationStatus(NotificationStatusPacket),
    /// (0x11, 0x86) Login History Request.
    ///
    /// (C -> S) Sent when a client wants information about previous logins.
    ///
    /// Respond with: [`Packet::LoginHistoryResponse`]
    #[Id(0x11, 0x86)]
    LoginHistoryRequest,
    /// (0x11, 0x87) Login History.
    #[Id(0x11, 0x87)]
    LoginHistoryResponse(LoginHistoryPacket),
    /// (0x11, 0x8B) 2nd Password Operation Request.
    #[Id(0x11, 0x8B)]
    SecondPwdOperationRequest(SecondPwdOperationRequestPacket),
    /// (0x11, 0x8C) 2nd Password Operation.
    #[Id(0x11, 0x8C)]
    SecondPwdOperation(SecondPwdOperationPacket),
    /// (0x11, 0x90) Character Undeletion Request.
    #[Id(0x11, 0x90)]
    CharacterUndeletionRequest(CharacterUndeletionRequestPacket),
    /// (0x11, 0x91) Character Undeletion.
    #[Id(0x11, 0x91)]
    CharacterUndeletion(CharacterUndeletionPacket),
    /// (0x11, 0x97) Character Rename Request.
    #[Id(0x11, 0x97)]
    CharacterRenameRequest(CharacterRenameRequestPacket),
    /// (0x11, 0x98) Character Rename Response.
    #[Id(0x11, 0x98)]
    CharacterRename(CharacterRenamePacket),
    /// (0x11, 0x9B) Set New Character Name Request.
    #[Id(0x11, 0x9B)]
    CharacterNewNameRequest(CharacterNewNameRequestPacket),
    /// (0x11, 0x9C) Set New Character Name.
    #[Id(0x11, 0x9C)]
    CharacterNewName(CharacterNewNamePacket),
    /// (0x11, 0x9D) Nickname Change Request.
    ///
    /// (C -> S) Sent when a client wants to get nickname changing rights
    /// (i.e. clicked on "change nickname").
    #[Id(0x11, 0x9D)]
    NicknameChangeRequest,
    /// (0x11, 0xAF) Unknown.
    #[Id(0x11, 0xAF)]
    Unk11AF(Unk11AFPacket),
    /// (0x11, 0xB0) Unknown.
    #[Id(0x11, 0xB0)]
    Unk11B0(Unk11B0Packet),
    /// (0x11, 0xB8) Character Ship Transfer Rights Request.
    #[Id(0x11, 0xB8)]
    CharacterMoveRequest(CharacterMoveRequestPacket),
    /// (0x11, 0xB9) Character Ship Transfer Rights.
    #[Id(0x11, 0xB9)]
    CharacterMove(CharacterMovePacket),
    /// (0x11, 0xD7) Unknown.
    #[Id(0x11, 0xD7)]
    Unk11D7(Unk11D7Packet),
    /// (0x11, 0xDE) Player Report Request
    #[Id(0x11, 0xDE)]
    PlayerReported(PlayerReportedPacket),
    /// (0x11, 0xEA) Prohibited Nickname Entered.
    #[Id(0x11, 0xEA)]
    NicknameError(NicknameErrorPacket),
    /// (0x11, 0xED) Banner List.
    #[Id(0x11, 0xED)]
    BannerList(BannerListPacket),
    /// (0x11, 0xEE) Email 2FA Code Request.
    #[Id(0x11, 0xEE)]
    EmailCodeRequest(EmailCodeRequestPacket),
    /// (0x11, 0xFF) Unknown.
    #[Id(0x11, 0xFF)]
    #[Classic]
    Unk11FF(Unk11FFPacket),

    // Emergency packets [0x15]
    #[Category(PacketCategory::Emergency)]
    /// (0x15, 0x02) Start Emergency (broadcast).
    #[Id(0x15, 0x02)]
    SpawnEmergency(SpawnEmergencyPacket),
    /// (0x15, 0x03) End Emergency (broadcast).
    #[Id(0x15, 0x03)]
    EmergencyEnd(EmergencyEndPacket),
    /// (0x15, 0x05) Emergency Progress (broadcast).
    #[Id(0x15, 0x05)]
    EmergencyProgress(EmergencyProgressPacket),
    /// (0x15, 0x08) Unknown.
    #[Id(0x15, 0x08)]
    Unk1508(Unk1508Packet),
    /// (0x15, 0x11) Available Emergencies (?).
    #[Id(0x15, 0x11)]
    AvailableEmergencies(AvailableEmergenciesPacket),

    // Friends packets [0x18]
    #[Category(PacketCategory::Friends)]
    /// (0x18, 0x14) Friend List Request.
    #[Id(0x18, 0x14)]
    FriendListRequest(FriendListRequestPacket),
    /// (0x18, 0x15) Friend List.
    #[Id(0x18, 0x15)]
    FriendList(FriendListPacket),
    /// (0x18, 0x18) Send Friend Request.
    #[Id(0x18, 0x18)]
    SendFriendRequest(SendFriendRequestPacket),
    /// (0x18, 0x1A) Friend Request Sent.
    #[Id(0x18, 0x1A)]
    AddedRequest(AddedRequestPacket),

    // Unknown 0x19 packets [0x19]
    #[Category(PacketCategory::Unk19)]
    /// (0x19, 0x01) System Message (broadcast).
    #[Id(0x19, 0x01)]
    SystemMessage(SystemMessagePacket),
    /// (0x19, 0x04) Unknown.
    ///
    /// (C -> S)
    #[Id(0x19, 0x04)]
    Unk1904,
    /// (0x19, 0x06) Unknown.
    ///
    /// (C -> S)
    #[Id(0x19, 0x06)]
    Unk1906,
    /// (0x19, 0x09) Set Lobby Event (broadcast).
    #[Id(0x19, 0x09)]
    SetLobbyEvent(SetLobbyEventPacket),
    /// (0x19, 0x0F) Set Lobby Monitor Video (broadcast).
    #[Id(0x19, 0x0F)]
    LobbyMonitor(LobbyMonitorPacket),
    /// (0x19, 0x1C) Unknown.
    #[Id(0x19, 0x1C)]
    Unk191C(Unk191CPacket),

    // Mail packets [0x1A]
    #[Category(PacketCategory::Mail)]
    /// (0x1A, 0x00) Mail List Request.
    #[Id(0x1A, 0x00)]
    MailListRequest(MailListRequestPacket),
    /// (0x1A, 0x01) Mail List.
    #[Id(0x1A, 0x01)]
    MailList(MailListPacket),
    /// (0x1A, 0x02) Delete Mail Request.
    #[Id(0x1A, 0x02)]
    DeleteMailRequest(DeleteMailRequestPacket),
    /// (0x1A, 0x03) Deleted Mail.
    #[Id(0x1A, 0x03)]
    DeletedMail(DeletedMailPacket),
    /// (0x1A, 0x06) Mail Body Request.
    #[Id(0x1A, 0x06)]
    MailBodyRequest(MailBodyRequestPacket),
    /// (0x1A, 0x07) Mail Body.
    #[Id(0x1A, 0x07)]
    MailBody(MailBodyPacket),
    /// (0x1A, 0x0D) New Mail Marker.
    ///
    /// (S -> C) Sent when the client receives a new mail message.
    #[Id(0x1A, 0x0D)]
    NewMailMarker,

    // Alliance packets [0x1C]
    #[Category(PacketCategory::Characters)]
    /// (0x1C, 0x10) Get Alliance Status.
    ///
    /// (C -> S) Sent when the client wants to get information about their alliance status (during
    /// initial load or when clicking on other players).
    #[Id(0x1C, 0x10)]
    GetAllianceStatus,

    // Unknown 0x1E packets [0x1E]
    #[Category(PacketCategory::Unk1E)]
    /// (0x1E, 0x0C) Unknown.
    #[Id(0x1E, 0x0C)]
    Unk1E0C(Unk1E0CPacket),

    // Daily order packets [0x1F]
    #[Category(PacketCategory::DailyOrders)]
    /// (0x1F, 0x01) Taken Daily Orders Request.
    #[Id(0x1F, 0x01)]
    TakenOrdersRequest(TakenOrdersRequestPacket),
    /// (0x1F, 0x02) Daily Orders Request.
    #[Id(0x1F, 0x02)]
    OrderListRequest(OrderListRequestPacket),
    /// (0x1F, 0x03) Taken Daily Orders.
    #[Id(0x1F, 0x03)]
    OrderList(OrderListPacket),
    /// (0x1F, 0x08) Taken Daily Orders.
    #[Id(0x1F, 0x08)]
    TakenOrders(TakenOrdersPacket),
    /// (0x1F, 0x0F) Unknown
    #[Id(0x1F, 0x0F)]
    Unk1F0F(Unk1F0FPacket),

    // Palette packets [0x21]
    #[Category(PacketCategory::Palette)]
    /// (0x21, 0x01) Load Palettes.
    #[Id(0x21, 0x01)]
    LoadPalette(LoadPalettePacket),
    /// (0x21, 0x02) Full Palette Info Request.
    ///
    /// (C -> S) Sent when a player wants information about palettes and photon arts.
    ///
    /// Respond with: [`Packet::FullPaletteInfo`]
    #[Id(0x21, 0x02)]
    FullPaletteInfoRequest,
    /// (0x21, 0x03) Full Palette Info.
    #[Id(0x21, 0x03)]
    FullPaletteInfo(FullPaletteInfoPacket),
    /// (0x21, 0x04) Set Active Palette.
    #[Id(0x21, 0x04)]
    SetPalette(SetPalettePacket),
    /// (0x21, 0x05) Update Subpalette.
    #[Id(0x21, 0x05)]
    UpdateSubPalette(UpdateSubPalettePacket),
    /// (0x21, 0x06) Update Palette.
    #[Id(0x21, 0x06)]
    UpdatePalette(UpdatePalettePacket),
    /// (0x21, 0x08) Set Active Subpalette.
    #[Id(0x21, 0x08)]
    SetSubPalette(SetSubPalettePacket),
    /// (0x21, 0x0A) Set Default Photon Arts (?)
    #[Id(0x21, 0x0A)]
    SetDefaultPAs(SetDefaultPAsPacket),
    /// (0x21, 0x0F) New Default Photon Arts (?)
    #[Id(0x21, 0x0F)]
    NewDefaultPAs(NewDefaultPAsPacket),

    // Flag packets [0x23]
    #[Category(PacketCategory::Flag)]
    /// (0x23, 0x02) Set Flag.
    #[Id(0x23, 0x02)]
    SetFlag(SetFlagPacket),
    /// (0x23, 0x04) Server Set Flag.
    #[Id(0x23, 0x04)]
    ServerSetFlag(ServerSetFlagPacket),
    /// (0x23, 0x05) Server Set Parameter.
    #[Id(0x23, 0x05)]
    ServerSetParam(ServerSetParamPacket),
    /// (0x23, 0x06) Load Account Flags.
    #[Id(0x23, 0x06)]
    AccountFlags(AccountFlagsPacket),
    /// (0x23, 0x07) Load Character Flags.
    #[Id(0x23, 0x07)]
    CharacterFlags(CharacterFlagsPacket),
    /// (0x23, 0x0A) Cutscene Ended.
    #[Id(0x23, 0x0A)]
    CutsceneEnd(CutsceneEndPacket),
    /// (0x23, 0x0B) Skit Item Add Request.
    #[Id(0x23, 0x0B)]
    SkitItemAddRequest(SkitItemAddRequestPacket),
    /// (0x23, 0x0C) Skit Item Add Response.
    #[Id(0x23, 0x0C)]
    SkitItemAddResponse(SkitItemAddResponsePacket),
    /// (0x23, 0x0D) Unknown.
    #[Id(0x23, 0x0D)]
    Unk230D(Unk230DPacket),
    /// (0x23, 0x0E) Unknown.
    #[Id(0x23, 0x0E)]
    Unk230E(Unk230EPacket),
    /// (0x23, 0x10) Unknown.
    ///
    /// (C -> S)
    #[Id(0x23, 0x10)]
    Unk2310,
    /// (0x23, 0x15) Unknown
    #[Id(0x23, 0x15)]
    Unk2315(Unk2315Packet),

    // Friend avatar packets [0x26]
    #[Category(PacketCategory::FriendAvatar)]
    /// (0x26, 0x00) Friend Avatar Data Request.
    #[Id(0x26, 0x00)]
    FriendAvatarDataRequest(FriendAvatarDataRequestPacket),
    /// (0x26, 0x08) Friend Avatar Data Response.
    #[Id(0x26, 0x08)]
    FriendAvatarDataResponse(FriendAvatarDataResponsePacket),

    // Unknown 0x2A packets [0x2A]
    #[Category(PacketCategory::Unk2A)]
    /// (0x2A, 0x08) Unknown.
    #[Id(0x2A, 0x08)]
    Unk2A08(Unk2A08Packet),

    // Settings packets [0x2B]
    #[Category(PacketCategory::Settings)]
    /// (0x2B, 0x00) Settings Request.
    ///
    /// (C -> S) Sent when the client wants to get player settings.
    ///
    /// Response to: [`Packet::LoginResponse`]
    ///
    /// Respond with: [`Packet::LoadSettings`]
    #[Id(0x2B, 0x00)]
    SettingsRequest,
    /// (0x2B, 0x01) Save Player Settings.
    #[Id(0x2B, 0x01)]
    SaveSettings(SaveSettingsPacket),
    /// (0x2B, 0x02) Load Player Settings.
    #[Id(0x2B, 0x02)]
    LoadSettings(LoadSettingsPacket),

    // Player shop packets [0x2D]
    #[Category(PacketCategory::PlayerShop)]
    /// (0x2D, 0x02) Player Shop Item Search Request.
    #[Id(0x2D, 0x02)]
    ProductSearchRequest(ProductSearchRequestPacket),
    /// (0x2D, 0x03) Player Shop Item Search Request.
    #[Id(0x2D, 0x03)]
    ProductSearchResponse(ProductSearchResponsePacket),
    /// (0x2D, 0x0B) Player Shop Details Request.
    #[Id(0x2D, 0x0B)]
    PlayerShopDetailsRequest(PlayerShopDetailsRequestPacket),
    /// (0x2D, 0x0C) Player Shop Details Response.
    #[Id(0x2D, 0x0C)]
    PlayerShopDetailsResponse(PlayerShopDetailsResponsePacket),
    /// (0x2D, 0x0D) Character Search Request.
    #[Id(0x2D, 0x0D)]
    CharacterSearchRequest(CharacterSearchRequestPacket),
    /// (0x2D, 0x0E) Character Search Response.
    #[Id(0x2D, 0x0E)]
    CharacterSearchResponse(CharacterSearchResponsePacket),
    /// (0x2D, 0x12) Recruiting Alliances List Request.
    #[Id(0x2D, 0x12)]
    RecruitingAlliancesRequest(RecruitingAlliancesRequestPacket),
    /// (0x2D, 0x13) Recruiting Alliances List Response.
    #[Id(0x2D, 0x13)]
    RecruitingAlliancesResponse(RecruitingAlliancesResponsePacket),

    // Symbol art packets [0x2F]
    #[Category(PacketCategory::SymbolArt)]
    /// (0x2F, 0x00) Request Symbol Art Data (client).
    #[Id(0x2F, 0x00)]
    SymbolArtClientDataRequest(SymbolArtClientDataRequestPacket),
    /// (0x2F, 0x01) Request Symbol Art Data (server).
    #[Id(0x2F, 0x01)]
    SymbolArtDataRequest(SymbolArtDataRequestPacket),
    /// (0x2F, 0x02) Symbol Art Data (serverbound).
    #[Id(0x2F, 0x02)]
    SymbolArtData(SymbolArtDataPacket),
    /// (0x2F, 0x03) Symbol Art Data (clientbound).
    #[Id(0x2F, 0x03)]
    SymbolArtClientData(SymbolArtClientDataPacket),
    /// (0x2F, 0x04) Change Symbol Art Slot.
    #[Id(0x2F, 0x04)]
    ChangeSymbolArt(ChangeSymbolArtPacket),
    /// (0x2F, 0x05) Change Symbol Art Slot Result.
    #[Id(0x2F, 0x05)]
    SymbolArtResult(SymbolArtResultPacket),
    /// (0x2F, 0x06) Request Symbol Art List.
    ///
    /// (C -> S) Sent when the client wants to get the list of player's saved symbol arts.
    ///
    /// Respond with: [`Packet::SymbolArtList`]
    #[Id(0x2F, 0x06)]
    SymbolArtListRequest,
    /// (0x2F, 0x07) Saved Symbol Art List.
    #[Id(0x2F, 0x07)]
    SymbolArtList(SymbolArtListPacket),
    /// (0x2F, 0x08) Send Symbol Art.
    #[Id(0x2F, 0x08)]
    #[Classic]
    SendSymbolArt(SendSymbolArtPacket),
    /// (0x2F, 0x09) Received Symbol Art.
    #[Id(0x2F, 0x09)]
    #[Classic]
    ReceiveSymbolArt(ReceiveSymbolArtPacket),

    // Unknown 0x31 packets [0x31]
    #[Category(PacketCategory::Unk31)]
    /// (0x31, 0x01) New Titles Request.
    ///
    /// (C -> S) Sent whet the client interacts with the title counter.
    ///
    /// Respond with: [`Packet::NewTitles`].
    #[Id(0x31, 0x01)]
    NewTitlesRequest,
    /// (0x31, 0x02) New Titles.
    #[Id(0x31, 0x02)]
    NewTitles(NewTitlesPacket),
    /// (0x31, 0x03) Title List Request.
    ///
    /// (C -> S) Sent whet the client interacts with the title counter.
    ///
    /// Respond with: [`Packet::TitleList`].
    #[Id(0x31, 0x03)]
    TitleListRequest,
    /// (0x31, 0x04) Title List.
    #[Id(0x31, 0x04)]
    TitleList(TitleListPacket),
    /// (0x31, 0x05) Load Title Names.
    #[Id(0x31, 0x05)]
    LoadTitles(LoadTitlesPacket),
    /// (0x31, 0x06) Title Condition Request.
    #[Id(0x31, 0x06)]
    GetTitleCondition(GetTitleConditionPacket),
    /// (0x31, 0x07) Title Condition.
    #[Id(0x31, 0x07)]
    LoadTitleCondition(LoadTitleConditionPacket),
    /// (0x31, 0x08) Play Achievements Request.
    ///
    /// (C -> S) Sent when the client request play achievements list.
    ///
    /// Respond with: [`Packet::PlayAchievementsResponse`]
    #[Id(0x31, 0x08)]
    PlayAchievementsRequest,
    /// (0x31, 0x09) Play Achievements Request.
    #[Id(0x31, 0x09)]
    PlayAchievementsResponse(PlayAchievementsResponsePacket),
    /// (0x31, 0x0A) Receive Title Reward Request.
    #[Id(0x31, 0x0A)]
    ReceiveTitleRewardRequest(ReceiveTitleRewardRequestPacket),
    /// (0x31, 0x0B) Receive Title Reward Response.
    #[Id(0x31, 0x0B)]
    ReceiveTitleReward(ReceiveTitleRewardPacket),

    // Unknown 0x34 packets [0x34]
    #[Category(PacketCategory::Unk34)]
    /// (0x34, 0x35) Unknown.
    #[Id(0x34, 0x35)]
    Unk3435(Unk3435Packet),
    /// (0x34, 0x5C) Unknown.
    #[Id(0x34, 0x5C)]
    Unk345C(Unk345CPacket),
    /// (0x34, 0x70) Player Shop Top Items List Request.
    ///
    /// (C -> S) Sent when the client enters the player shop.
    ///
    /// Respond with: [`Packet::PlayerShopListResponse`]
    #[Id(0x34, 0x70)]
    PlayerShopListRequest,
    /// (0x34, 0x71) Player Shop Top Items List Response.
    #[Id(0x34, 0x71)]
    PlayerShopListResponse(PlayerShopListResponsePacket),

    // ARKS Misions packets [0x4A]
    #[Category(PacketCategory::ARKSMissions)]
    /// (0x4A, 0x00) ARKS Mission List Request.
    ///
    /// (C -> S) Sent when the client wants to get a list of missions (i.e. every time the client
    /// enters the system menu).
    ///
    /// Respond with: [`Packet::MissionList`]
    #[Id(0x4A, 0x00)]
    MissionListRequest,
    /// (0x4A, 0x01) ARKS Mission List.
    #[Id(0x4A, 0x01)]
    MissionList(MissionListPacket),
    /// (0x4A, 0x03) Unknown.
    #[Id(0x4A, 0x03)]
    Unk4A03(Unk4A03Packet),
    /// (0x4A, 0x0C) Set Tracked Mission Request.
    #[Id(0x4A, 0x0C)]
    SetTrackedMission(SetTrackedMissionPacket),

    // Classic Mission Pass packets [0x4D]
    #[Category(PacketCategory::MissionPass)]
    /// (0x4D, 0x00) Mission Pass Info Request.
    ///
    /// (C -> S) Sent when the client wants to get information about the mission pass (i.e. every
    /// time the client enters the system menu).
    ///
    /// Respond with: [`Packet::MissionPassInfo`]
    #[Id(0x4D, 0x00)]
    MissionPassInfoRequest,
    /// (0x4D, 0x01) Mission Pass Info.
    #[Id(0x4D, 0x01)]
    MissionPassInfo(MissionPassInfoPacket),
    /// (0x4D, 0x02) Mission Pass  Request.
    ///
    /// (C -> S) Sent when the client wants to get the mission pass (i.e. every time the client
    /// enters the system menu).
    ///
    /// Respond with: [`Packet::MissionPass`]
    #[Id(0x4D, 0x02)]
    MissionPassRequest,
    /// (0x4D, 0x03) Mission Pass.
    #[Id(0x4D, 0x03)]
    MissionPass(MissionPassPacket),

    //Other packets
    /// Raw packet with header.
    #[Raw]
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
    #[Raw]
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
    /// Unknown 0x1E packets. See [`unk1e`]
    Unk1E,
    /// Daily orders related packets. See [`orders`]
    DailyOrders,
    /// Palette related packets. See [`palette`]
    Palette,
    /// Flag packets. See [`flag`]
    Flag,
    /// Friend avatar relater packets. See [`friendavatar`]
    FriendAvatar,
    /// Unknown 0x2A packets. See [`unk2a`]
    Unk2A,
    /// Settings related packets. See [`settings`]
    Settings,
    /// Player shop related packets. See [`playershop`]
    PlayerShop,
    /// Symbol Art related packets. See [`symbolart`]
    SymbolArt,
    /// Unknown 0x31 packets. See [`unk31`]
    Unk31,
    /// Unknown 0x34 packets. See [`unk34`]
    Unk34,
    /// ARKS Missions related packets. See [`missions`]
    ARKSMissions,
    /// Classic Mission pass related packets. See [`missionpass`]
    MissionPass,
}

// ----------------------------------------------------------------
// PacketEncryption impls
// ----------------------------------------------------------------
impl PacketEncryption for Packet {
    fn is_enc_data(&self) -> bool {
        matches!(self, Self::EncryptionRequest(_))
    }
    fn as_enc_data(&self) -> Option<&[u8]> {
        if let Self::EncryptionRequest(data) = self {
            Some(&data.rsa_data)
        } else {
            None
        }
    }
    fn mut_enc_data(&mut self) -> Option<&mut Vec<u8>> {
        if let Self::EncryptionRequest(data) = self {
            Some(&mut data.rsa_data)
        } else {
            None
        }
    }
}
#[cfg(feature = "proxy")]
impl PacketEncryption for ProxyPacket {
    fn is_enc_data(&self) -> bool {
        matches!(self, Self::EncryptionRequest(_))
    }
    fn as_enc_data(&self) -> Option<&[u8]> {
        if let Self::EncryptionRequest(data) = self {
            Some(&data.rsa_data)
        } else {
            None
        }
    }
    fn mut_enc_data(&mut self) -> Option<&mut Vec<u8>> {
        if let Self::EncryptionRequest(data) = self {
            Some(&mut data.rsa_data)
        } else {
            None
        }
    }
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
    /// Creates a new header.
    pub fn new(id: u8, subid: u16, flag: Flags) -> Self {
        Self { id, subid, flag }
    }
    pub fn read(
        reader: &mut (impl Read + Seek),
        packet_type: PacketType,
    ) -> Result<Self, PacketError> {
        let (id, subid, flag) = if !matches!(packet_type, PacketType::NGS) {
            let id = reader.read_u8().map_err(|e| PacketError::FieldError {
                packet_name: "PacketHeader",
                field_name: "id",
                error: e,
            })?;
            let subid = reader.read_u8().map_err(|e| PacketError::FieldError {
                packet_name: "PacketHeader",
                field_name: "subid",
                error: e,
            })? as u16;
            let flag = Flags::read(reader, packet_type, 0, 0)?;
            reader.read_u8().map_err(|e| PacketError::PaddingError {
                packet_name: "PacketHeader",
                field_name: "flag",
                error: e,
            })?;
            (id, subid, flag)
        } else {
            let flag = Flags::read(reader, packet_type, 0, 0)?;
            let id = reader.read_u8().map_err(|e| PacketError::FieldError {
                packet_name: "PacketHeader",
                field_name: "id",
                error: e,
            })?;
            let subid = reader
                .read_u16::<LittleEndian>()
                .map_err(|e| PacketError::FieldError {
                    packet_name: "PacketHeader",
                    field_name: "subid",
                    error: e,
                })?;
            (id, subid, flag)
        };

        Ok(Self { id, subid, flag })
    }
    pub fn write(&self, packet_type: PacketType) -> Vec<u8> {
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

bitflags::bitflags! {
    /// Packet flags.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
    #[BitFlags(u8)]
    pub struct Flags: u8 {
        /// Set when the packet contains variable length data.
        const PACKED = 1 << 2;
        const FLAG_10 = 1 << 4;
        /// Set when the [`Packet::Movement`] has all fields set.
        const FULL_MOVEMENT = 1 << 5;
        /// Set for all (?) of (0x04) packets.
        const OBJECT_RELATED = 1 << 6;
    }
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
    /// World (in map pool) object.
    World = 16,
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

// temporarily hidden
#[doc(hidden)]
#[inline(always)]
pub fn read_magic(reader: &mut impl Read, sub: u32, xor: u32) -> std::io::Result<u32> {
    Ok((reader.read_u32::<LittleEndian>()? ^ xor) - sub)
}
#[doc(hidden)]
#[inline(always)]
pub fn write_magic(num: u32, sub: u32, xor: u32) -> u32 {
    (num + sub) ^ xor
}

// ----------------------------------------------------------------
// Tests
// ----------------------------------------------------------------

#[cfg(all(feature = "ppac", test))]
mod tests {
    use super::Packet;
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
                            let error = packet.parse_error.unwrap();
                            println!("{entry:?}, {id} - FAIL (can't read): {error}");
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

                    // failing packets
                    if matches!(
                        packet,
                        // extra data at the end
                        Packet::Movement(_)
                            // unclean name string
                            | Packet::EventSpawn(_)
                            // unclean name string
                            | Packet::ObjectSpawn(_)
                            // lengths are sometimes 0 instead of magicked 0
                            | Packet::FriendList(_)
                            | Packet::ShipList(_)
                            // current testing structure replaces the last char with null byte
                            | Packet::LoadItem(_)
                            // current testing structure replaces the last char with null byte
                            | Packet::LoadTitles(_)
                            // current testing structure replaces the last char with null byte
                            | Packet::PlayAchievementsResponse(_)
                    ) {
                        continue;
                    }
                    // NGS specific failing packets
                    #[cfg(feature = "ngs_packets")]
                    if matches!(
                        packet,
                        // changed across version and currently extremely unfinished
                        Packet::CharacterSpawnNGS(_)
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
