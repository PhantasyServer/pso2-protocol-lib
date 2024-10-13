//! Palette related packets. \[0x21\]
use crate::fixed_types::FixedVec;

use super::{HelperReadWrite, PacketReadWrite};

// ----------------------------------------------------------------
// Palette packets
// ----------------------------------------------------------------

/// (0x21, 0x01) Load Palettes.
///
/// (S -> C) Sent when a player starts a game or when a palette is updated.
///
/// Response to:
/// [`crate::protocol::Packet::StartGame`],
/// [`crate::protocol::Packet::UpdatePalette`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x21, 0x01)]
pub struct LoadPalettePacket {
    /// Current palette index.
    pub cur_palette: u32,
    /// Current subpalette index.
    pub cur_subpalette: u32,
    /// Current book index.
    pub cur_book: u32,
    /// Palettes in the first book.
    pub palettes: [WeaponPalette; 6],
    /// Subpalettes in the first book.
    #[SeekAfter(0x240)] // maybe other books
    pub subpalettes: [SubPalette; 6],
}

/// (0x21, 0x03) Full Palette Info.
///
/// (S -> C) Sent in response to the request.
///
/// Response to:
/// [`crate::protocol::Packet::FullPaletteInfoRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x21, 0x03)]
pub struct FullPaletteInfoPacket {
    // from packet 0x21, 0x01
    /// Current palette index.
    pub cur_palette: u32,
    /// Current subpalette index.
    pub cur_subpalette: u32,
    /// Current book index.
    pub cur_book: u32,
    /// Palettes in the first book.
    pub palettes: [WeaponPalette; 6],
    /// Subpalettes in the first book.
    pub subpalettes: [SubPalette; 6],
    #[Seek(0x240)] // maybe other books
    // from packet 0x21, 0x0F
    /// Default photon arts (?).
    #[SeekAfter(0x240)] // padding??
    pub default_pa: FixedVec<0x1A0, u32>,
}

/// (0x21, 0x04) Set Active Palette.
///
/// (C -> S) Sent when a player changes their active palette.
///
/// Respond with:
/// [`crate::protocol::Packet::EquipedWeapon`],
/// [`crate::protocol::Packet::ChangeWeaponPalette`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x21, 0x04)]
pub struct SetPalettePacket {
    /// Selected palette index.
    pub palette: u32,
    pub unk: u32,
}

/// (0x21, 0x05) Update Subpalette.
///
/// (C -> S) Sent when a player updates their subpalette (i.e. changes installed items).
///
/// Respond with:
/// [`crate::protocol::Packet::LoadPalette`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x21, 0x05)]
pub struct UpdateSubPalettePacket {
    /// New subpalettes.
    pub subpalettes: [SubPalette; 6],
    pub unk: FixedVec<0x90, u32>,
    /// Current subpalette index.
    pub cur_subpalette: u32,
    /// Current book index.
    pub cur_book: u32,
}

/// (0x21, 0x06) Update Palette.
///
/// (C -> S) Sent when a player updates their palette (i.e. changes installed weapons).
///
/// Respond with:
/// [`crate::protocol::Packet::LoadPalette`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x21, 0x06)]
pub struct UpdatePalettePacket {
    /// Current palette index.
    pub cur_palette: u32,
    /// New palettes.
    pub palettes: [WeaponPalette; 6],
}

/// (0x21, 0x08) Set Active Subpalette.
///
/// (C -> S) Sent when a player changes their active subpalette.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x21, 0x08)]
pub struct SetSubPalettePacket {
    /// New subpalette index.
    pub subpalette: u32,
}

/// (0x21, 0x0A) Set Default Photon Arts (?)
///
/// (C -> S) Reasons for sending currently unknown.
///
/// Respond with: [`crate::protocol::Packet::NewDefaultPAs`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x21, 0x0A)]
pub struct SetDefaultPAsPacket {
    pub default: FixedVec<0x1A0, u32>,
}

/// (0x21, 0x0F) New Default Photon Arts (?)
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::SetDefaultPAs`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x21, 0x0F)]
pub struct NewDefaultPAsPacket {
    #[SeekAfter(0x240)] // padding??
    pub default: FixedVec<0x1A0, u32>,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

/// Weapon in the palette.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct WeaponPalette {
    /// Item UUID.
    pub uuid: u64,
    pub unk1: u32,
    pub unk2: PalettePA,
    pub unk3: PalettePA,
    pub unk4: PalettePA,
    pub unk: [u32; 3],
    /// Pet ID (for summoner).
    pub pet_id: u32,
    /// Weapon skills.
    pub skills: [PalettePA; 6],
}

/// Subpalette definition.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct SubPalette {
    /// Items in the subpalette.
    pub items: [PalettePA; 12],
}

/// Photon Art in the palette.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct PalettePA {
    /// PA ID.
    pub id: u8,
    /// PA category.
    pub category: u8,
    pub unk: u8,
    /// PA level.
    pub level: u8,
}
