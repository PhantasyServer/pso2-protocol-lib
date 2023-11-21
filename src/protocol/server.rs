use super::{HelperReadWrite, ObjectHeader, PacketReadWrite};
use crate::AsciiString;

// ----------------------------------------------------------------
// Server packets
// ----------------------------------------------------------------

// 0x03, 0x00
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x00)]
pub struct MapTransferPacket {
    pub map: ObjectHeader,
    pub target: ObjectHeader,
    pub settings: ZoneSettings,
}

// 0x03, 0x06
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x06)]
pub struct Unk0306Packet {
    pub unk: [u8; 0xC],
}

// 0x03, 0x08
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x08)]
pub struct ServerHelloPacket {
    pub unk1: u16,
    #[SeekAfter(4)]
    pub blockid: u16,
    pub unk2: u32,
}

// 0x03, 0x10
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x10)]
pub struct MapLoadedPacket {
    pub map_object: ObjectHeader,
    pub unk: [u8; 0x20],
}

// 0x03, 0x24
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x24)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x7542, 0x5E)]
pub struct LoadLevelPacket {
    pub map_object: ObjectHeader,
    #[cfg_attr(feature = "serde", serde(skip))]
    pub receiver: ObjectHeader,
    pub settings: ZoneSettings,
    pub unk4: [u8; 0xC],
    pub unk5: [u8; 0xC],
    pub unk6: [u8; 0xC],
    pub unk7: AsciiString,
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
    #[FixedLen(0x3C)]
    pub unk21: Vec<u8>,
    pub unk22: u32,
    pub unk23: [u8; 0x10],
    pub unk24: [u8; 0x10],
    pub unk25: Vec<u32>,
    #[FixedLen(0x200)]
    pub unk26: Vec<u8>,
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

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct ZoneSettings {
    pub world_id: u32,
    pub unk1: u32,
    pub zone_id: u32,
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
