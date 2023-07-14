use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use super::{HelperReadWrite, ObjectHeader, PacketReadWrite};

// ----------------------------------------------------------------
// Loading packets
// ----------------------------------------------------------------

// 0x0F, 0x00
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x00)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct ItemAttributesPacket {
    pub id: u16,
    pub segment: u16,
    pub total_size: u32,
    // data contains an ice archive that includes a "item_parameter.bin".
    #[Magic(0x8A92, 0x30)]
    pub data: Vec<u8>,
}

// 0x0F, 0x0D
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x0D)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct LoadPlayerInventoryPacket {
    pub object: ObjectHeader,
    #[VariableUtf16(0x5533, 0x1)]
    pub name: String,
    pub meseta: u64,
    pub max_capacity: u32,
    #[Magic(0x5533, 0x1)]
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x0D)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct LoadPlayerInventoryNGSPacket {
    pub object: ObjectHeader,
    #[VariableUtf16(0x5533, 0x1)]
    pub name: String,
    pub meseta: u64,
    pub max_capacity: u32,
    #[Magic(0x5533, 0x1)]
    pub items: Vec<ItemNGS>,
}

// 0x0F, 0x13
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x13)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct LoadStoragesPacket {
    pub stored_meseta: u32,
    pub unk2: u32,
    #[Magic(0x77A5, 0xC3)]
    pub unk3: Vec<Unk0f13>,
    #[Magic(0x77A5, 0xC3)]
    pub items: Vec<Item>,
    pub unk5: u32,
}

#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x13)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct LoadStoragesNGSPacket {
    pub stored_meseta: u32,
    pub unk2: u32,
    #[Magic(0x77A5, 0xC3)]
    pub unk3: Vec<Unk0f13>,
    #[Magic(0x77A5, 0xC3)]
    pub items: Vec<ItemNGS>,
    pub unk5: u32,
}

// 0x0F, 0x1C
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x1C)]
pub struct GetItemDescriptionPacket {
    pub item: ItemId,
}

// 0x0F, 0x1D
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x1D)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct LoadItemDescriptionPacket {
    pub unk1: u32,
    pub item: ItemId,
    #[VariableUtf16(0xB10E, 0xB2)]
    pub desc: String,
}

// 0x0F, 0x30
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x30)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct LoadItemPacket {
    #[Magic(0x9E22, 0x46)]
    pub ids: Vec<ItemId>,
    #[VariableUtf16(0x9E22, 0x46)]
    pub names: String,
    #[Magic(0x9E22, 0x46)]
    pub name_length: Vec<u8>,
}

// 0x0F, 0x9C
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x9C)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct Unk0f9cPacket {
    #[Magic(0xA25, 0xF6)]
    pub ids: Vec<Unk0f9c>,
}

// 0x0F, 0xDF
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xDF)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct LoadMaterialStoragePacket {
    pub unk1: u32,
    #[Magic(0xAC9, 0x9F)]
    pub items: Vec<Unk0fdf>,
    pub unk3: [u8; 0xC],
}

// 0x0F, 0xEF
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xEF)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct Unk0fefPacket {
    #[Magic(0x66A4, 0x51)]
    pub ids: Vec<ItemId>,
}

// 0x0F, 0xFC
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0xFC)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct Unk0ffcPacket {
    #[Magic(0xB703, 0x6C)]
    pub ids: Vec<Unk0ffc>,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Item {
    pub uuid: u64,
    pub unk5: ItemId,
    pub unk6: ItemType,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ItemNGS {
    pub uuid: u64,
    pub unk5: ItemId,
    pub unk6: ItemTypeNGS,

    pub unk29: [u16; 12],
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Weapon(WeaponItem),
    Unknown([u8; 0x28]),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemTypeNGS {
    Weapon(WeaponItemNGS),
    Unknown([u8; 0x38]),
}

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

#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct ItemId {
    pub item_type: u16,
    pub id: u16,
    pub unk3: u16,
    pub subid: u16,
}

#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk0f13 {
    /*unk3 - storage type?
        0 - default
        1 - premium storage??
        2 - extend 1
        3 - extend 2

    unk4 - flags?
        1 - unable to deposit
    unk5 - storage not purchased
    unk6 - storage enabled? */
    pub total_space: u32,
    pub used_space: u32,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: u8,
    pub unk6: u8,
}

#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk0f9c {
    pub unk1: u16,
    pub unk2: u16,
    pub unk3: u16,
    pub unk4: u16,
    pub unk5: u32,
}

