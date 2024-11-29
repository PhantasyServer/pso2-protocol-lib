//! Item related packets. \[0x0F\]
use crate::fixed_types::{Bytes, FixedString, FixedVec};

use super::{
    models::{character::HSVColor, Position},
    HelperReadWrite, ObjectHeader, PacketError, PacketReadWrite, PacketType,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{io::SeekFrom, time::Duration};

// ----------------------------------------------------------------
// Items packets
// ----------------------------------------------------------------

/// (0x0F, 0x00) Item Attribute Data.
///
/// (S -> C) Sent after the client has logged in.
///
/// Followed by: [`crate::protocol::Packet::LoadItemAttributes`]
///
/// Following: [`crate::protocol::Packet::LoginResponse`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x00)]
#[Flags(Flags::PACKED)]
#[Magic(0x8A92, 0x30)]
pub struct ItemAttributesPacket {
    /// Attribute ID (?) (seen only 0 or 1).
    pub id: u16,
    /// Segment ID.
    pub segment: u16,
    /// Total data size.
    pub total_size: u32,
    // data contains an ice archive that includes a "item_parameter.bin".
    /// ICE archive data segment.
    pub data: Bytes,
}

/// (0x0F, 0x01) Item Pickup Request.
///
/// (C -> S) Sent when the client wants to pickup an item.
///
/// Respond with:
/// [`crate::protocol::Packet::InventoryMeseta`] (if the item is meseta),
/// [`crate::protocol::Packet::DespawnObject`] (if the item was actually picked up),
/// [`crate::protocol::Packet::ItemPickupResponse`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x01)]
pub struct ItemPickupRequestPacket {
    /// Item drop ID.
    pub drop_id: u32,
    pub unk: u32,
}

/// (0x0F, 0x02) Item Pickup Response.
///
/// (S -> C) Sent in response to the request.
///
/// Response to:
/// [`crate::protocol::Packet::ItemPickupRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x02)]
pub struct ItemPickupResponsePacket {
    /// Packet receiver object (? or player, who picked up the item, unsure)
    pub target: ObjectHeader,
    /// Item drop ID.
    pub drop_id: u32,
    /// Was the item actually picked up.
    pub was_pickedup: u32,
    pub unk: u32,
}

/// (0x0F, 0x04) New Item Drop.
///
/// (S -> C) Sent when a new item drop has spawned.
///
/// Following: [`crate::protocol::Packet::ObjectSpawn`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x04)]
pub struct NewItemDropPacket {
    /// Item drop object.
    pub item_obj: ObjectHeader,
    /// Item ID.
    pub item_id: ItemId,
    pub unk1: u32,
    pub unk2: u16,
    /// Drop position.
    pub pos: Position,
    pub unk3: u16,
    pub unk4: u32,
    pub unk5: u16,
    pub unk6: u32,
    /// Item drop ID.
    pub drop_id: u32,
    pub unk7: u32,
}

/// (0x0F, 0x05) Add Item To Inventory.
///
/// (S -> C) Sent when a new item is added to the inventory (e.g. picking up an item, completing an
/// emergency).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x05)]
pub struct AddedItemPacket {
    /// Added item data.
    pub item: Item,
    pub unk: u32,
}

/// (0x0F, 0x06) Update Inventory.
///
/// (S -> C) Sent when items in the inventory are updated in some way (e.g. discarding an item,
/// modifying the consumables amount, using an item).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x06)]
#[Flags(Flags::PACKED)]
#[Magic(0xAD04, 0xF3)]
pub struct UpdateInventoryPacket {
    /// Items being updated.
    pub updated: Vec<UpdatedInventoryItem>,
    pub unk: Vec<UpdatedInventoryItem>,
    pub unk2: u32,
}

/// (0x0F, 0x08) Equip Item Request.
///
/// (C -> S) Sent when a player equips an item.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x08)]
pub struct EquipItemRequestPacket {
    /// Equiped item UUID.
    pub uuid: u64,
    /// Equipment slot ID.
    pub equipment_pos: u32,
    pub unk: u32,
}

/// (0x0F, 0x09) Equip Item. (broadcast?)
///
/// (S -> C) Sent in response to the request.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x09)]
#[Flags(Flags::PACKED)]
#[Magic(0x3E3D, 0xD4)]
pub struct EquipItemPacket {
    /// Player who equiped an item (?).
    pub player_equiped: ObjectHeader,
    /// Equiped item.
    pub equiped_item: Item,
    /// Equipment slot ID.
    pub equipment_pos: u32,
    pub unk1: Bytes,
    pub unk2: u64,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(PacketType::NGS)]
    pub unk3: FixedVec<0x58, u8>,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(PacketType::NGS)]
    pub unk4: u32,
}

/// (0x0F, 0x0A) Unequip Item Request.
///
/// (C -> S) Sent when a player unequips an item.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x0A)]
pub struct UnequipItemRequestPacket {
    /// Unequiped item UUID.
    pub uuid: u64,
    /// Equipment slot ID.
    pub equipment_pos: u32,
    pub unk: u32,
}

