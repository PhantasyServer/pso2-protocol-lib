use super::{HelperReadWrite, PacketReadWrite};

// ----------------------------------------------------------------
// Palette packets
// ----------------------------------------------------------------

// 0x21, 0x01
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x21, 0x01)]
pub struct LoadPalettePacket {
    pub cur_palette: u32,
    pub cur_subpalette: u32,
    pub cur_book: u32,
    pub palettes: [WeaponPalette; 6],
    #[SeekAfter(0x240)] // maybe other books
    pub subpalettes: [SubPalette; 6],
}

// 0x21, 0x03
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x21, 0x03)]
pub struct FullPaletteInfoPacket {
    // 2101
    pub cur_palette: u32,
    pub cur_subpalette: u32,
    pub cur_book: u32,
    pub palettes: [WeaponPalette; 6],
    pub subpalettes: [SubPalette; 6],
    #[Seek(0x240)] // maybe other books
    // 210F
    #[FixedLen(0x1A0)]
    #[SeekAfter(0x240)] // padding??
    pub default_pa: Vec<u32>,
}

// 0x21, 0x04
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x21, 0x04)]
pub struct SetPalettePacket {
    pub palette: u32,
    pub unk: u32,
}

// 0x21, 0x05
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x21, 0x05)]
pub struct UpdateSubPalettePacket {
    pub subpalettes: [SubPalette; 6],
    #[FixedLen(0x90)]
    pub unk: Vec<u32>,
    pub cur_subpalette: u32,
    pub cur_book: u32,
}

// 0x21, 0x06
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x21, 0x06)]
pub struct UpdatePalettePacket {
    pub cur_palette: u32,
    pub palettes: [WeaponPalette; 6],
}

// 0x21, 0x08
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x21, 0x08)]
pub struct SetSubPalettePacket {
    pub subpalette: u32,
}

// 0x21, 0x0A
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x21, 0x0A)]
pub struct SetDefaultPAsPacket {
    #[FixedLen(0x1A0)]
    pub default: Vec<u32>,
}

// 0x21, 0x0F
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x21, 0x0F)]
pub struct NewDefaultPAsPacket {
    #[FixedLen(0x1A0)]
    #[SeekAfter(0x240)] // padding??
    pub default: Vec<u32>,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct WeaponPalette {
    pub uuid: u64,
    pub unk1: u32,
    pub unk2: PalettePA,
    pub unk3: PalettePA,
    pub unk4: PalettePA,
    pub unk: [u32; 3],
    pub pet_id: u32,
    pub skills: [PalettePA; 6],
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct SubPalette {
    pub items: [PalettePA; 12],
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct PalettePA {
    pub id: u8,
    pub category: u8,
    pub unk: u8,
    pub level: u8,
}