#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk0fdf {
    pub id: u16,
    pub subid: u16,
    pub amount: u16,
    pub unk4: u16,
}

#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Unk0ffc {
    pub unk1: u16,
    pub unk2: u16,
    pub unk3: u16,
    pub unk4: u16,
}

// ----------------------------------------------------------------
// Read/Write implementations
// ----------------------------------------------------------------

impl HelperReadWrite for Item {
    fn read(reader: &mut (impl std::io::Read + std::io::Seek)) -> std::io::Result<Self> {
        let uuid = reader.read_u64::<LittleEndian>()?;
        let unk5 = ItemId::read(reader)?;
        let unk6 = ItemType::read(reader, &unk5)?;
        Ok(Self { uuid, unk5, unk6 })
    }
    fn write(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        writer.write_u64::<LittleEndian>(self.uuid)?;
        self.unk5.write(writer)?;
        self.unk6.write(writer)?;
        Ok(())
    }
}

impl HelperReadWrite for ItemNGS {
    fn read(reader: &mut (impl std::io::Read + std::io::Seek)) -> std::io::Result<Self> {
        let uuid = reader.read_u64::<LittleEndian>()?;
        let unk5 = ItemId::read(reader)?;
        let unk6 = ItemTypeNGS::read(reader, &unk5)?;
        let mut unk29 = [0u16; 12];
        reader.read_u16_into::<LittleEndian>(&mut unk29)?;
        Ok(Self {
            uuid,
            unk5,
            unk6,
            unk29,
        })
    }
    fn write(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        writer.write_u64::<LittleEndian>(self.uuid)?;
        self.unk5.write(writer)?;
        self.unk6.write(writer)?;
        for n in self.unk29 {
            writer.write_u16::<LittleEndian>(n)?;
        }
        Ok(())
    }
}

impl ItemType {
    pub(crate) fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        item: &ItemId,
    ) -> std::io::Result<Self> {
        Ok(match item.item_type {
            1 => Self::Weapon(WeaponItem::read(reader)?),
            _ => Self::Unknown({
                let mut tmp = [0u8; 0x28];
                reader.read_exact(&mut tmp)?;
                tmp
            }),
        })
    }
    pub(crate) fn write(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        match self {
            Self::Weapon(x) => x.write(writer)?,
            Self::Unknown(x) => writer.write_all(x)?,
        }
        Ok(())
    }
}

impl ItemTypeNGS {
    pub(crate) fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        item: &ItemId,
    ) -> std::io::Result<Self> {
        Ok(match item.item_type {
            1 => Self::Weapon(WeaponItemNGS::read(reader)?),
            _ => Self::Unknown({
                let mut tmp = [0u8; 0x38];
                reader.read_exact(&mut tmp)?;
                tmp
            }),
        })
    }
    pub(crate) fn write(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        match self {
            Self::Weapon(x) => x.write(writer)?,
            Self::Unknown(x) => writer.write_all(x)?,
        }
        Ok(())
    }
}

// ----------------------------------------------------------------
// Default implementations
// ----------------------------------------------------------------

impl Default for ItemType {
    fn default() -> Self {
        Self::Weapon(Default::default())
    }
}

impl Default for ItemTypeNGS {
    fn default() -> Self {
        Self::Weapon(Default::default())
    }
}