/// (0x0F, 0x0B) Unequip Item. (broadcast?)
///
/// (S -> C) Sent in response to the request.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x0B)]
pub struct UnequipItemPacket {
    /// Player who unequiped an item (?).
    pub player_unequiped: ObjectHeader,
    /// Unequiped item.
    pub unequiped_item: Item,
    /// Equipment slot ID.
    pub equipment_pos: u32,
    pub unk1: u64,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(PacketType::NGS)]
    pub unk2: FixedVec<0x58, u8>,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(PacketType::NGS)]
    pub unk3: u32,
}

/// (0x0F, 0x0C) Load Player's Equipment (broadcast).
///
/// (S -> C) Sent when player's equiped items change, when the player object is created or when map
/// changes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x0C)]
#[Flags(Flags::PACKED)]
#[Magic(0xCF76, 0xB5)]
pub struct LoadEquipedPacket {
    /// Player whose equipment is loaded.
    pub player: ObjectHeader,
    /// Player's equiped items.
    pub items: Vec<EquipedItem>,
    pub unk1: u32,
    pub unk2: FixedVec<0x28, u8>,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(PacketType::NGS)]
    pub unk3: FixedVec<0x58, u8>,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(PacketType::NGS)]
    pub unk4: u32,
}

/// (0x0F, 0x0D) Load Player's Inventory.
///
/// (S -> C) Sent when the player selects the character.
///
/// Response to: [`crate::protocol::Packet::StartGame`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x0D)]
#[Flags(Flags::PACKED)]
#[Magic(0x5533, 0x1)]
pub struct LoadPlayerInventoryPacket {
    /// Player object.
    pub object: ObjectHeader,
    /// Character's name.
    pub name: String,
    /// Meseta currently held.
    pub meseta: u64,
    /// Max inventory capacity.
    pub max_capacity: u32,
    /// Items in the inventory.
    pub items: Vec<Item>,
}

/// (0x0F, 0x0F) Move Items From Inventory To Storage Request.
///
/// (C -> S) Sent when the client wants to move items from inventory to storage.
///
/// Respond with: [`crate::protocol::Packet::MoveToStorage`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x0F)]
#[Flags(Flags::PACKED)]
#[Magic(0x60AF, 0x97)]
pub struct MoveToStorageRequestPacket {
    /// Information about items being moved.
    pub uuids: Vec<MoveStorageItemRequest>,
}

/// (0x0F, 0x10) Move Items From Inventory To Storage.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::MoveToStorageRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x10)]
#[Flags(Flags::PACKED)]
#[Magic(0xE66C, 0xE2)]
pub struct MoveToStoragePacket {
    /// New item status in the players inventory.
    pub updated_inventory: Vec<UpdatedInventoryItem>,
    /// New items in storage.
    pub new_items: Vec<NewStorageItem>,
    /// Updated items in storage.
    pub updated: Vec<UpdatedItem>,
}

/// (0x0F, 0x11) Move Items From Storage To Inventory Request.
///
/// (C -> S) Sent when the client wants to move items from storage to inventory.
///
/// Respond with: [`crate::protocol::Packet::MoveToInventory`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x11)]
#[Flags(Flags::PACKED)]
#[Magic(0x6C2A, 0x2D)]
pub struct MoveToInventoryRequestPacket {
    /// Information about items being moved.
    pub uuids: Vec<MoveStorageItemRequest>,
}

/// (0x0F, 0x12) Move Items From Storage To Inventory.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::MoveToInventoryRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x12)]
#[Flags(Flags::PACKED)]
#[Magic(0xF1E8, 0x78)]
pub struct MoveToInventoryPacket {
    /// New item status in the players storage.
    pub updated: Vec<UpdatedStorageItem>,
    /// New (or updated) items in the inventory.
    pub new_items: Vec<NewInventoryItem>,
}

/// (0x0F, 0x13) Load Player's Storages.
///
/// (S -> C) Sent when the player selects the character.
///
/// Response to: [`crate::protocol::Packet::StartGame`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x13)]
#[Flags(Flags::PACKED)]
#[Magic(0x77A5, 0xC3)]
pub struct LoadStoragesPacket {
    /// Currently stored meseta.
    pub stored_meseta: u64,
    /// Information about the storages.
    pub unk1: Vec<StorageInfo>,
    /// Items in the storages.
    pub items: Vec<Item>,
    pub unk2: u32,
}

/// (0x0F, 0x14) New Inventory Meseta Amount.
///
/// (S -> C) Sent when the players held meseta amount is changed (e.g. when meseta is picked up).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x14)]
pub struct InventoryMesetaPacket {
    /// New meseta amount.
    pub meseta: u64,
}

/// (0x0F, 0x15) Move Meseta Request.
///
/// (C -> S) Sent when the player wants to move meseta between the inventory and the storage.
///
/// Respond with:
/// [`crate::protocol::Packet::StorageMeseta`]
/// [`crate::protocol::Packet::InventoryMeseta`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x15)]
pub struct MoveMesetaPacket {
    /// Amount to move.
    pub meseta: u64,
    /// Direction to move.
    pub direction: MesetaDirection,
}

/// (0x0F, 0x16) New Storage Meseta Amount.
///
/// (S -> C) Sent when the players storage meseta amount is changed.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x16)]
pub struct StorageMesetaPacket {
    /// New meseta amount.
    pub meseta: u64,
}

/// (0x0F, 0x17) Discard Item Request.
///
/// (C -> S) Sent when the client wants to discard an item from the inventory.
///
/// Respond with: [`crate::protocol::Packet::UpdateInventory`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x17)]
#[Flags(Flags::PACKED)]
#[Magic(0x8E9C, 0xF0)]
pub struct DiscardItemRequestPacket {
    /// UUIDs and amount of items being discarded.
    pub items: Vec<UUIDAmount>,
}

