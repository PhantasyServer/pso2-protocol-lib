//! Server related packets. \[0x03\]
use super::{HelperReadWrite, ObjectHeader, PacketReadWrite};
use crate::{fixed_types::FixedBytes, AsciiString};

// ----------------------------------------------------------------
// Server packets
// ----------------------------------------------------------------

/// (0x03, 0x00) Map Transfer.
///
/// (S -> C) Sent when the client is being moved between zones (e.g. arks lobby <-> casino).
///
/// Respond with: [`crate::protocol::Packet::MapLoaded`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x00)]
pub struct MapTransferPacket {
    /// Target zone object.
    pub map: ObjectHeader,
    /// Receiving player object.
    pub target: ObjectHeader,
    /// Target zone settings.
    pub settings: ZoneSettings,
}

/// (0x03, 0x05) Move Quest Zone.
///
/// (C -> S) Sent when the player moves between quest zones.
///
/// Respond with: load zone map.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x05)]
pub struct MoveZonePacket {
    /// Current world object.
    pub world: ObjectHeader,
    /// ID of the zone that the player is currently in.
    pub current_zone_id: u32,
    /// Interacted "door" ID.
    pub door_id: u32,
}

/// (0x03, 0x06) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x06)]
pub struct Unk0306Packet {
    pub unk: [u8; 0xC],
}

/// (0x03, 0x08) Server Hello.
///
/// (S -> C) Sent when the client connects to the block server.
///
/// Respond with: [`crate::protocol::Packet::EncryptionRequest`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x08)]
pub struct ServerHelloPacket {
    /// Unknown. Seems to be always 0x03.
    pub unk1: u16,
    #[SeekAfter(4)]
    /// Block Id.
    pub blockid: u16,
    pub unk2: u32,
}

/// (0x03, 0x10) Map Loading Finished.
///
/// (C -> S) Sent when the client has finished loading the map.
///
/// Response to:
/// [`crate::protocol::Packet::MapTransfer`] or
/// [`crate::protocol::Packet::LoadLevel`].
///
/// Respond with:
/// user data,
/// object spawn packets,
/// [`crate::protocol::Packet::UnlockControls`] and
/// [`crate::protocol::Packet::FinishLoading`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x10)]
pub struct MapLoadedPacket {
    /// Loaded zone object.
    pub map_object: ObjectHeader,
    pub unk: [u8; 0x20],
}

/// (0x03, 0x11) Move Campship -> Quest Level (selected area).
///
/// (C -> S) Sent when the client wants to move to the quest level with specified area.
///
/// Respond with: load quest map.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x11)]
pub struct CampshipDownAreaPacket {
    pub world: ObjectHeader,
    pub unk4: u32,
    pub area: u32,
}

/// (0x03, 0x12) Move Lobby -> Campship.
///
/// (C -> S) Sent when the client wants to move to campship.
///
/// Respond with: load campship map.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x12)]
pub struct ToCampshipPacket {
    pub world: ObjectHeader,
    pub unk4: u32,
}

/// (0x03, 0x16) Move Campship -> Quest Level.
///
/// (C -> S) Sent when the client wants to move to the quest level.
///
/// Respond with: load quest map.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x16)]
pub struct CampshipDownPacket {
    pub world: ObjectHeader,
    pub unk4: u32,
}

/// (0x03, 0x17) Move Quest Level -> Campship.
///
/// (C -> S) Sent when the player interacts with the spawn telepipe
/// during the quest.
///
/// Respond with: load campship map.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x17)]
pub struct ReturnToCampshipPacket {
    pub world: ObjectHeader,
}

/// (0x03, 0x19) Move Quest Level Finish -> Campship.
///
/// (C -> S) Sent when the player finishes the quest and interacts with the telepipe.
///
/// Respond with: load campship map.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x19)]
pub struct ReturnToCampshipFinalPacket {
    pub world: ObjectHeader,
}

/// (0x03, 0x1A) Move Quest Level Death -> Campship.
///
/// (C -> S) Sent when the player dies and selects to teleport to campship
///
/// Respond with: load campship map.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x1A)]
pub struct DeathToCampshipPacket {
    pub world: ObjectHeader,
}

/// (0x03, 0x1C) Move Campship -> Lobby.
///
/// (C -> S) Sent when the client wants to move to the lobby from campship.
///
/// Respond with: load lobby map.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x1C)]
pub struct CampshipToLobbyPacket {
    pub world: ObjectHeader,
}

