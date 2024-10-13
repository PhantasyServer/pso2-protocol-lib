//! Unknown \[0x2A\] packets.
use crate::fixed_types::{Bytes, FixedBytes};
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
#[Flags(Flags::PACKED)]
#[Magic(0xB976, 0xA5)]
pub struct Unk2A08Packet {
    pub unk1: Vec<Unk2A08_1>,
    pub unk2: Vec<u32>,
    pub unk3: Vec<Unk2A08_2>,
    pub unk4: Bytes,
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
    pub unk1: FixedBytes<0x10>,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(super::PacketType::NGS)]
    pub unk2: FixedBytes<0x4>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk2A08_2 {
    pub unk: FixedBytes<0xE, true>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk2A08_3 {
    pub unk: FixedBytes<0x6, true>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk2A08_4 {
    pub unk: FixedBytes<0x8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk2A08_5 {
    pub unk: FixedBytes<0x4>,
}