/// (0x0F, 0x18) Move Items Between Storages Request.
///
/// (C -> S) Sent when the client wants to move items between storages.
///
/// Respond with: [`crate::protocol::Packet::MoveStorages`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x18)]
#[Flags(Flags::PACKED)]
#[Magic(0x145A, 0x3B)]
pub struct MoveStoragesRequestPacket {
    /// Old storage ID.
    pub old_id: u16,
    /// New storage ID.
    pub new_id: u16,
    /// Items being moved.
    pub items: Vec<UUIDAmount>,
}

/// (0x0F, 0x19) Move Items Between Storages.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::MoveStoragesRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x19)]
#[Flags(Flags::PACKED)]
#[Magic(0x9A17, 0x86)]
pub struct MoveStoragesPacket {
    /// New items in the receiving storage.
    pub new_items: Vec<NewStorageItem>,
    /// Updated items in the sending storage.
    pub updated_new: Vec<UpdatedStorageItem>,
    /// Updated items in the receiving storage.
    pub updated_old: Vec<UpdatedStorageItem>,
}

/// (0x0F, 0x1C) Get Item Description.
///
/// (C -> S) Sent when the client wants the items description.
///
/// Respond with: [`crate::protocol::Packet::LoadItemDescription`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x1C)]
pub struct GetItemDescriptionPacket {
    /// Item ID which description is requested.
    pub item: ItemId,
}

/// (0x0F, 0x1D) Load Item Description.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::GetItemDescription`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x1D)]
#[Flags(Flags::PACKED)]
#[Magic(0xB10E, 0xB2)]
pub struct LoadItemDescriptionPacket {
    pub unk1: u32,
    /// Item ID which description is requested.
    pub item: ItemId,
    /// Items description.
    pub desc: String,
}

/// (0x0F, 0x21) Change Equiped Weapon (broadcast).
///
/// (S -> C) Sent when a player changes their held weapon (e.g. switches the palette).
///
/// Response to:
/// [`crate::protocol::Packet::StartGame`]
/// [`crate::protocol::Packet::SetPalette`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x21)]
pub struct EquipedWeaponPacket {
    /// Player changing the weapon.
    pub player: ObjectHeader,
    /// New weapon.
    pub item: Item,
}

/// (0x0F, 0x22) Update Storage.
///
/// (S -> C) Sent when items in the storages are updated in some way (e.g. auto item moving to
/// storage on full inventory).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x22)]
#[Flags(Flags::PACKED)]
#[Magic(0x4DC2, 0x2A)]
pub struct UpdateStoragePacket {
    pub unk: Vec<UpdatedStorageItem>,
    /// Already existing items being updated.
    pub updated: Vec<UpdatedStorageItem>,
    /// New items added.
    pub new_items: Vec<NewStorageItem>,
    pub unk2: u32,
    pub unk3: u64,
}

/// (0x0F, 0x25) Discard Storage Item Request.
///
/// (C -> S) Sent when the client wants to discard an item from the storage.
///
/// Respond with: [`crate::protocol::Packet::UpdateStorage`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x25)]
#[Flags(Flags::PACKED)]
#[Magic(0xDEFB, 0x0B)]
pub struct DiscardStorageItemRequestPacket {
    /// Items being discarded.
    pub items: Vec<MoveStorageItemRequest>,
}

/// (0x0F, 0x2B) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x2B)]
#[Flags(Flags::PACKED)]
#[Magic(0x016D, 0xCE)]
pub struct Unk0F2BPacket {
    pub items: Vec<Item>,
}

// for impl see [`LoadItemInternal`]
/// (0x0F, 0x30) Load Item Name.
///
/// (S -> C) Sent when the client sees an item for the first time since logging in.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LoadItemPacket {
    /// Item ID - Name pairs.
    pub items: Vec<NamedId>,
}

/// (0x0F, 0x33) Learn Photon Art.
///
/// (S -> C) Sent when a player has learned a new photon art.
///
/// Following: [`crate::protocol::Packet::LoadPAs`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x33)]
pub struct LearnedPAPacket {
    /// Player learning a PA.
    pub player: ObjectHeader,
    /// New PAs level.
    pub new_level: u32,
    /// PA ID.
    pub pa_id: u32,
    /// Used disc ID.
    pub item_id: ItemId,
}

/// (0x0F, 0x65) Weapon Potential List.
///
/// (S -> C) Sent when a player wants to switch a potential of an item.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x65)]
#[Flags(Flags::PACKED)]
#[Magic(0x4E66, 0xD3)]
pub struct PotentialListPacket {
    pub unk1: u16,
    pub unk2: u16,
    pub potential_ids: Vec<u32>,
    pub unk4: Bytes,
    pub target_items: Vec<ShortItemId>,
    pub unk6: Vec<u32>,
    pub unk7: u32,
}

/// (0x0F, 0x70) Account Campaign List.
///
/// (S -> C) Sent when a player has requested a list of available account campaigns.
///
/// Respond with: [`crate::protocol::Packet::CampaignItemsRequest`]
///
/// Response to: [`crate::protocol::Packet::AccountCampaignsRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x70)]
#[Flags(Flags::PACKED)]
#[Magic(0x0D8C, 0x0D)]
pub struct AccountCampaignsPacket {
    pub unk1: u32,
    /// Available campaigns.
    pub campaigns: Vec<Campaign>,
}

