//! Collection Folder related packets. \[0x42\]

use std::time::Duration;

use super::{HelperReadWrite, ItemData, ItemId, PacketReadWrite};
use crate::fixed_types::{FixedString, FixedVec};

// ----------------------------------------------------------------
// Collection Folder related packets
// ----------------------------------------------------------------

/// (0x42, 0x01) Currently Distibuted Collection Folder List.
///
/// (S -> C) Sent in response to the list request.
///
/// Response to: [`crate::protocol::Packet::GetCollectionList`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x42, 0x01)]
#[Flags(Flags::PACKED)]
#[Magic(0xD93F, 0x5B)]
pub struct CollectionNameListPacket {
    pub folders: Vec<CollectionFolderName>,
    pub unk: u32,
}

/// (0x42, 0x02) Get Collection Folder Rewards List Request.
///
/// (C -> S) Sent when the client selects a collection folder to receive a list of rewards.
///
/// Respond with: [`crate::protocol::Packet::FolderItemList`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x42, 0x02)]
pub struct GetFolderItemListPacket {
    /// Requested folder ID.
    pub folder_id: u32,
}

/// (0x42, 0x03) Collection Folder Reward List.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::GetFolderItemList`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x42, 0x03)]
#[Flags(Flags::PACKED)]
#[Magic(0xE4BA, 0xF1)]
pub struct FolderItemListPacket {
    pub items: Vec<CollectionFolderItem>,
    pub unk: u32,
}

/// (0x42, 0x05) Active Sheets List.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::GetActiveSheets`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x42, 0x05)]
#[Flags(Flags::PACKED)]
#[Magic(0xF035, 0x87)]
pub struct ActiveSheetsPacket {
    pub items: Vec<CollectionFolderItem>,
    pub progress: Vec<CollectionFolderProgress>,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

/// (0x42, 0x06) Claim Collection Sheet Request.
///
/// (C -> S) Sent when the client wants to claim a collection sheet.
///
/// Respond with: [`crate::protocol::Packet::ClaimSheetResult`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x42, 0x06)]
pub struct ClaimSheetRequestPacket {
    /// Requested folder ID.
    pub folder_id: u32,
    /// Requested reward ID.
    pub reward_id: u32,
}

/// (0x42, 0x07) Claim Collection Sheet Result.
///
/// (S -> C) Sent in response to request.
///
/// Response to: [`crate::protocol::Packet::ClaimSheetRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x42, 0x07)]
pub struct ClaimSheetResultPacket {
    pub unk: u32,
}

/// (0x42, 0x0F) Claim Collection Sheet Action (?).
///
/// (S -> C) Sent in response to request.
///
/// Response to: [`crate::protocol::Packet::ClaimSheetRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x42, 0x0F)]
#[Flags(Flags::PACKED)]
#[Magic(0x299E, 0x76)]
pub struct ClaimSheetActionPacket {
    pub action: String,
    pub folder_name: String,
    pub reward: String,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

/// Entry in the list of collection folders.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct CollectionFolderName {
    pub folder_id: u32,
    pub folder_name: FixedString<0x20>,
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u16,
    pub unk4: u16,
    pub unk5: u32,
}

/// Reward item from collection folder.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Default, Debug, Clone, PartialEq, HelperReadWrite)]
pub struct CollectionFolderItem {
    /// ID of the associated collection folder.
    pub folder_id: u32,
    /// ID of this reward.
    pub reward_id: u32,
    pub unk1: FixedVec<10, u16>,
    /// Received item.
    pub target_item: ItemData,
    /// Required items.
    pub required_items: FixedVec<8, CollectionReqItem>,
    pub unk2: FixedVec<14, u16>,
}

/// Required item for a reward item.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Default, Debug, Clone, PartialEq, HelperReadWrite)]
pub struct CollectionReqItem {
    pub item_id: ItemId,
    pub unk1: FixedVec<60, u16>,
    pub unk2: FixedVec<52, u16>,
}

/// Current collection sheet progress.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Default, Debug, Clone, PartialEq, HelperReadWrite)]
pub struct CollectionFolderProgress {
    /// ID of the associated collection folder.
    pub folder_id: u32,
    /// ID of the associated reward.
    pub reward_id: u32,
    pub unk: FixedVec<0x1B, u32>,
    pub start_time: Duration,
}
