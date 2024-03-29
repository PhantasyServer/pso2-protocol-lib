use super::{
    models::{character::HSVColor, Position},
    HelperReadWrite, ObjectHeader, PacketReadWrite, PacketType,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::time::Duration;

// ----------------------------------------------------------------
// Items packets
// ----------------------------------------------------------------

// 0x0F, 0x00
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x00)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x8A92, 0x30)]
pub struct ItemAttributesPacket {
    pub id: u16,
    pub segment: u16,
    pub total_size: u32,
    // data contains an ice archive that includes a "item_parameter.bin".
    pub data: Vec<u8>,
}

// 0x0F, 0x01
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x01)]
pub struct ItemPickupRequestPacket {
    pub drop_id: u32,
    pub unk: u32,
}

// 0x0F, 0x02
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x02)]
pub struct ItemPickupResponsePacket {
    pub target: ObjectHeader,
    pub drop_id: u32,
    pub was_pickedup: u32,
    pub unk: u32,
}

// 0x0F, 0x04
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x04)]
pub struct NewItemDropPacket {
    pub item_obj: ObjectHeader,
    pub item_id: ItemId,
    pub unk1: u32,
    pub unk2: u16,
    pub pos: Position,
    pub unk3: u16,
    pub unk4: u32,
    pub unk5: u16,
    pub unk6: u32,
    pub drop_id: u32,
    pub unk7: u32,
}

// 0x0F, 0x05
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x05)]
pub struct AddedItemPacket {
    pub item: Item,
    pub unk: u32,
}

// 0x0F, 0x06
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x06)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xAD04, 0xF3)]
pub struct UpdateInventoryPacket {
    pub updated: Vec<UpdatedInventoryItem>,
    pub unk: Vec<UpdatedInventoryItem>,
    pub unk2: u32,
}

// 0x0F, 0x0C
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x0C)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xCF76, 0xB5)]
pub struct LoadEquipedPacket {
    pub player: ObjectHeader,
    pub items: Vec<EquipedItem>,
    pub unk1: u32,
    #[FixedLen(0x28)]
    pub unk2: Vec<u8>,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(PacketType::NGS)]
    #[FixedLen(0x58)]
    pub unk3: Vec<u8>,
    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    #[OnlyOn(PacketType::NGS)]
    pub unk4: u32,
}

// 0x0F, 0x0D
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x0D)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x5533, 0x1)]
pub struct LoadPlayerInventoryPacket {
    pub object: ObjectHeader,
    pub name: String,
    pub meseta: u64,
    pub max_capacity: u32,
    pub items: Vec<Item>,
}

// 0x0F, 0x0F
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x0F)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x60AF, 0x97)]
pub struct MoveToStorageRequestPacket {
    pub uuids: Vec<MoveStorageItemRequest>,
}

// 0x0F, 0x10
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x10)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xE66C, 0xE2)]
pub struct MoveToStoragePacket {
    pub updated_inventory: Vec<UpdatedInventoryItem>,
    pub new_items: Vec<NewStorageItem>,
    pub updated: Vec<UpdatedItem>,
}

// 0x0F, 0x11
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x11)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x6C2A, 0x2D)]
pub struct MoveToInventoryRequestPacket {
    pub uuids: Vec<MoveStorageItemRequest>,
}

// 0x0F, 0x12
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x12)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xF1E8, 0x78)]
pub struct MoveToInventoryPacket {
    pub updated: Vec<UpdatedStorageItem>,
    pub new_items: Vec<NewInventoryItem>,
}

// 0x0F, 0x13
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x13)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x77A5, 0xC3)]
pub struct LoadStoragesPacket {
    pub stored_meseta: u64,
    pub unk1: Vec<StorageInfo>,
    pub items: Vec<Item>,
    pub unk2: u32,
}

// 0x0F, 0x14
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x14)]
pub struct InventoryMesetaPacket {
    pub meseta: u64,
}

// 0x0F, 0x15
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x15)]
pub struct MoveMesetaPacket {
    pub meseta: u64,
    pub direction: MesetaDirection,
}

// 0x0F, 0x16
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x16)]
pub struct StorageMesetaPacket {
    pub meseta: u64,
}

// 0x0F, 0x17
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x17)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x8E9C, 0xF0)]
pub struct DiscardItemRequestPacket {
    pub items: Vec<UUIDAmount>,
}

// 0x0F, 0x18
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x18)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x145A, 0x3B)]
pub struct MoveStoragesRequestPacket {
    pub old_id: u16,
    pub new_id: u16,
    pub items: Vec<UUIDAmount>,
}

// 0x0F, 0x19
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x19)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x9A17, 0x86)]
pub struct MoveStoragesPacket {
    pub new_items: Vec<NewStorageItem>,
    pub updated_new: Vec<UpdatedStorageItem>,
    pub updated_old: Vec<UpdatedStorageItem>,
}