/// (0x0F, 0x71) Campaign Item List Request.
///
/// (C -> S) Sent when a player has requested a list of items in campaigns.
///
/// Respond with: [`crate::protocol::Packet::CampaignItemList`]
///
/// Response to: [`crate::protocol::Packet::AccountCampaigns`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x71)]
#[Flags(Flags::PACKED)]
#[Magic(0x934A, 0x58)]
pub struct CampaignItemsRequestPacket {
    /// Campaign IDs.
    pub ids: Vec<u32>,
}

/// (0x0F, 0x72) Campaign Item List.
///
/// (S -> C) Sent when a player has requested a list of items in campaigns.
///
/// Response to: [`crate::protocol::Packet::CampaignItemsRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x72)]
#[Flags(Flags::PACKED)]
#[Magic(0x1908, 0xA3)]
pub struct CampaignItemListPacket {
    pub unk1: u32,
    /// Campaign items.
    pub items: Vec<CampaignItemDefinition>,
}

/// (0x0F, 0x73) Receive Campaign Rewards Request.
///
/// (C -> S) Sent when a player wants to receive a campaign reward.
///
/// Respond with: (0x0F, 0x74)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x73)]
pub struct ReceiveCampaignRequestPacket {
    /// Campaign ID.
    pub id: u32,
}

/// (0x0F, 0x9C) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x9C)]
#[Flags(Flags::PACKED)]
#[Magic(0xA25, 0xF6)]
pub struct Unk0F9CPacket {
    pub ids: Vec<Unk0f9c>,
}

/// (0x0F, 0xBC) Change Player's Current Palette (broadcast).
///
/// (S -> C) Sent when a player has switched their palette.
///
/// Response to: [`crate::protocol::Packet::SetPalette`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xBC)]
pub struct ChangeWeaponPalettePacket {
    /// Player switching the palette.
    pub player: ObjectHeader,
    pub unk: FixedVec<0x12, u16>,
    /// New palette ID.
    #[SeekAfter(0x4)]
    pub cur_palette: u32,
}

/// (0x0F, 0xDF) Load Player's Material Storage.
///
/// (S -> C) Sent when the player selects the character.
///
/// Response to: [`crate::protocol::Packet::StartGame`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xDF)]
#[Flags(Flags::PACKED)]
#[Magic(0xAC9, 0x9F)]
pub struct LoadMaterialStoragePacket {
    pub player_id: u32,
    /// Items in the material storage.
    pub items: Vec<MaterialStorageItem>,
    /// Info about the material storage.
    pub info: StorageInfo,
}

/// (0x0F, 0xE0) Move Item From Inventory To Material Storage Request.
///
/// (C -> S) Sent when the player wants to move items from the inventory to the material storage.
///
/// Respond with: [`crate::protocol::Packet::MoveToMatStorage`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xE0)]
#[Flags(Flags::PACKED)]
#[Magic(0x9087, 0xEA)]
pub struct MoveToMatStorageRequestPacket {
    /// Information about items being moved.
    pub items: Vec<MaterialStorageItem>,
}

/// (0x0F, 0xE1) Move Item From Inventory To Material Storage.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::MoveToMatStorageRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xE1)]
#[Flags(Flags::PACKED)]
#[Magic(0x1644, 0x35)]
pub struct MoveToMatStoragePacket {
    /// Items updated in the inventory.
    pub updated_inventory: Vec<UpdatedInventoryItem>,
    /// Items updated in the material storage.
    pub items: Vec<MaterialStorageItem>,
}

/// (0x0F, 0xE2) Move Item From Material Storage To Inventory Request.
///
/// (C -> S) Sent when the player wants to move items from the material storage to the inventory.
///
/// Respond with: [`crate::protocol::Packet::MoveFromMatStorage`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xE2)]
#[Flags(Flags::PACKED)]
#[Magic(0x9C02, 0x80)]
pub struct MoveFromMatStorageRequestPacket {
    /// Information about items being moved.
    pub items: Vec<MaterialStorageItem>,
}

/// (0x0F, 0xE3) Move Item From Material Storage To Inventory.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::MoveFromMatStorageRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xE3)]
#[Flags(Flags::PACKED)]
#[Magic(0x21C0, 0xCB)]
pub struct MoveFromMatStoragePacket {
    /// Items updated in the material storage.
    pub mat_items: Vec<MaterialStorageItem>,
    /// Items updated in the inventory.
    pub new_items: Vec<NewInventoryItem>,
}

/// (0x0F, 0xE8) Move Item From Material Storage To Storage Request.
///
/// (C -> S) Sent when the player wants to move items from the material storage to the storage.
///
/// Respond with: [`crate::protocol::Packet::MoveMSToStorage`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xE8)]
#[Flags(Flags::PACKED)]
#[Magic(0xBE74, 0x43)]
pub struct MoveMSToStorageRequestPacket {
    /// New storage ID.
    pub storage_id: u32,
    /// Information about items being moved.
    pub items: Vec<MaterialStorageItem>,
}