/// (0x03, 0x24) Load Level.
///
/// (S -> C) Sent when the client is moved to a new map. (e.g. lobby <-> campship)
///
/// Respond with: [`crate::protocol::Packet::MapLoaded`]
///
/// Followed by: [`crate::protocol::Packet::SetPlayerID`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x24)]
#[Flags(Flags::PACKED)]
#[Magic(0x7542, 0x5E)]
pub struct LoadLevelPacket {
    /// Initial zone object.
    pub map_object: ObjectHeader,
    /// Host player.
    pub host: ObjectHeader,
    /// Settings for the initial zone (i.e. first zone that the player will appear in).
    pub settings: ZoneSettings,
    pub world_obj: ObjectHeader,
    pub quest: ObjectHeader,
    pub party: ObjectHeader,
    pub unk7: AsciiString,
    /// Settings for other zones.
    pub other_settings: Vec<ZoneSettings>,
    pub warps: Vec<WarpInfo>,
    pub unk10: Vec<LoadLevelThing3>,
    pub unk11: Vec<LoadLevelThing4>,
    pub unk12: Vec<LoadLevelThing5>,
    pub unk13: Vec<LoadLevelThing6>,
    pub unk14: Vec<LoadLevelThing7>,
    pub unk15: Vec<LoadLevelThing8>,
    pub unk16: Vec<UnkThing1>,
    pub unk17: AsciiString,
    pub unk18: u32,
    pub unk19: u32,
    pub unk20: u32,
    pub unk21: FixedBytes<0x3C>,
    pub unk22: u32,
    pub unk23: [u8; 0x10],
    pub unk24: [u8; 0x10],
    pub unk25: Vec<u32>,
    pub unk26: FixedBytes<0x200>,
    pub unk27: Vec<UnkThing2>,
    pub unk28: AsciiString,
    pub unk29: AsciiString,
    pub unk30: u64,
    pub unk31: u64,
    pub unk32: u8,
    pub unk33: u8,
    pub unk34: u8,
    pub unk35: u8,
    pub unk36: u32,
    pub unk37: [u8; 0x14],
    pub unk38: u64,
    pub unk39: u32,
    pub unk40: u32,
    pub unk41: [u8; 0x12],
    pub unk42: u64,
    pub unk43: u8,
    pub unk44: u8,
    pub unk45: Vec<LoadLevelThing9>,
    pub unk46: AsciiString,
    pub unk47: Vec<LoadLevelThing10>,
    pub unk48: u32,
    pub unk49: [u8; 0x14],
    pub unk50: [u8; 0x14],
    pub unk51: u32,
}

/// (0x03, 0x34) Move Casino -> Lobby.
///
/// (C -> S) Sent when the client wants to move from casino to lobby.
///
/// Respond with: load lobby map.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x34)]
pub struct CasinoToLobbyPacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
}

/// (0x03, 0x35) Move Lobby -> Casino.
///
/// (C -> S) Sent when the client wants to move from lobby to casino.
///
/// Respond with: load casino map.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x35)]
pub struct CasinoTransportPacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

/// (0x03, 0x38) Move Bridge -> Lobby.
///
/// (C -> S) Sent when the client wants to move from bridge to lobby.
///
/// Respond with: load lobby map.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x38)]
pub struct BridgeToLobbyPacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub zone_id: u32,
    pub unk4: u32,
}

/// (0x03, 0x39) Move Lobby -> Bridge.
///
/// (C -> S) Sent when the client wants to move from lobby to bridge.
///
/// Respond with: load bridge map.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x39)]
pub struct BridgeTransportPacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

/// (0x03, 0x3B) Move Cafe -> Lobby.
///
/// (C -> S) Sent when the client wants to move from cafe to lobby.
///
/// Respond with: load lobby map.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x3B)]
pub struct CafeToLobbyPacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub zone_id: u32,
    pub unk4: u32,
}

/// (0x03, 0x3C) Move Lobby -> Cafe.
///
/// (C -> S) Sent when the client wants to move from lobby to cafe.
///
/// Respond with: load cafe map.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x3C)]
pub struct CafeTransportPacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

/// (0x03, 0x41) Move Story Quest Selection -> Lobby.
///
/// (C -> S) Sent when the client wants to move from the story quest selection screen
/// to lobby.
///
/// Respond with: load lobby map.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x41)]
pub struct StoryToLobbyPacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

