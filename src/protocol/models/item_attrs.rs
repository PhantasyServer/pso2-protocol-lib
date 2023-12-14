use crate::protocol::{HelperReadWrite, PacketType};

use super::character::ClassFlags;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ItemAttributes {
    PC(ItemAttributesPC),
    Vita(ItemAttributesVita),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct ItemAttributesPC {
    pub unk1: u32,
    pub unk2: u128,
    #[Len_u16]
    pub weapons: Vec<WeaponAttrs>,
    #[Len_u16]
    pub human_costumes: Vec<HumanCostume>,
    #[Len_u16]
    pub cast_parts: Vec<CastPart>,
    #[Len_u16]
    pub consumables: Vec<Consumable>,
    #[Len_u16]
    pub data5: Vec<Data5>,
    #[Len_u16]
    pub data6: Vec<Data6>,
    #[Len_u16]
    pub data7: Vec<Data7>,
    #[Len_u16]
    pub data8: Vec<Data8>,
    #[Len_u16]
    pub data9: Vec<Data9>,
    #[Len_u16]
    pub data10: Vec<Data10>,
    #[Len_u16]
    pub data11: Vec<Data11>,
    #[Len_u16]
    pub data12: Vec<Data12>,
    #[Len_u16]
    pub data13: Vec<Data13>,
    #[Len_u16]
    pub data14: Vec<Data14>,
    #[Len_u16]
    pub data15: Vec<Data15>,
    #[Len_u16]
    pub data16: Vec<Data16>,
    #[Len_u16]
    pub data17: Vec<Data17>,
    #[FixedLen(406)]
    pub data18: Vec<ShortData>,
    #[Len_u16]
    pub data19: Vec<Data19>,
    #[Len_u16]
    pub data20: Vec<Data20>,
}

// different struct because of `FixedLen`
// maybe someday i'll merge them
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct ItemAttributesVita {
    pub unk1: u32,
    pub unk2: u128,
    #[Len_u16]
    pub weapons: Vec<WeaponAttrs>,
    #[Len_u16]
    pub human_costumes: Vec<HumanCostume>,
    #[Len_u16]
    pub cast_parts: Vec<CastPart>,
    #[Len_u16]
    pub consumables: Vec<Consumable>,
    #[Len_u16]
    pub data5: Vec<Data5>,
    #[Len_u16]
    pub data6: Vec<Data6>,
    #[Len_u16]
    pub data7: Vec<Data7>,
    #[Len_u16]
    pub data8: Vec<Data8>,
    #[Len_u16]
    pub data9: Vec<Data9>,
    #[Len_u16]
    pub data10: Vec<Data10>,
    #[Len_u16]
    pub data11: Vec<Data11>,
    #[Len_u16]
    pub data12: Vec<Data12>,
    #[Len_u16]
    pub data13: Vec<Data13>,
    #[Len_u16]
    pub data14: Vec<Data14>,
    #[Len_u16]
    pub data15: Vec<Data15>,
    #[Len_u16]
    pub data16: Vec<Data16>,
    #[Len_u16]
    pub data17: Vec<Data17>,
    // #[FixedLen(46)] // inside game files
    #[FixedLen(406)]
    pub data18: Vec<ShortData>,
    #[Len_u16]
    pub data19: Vec<Data19Vita>,
    #[Len_u16]
    pub data20: Vec<Data20>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct WeaponAttrs {
    pub id: u16,
    pub subid: u16,
    pub unk1: u8,
    pub priority: u8,
    pub unk2: u8,
    pub priority2: u8,
    pub rarity: u8,
    pub flags: u16,
    pub unk3: u8,
    pub icon_list: u16,
    pub icon_index: u16,
    pub range_dmg: u16,
    pub unk4: u8,
    pub melee_dmg: u16,
    pub unk5: u8,
    pub unk6: u32,
    pub gender_force_dmg: u16,
    pub unk8: [u8; 4],
    pub race: u8,
    pub flags2: u8,
    pub class: ClassFlags,
    pub req_stat: u16,
    pub req_stat_type: u8,
    pub unk9: u8,
    pub model: u16,
    pub unk10: u32,
    pub unk11: u16,
    pub affix_flag: u16,
    pub unk12: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct HumanCostume {
    pub id: u16,
    pub subid: u16,
    pub unk1: u16,
    pub unk2: u16,
    pub rarity: u8,
    pub flags: u16,
    pub unk3: u8,
    pub icon_list: u16,
    pub icon_index: u16,
    pub gender_flags: u8,
    pub color_flags: u8,
    pub race_flags: u8,
    pub unk4: u8,
    pub model: u16,
    pub unk5: [u16; 3],
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct CastPart {
    pub id: u16,
    pub subid: u16,
    pub unk1: u16,
    pub unk2: u16,
    pub rarity: u8,
    pub flags: u16,
    pub unk3: u8,
    pub icon_list: u16,
    pub icon_index: u16,
    pub gender_flags: u8,
    pub unk4: u8,
    pub race_flags: u8,
    pub unk5: u8,
    pub unk6: u16,
    pub model: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Consumable {
    pub id: u16,
    pub subid: u16,
    pub unk1: u16,
    pub unk2: u16,
    pub rarity: u8,
    pub flags: u16,
    pub unk3: u8,
    pub icon_list: u16,
    pub icon_index: u16,
    pub max_qty: u8,
    pub unk4: [u8; 3],
    pub unk5: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Data5 {
    pub id: u16,
    pub subid: u16,
    pub unk1: u16,
    pub unk2: u16,
    pub unk6: u8,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: u8,
    pub icon_list: u16,
    pub icon_index: u16,
    #[FixedLen(0x30)]
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Data6 {
    pub id: u16,
    pub subid: u16,
    pub unk1: u16,
    pub unk2: u16,
    pub rarity: u8,
    pub flags: u16,
    pub unk3: u8,
    pub icon_list: u16,
    pub icon_index: u16,
    #[FixedLen(0x30)]
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Data7 {
    #[FixedLen(0x38)]
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Data8 {
    #[FixedLen(0x10)]
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Data9 {
    #[FixedLen(0x3C)]
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Data10 {
    #[FixedLen(0x24)]
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Data11 {
    #[FixedLen(0x10)]
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Data12 {
    #[FixedLen(0x24)]
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Data13 {
    #[FixedLen(0x50)]
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Data14 {
    #[FixedLen(0x7C)]
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Data15 {
    #[FixedLen(0x10)]
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Data16 {
    #[FixedLen(0x12)]
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Data17 {
    #[FixedLen(0x5A)]
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Data18 {
    #[FixedLen(0x8)]
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct ShortData {
    #[Len_u16]
    pub unk: Vec<Data18>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Data19 {
    #[FixedLen(0x54)] //sent by jp server + used by pc
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Data19Vita {
    #[FixedLen(0x2C)] //inside na/vita client + used by vita
    pub unk: Vec<u8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[NoPadding]
pub struct Data20 {
    #[FixedLen(0x1C)]
    pub unk: Vec<u8>,
}

impl ItemAttributes {
    pub fn read_attrs(
        reader: &mut (impl std::io::Read + std::io::Seek),
        packet_type: PacketType,
    ) -> std::io::Result<Self> {
        match packet_type {
            PacketType::Vita => Ok(Self::Vita(ItemAttributesVita::read(
                reader,
                packet_type,
                0,
                0,
            )?)),
            _ => Ok(Self::PC(ItemAttributesPC::read(reader, packet_type, 0, 0)?)),
        }
    }
    pub fn write_attrs(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        match self {
            ItemAttributes::PC(x) => x.write_attrs(writer),
            ItemAttributes::Vita(x) => x.write_attrs(writer),
        }
    }
}

impl ItemAttributesPC {
    pub fn read_attrs(reader: &mut (impl std::io::Read + std::io::Seek)) -> std::io::Result<Self> {
        Self::read(reader, crate::protocol::PacketType::Classic, 0, 0)
    }
    pub fn write_attrs(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        self.write(writer, crate::protocol::PacketType::Classic, 0, 0)
    }
}

impl ItemAttributesVita {
    pub fn read_attrs(reader: &mut (impl std::io::Read + std::io::Seek)) -> std::io::Result<Self> {
        Self::read(reader, crate::protocol::PacketType::Classic, 0, 0)
    }
    pub fn write_attrs(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        self.write(writer, crate::protocol::PacketType::Classic, 0, 0)
    }
}

impl Default for ItemAttributesPC {
    fn default() -> Self {
        Self {
            unk1: 0,
            unk2: 0,
            weapons: vec![Default::default()],
            human_costumes: vec![Default::default()],
            cast_parts: vec![Default::default()],
            consumables: vec![Default::default()],
            data5: vec![Default::default()],
            data6: vec![Default::default()],
            data7: vec![Default::default()],
            data8: vec![Default::default()],
            data9: vec![Default::default()],
            data10: vec![Default::default()],
            data11: vec![Default::default()],
            data12: vec![Default::default()],
            data13: vec![Default::default()],
            data14: vec![Default::default()],
            data15: vec![Default::default()],
            data16: vec![Default::default()],
            data17: vec![Default::default()],
            data18: vec![Default::default()],
            data19: vec![Default::default()],
            data20: vec![Default::default()],
        }
    }
}

impl Default for ItemAttributesVita {
    fn default() -> Self {
        Self {
            unk1: 0,
            unk2: 0,
            weapons: vec![Default::default()],
            human_costumes: vec![Default::default()],
            cast_parts: vec![Default::default()],
            consumables: vec![Default::default()],
            data5: vec![Default::default()],
            data6: vec![Default::default()],
            data7: vec![Default::default()],
            data8: vec![Default::default()],
            data9: vec![Default::default()],
            data10: vec![Default::default()],
            data11: vec![Default::default()],
            data12: vec![Default::default()],
            data13: vec![Default::default()],
            data14: vec![Default::default()],
            data15: vec![Default::default()],
            data16: vec![Default::default()],
            data17: vec![Default::default()],
            data18: vec![Default::default()],
            data19: vec![Default::default()],
            data20: vec![Default::default()],
        }
    }
}

impl Default for ShortData {
    fn default() -> Self {
        Self {
            unk: vec![Default::default()],
        }
    }
}

impl From<Data19Vita> for Data19 {
    fn from(value: Data19Vita) -> Self {
        Self { unk: value.unk }
    }
}

impl From<Data19> for Data19Vita {
    fn from(value: Data19) -> Self {
        Self { unk: value.unk }
    }
}

impl From<ItemAttributesPC> for ItemAttributesVita {
    fn from(value: ItemAttributesPC) -> Self {
        Self {
            unk1: value.unk1,
            unk2: value.unk2,
            weapons: value.weapons,
            human_costumes: value.human_costumes,
            cast_parts: value.cast_parts,
            consumables: value.consumables,
            data5: value.data5,
            data6: value.data6,
            data7: value.data7,
            data8: value.data8,
            data9: value.data9,
            data10: value.data10,
            data11: value.data11,
            data12: value.data12,
            data13: value.data13,
            data14: value.data14,
            data15: value.data15,
            data16: value.data16,
            data17: value.data17,
            data18: value.data18,
            data19: value.data19.into_iter().map(|x| x.into()).collect(),
            data20: value.data20,
        }
    }
}

impl From<ItemAttributesVita> for ItemAttributesPC {
    fn from(value: ItemAttributesVita) -> Self {
        Self {
            unk1: value.unk1,
            unk2: value.unk2,
            weapons: value.weapons,
            human_costumes: value.human_costumes,
            cast_parts: value.cast_parts,
            consumables: value.consumables,
            data5: value.data5,
            data6: value.data6,
            data7: value.data7,
            data8: value.data8,
            data9: value.data9,
            data10: value.data10,
            data11: value.data11,
            data12: value.data12,
            data13: value.data13,
            data14: value.data14,
            data15: value.data15,
            data16: value.data16,
            data17: value.data17,
            data18: value.data18,
            data19: value.data19.into_iter().map(|x| x.into()).collect(),
            data20: value.data20,
        }
    }
}

impl From<ItemAttributes> for ItemAttributesPC {
    fn from(value: ItemAttributes) -> Self {
        match value {
            ItemAttributes::PC(x) => x,
            ItemAttributes::Vita(x) => x.into(),
        }
    }
}

impl From<ItemAttributesPC> for ItemAttributes {
    fn from(value: ItemAttributesPC) -> Self {
        Self::PC(value)
    }
}

impl From<ItemAttributes> for ItemAttributesVita {
    fn from(value: ItemAttributes) -> Self {
        match value {
            ItemAttributes::PC(x) => x.into(),
            ItemAttributes::Vita(x) => x,
        }
    }
}

impl From<ItemAttributesVita> for ItemAttributes {
    fn from(value: ItemAttributesVita) -> Self {
        Self::Vita(value)
    }
}