/// (0x0F, 0xE9) Move Item From Material Storage To Storage.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::MoveMSToStorageRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xE9)]
#[Flags(Flags::PACKED)]
#[Magic(0x4432, 0x8E)]
pub struct MoveMSToStoragePacket {
    /// Items updated in the material storage.
    pub mat_items: Vec<MaterialStorageItem>,
    /// New items in the storage.
    pub new_items: Vec<NewStorageItem>,
    /// Items updated in the storage.
    pub updated: Vec<UpdatedStorageItem>,
}

/// (0x0F, 0xEF) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xEF)]
#[Flags(Flags::PACKED)]
#[Magic(0x66A4, 0x51)]
pub struct Unk0FEFPacket {
    pub ids: Vec<ItemId>,
}

/// (0x0F, 0xFC) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xFC)]
#[Flags(Flags::PACKED)]
#[Magic(0x3145, 0x21)]
pub struct Unk0FFCPacket {
    pub ids: Vec<Unk0ffc>,
    pub unk: u32,
}

// ----------------------------------------------------------------
// Internal structs
// ----------------------------------------------------------------

// 0x0F, 0x30
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x30)]
#[Flags(Flags::PACKED)]
#[Magic(0x9E22, 0x46)]
struct LoadItemInternal {
    ids: Vec<ItemId>,
    names: String,
    name_length: Vec<u8>,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

/// Item ID with a name.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NamedId {
    /// Items name.
    pub name: String,
    /// Items ID.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub id: ItemId,
}

/// In game item.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Item {
    /// Items UUID.
    pub uuid: u64,
    /// Items ID.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub id: ItemId,
    /// Items data.
    pub data: ItemType,

    /// Extra NGS data.
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    pub unk: [u16; 12],
}

/// Items UUID and amount.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct UUIDAmount {
    /// Items UUID.
    pub uuid: u64,
    /// Items amount.
    pub amount: u16,
    pub unk: u16,
}

/// Player equiped item.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct EquipedItem {
    pub item: Item,
    pub unk: u32,
}

/// Moved item to/from storage.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct MoveStorageItemRequest {
    /// Items UUID.
    pub uuid: u64,
    /// Items amount.
    pub amount: u8,
    pub unk: u8,
    /// Current items storage ID.
    pub storage_id: u16,
}

/// New item in the storage.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct NewStorageItem {
    pub item: Item,
    /// New items storage ID.
    pub storage_id: u32,
}

/// New item in the inventory.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct NewInventoryItem {
    pub item: Item,
    /// Items amount added.
    pub amount: u16,
    /// Is the item actually new.
    pub is_new: u16,
}

/// Updated storage item.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct UpdatedItem {
    /// Items UUID.
    pub uuid: u64,
    /// New items amount.
    pub new_amount: u32,
    /// Items storage ID.
    pub storage_id: u32,
}

/// Updated inventory item.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct UpdatedInventoryItem {
    /// Items UUID.
    pub uuid: u64,
    /// New items amount.
    pub new_amount: u16,
    /// Amount moved/deleted.
    pub moved: u16,
}

/// Updated storage item.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct UpdatedStorageItem {
    /// Items UUID.
    pub uuid: u64,
    /// New items amount.
    pub new_amount: u16,
    /// Amount moved/deleted.
    pub moved: u16,
    /// Items storage ID.
    pub storage_id: u32,
}

/// Item types.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ItemType {
    #[default]
    NoItem,
    Weapon(WeaponItem),
    Clothing(ClothingItem),
    Consumable(ConsumableItem),
    Camo(CamoItem),
    Unit(UnitItem),
    Unknown(Bytes),
    // NGS Options
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    NoItemNGS,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    WeaponNGS(WeaponItemNGS),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    ClothingNGS(ClothingNGSItem),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    ConsumableNGS(ConsumableNGSItem),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    CamoNGS(CamoNGSItem),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    UnitNGS(UnitItemNGS),
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    UnknownNGS(Bytes),
}

/// Weapon type item data.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct WeaponItem {
    /// Item flags.
    pub flags: u8,
    /// Item element.
    pub element: u8,
    /// Item force.
    pub force: u8,
    pub grind: u8,
    pub grind_percent: u8,
    pub unk1: u8,
    pub unk2: u16,
    /// Item affix IDs.
    pub affixes: [u16; 8],
    /// Item potential.
    pub potential: u32,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: u16,
    pub unk6: u32,
    pub unk7: u32,
}

/// NGS Weapon type item data.
#[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct WeaponItemNGS {
    /// Item flags.
    pub flags: u8,
    /// Item element.
    pub element: u8,
    /// Item force.
    pub force: u8,
    pub grind: u8,
    pub grind_percent: u8,
    pub unk1: u8,
    pub unk2: u16,
    /// Item affix IDs.
    pub affixes: [u32; 8],
    /// Item potential.
    pub potential: u32,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: u16,
    pub unk6: u32,
    pub unk7: u32,
}

/// Unit type item data.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct UnitItem {
    /// Item flags.
    pub flags: u8,
    /// Item enhancement level.
    pub enh_level: u8,
    /// Item enhancement percentage.
    pub enh_percent: u8,
    pub unk1: u8,
    /// Item affix IDs (ranging from 0 to 4095).
    #[ManualRW(read_packed_affixes, write_packed_affixes)]
    pub affixes: [u16; 8],
    /// Item potential.
    #[SeekAfter(0x7)]
    pub potential: u32,
    pub unk2: [u8; 4],
    #[Seek(1)]
    pub unk3: u32,
    pub unk4: u16,
    pub unk5: u16,
}