// 0x0F, 0x1C
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x1C)]
pub struct GetItemDescriptionPacket {
    pub item: ItemId,
}

// 0x0F, 0x1D
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x1D)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xB10E, 0xB2)]
pub struct LoadItemDescriptionPacket {
    pub unk1: u32,
    pub item: ItemId,
    pub desc: String,
}

// 0x0F, 0x21
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x21)]
pub struct EquipedWeaponPacket {
    pub player: ObjectHeader,
    pub item: Item,
}

// 0x0F, 0x22
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x22)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x4DC2, 0x2A)]
pub struct UpdateStoragePacket {
    pub unk: Vec<UpdatedStorageItem>,
    pub updated: Vec<UpdatedStorageItem>,
    pub new_items: Vec<NewStorageItem>,
    pub unk2: u32,
    pub unk3: u64,
}

// 0x0F, 0x25
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x25)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xDEFB, 0x0B)]
pub struct DiscardStorageItemRequestPacket {
    pub items: Vec<MoveStorageItemRequest>,
}

// 0x0F, 0x30 see internal repr
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LoadItemPacket {
    pub items: Vec<NamedId>,
}

// 0x0F, 0x33
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x33)]
pub struct LearnedPAPacket {
    pub player: ObjectHeader,
    pub new_level: u32,
    pub pa_id: u32,
    pub item_id: ItemId,
}

// 0x0F, 0x65
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x65)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x4E66, 0xD3)]
pub struct PotentialListPacket {
    pub unk1: u16,
    pub unk2: u16,
    pub potential_ids: Vec<u32>,
    pub unk4: Vec<u8>,
    pub target_items: Vec<ShortItemId>,
    pub unk6: Vec<u32>,
    pub unk7: u32,
}

// 0x0F, 0x70
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x70)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x0D8C, 0x0D)]
pub struct AccountCapaignsPacket {
    pub unk1: u32,
    pub campaigns: Vec<Campaign>,
}

// 0x0F, 0x71
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x71)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x934A, 0x58)]
pub struct CampaignItemsRequestPacket {
    pub ids: Vec<u32>,
}

// 0x0F, 0x72
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x72)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x1908, 0xA3)]
pub struct CampaignItemListPacket {
    pub unk1: u32,
    pub items: Vec<CampaignItemDefinition>,
}

// 0x0F, 0x73
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x73)]
pub struct ReceiveCampaignRequestPacket {
    pub id: u32,
}

// 0x0F, 0x9C
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x9C)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xA25, 0xF6)]
pub struct Unk0F9CPacket {
    pub ids: Vec<Unk0f9c>,
}

// 0x0F, 0xBC
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xBC)]
pub struct ChangeWeaponPalettePacket {
    pub player: ObjectHeader,
    #[FixedLen(0x12)]
    pub unk: Vec<u16>,
    #[SeekAfter(0x4)]
    pub cur_palette: u32,
}

// 0x0F, 0xDF
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xDF)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xAC9, 0x9F)]
pub struct LoadMaterialStoragePacket {
    pub player_id: u32,
    pub items: Vec<MaterialStorageItem>,
    pub info: StorageInfo,
}

// 0x0F, 0xE0
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xE0)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x9087, 0xEA)]
pub struct MoveToMatStorageRequestPacket {
    pub items: Vec<MaterialStorageItem>,
}

// 0x0F, 0xE1
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xE1)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x1644, 0x35)]
pub struct MoveToMatStoragePacket {
    pub updated_inventory: Vec<UpdatedInventoryItem>,
    pub items: Vec<MaterialStorageItem>,
}

// 0x0F, 0xE2
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xE2)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x9C02, 0x80)]
pub struct MoveFromMatStorageRequestPacket {
    pub items: Vec<MaterialStorageItem>,
}

// 0x0F, 0xE3
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xE3)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x21C0, 0xCB)]
pub struct MoveFromMatStoragePacket {
    pub mat_items: Vec<MaterialStorageItem>,
    pub new_items: Vec<NewInventoryItem>,
}

// 0x0F, 0xE8
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xE8)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xBE74, 0x43)]
pub struct MoveMSToStorageRequestPacket {
    pub storage_id: u32,
    pub items: Vec<MaterialStorageItem>,
}

// 0x0F, 0xE9
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xE9)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x4432, 0x8E)]
pub struct MoveMSToStoragePacket {
    pub mat_items: Vec<MaterialStorageItem>,
    pub new_items: Vec<NewStorageItem>,
    pub updated: Vec<UpdatedStorageItem>,
}

