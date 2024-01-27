use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

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
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
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
    pub data6: Vec<Unit>,
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
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
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
    pub data6: Vec<Unit>,
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
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub gender_force_dmg: GenderDmg,
    pub unk8: [u8; 4],
    pub race: RaceFlags,
    pub flags2: u8,
    pub class: ClassFlags,
    pub req_stat: u16,
    pub req_stat_type: StatType,
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
    pub gender_flags: GenderFlags,
    pub color_flags: u8,
    pub race_flags: RaceFlags,
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
pub struct Unit {
    pub id: u16,
    pub subid: u16,
    pub unk1: u16,
    pub unk2: u16,
    pub rarity: u8,
    pub flags: u16,
    pub unk3: u8,
    pub icon_list: u16,
    pub icon_index: u16,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub stats: UnitRes,
    pub req_stat_type: StatType,
    pub unk4: u8,
    pub unk5: u8,
    pub unk6: u16,
    pub unk7: u16,
    pub unk8: u16,
    pub unk9: u16,
    pub req_stat: u16,
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct GenderDmg {
    pub force_dmg: u16,
    pub gender: GenderFlags,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct UnitRes {
    pub tec_res: u8,
    pub tec_def: u16,
    pub rng_def: u16,
    pub mel_def: u16,
    pub hp: u16,
    pub pp: u8,
    pub dark_res: u8,
    pub light_res: u8,
    pub wind_res: u8,
    pub lightning_res: u8,
    pub ice_res: u8,
    pub fire_res: u8,
    pub rng_res: u8,
    pub mel_res: u8,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct UnitAtk {
    pub mel_atk: u16,
    pub rng_atk: u16,
    pub tec_atk: u16,
    pub dex: u16,
    pub unk_atk: u8,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[Flags(u8)]
pub struct GenderFlags {
    pub male: bool,
    pub female: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[Flags(u8)]
pub struct RaceFlags {
    pub human: bool,
    pub newman: bool,
    pub cast: bool,
    pub deuman: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u8)]
pub enum StatType {
    #[default]
    #[Read_default]
    MELPwr,
    RNGPwr,
    TECPwr,
    DEX,
    MELDef,
    RNGDef,
    TECDef,
}

// ----------------------------------------------------------------
// Read/Write implementations
// ----------------------------------------------------------------

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

impl HelperReadWrite for GenderDmg {
    fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        pt: PacketType,
        _: u32,
        _: u32,
    ) -> std::io::Result<Self> {
        let bits = reader.read_u16::<LittleEndian>()?;
        // 14 bits
        let force_dmg = bits & 0x3FFF;
        // hacky solution but it works
        let gender_bits = (bits >> 14) as u8;
        let mut gender_slice = std::io::Cursor::new(std::slice::from_ref(&gender_bits));
        let gender = GenderFlags::read(&mut gender_slice, pt, 0, 0)?;
        Ok(Self { force_dmg, gender })
    }

    fn write(
        &self,
        writer: &mut impl std::io::Write,
        pt: PacketType,
        _: u32,
        _: u32,
    ) -> std::io::Result<()> {
        let mut gender = [0u8];
        self.gender.write(&mut gender.as_mut_slice(), pt, 0, 0)?;
        let mut bits = 0u16;
        bits |= self.force_dmg & 0x3FFF;
        bits |= (gender[0] as u16) << 14;
        writer.write_u16::<LittleEndian>(bits)?;
        Ok(())
    }
}

impl HelperReadWrite for UnitRes {
    fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        _: PacketType,
        _: u32,
        _: u32,
    ) -> std::io::Result<Self> {
        let mut bytes = [0u8; 16];
        reader.read_exact(&mut bytes[..0xF])?;
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
    ) -> std::io::Result<()> {
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
        writer.write_all(&bytes[..0xF])?;
        Ok(())
    }
}

impl HelperReadWrite for UnitAtk {
    fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        _: PacketType,
        _: u32,
        _: u32,
    ) -> std::io::Result<Self> {
        let mut bytes = [0u8; 8];
        reader.read_exact(&mut bytes[..0x7])?;
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
    ) -> std::io::Result<()> {
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
        writer.write_all(&bytes[..0x7])?;
        Ok(())
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