/// NGS Unit type item data.
#[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct UnitItemNGS {
    /// Item flags.
    pub flags: u8,
    /// Item enhancement level.
    pub enh_level: u8,
    /// Item enhancement percentage.
    pub enh_percent: u8,
    pub unk1: u8,
    /// Item affix IDs.
    pub affixes: [u32; 8],
    /// Item potential.
    #[SeekAfter(0x6)]
    pub potential: u32,
    pub unk: [u8; 0xA],
}

/// Clothing type item data.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct ClothingItem {
    /// Item flags.
    pub flags: u16,
    /// Clothing color (if applicable).
    #[SeekAfter(0x14)]
    pub color: HSVColor,
    #[SeekAfter(0xA)]
    pub unk1: u16,
}

/// NGS Clothing type item data.
#[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct ClothingNGSItem {
    /// Item flags.
    pub flags: u16,
    /// Clothing color (if applicable).
    pub color: HSVColor,
    /// Clothing red color (if applicable).
    pub r_color: u8,
    /// Clothing green color (if applicable).
    pub g_color: u8,
    /// Clothing blue color (if applicable).
    pub b_color: u8,
    pub unk1: [u8; 4],
    #[Seek(0x5)]
    pub unk2: [u8; 3],
    #[SeekAfter(0x20)]
    pub unk3: u8,
}

/// Consumable type item data.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct ConsumableItem {
    /// Item flags.
    pub flags: u16,
    /// Item amount.
    #[SeekAfter(0x24)]
    pub amount: u16,
}

/// NGS Consumable type item data.
#[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct ConsumableNGSItem {
    /// Item flags.
    pub flags: u16,
    /// Item amount.
    #[SeekAfter(0x34)]
    pub amount: u16,
}

/// Camouflage type item data.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct CamoItem {
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u8,
    #[SeekAfter(0x24)]
    pub unk4: u8,
}

/// NGS Camouflage type item data.
#[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct CamoNGSItem {
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u8,
    #[SeekAfter(0x34)]
    pub unk4: u8,
}

/// Item ID.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Copy, Clone, PartialEq, HelperReadWrite)]
pub struct ItemId {
    /// Item type.
    pub item_type: u16,
    /// Item category.
    pub id: u16,
    /// Item ID after appraisal.
    pub unk3: u16,
    /// Item ID.
    pub subid: u16,
}

/// Short item ID.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Copy, Clone, PartialEq, HelperReadWrite)]
pub struct ShortItemId {
    /// Item type.
    pub item_type: u8,
    /// Item category.
    pub id: u8,
    /// Item ID.
    pub subid: u16,
}

/// Campaign definition.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Campaign {
    /// Campaign ID.
    pub id: u32,
    /// Start timestamp.
    pub start_date: Duration,
    /// End timestamp.
    pub end_date: Duration,
    /// Campaign title.
    pub title: FixedString<0x3E>,
    /// Campaign conditions (description).
    pub conditions: FixedString<0x102>,
}

/// Campaign item list definition.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct CampaignItemDefinition {
    /// Campaign ID.
    pub campaign_id: u32,
    /// Number of items in the following array.
    pub item_amount: u32,
    /// Campaign items.
    pub items: [CampaignItem; 8],
}

/// Campaign item definition.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Copy, Clone, PartialEq, HelperReadWrite)]
pub struct CampaignItem {
    /// Item ID.
    pub id: ItemId,
    /// Item amount.
    pub amount: u32,
    pub unk: u32,
}

/// Storage information.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct StorageInfo {
    /// Total space in the storage.
    pub total_space: u32,
    /// Used space in the storage.
    pub used_space: u32,
    /// Storage ID.
    pub storage_id: u8,
    /// Storage type (?).
    pub storage_type: u8,
    /// Is the storage locked (?).
    pub is_locked: u8,
    /// Is the storage enabled (?).
    pub is_enabled: u8,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk0f9c {
    pub unk1: u16,
    pub unk2: u16,
    pub unk3: u16,
    pub unk4: u16,
    pub unk5: u32,
}

/// Material storage item definition.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct MaterialStorageItem {
    /// Item category.
    pub id: u16,
    /// Item ID.
    pub subid: u16,
    /// Item amount.
    pub amount: u16,
    pub unk4: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk0ffc {
    pub unk1: u16,
    pub unk2: u16,
    pub unk3: u16,
    pub unk4: u16,
}

/// Meseta transfer direction (for [`MoveMesetaPacket`]).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum MesetaDirection {
    /// Meseta is moved Inventory -> Storage.
    #[default]
    #[Read_default]
    ToStorage = 1,
    /// Meseta is moved Storage -> Inventory.
    ToInventory,
}

// ----------------------------------------------------------------
// Read/Write implementations
// ----------------------------------------------------------------

