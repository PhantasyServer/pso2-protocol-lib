use super::PacketReadWrite;

// ----------------------------------------------------------------
// Unknown 0x23 packets
// ----------------------------------------------------------------

// 0x23, 0x04
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x23, 0x04)]
pub struct Unk2304Packet {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
}
