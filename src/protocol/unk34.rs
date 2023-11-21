use super::{HelperReadWrite, PacketReadWrite};

// ----------------------------------------------------------------
// Unknown 0x34 packets
// ----------------------------------------------------------------

// 0x34, 0x35
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x34, 0x35)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct Unk3435Packet {
    pub unk1: u32,
    #[Magic(0xA475, 0x100)]
    pub unk2: Vec<Unk3435_1>,
}

// 0x34, 0x5C
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x34, 0x5C)]
pub struct Unk345CPacket {
    pub unk: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk3435_1 {
    #[FixedLen(0xC)]
    pub unk: Vec<u8>,
}