impl PacketReadWrite for LoadItemPacket {
    fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        flags: &super::Flags,
        packet_type: PacketType,
    ) -> Result<Self, PacketError> {
        let packet = LoadItemInternal::read(reader, flags, packet_type).map_err(|e| {
            PacketError::CompositeFieldError {
                packet_name: "LoadItemPacket",
                field_name: "internal",
                error: Box::new(e),
            }
        })?;
        let mut names = packet.names.chars();
        let mut items = vec![];
        for (id, name_length) in packet.ids.into_iter().zip(packet.name_length.into_iter()) {
            let name = names.by_ref().take(name_length as usize).collect();
            items.push(NamedId { name, id });
        }
        Ok(Self { items })
    }

    fn write(&self, packet_type: PacketType) -> Result<Vec<u8>, PacketError> {
        let mut names = String::new();
        let mut name_length = vec![];
        let mut ids = vec![];
        for item in self.items.iter() {
            name_length.push(item.name.chars().count() as u8);
            names.push_str(&item.name);
            ids.push(item.id);
        }
        LoadItemInternal {
            ids,
            names,
            name_length,
        }
        .write(packet_type)
        .map_err(|e| PacketError::CompositeFieldError {
            packet_name: "LoadItemPacket",
            field_name: "internal",
            error: Box::new(e),
        })
    }
}

impl HelperReadWrite for Item {
    fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        packet_type: PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<Self, PacketError> {
        let uuid = reader
            .read_u64::<LittleEndian>()
            .map_err(|e| PacketError::FieldError {
                packet_name: "Item",
                field_name: "uuid",
                error: e,
            })?;
        let id = ItemId::read(reader, packet_type, xor, sub).map_err(|e| {
            PacketError::CompositeFieldError {
                packet_name: "Item",
                field_name: "id",
                error: Box::new(e),
            }
        })?;
        let data = ItemType::read(reader, &id, packet_type).map_err(|e| {
            PacketError::CompositeFieldError {
                packet_name: "Item",
                field_name: "data",
                error: Box::new(e),
            }
        })?;
        #[cfg(feature = "ngs_packets")]
        let unk = match packet_type {
            PacketType::NGS => {
                let mut data = [0u16; 12];
                for byte in data.iter_mut() {
                    *byte =
                        reader
                            .read_u16::<LittleEndian>()
                            .map_err(|e| PacketError::FieldError {
                                packet_name: "Item",
                                field_name: "unk",
                                error: e,
                            })?;
                }
                data
            }
            _ => [0u16; 12],
        };
        Ok(Self {
            uuid,
            id,
            data,
            #[cfg(feature = "ngs_packets")]
            unk,
        })
    }
    fn write(
        &self,
        writer: &mut impl std::io::Write,
        packet_type: PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<(), PacketError> {
        writer
            .write_u64::<LittleEndian>(self.uuid)
            .map_err(|e| PacketError::FieldError {
                packet_name: "Item",
                field_name: "uuid",
                error: e,
            })?;
        self.id.write(writer, packet_type, xor, sub).map_err(|e| {
            PacketError::CompositeFieldError {
                packet_name: "Item",
                field_name: "id",
                error: Box::new(e),
            }
        })?;
        self.data
            .write(writer, packet_type)
            .map_err(|e| PacketError::CompositeFieldError {
                packet_name: "Item",
                field_name: "data",
                error: Box::new(e),
            })?;
        #[cfg(feature = "ngs_packets")]
        if packet_type == PacketType::NGS {
            for byte in self.unk {
                writer
                    .write_u16::<LittleEndian>(byte)
                    .map_err(|e| PacketError::FieldError {
                        packet_name: "Item",
                        field_name: "unk",
                        error: e,
                    })?;
            }
        }
        Ok(())
    }
}

impl ItemType {
    pub(crate) fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        item: &ItemId,
        packet_type: PacketType,
    ) -> Result<Self, PacketError> {
        Ok(match (item.item_type, packet_type) {
            #[cfg(feature = "ngs_packets")]
            (0, PacketType::NGS) => {
                reader
                    .seek(SeekFrom::Current(0x38))
                    .map_err(|e| PacketError::FieldError {
                        packet_name: "ItemType",
                        field_name: "field_0",
                        error: e,
                    })?;
                Self::NoItemNGS
            }
            #[cfg(feature = "ngs_packets")]
            (1, PacketType::NGS) => {
                Self::WeaponNGS(WeaponItemNGS::read(reader, packet_type, 0, 0)?)
            }
            #[cfg(feature = "ngs_packets")]
            (2, PacketType::NGS) => {
                Self::ClothingNGS(ClothingNGSItem::read(reader, packet_type, 0, 0)?)
            }
            #[cfg(feature = "ngs_packets")]
            (3, PacketType::NGS) => {
                Self::ConsumableNGS(ConsumableNGSItem::read(reader, packet_type, 0, 0)?)
            }
            #[cfg(feature = "ngs_packets")]
            (5, PacketType::NGS) => Self::UnitNGS(UnitItemNGS::read(reader, packet_type, 0, 0)?),
            #[cfg(feature = "ngs_packets")]
            (10, PacketType::NGS) => Self::CamoNGS(CamoNGSItem::read(reader, packet_type, 0, 0)?),
            #[cfg(feature = "ngs_packets")]
            (_, PacketType::NGS) => Self::UnknownNGS({
                let mut tmp = [0u8; 0x38];
                reader
                    .read_exact(&mut tmp)
                    .map_err(|e| PacketError::FieldError {
                        packet_name: "ItemType",
                        field_name: "field_0",
                        error: e,
                    })?;
                tmp.to_vec().into()
            }),
            (0, _) => {
                reader
                    .seek(SeekFrom::Current(0x28))
                    .map_err(|e| PacketError::FieldError {
                        packet_name: "ItemType",
                        field_name: "field_0",
                        error: e,
                    })?;
                Self::NoItem
            }
            (1, _) => Self::Weapon(WeaponItem::read(reader, packet_type, 0, 0)?),
            (2, _) => Self::Clothing(ClothingItem::read(reader, packet_type, 0, 0)?),
            (3, _) => Self::Consumable(ConsumableItem::read(reader, packet_type, 0, 0)?),
            (5, _) => Self::Unit(UnitItem::read(reader, packet_type, 0, 0)?),
            (10, _) => Self::Camo(CamoItem::read(reader, packet_type, 0, 0)?),
            _ => Self::Unknown({
                let mut tmp = [0u8; 0x28];
                reader
                    .read_exact(&mut tmp)
                    .map_err(|e| PacketError::FieldError {
                        packet_name: "ItemType",
                        field_name: "field_0",
                        error: e,
                    })?;

                tmp.to_vec().into()
            }),
        })
    }
    pub(crate) fn write(
        &self,
        writer: &mut impl std::io::Write,
        packet_type: PacketType,
    ) -> Result<(), PacketError> {
        match self {
            Self::NoItem => {
                writer
                    .write_all(&[0; 0x28])
                    .map_err(|e| PacketError::FieldError {
                        packet_name: "ItemType",
                        field_name: "field_0",
                        error: e,
                    })?;
            }
            Self::Weapon(x) => x.write(writer, packet_type, 0, 0)?,
            Self::Clothing(x) => x.write(writer, packet_type, 0, 0)?,
            Self::Consumable(x) => x.write(writer, packet_type, 0, 0)?,
            Self::Camo(x) => x.write(writer, packet_type, 0, 0)?,
            Self::Unit(x) => x.write(writer, packet_type, 0, 0)?,
            Self::Unknown(x) => {
                let mut data = x.to_vec();
                data.resize(0x28, 0);
                writer
                    .write_all(&data)
                    .map_err(|e| PacketError::FieldError {
                        packet_name: "ItemType",
                        field_name: "field_0",
                        error: e,
                    })?;
            }
            #[cfg(feature = "ngs_packets")]
            Self::NoItemNGS => {
                writer
                    .write_all(&[0; 0x38])
                    .map_err(|e| PacketError::FieldError {
                        packet_name: "ItemType",
                        field_name: "field_0",
                        error: e,
                    })?;
            }
            #[cfg(feature = "ngs_packets")]
            Self::WeaponNGS(x) => x.write(writer, packet_type, 0, 0)?,
            #[cfg(feature = "ngs_packets")]
            Self::ClothingNGS(x) => x.write(writer, packet_type, 0, 0)?,
            #[cfg(feature = "ngs_packets")]
            Self::ConsumableNGS(x) => x.write(writer, packet_type, 0, 0)?,
            #[cfg(feature = "ngs_packets")]
            Self::CamoNGS(x) => x.write(writer, packet_type, 0, 0)?,
            #[cfg(feature = "ngs_packets")]
            Self::UnitNGS(x) => x.write(writer, packet_type, 0, 0)?,
            #[cfg(feature = "ngs_packets")]
            Self::UnknownNGS(x) => {
                let mut data = x.to_vec();
                data.resize(0x38, 0);
                writer
                    .write_all(&data)
                    .map_err(|e| PacketError::FieldError {
                        packet_name: "ItemType",
                        field_name: "field_0",
                        error: e,
                    })?;
            }
        }
        Ok(())
    }
}

