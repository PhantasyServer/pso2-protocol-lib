use super::{HelperReadWrite, ObjectHeader, PacketReadWrite};

// ----------------------------------------------------------------
// Chat packets
// ----------------------------------------------------------------

// 0x07, 0x00
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x07, 0x00)]
#[Flags(Flags {packed: true, object_related: true, ..Default::default()})]
#[Magic(0x9D3F, 0x44)]
pub struct ChatMessage {
    pub object: ObjectHeader,
    pub area: ChatArea,
    pub unk3: u8,
    pub unk4: u16,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(super::PacketType::NGS)]
    pub unk5: u16,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(super::PacketType::NGS)]
    pub unk6: u16,
    pub unk7: String,
    pub message: String,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
pub enum ChatArea {
    #[default]
    Map,
    Party,
    // the following is only speculation
    Alliance,
    Whisper,
    Group,

    #[Read_default]
    Undefined = 0xFF,
}
