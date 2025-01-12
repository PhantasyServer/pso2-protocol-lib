//! Emergency related packets. \[0x15\]
use super::{HelperReadWrite, ObjectHeader, PacketReadWrite};
use crate::{fixed_types::{FixedBytes, FixedVec}, AsciiString};

// ----------------------------------------------------------------
// Emergency packets
// ----------------------------------------------------------------

/// (0x15, 0x02) Start Emergency (broadcast).
///
/// (S -> C) Sent when an emergency trial has started.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x15, 0x02)]
#[Magic(0x080B, 0x77)]
#[Flags(Flags::PACKED)]
pub struct SpawnEmergencyPacket {
    /// Emergency object.
    pub object: ObjectHeader,
    /// Trial string ID.
    pub trial_id: AsciiString,
    pub unk1: FixedBytes<0x40>,
    pub unk2: AsciiString,
    pub unk3: Vec<Unk1502_1>,
    pub unk4: AsciiString,
    pub unk5: Vec<Unk1502_1>,
    /// Trial fail conditions.
    pub fail_conds: FixedVec<3, EmergencyCondition>,
    /// Trial pass conditions.
    pub pass_conds: FixedVec<2, EmergencyCondition>,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: AsciiString,
    pub unk11: Vec<Unk1502_1>,
    pub unk12: AsciiString,
    pub unk13: Vec<Unk1502_1>,
    pub unk14: u32,
    pub unk15: FixedBytes<0x20>,
    pub unk16: u32,
    pub unk17: u32,
    pub unk18: AsciiString,
    pub unk19: Vec<Unk1502_1>,
    pub unk20: Vec<Unk1502_3>,
    pub unk21: u32,
}

/// (0x15, 0x03) End Emergency (broadcast).
///
/// (S -> C) Sent when an emergency trial has ended.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x15, 0x03)]
#[Flags(Flags::PACKED)]
#[Magic(0x8DC9, 0xC2)]
pub struct EmergencyEndPacket {
    /// Emergency object.
    pub object: ObjectHeader,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: FixedBytes<0x1C>,
    pub unk6: u32,
    pub unk7: AsciiString,
    pub unk8: Vec<Unk1502_1>,
    pub unk9: AsciiString,
    pub unk10: Vec<Unk1502_1>,
}

/// (0x15, 0x05) Emergency Progress (broadcast).
///
/// (S -> C) Sent when an emergency trial progress is updated.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x15, 0x05)]
pub struct EmergencyProgressPacket {
    /// Emergency object.
    pub emergency: ObjectHeader,
    pub unk2: u32,
    pub unk3: u32,
    pub done: u32,
    pub unk5: u32,
}

/// (0x15, 0x08) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x15, 0x08)]
pub struct Unk1508Packet {
    /// Emergency object.
    pub emergency: ObjectHeader,
    pub unk2: u32,
    pub unk3: u32,
}

/// (0x15, 0x11) Available Emergencies (?).
///
/// (S -> C) Sent during login to list available emergency trials (?).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x15, 0x11)]
#[Flags(Flags::PACKED)]
#[Magic(0xDE28, 0xDE)]
pub struct AvailableEmergenciesPacket {
    /// Emergency definitions.
    pub definitions: FixedVec<0x40, EmergencyDefinition>,
    /// Number of definitions in the above array.
    pub count: u32,
}

/// (0x15, 0x14) Unknown
///
/// (S -> C) 
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x15, 0x14)]
pub struct Unk1514Packet {
    pub zone_id: u32,
    pub unk2: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk1502_1 {
    pub unk1: FixedBytes<0x24>,
}

/// Emergency trial pass/fail condition.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct EmergencyCondition {
    /// Condition string ID.
    pub cond_name: AsciiString,
    pub cond_data: Vec<Unk1502_1>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk1502_3 {
    pub unk1: FixedBytes<0x4C>,
}

/// Emergency definition
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct EmergencyDefinition {
    /// Condition string ID.
    pub trial_id: AsciiString,
    pub unk1: u16,
    pub unk2: u16,
}