fn read_packed_affixes(
    reader: &mut (impl std::io::Read + std::io::Seek),
    _: PacketType,
    _: u32,
    _: u32,
) -> Result<[u16; 8], PacketError> {
    let mut packed = [0u8; 12];
    let mut affixes = vec![];
    reader
        .read_exact(&mut packed)
        .map_err(|e| PacketError::FieldError {
            packet_name: "PackedAffixes",
            field_name: "affixes",
            error: e,
        })?;
    for i in 0..4 {
        let affix_1 = u16::from_le_bytes([packed[i * 3], (packed[i * 3 + 2] & 0xF0) >> 4]);
        let affix_2 = u16::from_le_bytes([packed[i * 3 + 1], (packed[i * 3 + 2] & 0xF)]);
        affixes.push(affix_1);
        affixes.push(affix_2);
    }
    Ok(affixes.try_into().unwrap())
}

fn write_packed_affixes(
    affixes: &[u16; 8],
    writer: &mut impl std::io::Write,
    _: PacketType,
    _: u32,
    _: u32,
) -> Result<(), PacketError> {
    let mut packed = vec![];
    for i in 0..4 {
        let affix_1 = affixes[i * 2].to_le_bytes();
        let affix_2 = affixes[i * 2 + 1].to_le_bytes();
        packed.push(affix_1[0]);
        packed.push(affix_2[0]);
        let packed_int = (affix_1[1] << 4 & 0xF0) | (affix_2[1] & 0xF);
        packed.push(packed_int);
    }
    writer
        .write_all(&packed)
        .map_err(|e| PacketError::FieldError {
            packet_name: "PackedAffixes",
            field_name: "affixes",
            error: e,
        })?;
    Ok(())
}
