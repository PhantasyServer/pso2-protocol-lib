//! Item attribute related structures.
use super::character::ClassFlags;
use crate::{
    fixed_types::{FixedBytes, FixedVec, VecUSize},
    protocol::{HelperReadWrite, PacketError, PacketType},
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

/// Item attributes found in the `item_parameter.bin` file in the ICE archive from
/// [`crate::protocol::Packet::LoadItemAttributes`].
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum ItemAttributes {
    /// NA and JP client version.
    PC(ItemAttributesPC),
    /// Vita client version.
    Vita(ItemAttributesVita),
}

/// Item attributes found in the `item_parameter.bin` file in the ICE archive from
/// [`crate::protocol::Packet::LoadItemAttributes`] (NA and JP client).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct ItemAttributesPC {
    pub unk1: u32,
    pub unk2: u128,
    /// Attributes for weapons.
    pub weapons: VecUSize<u16, WeaponAttrs>,
    /// Attributes for costumes.
    pub human_costumes: VecUSize<u16, HumanCostume>,
    /// Attributes for CAST parts.
    pub cast_parts: VecUSize<u16, CastPart>,
    /// Attributes for consumables.
    pub consumables: VecUSize<u16, Consumable>,
    pub data5: VecUSize<u16, Data5>,
    /// Attributes for units.
    pub data6: VecUSize<u16, Unit>,
    pub data7: VecUSize<u16, Data7>,
    pub data8: VecUSize<u16, Data8>,
    pub data9: VecUSize<u16, Data9>,
    pub data10: VecUSize<u16, Data10>,
    pub data11: VecUSize<u16, Data11>,
    pub data12: VecUSize<u16, Data12>,
    pub data13: VecUSize<u16, Data13>,
    pub data14: VecUSize<u16, Data14>,
    pub data15: VecUSize<u16, Data15>,
    pub data16: VecUSize<u16, Data16>,
    pub data17: VecUSize<u16, Data17>,
    pub data18: FixedVec<406, ShortData>,
    pub data19: VecUSize<u16, Data19>,
    pub data20: VecUSize<u16, Data20>,
}

/// Item attributes found in the `item_parameter.bin` file in the ICE archive from
/// [`crate::protocol::Packet::LoadItemAttributes`] (Vita client).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct ItemAttributesVita {
    pub unk1: u32,
    pub unk2: u128,
    /// Attributes for weapons.
    pub weapons: VecUSize<u16, WeaponAttrs>,
    /// Attributes for costumes.
    pub human_costumes: VecUSize<u16, HumanCostume>,
    /// Attributes for CAST parts.
    pub cast_parts: VecUSize<u16, CastPart>,
    /// Attributes for consumables.
    pub consumables: VecUSize<u16, Consumable>,
    pub data5: VecUSize<u16, Data5>,
    /// Attributes for units.
    pub data6: VecUSize<u16, Unit>,
    pub data7: VecUSize<u16, Data7>,
    pub data8: VecUSize<u16, Data8>,
    pub data9: VecUSize<u16, Data9>,
    pub data10: VecUSize<u16, Data10>,
    pub data11: VecUSize<u16, Data11>,
    pub data12: VecUSize<u16, Data12>,
    pub data13: VecUSize<u16, Data13>,
    pub data14: VecUSize<u16, Data14>,
    pub data15: VecUSize<u16, Data15>,
    pub data16: VecUSize<u16, Data16>,
    pub data17: VecUSize<u16, Data17>,
    // #[FixedLen(46)] // inside game files
    pub data18: FixedVec<406, ShortData>,
    pub data19: VecUSize<u16, Data19Vita>,
    pub data20: VecUSize<u16, Data20>,
}

