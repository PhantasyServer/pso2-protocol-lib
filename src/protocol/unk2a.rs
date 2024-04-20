//! Unknown \[0x2A\] packets.
use super::{HelperReadWrite, PacketReadWrite};

// ----------------------------------------------------------------
// Unknown 0x2A packets
// ----------------------------------------------------------------

/// (0x2A, 0x08) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2A, 0x08)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xB976, 0xA5)]
pub struct Unk2A08Packet {
    pub unk1: Vec<Unk2A08_1>,
    pub unk2: Vec<u32>,
    pub unk3: Vec<Unk2A08_2>,
    pub unk4: Vec<u8>,
    pub unk5: Vec<Unk2A08_3>,
    pub unk6: Vec<Unk2A08_4>,
    pub unk7: Vec<Unk2A08_5>,
    pub unk8: Vec<u16>,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk2A08_1 {
    #[FixedLen(0x10)]
    pub unk1: Vec<u8>,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(super::PacketType::NGS)]
    #[FixedLen(0x04)]
    pub unk2: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Unk2A08_2 {
    #[FixedLen(0xE)]
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Unk2A08_3 {
    #[FixedLen(0x6)]
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk2A08_4 {
    #[FixedLen(0x8)]
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk2A08_5 {
    #[FixedLen(0x4)]
    pub unk: Vec<u8>,
}
