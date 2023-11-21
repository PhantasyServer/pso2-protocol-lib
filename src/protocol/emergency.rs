use super::{HelperReadWrite, ObjectHeader, PacketReadWrite};
use crate::AsciiString;

// ----------------------------------------------------------------
// Emergency packets
// ----------------------------------------------------------------

// 0x15, 0x02
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x15, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SpawnEmergencyPacket {
    pub object: ObjectHeader,
    #[VariableStr(0x080B, 0x77)]
    pub trial_id: AsciiString,
    #[FixedLen(0x40)]
    pub unk1: Vec<u8>,
    #[VariableStr(0x080B, 0x77)]
    pub unk2: AsciiString,
    #[Magic(0x080B, 0x77)]
    pub unk3: Vec<Unk1502_1>,
    #[VariableStr(0x080B, 0x77)]
    pub unk4: AsciiString,
    #[Magic(0x080B, 0x77)]
    pub unk5: Vec<Unk1502_1>,
    #[FixedLen(3)]
    pub fail_conds: Vec<EmergencyCondition>,
    #[FixedLen(2)]
    pub pass_conds: Vec<EmergencyCondition>,
    pub unk8: u32,
    pub unk9: u32,
    #[VariableStr(0x080B, 0x77)]
    pub unk10: AsciiString,
    #[Magic(0x080B, 0x77)]
    pub unk11: Vec<Unk1502_1>,
    #[VariableStr(0x080B, 0x77)]
    pub unk12: AsciiString,
    #[Magic(0x080B, 0x77)]
    pub unk13: Vec<Unk1502_1>,
    pub unk14: u32,
    #[FixedLen(0x20)]
    pub unk15: Vec<u8>,
    pub unk16: u32,
    pub unk17: u32,
    #[VariableStr(0x080B, 0x77)]
    pub unk18: AsciiString,
    #[Magic(0x080B, 0x77)]
    pub unk19: Vec<Unk1502_1>,
    #[Magic(0x080B, 0x77)]
    pub unk20: Vec<Unk1502_3>,
    pub unk21: u32,
}

// 0x15, 0x03
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x15, 0x03)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct EmergencyEndPacket {
    pub object: ObjectHeader,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    #[FixedLen(0x1C)]
    pub unk5: Vec<u8>,
    pub unk6: u32,
    #[VariableStr(0x8DC9, 0xC2)]
    pub unk7: AsciiString,
    #[Magic(0x8DC9, 0xC2)]
    pub unk8: Vec<Unk1502_1>,
    #[VariableStr(0x8DC9, 0xC2)]
    pub unk9: AsciiString,
    #[Magic(0x8DC9, 0xC2)]
    pub unk10: Vec<Unk1502_1>,
}

// 0x15, 0x05
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x15, 0x05)]
pub struct EmergencyProgressPacket {
    pub emergency: ObjectHeader,
    pub unk2: u32,
    pub unk3: u32,
    pub done: u32,
    pub unk5: u32,
}

// 0x15, 0x08
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x15, 0x08)]
pub struct Unk1508Packet {
    pub emergency: ObjectHeader,
    pub unk2: u32,
    pub unk3: u32,
}

// 0x15, 0x11
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x15, 0x11)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct AvailableEmergenciesPacket {
    #[FixedLen(0x40)]
    pub definitions: Vec<EmergencyDefinition>,
    pub count: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk1502_1 {
    #[FixedLen(0x24)]
    pub unk1: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct EmergencyCondition {
    #[VariableStr(0x080B, 0x77)]
    pub cond_name: AsciiString,
    #[Magic(0x080B, 0x77)]
    pub cond_data: Vec<Unk1502_1>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk1502_3 {
    #[FixedLen(0x4C)]
    pub unk1: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct EmergencyDefinition {
    #[VariableStr(0xDE28, 0xDE)]
    pub trial_id: AsciiString,
    pub unk1: u16,
    pub unk2: u16,
}