/// Weapon attributes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct WeaponAttrs {
    /// Item category.
    pub id: u16,
    /// Item ID.
    pub subid: u16,
    pub unk1: u8,
    pub priority: u8,
    pub unk2: u8,
    pub priority2: u8,
    /// Item rarity in stars.
    pub rarity: u8,
    pub flags: u16,
    pub unk3: u8,
    pub icon_list: u16,
    pub icon_index: u16,
    /// Range damage.
    pub range_dmg: u16,
    pub unk4: u8,
    /// Melee damage.
    pub melee_dmg: u16,
    pub unk5: u8,
    pub unk6: u32,
    /// Force damage and equipable genders.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub gender_force_dmg: GenderDmg,
    pub unk8: [u8; 4],
    /// Equipable races.
    pub race: RaceFlags,
    pub flags2: u8,
    /// Equipable classes.
    pub class: ClassFlags,
    /// Required stat value.
    pub req_stat: u16,
    /// Required stat type.
    pub req_stat_type: StatType,
    pub unk9: u8,
    pub model: u16,
    pub unk10: u32,
    pub unk11: u16,
    pub affix_flag: u16,
    pub unk12: u16,
}

/// Costume attributes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct HumanCostume {
    /// Item category.
    pub id: u16,
    /// Item ID.
    pub subid: u16,
    pub unk1: u16,
    pub unk2: u16,
    /// Item rarity in stars.
    pub rarity: u8,
    pub flags: u16,
    pub unk3: u8,
    pub icon_list: u16,
    pub icon_index: u16,
    /// Equipable genders.
    pub gender_flags: GenderFlags,
    pub color_flags: u8,
    /// Equipable races.
    pub race_flags: RaceFlags,
    pub unk4: u8,
    pub model: u16,
    pub unk5: [u16; 3],
}

/// CAST part attributes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct CastPart {
    /// Item category.
    pub id: u16,
    /// Item ID.
    pub subid: u16,
    pub unk1: u16,
    pub unk2: u16,
    /// Item rarity in stars.
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

/// Consumable attributes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Consumable {
    /// Item category.
    pub id: u16,
    /// Item ID.
    pub subid: u16,
    pub unk1: u16,
    pub unk2: u16,
    /// Item rarity in stars.
    pub rarity: u8,
    pub flags: u16,
    pub unk3: u8,
    pub icon_list: u16,
    pub icon_index: u16,
    /// Max item quantity.
    pub max_qty: u8,
    pub unk4: [u8; 3],
    pub unk5: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
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
    pub unk: FixedBytes<0x30>,
}