// 0x0F, 0xEF
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xEF)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x66A4, 0x51)]
pub struct Unk0FEFPacket {
    pub ids: Vec<ItemId>,
}

// 0x0F, 0xFC
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xFC)]
#[Flags(Flags {packed: true, ..Default::default()})]
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
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x9E22, 0x46)]
pub struct LoadItemInternal {
    pub ids: Vec<ItemId>,
    pub names: String,
    pub name_length: Vec<u8>,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NamedId {
    pub name: String,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub id: ItemId,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Item {
    pub uuid: u64,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub id: ItemId,
    pub data: ItemType,

    #[cfg(feature = "ngs_packets")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
    pub unk: [u16; 12],
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct UUIDAmount {
    pub uuid: u64,
    pub amount: u16,
    pub unk: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct EquipedItem {
    pub item: Item,
    pub unk: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct MoveStorageItemRequest {
    pub uuid: u64,
    pub amount: u8,
    pub unk: u8,
    pub storage_id: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct NewStorageItem {
    pub item: Item,
    pub storage_id: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct NewInventoryItem {
    pub item: Item,
    pub amount: u16,
    pub is_new: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct UpdatedItem {
    pub uuid: u64,
    pub new_amount: u32,
    pub storage_id: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct UpdatedInventoryItem {
    pub uuid: u64,
    pub new_amount: u16,
    pub moved: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct UpdatedStorageItem {
    pub uuid: u64,
    pub new_amount: u16,
    pub moved: u16,
    pub storage_id: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Weapon(WeaponItem),
    Clothing(ClothingItem),
    Consumable(ConsumableItem),
    Camo(CamoItem),
    Unit(UnitItem),
    Unknown(Vec<u8>),
    // NGS Options
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
    UnknownNGS(Vec<u8>),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct WeaponItem {
    pub flags: u8,
    pub element: u8,
    pub force: u8,
    pub grind: u8,
    pub grind_percent: u8,
    pub unk1: u8,
    pub unk2: u16,
    pub affixes: [u16; 8],
    pub potential: u32,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: u16,
    pub unk6: u32,
    pub unk7: u32,
}

#[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct WeaponItemNGS {
    pub flags: u8,
    pub element: u8,
    pub force: u8,
    pub grind: u8,
    pub grind_percent: u8,
    pub unk1: u8,
    pub unk2: u16,
    pub affixes: [u32; 8],
    pub potential: u32,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: u16,
    pub unk6: u32,
    pub unk7: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct UnitItem {
    pub flags: u8,
    pub enh_level: u8,
    pub enh_percent: u8,
    pub unk1: u8,
    #[ManualRW(read_packed_affixes, write_packed_affixes)]
    pub affixes: [u16; 8],
    #[SeekAfter(0x7)]
    pub potential: u32,
    pub unk2: [u8; 4],
    #[Seek(1)]
    pub unk3: u32,
    pub unk4: u16,
    pub unk5: u16,
}

#[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct UnitItemNGS {
    pub flags: u8,
    pub enh_level: u8,
    pub enh_percent: u8,
    pub unk1: u8,
    pub affixes: [u32; 8],
    #[SeekAfter(0x6)]
    pub potential: u32,
    pub unk: [u8; 0xA],
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct ClothingItem {
    pub flags: u16,
    #[SeekAfter(0x14)]
    pub color: HSVColor,
    #[SeekAfter(0xA)]
    pub unk1: u16,
}

#[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct ClothingNGSItem {
    pub flags: u16,
    pub color: HSVColor,
    pub r_color: u8,
    pub g_color: u8,
    pub b_color: u8,
    pub unk1: [u8; 4],
    #[Seek(0x5)]
    pub unk2: [u8; 3],
    #[SeekAfter(0x20)]
    pub unk3: u8,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct ConsumableItem {
    pub flags: u16,
    #[SeekAfter(0x24)]
    pub amount: u16,
}

#[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct ConsumableNGSItem {
    pub flags: u16,
    #[SeekAfter(0x34)]
    pub amount: u16,
}

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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Copy, Clone, PartialEq, HelperReadWrite)]
pub struct ItemId {
    pub item_type: u16,
    pub id: u16,
    pub unk3: u16,
    pub subid: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Copy, Clone, PartialEq, HelperReadWrite)]
pub struct ShortItemId {
    pub item_type: u8,
    pub id: u8,
    pub subid: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Campaign {
    pub id: u32,
    pub start_date: Duration,
    pub end_date: Duration,
    #[FixedStr(0x3E)]
    pub title: String,
    #[FixedStr(0x102)]
    pub conditions: String,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct CampaignItemDefinition {
    pub campaign_id: u32,
    pub item_amount: u32,
    pub items: [CampaignItem; 8],
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Copy, Clone, PartialEq, HelperReadWrite)]
pub struct CampaignItem {
    pub id: ItemId,
    pub amount: u32,
    pub unk: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct StorageInfo {
    pub total_space: u32,
    pub used_space: u32,
    pub storage_id: u8,
    pub storage_type: u8,
    pub is_locked: u8,
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct MaterialStorageItem {
    pub id: u16,
    pub subid: u16,
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum MesetaDirection {
    #[default]
    #[Read_default]
    ToStorage = 1,
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
    ) -> std::io::Result<Self> {
        let packet = LoadItemInternal::read(reader, flags, packet_type)?;
        let mut names = packet.names.chars();
        let mut items = vec![];
        for (id, name_length) in packet.ids.into_iter().zip(packet.name_length.into_iter()) {
            let name = names.by_ref().take(name_length as usize).collect();
            items.push(NamedId { name, id });
        }
        Ok(Self { items })
    }

    fn write(&self, packet_type: PacketType) -> std::io::Result<Vec<u8>> {
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
    }
}

impl HelperReadWrite for Item {
    fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        packet_type: PacketType,
        xor: u32,
        sub: u32,
    ) -> std::io::Result<Self> {
        let uuid = reader.read_u64::<LittleEndian>()?;
        let unk5 = ItemId::read(reader, packet_type, xor, sub)?;
        let unk6 = ItemType::read(reader, &unk5, packet_type)?;
        #[cfg(feature = "ngs_packets")]
        let unk = match packet_type {
            PacketType::NGS => {
                let mut data = [0u16; 12];
                for byte in data.iter_mut() {
                    *byte = reader.read_u16::<LittleEndian>()?;
                }
                data
            }
            _ => [0u16; 12],
        };
        Ok(Self {
            uuid,
            id: unk5,
            data: unk6,
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
    ) -> std::io::Result<()> {
        writer.write_u64::<LittleEndian>(self.uuid)?;
        self.id.write(writer, packet_type, xor, sub)?;
        self.data.write(writer, packet_type)?;
        #[cfg(feature = "ngs_packets")]
        if packet_type == PacketType::NGS {
            for byte in self.unk {
                writer.write_u16::<LittleEndian>(byte)?;
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
    ) -> std::io::Result<Self> {
        Ok(match (item.item_type, packet_type) {
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
                reader.read_exact(&mut tmp)?;
                tmp.into()
            }),
            (1, _) => Self::Weapon(WeaponItem::read(reader, packet_type, 0, 0)?),
            (2, _) => Self::Clothing(ClothingItem::read(reader, packet_type, 0, 0)?),
            (3, _) => Self::Consumable(ConsumableItem::read(reader, packet_type, 0, 0)?),
            (5, _) => Self::Unit(UnitItem::read(reader, packet_type, 0, 0)?),
            (10, _) => Self::Camo(CamoItem::read(reader, packet_type, 0, 0)?),
            _ => Self::Unknown({
                let mut tmp = [0u8; 0x28];
                reader.read_exact(&mut tmp)?;
                tmp.into()
            }),
        })
    }
    pub(crate) fn write(
        &self,
        writer: &mut impl std::io::Write,
        packet_type: PacketType,
    ) -> std::io::Result<()> {
        match self {
            Self::Weapon(x) => x.write(writer, packet_type, 0, 0)?,
            Self::Clothing(x) => x.write(writer, packet_type, 0, 0)?,
            Self::Consumable(x) => x.write(writer, packet_type, 0, 0)?,
            Self::Camo(x) => x.write(writer, packet_type, 0, 0)?,
            Self::Unit(x) => x.write(writer, packet_type, 0, 0)?,
            Self::Unknown(x) => {
                let mut data = x.to_vec();
                data.resize(0x28, 0);
                writer.write_all(&data)?
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
                writer.write_all(&data)?
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
) -> std::io::Result<[u16; 8]> {
    let mut packed = [0u8; 12];
    let mut affixes = vec![];
    reader.read_exact(&mut packed)?;
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
) -> std::io::Result<()> {
    let mut packed = vec![];
    for i in 0..4 {
        let affix_1 = affixes[i * 2].to_le_bytes();
        let affix_2 = affixes[i * 2 + 1].to_le_bytes();
        packed.push(affix_1[0]);
        packed.push(affix_2[0]);
        let packed_int = (affix_1[1] << 4 & 0xF0) | (affix_2[1] & 0xF);
        packed.push(packed_int);
    }
    writer.write_all(&packed)?;
    Ok(())
}

// ----------------------------------------------------------------
// Default implementations
// ----------------------------------------------------------------

impl Default for ItemType {
    fn default() -> Self {
        Self::Weapon(Default::default())
    }
}
