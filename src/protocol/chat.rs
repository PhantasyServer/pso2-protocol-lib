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
pub struct ChatMessage {
    pub object: ObjectHeader,
    pub area: ChatArea,
    pub unk3: u8,
    pub unk4: u16,
    #[VariableStr(0x9D3F, 0x44)]
    pub unk5: String,
    #[VariableStr(0x9D3F, 0x44)]
    pub message: String,
}

#[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x07, 0x00)]
#[Flags(Flags {packed: true, object_related: true, ..Default::default()})]
pub struct ChatMessageNGS {
    pub object: ObjectHeader,
    pub unk2: ChatArea,
    pub unk3: u8,
    pub unk4: u16,
    pub unk5: u16,
    pub unk6: u16,
    #[VariableStr(0x9D3F, 0x44)]
    pub unk7: String,
    #[VariableStr(0x9D3F, 0x44)]
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