/// Unit attributes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unit {
    /// Item category.
    pub id: u16,
    /// Item ID.
    pub subid: u16,
    pub unk1: u16,
    pub unk2: u16,
    /// Item rarity in stars.
    pub rarity: u8,
    pub flags: u16,
    pub unk3: u8,
    pub icon_list: u16,
    pub icon_index: u16,
    /// Unit stats.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub stats: UnitRes,
    /// Required stat type.
    pub req_stat_type: StatType,
    pub unk4: u8,
    pub unk5: u8,
    pub unk6: u16,
    pub unk7: u16,
    pub unk8: u16,
    pub unk9: u16,
    /// Required stat value.
    pub req_stat: u16,
    /// Unit attack values.
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub atk: UnitAtk,
    pub unk10: u8,
    pub unk11: u8,
    pub unk12: u8,
    pub unk13: u16,
    pub unk14: u32,
    pub unk15: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Data7 {
    pub unk: FixedBytes<0x38>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Data8 {
    pub unk: FixedBytes<0x10>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Data9 {
    pub unk: FixedBytes<0x3C>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Data10 {
    pub unk: FixedBytes<0x24>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Data11 {
    pub unk: FixedBytes<0x10>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Data12 {
    pub unk: FixedBytes<0x24>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Data13 {
    pub unk: FixedBytes<0x50>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Data14 {
    pub unk: FixedBytes<0x7C>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Data15 {
    pub unk: FixedBytes<0x10>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Data16 {
    pub unk: FixedBytes<0x12, true>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Data17 {
    pub unk: FixedBytes<0x5A>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Data18 {
    pub unk: FixedBytes<0x8>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct ShortData {
    pub unk: VecUSize<u16, Data18>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Data19 {
    pub unk: FixedBytes<0x54>, //sent by jp server + used by pc
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Data19Vita {
    pub unk: FixedBytes<0x2C>, //inside na/vita client + used by vita
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Data20 {
    pub unk: FixedBytes<0x1C>,
}

/// Force damage and equipable genders.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GenderDmg {
    /// Force damage.
    pub force_dmg: u16,
    /// Equipable genders.
    pub gender: GenderFlags,
}

/// Unit stats.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct UnitRes {
    /// TEC resistance.
    pub tec_res: u8,
    /// TEC defence.
    pub tec_def: u16,
    /// RNG defence.
    pub rng_def: u16,
    /// MEL defence.
    pub mel_def: u16,
    /// Additional HP.
    pub hp: u16,
    /// Additional PP.
    pub pp: u8,
    /// Dark resistance.
    pub dark_res: u8,
    /// Light resistance.
    pub light_res: u8,
    /// Wind resistance.
    pub wind_res: u8,
    /// Lightning resistance.
    pub lightning_res: u8,
    /// Ice resistance.
    pub ice_res: u8,
    /// Fire resistance.
    pub fire_res: u8,
    /// RNG resistance.
    pub rng_res: u8,
    /// MEL resistance.
    pub mel_res: u8,
}

/// Unit attack values.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct UnitAtk {
    /// Additional MEL attack.
    pub mel_atk: u16,
    /// Additional RNG attack.
    pub rng_atk: u16,
    /// Additional TEC attack.
    pub tec_atk: u16,
    /// Additional DEX.
    pub dex: u16,
    pub unk_atk: u8,
}

bitflags::bitflags! {
    /// Equipable genders.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
    #[BitFlags(u8)]
    pub struct GenderFlags: u8 {
        /// Males can equip.
        const MALE = 1 << 0;
        /// Females can equip.
        const FEMALE = 1 << 1;
    }
}

bitflags::bitflags! {
    /// Equipable races.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
    #[BitFlags(u8)]
    pub struct RaceFlags: u8 {
        /// Humans can equip.
        const HUMAN = 1 << 0;
        /// Newmans can equip.
        const NEWMAN = 1 << 1;
        /// CASTs can equip.
        const CAST = 1 << 2;
        /// Deumans can equip.
        const DEUMAN = 1 << 3;
    }
}

/// Required stat type.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u8)]
pub enum StatType {
    /// MEL power.
    #[default]
    #[Read_default]
    MELPwr,
    /// RNG power.
    RNGPwr,
    /// TEC power.
    TECPwr,
    /// DEX.
    DEX,
    /// MEL defence.
    MELDef,
    /// RNG defence.
    RNGDef,
    /// TEC defence.
    TECDef,
}

// ----------------------------------------------------------------
// Read/Write implementations
// ----------------------------------------------------------------

impl ItemAttributes {
    pub fn read_attrs(
        reader: &mut (impl std::io::Read + std::io::Seek),
        packet_type: PacketType,
    ) -> Result<Self, PacketError> {
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
    pub fn write_attrs(&self, writer: &mut impl std::io::Write) -> Result<(), PacketError> {
        match self {
            ItemAttributes::PC(x) => x.write_attrs(writer),
            ItemAttributes::Vita(x) => x.write_attrs(writer),
        }
    }
}

impl ItemAttributesPC {
    pub fn read_attrs(
        reader: &mut (impl std::io::Read + std::io::Seek),
    ) -> Result<Self, PacketError> {
        Self::read(reader, crate::protocol::PacketType::Classic, 0, 0)
    }
    pub fn write_attrs(&self, writer: &mut impl std::io::Write) -> Result<(), PacketError> {
        self.write(writer, crate::protocol::PacketType::Classic, 0, 0)
    }
}

impl ItemAttributesVita {
    pub fn read_attrs(
        reader: &mut (impl std::io::Read + std::io::Seek),
    ) -> Result<Self, PacketError> {
        Self::read(reader, crate::protocol::PacketType::Classic, 0, 0)
    }
    pub fn write_attrs(&self, writer: &mut impl std::io::Write) -> Result<(), PacketError> {
        self.write(writer, crate::protocol::PacketType::Classic, 0, 0)
    }
}

impl HelperReadWrite for GenderDmg {
    fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        pt: PacketType,
        _: u32,
        _: u32,
    ) -> Result<Self, PacketError> {
        let bits = reader
            .read_u16::<LittleEndian>()
            .map_err(|e| PacketError::ValueError {
                packet_name: "GenderDmg",
                error: e,
            })?;
        // 14 bits
        let force_dmg = bits & 0x3FFF;
        // hacky solution but it works
        let gender_bits = (bits >> 14) as u8;
        let mut gender_slice = std::io::Cursor::new(std::slice::from_ref(&gender_bits));
        let gender = GenderFlags::read(&mut gender_slice, pt, 0, 0).map_err(|e| {
            PacketError::CompositeFieldError {
                packet_name: "GenderDmg",
                field_name: "gender",
                error: Box::new(e),
            }
        })?;
        Ok(Self { force_dmg, gender })
    }

    fn write(
        &self,
        writer: &mut impl std::io::Write,
        pt: PacketType,
        _: u32,
        _: u32,
    ) -> Result<(), PacketError> {
        let mut gender = [0u8];
        self.gender
            .write(&mut gender.as_mut_slice(), pt, 0, 0)
            .map_err(|e| PacketError::CompositeFieldError {
                packet_name: "GenderDmg",
                field_name: "gender",
                error: Box::new(e),
            })?;

        let mut bits = 0u16;
        bits |= self.force_dmg & 0x3FFF;
        bits |= (gender[0] as u16) << 14;
        writer
            .write_u16::<LittleEndian>(bits)
            .map_err(|e| PacketError::ValueError {
                packet_name: "GenderDmg",
                error: e,
            })?;
        Ok(())
    }
}

impl HelperReadWrite for UnitRes {
    fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        _: PacketType,
        _: u32,
        _: u32,
    ) -> Result<Self, PacketError> {
        let mut bytes = [0u8; 16];
        reader
            .read_exact(&mut bytes[..0xF])
            .map_err(|e| PacketError::ValueError {
                packet_name: "UnitRes",
                error: e,
            })?;
        let mut bits = u128::from_le_bytes(bytes);
        // 7 bits
        let tec_res = (bits & 0x7F) as u8;
        bits >>= 7;
        // 13 bits
        let tec_def = (bits & 0x1FFF) as u16;
        bits >>= 13;
        let rng_def = (bits & 0x1FFF) as u16;
        bits >>= 13;
        let mel_def = (bits & 0x1FFF) as u16;
        bits >>= 13;
        // 10 bits
        let hp = (bits & 0x3FF) as u16;
        bits >>= 10;
        // 8 bits
        let pp = (bits & 0xFF) as u8;
        bits >>= 8;
        // 7 bits
        let dark_res = (bits & 0x7F) as u8;
        bits >>= 7;
        let light_res = (bits & 0x7F) as u8;
        bits >>= 7;
        let wind_res = (bits & 0x7F) as u8;
        bits >>= 7;
        let lightning_res = (bits & 0x7F) as u8;
        bits >>= 7;
        let ice_res = (bits & 0x7F) as u8;
        bits >>= 7;
        let fire_res = (bits & 0x7F) as u8;
        bits >>= 7;
        let rng_res = (bits & 0x7F) as u8;
        bits >>= 7;
        let mel_res = (bits & 0x7F) as u8;
        Ok(Self {
            tec_res,
            tec_def,
            rng_def,
            mel_def,
            hp,
            pp,
            dark_res,
            light_res,
            wind_res,
            lightning_res,
            ice_res,
            fire_res,
            rng_res,
            mel_res,
        })
    }

    fn write(
        &self,
        writer: &mut impl std::io::Write,
        _: PacketType,
        _: u32,
        _: u32,
    ) -> Result<(), PacketError> {
        let mut bits = 0u128;
        bits |= (self.mel_res as u128) & 0x7F;
        bits <<= 7;
        bits |= (self.rng_res as u128) & 0x7F;
        bits <<= 7;
        bits |= (self.fire_res as u128) & 0x7F;
        bits <<= 7;
        bits |= (self.ice_res as u128) & 0x7F;
        bits <<= 7;
        bits |= (self.lightning_res as u128) & 0x7F;
        bits <<= 7;
        bits |= (self.wind_res as u128) & 0x7F;
        bits <<= 7;
        bits |= (self.light_res as u128) & 0x7F;
        bits <<= 7;
        bits |= (self.dark_res as u128) & 0x7F;
        bits <<= 8;
        bits |= (self.pp as u128) & 0xFF;
        bits <<= 10;
        bits |= (self.hp as u128) & 0x3FF;
        bits <<= 13;
        bits |= (self.mel_def as u128) & 0x1FFF;
        bits <<= 13;
        bits |= (self.rng_def as u128) & 0x1FFF;
        bits <<= 13;
        bits |= (self.tec_def as u128) & 0x1FFF;
        bits <<= 7;
        bits |= (self.tec_res as u128) & 0x7F;
        let bytes = bits.to_le_bytes();
        writer
            .write_all(&bytes[..0xF])
            .map_err(|e| PacketError::ValueError {
                packet_name: "UnitRes",
                error: e,
            })?;
        Ok(())
    }
}

impl HelperReadWrite for UnitAtk {
    fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        _: PacketType,
        _: u32,
        _: u32,
    ) -> Result<Self, PacketError> {
        let mut bytes = [0u8; 8];
        reader
            .read_exact(&mut bytes[..0x7])
            .map_err(|e| PacketError::ValueError {
                packet_name: "UnitAtk",
                error: e,
            })?;
        let mut bits = u64::from_le_bytes(bytes);
        // 13 bits
        let mel_atk = (bits & 0x1FFF) as u16;
        bits >>= 13;
        let rng_atk = (bits & 0x1FFF) as u16;
        bits >>= 13;
        let tec_atk = (bits & 0x1FFF) as u16;
        bits >>= 13;
        let dex = (bits & 0x1FFF) as u16;
        bits >>= 13;
        let unk_atk = (bits & 0xF) as u8;
        Ok(Self {
            mel_atk,
            rng_atk,
            tec_atk,
            dex,
            unk_atk,
        })
    }

    fn write(
        &self,
        writer: &mut impl std::io::Write,
        _: PacketType,
        _: u32,
        _: u32,
    ) -> Result<(), PacketError> {
        let mut bits = 0u64;
        bits |= (self.unk_atk as u64) & 0xF;
        bits <<= 13;
        bits |= (self.dex as u64) & 0x1FFF;
        bits <<= 13;
        bits |= (self.tec_atk as u64) & 0x1FFF;
        bits <<= 13;
        bits |= (self.rng_atk as u64) & 0x1FFF;
        bits <<= 13;
        bits |= (self.mel_atk as u64) & 0x1FFF;
        let bytes = bits.to_le_bytes();
        writer
            .write_all(&bytes[..0x7])
            .map_err(|e| PacketError::ValueError {
                packet_name: "UnitAtk",
                error: e,
            })?;
        Ok(())
    }
}

impl Default for ShortData {
    fn default() -> Self {
        Self {
            unk: vec![Default::default()].into(),
        }
    }
}

impl From<Data19Vita> for Data19 {
    fn from(value: Data19Vita) -> Self {
        Self {
            unk: Into::<Vec<_>>::into(value.unk).into(),
        }
    }
}

impl From<Data19> for Data19Vita {
    fn from(value: Data19) -> Self {
        Self {
            unk: Into::<Vec<_>>::into(value.unk).into(),
        }
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
            data19: Into::<Vec<_>>::into(value.data19)
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<_>>()
                .into(),
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
            data19: Into::<Vec<_>>::into(value.data19)
                .into_iter()
                .map(|x| x.into())
                .collect::<Vec<_>>()
                .into(),
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