/// Settings for map zone.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct ZoneSettings {
    pub world_id: u32,
    pub unk1: u32,
    pub zone_id: u32,
    /// Map layout id.
    pub map_id: u32,
    pub zone_type: u32,
    pub seed: u32,
    pub args: u32,
    pub size_x: u32,
    pub size_y: u32,
    pub unk2: u32,
    pub area_index: u32,
    pub sub_area: u32,
    pub unk3: u32,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct WarpInfo {
    pub unk1: u32,
    pub zone_id: u32,
    pub door_id: u32,
    pub dest_zone: u32,
    pub backdoor_id: u32,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing3 {
    pub unk1: u32,
    pub unk2: u32,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing4 {
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
    pub unk23: u32,
    pub unk24: u32,
    pub unk25: u32,
    pub unk26: u32,
    pub unk27: u32,
    pub unk28: u32,
    pub unk29: u32,
    pub unk30: u32,
    pub unk31: u32,
    pub unk32: u32,
    pub unk33: u32,
    pub unk34: u32,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing5 {
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
    pub unk23: u32,
    pub unk24: u32,
    pub unk25: u32,
    pub unk26: u32,
    pub unk27: u32,
    pub unk28: u32,
    pub unk29: u32,
    pub unk30: u32,
    pub unk31: u32,
    pub unk32: u32,
    pub unk33: u32,
    pub unk34: u32,
    pub unk35: u32,
    pub unk36: u32,
    pub unk37: u32,
    pub unk38: u32,
    pub unk39: u32,
    pub unk40: u32,
    pub unk41: u32,
    pub unk42: u32,
    pub unk43: u32,
    pub unk44: u32,
    pub unk45: u32,
    pub unk46: u32,
    pub unk47: u32,
    pub unk48: u32,
    pub unk49: u32,
    pub unk50: u32,
    pub unk51: u32,
    pub unk52: u32,
    pub unk53: u32,
    pub unk54: u32,
    pub unk55: u32,
    pub unk56: u32,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing6 {
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
    pub unk23: u32,
    pub unk24: u32,
    pub unk25: u32,
    pub unk26: u32,
    pub unk27: u32,
    pub unk28: u32,
    pub unk29: u32,
    pub unk30: u32,
    pub unk31: u32,
    pub unk32: u32,
    pub unk33: u32,
    pub unk34: u32,
    pub unk35: u32,
    pub unk36: u32,
    pub unk37: u32,
    pub unk38: u32,
    pub unk39: u32,
    pub unk40: u32,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing7 {
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
    pub unk23: u32,
    pub unk24: u32,
    pub unk25: u32,
    pub unk26: u32,
    pub unk27: u32,
    pub unk28: u32,
    pub unk29: u32,
    pub unk30: u32,
    pub unk31: u32,
    pub unk32: u32,
    pub unk33: u32,
    pub unk34: u32,
    pub unk35: u32,
    pub unk36: u32,
    pub unk37: u32,
    pub unk38: u32,
    pub unk39: u32,
    pub unk40: u32,
    pub unk41: u32,
    pub unk42: u32,
    pub unk43: u32,
    pub unk44: u32,
    pub unk45: u32,
    pub unk46: u32,
    pub unk47: u32,
    pub unk48: u32,
    pub unk49: u32,
    pub unk50: u32,
    pub unk51: u32,
    pub unk52: u32,
    pub unk53: u32,
    pub unk54: u32,
    pub unk55: u32,
    pub unk56: u32,
    pub unk57: u32,
    pub unk58: u32,
    pub unk59: u32,
    pub unk60: u32,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing8 {
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
    pub unk23: u32,
    pub unk24: u32,
    pub unk25: u32,
    pub unk26: u32,
    pub unk27: u32,
    pub unk28: u32,
    pub unk29: u32,
    pub unk30: u32,
    pub unk31: u32,
    pub unk32: u32,
    pub unk33: u32,
    pub unk34: u32,
    pub unk35: u32,
    pub unk36: u32,
    pub unk37: u32,
    pub unk38: u32,
    pub unk39: u32,
    pub unk40: u32,
    pub unk41: u32,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing9 {
    pub unk1: u32,
    pub unk2: u32,
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing10 {
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
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct UnkThing1 {
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
}
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct UnkThing2 {
    pub unk1: u32,
    pub unk2: u32,
}
