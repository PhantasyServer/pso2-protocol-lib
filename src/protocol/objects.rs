use super::{models::Position, Flags, ObjectHeader, PacketHeader, PacketReadWrite};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use half::f16;
use std::{
    io::{Read, Seek, Write},
    time::Duration,
};

// ----------------------------------------------------------------
// Object related packets
// ----------------------------------------------------------------

// 0x04, 0x02
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x02)]
#[Flags(Flags {object_related: true, ..Default::default()})]
pub struct TeleportTransferPacket {
    pub unk1: [u8; 0xC],
    pub source_tele: ObjectHeader,
    pub location: Position,
    pub unk2: u16,
}

// 0x04, 0x07
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MovementPacket {
    pub unk: [u8; 0x6],
    pub ent1_id: Option<u64>,
    pub ent1_type: Option<u16>,
    pub ent1_unk: Option<u16>,
    pub ent2_id: Option<u64>,
    pub ent2_type: Option<u16>,
    pub ent2_unk: Option<u16>,
    pub timestamp: Option<Duration>,
    pub rot_x: Option<f16>,
    pub rot_y: Option<f16>,
    pub rot_z: Option<f16>,
    pub rot_w: Option<f16>,
    pub cur_x: Option<f16>,
    pub cur_y: Option<f16>,
    pub cur_z: Option<f16>,
    pub unk1: Option<f16>,
    pub unk_x: Option<f16>,
    pub unk_y: Option<f16>,
    pub unk_z: Option<f16>,
    pub unk2: Option<f16>,
    pub unk3: Option<u32>,
    pub unk4: Option<u8>,
}

// 0x04, 0x08
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x08)]
#[Flags(Flags {packed: true, object_related: true, ..Default::default()})]
pub struct MovementActionPacket {
    pub unk1: ObjectHeader,
    pub unk2: ObjectHeader,
    pub unk3: u32,
    pub unk4: [u8; 0x10],
    pub unk5: [u8; 0x8],
    pub unk6: [u8; 0xC],
    #[VariableAscii(0x922D, 0x45)]
    pub action: String,
    pub unk7: u32,
    pub unk8: u32,
    #[Magic(0x922D, 0x45)]
    pub unk9: Vec<u32>,
    pub unk10: u32,
}

// 0x04, 0x13
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x13)]
#[Flags(Flags {object_related: true, ..Default::default()})]
pub struct Unk4_13Packet {
    pub unk1: [u8; 0xC],
    pub unk2: ObjectHeader,
    pub unk3: ObjectHeader,
    pub unk4: u32,
}

// 0x04, 0x14
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x14)]
#[Flags(Flags {packed: true, object_related: true, ..Default::default()})]
pub struct InteractPacket {
    pub unk1: [u8; 0xC],
    pub object1: ObjectHeader,
    pub unk2: [u8; 0x4],
    pub object3: ObjectHeader,
    pub object4: [u8; 0x10],
    #[VariableAscii(0xD711, 0xCA)]
    pub action: String,
}

// 0x04, 0x15
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x15)]
#[Flags(Flags {packed: true, object_related: true, ..Default::default()})]
pub struct SetTagPacket {
    pub object1: ObjectHeader,
    pub object2: ObjectHeader,
    pub unk1: u32,
    pub object3: ObjectHeader,
    pub object4: ObjectHeader,
    pub unk2: u8,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: u8,
    #[VariableAscii(0x5CCF, 0x15)]
    pub attribute: String,
}

// 0x04, 0x24
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x24)]
#[Flags(Flags {object_related: true, ..Default::default()})]
pub struct Unk4_24Packet {
    pub unk1: ObjectHeader,
    pub unk2: ObjectHeader,
    pub unk3: ObjectHeader,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: [u8; 0xC],
    pub unk7: [u8; 0xC],
}

// 0x04, 0x71
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x71)]
#[Flags(Flags {object_related: true, ..Default::default()})]
pub struct MovementEndPacket {
    pub unk1: ObjectHeader,
    pub unk2: ObjectHeader,
    pub unk3: u32,
    pub cur_pos: Position,
    pub unk5: u16,
    pub unk_x: f16,
    pub unk_y: f16,
    pub unk_z: f16,
    pub unk7: u16,
    pub unk8: u32,
}

// ----------------------------------------------------------------
// Read/Write implementations
// ----------------------------------------------------------------

impl PacketReadWrite for MovementPacket {
    fn read(reader: &mut (impl Read + Seek), flags: Flags) -> std::io::Result<Self> {
        let mut packet = Self::default();
        reader.read_exact(&mut packet.unk)?;
        if flags.full_movement {
            packet.ent1_id = Some(reader.read_u64::<LittleEndian>()?);
            packet.ent1_type = Some(reader.read_u16::<LittleEndian>()?);
            packet.ent1_unk = Some(reader.read_u16::<LittleEndian>()?);
            packet.ent2_id = Some(reader.read_u64::<LittleEndian>()?);
            packet.ent2_type = Some(reader.read_u16::<LittleEndian>()?);
            packet.ent2_unk = Some(reader.read_u16::<LittleEndian>()?);
            packet.timestamp = Some(Duration::from_secs(
                reader.read_u32::<LittleEndian>()? as u64
            ));
            packet.rot_x = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
            packet.rot_y = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
            packet.rot_z = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
            packet.rot_w = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
            packet.cur_x = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
            packet.cur_y = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
            packet.cur_z = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
            packet.unk1 = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
            packet.unk_x = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
            packet.unk_y = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
            packet.unk_z = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
            packet.unk2 = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
            packet.unk3 = Some(reader.read_u32::<LittleEndian>()?);
            return Ok(packet);
        }
        let flags = reader.read_u24::<LittleEndian>()?;
        if flags & 0x1 != 0 {
            packet.ent1_id = Some(reader.read_u64::<LittleEndian>()?);
        }
        if flags & 0x2 != 0 {
            packet.ent1_type = Some(reader.read_u16::<LittleEndian>()?);
        }
        if flags & 0x4 != 0 {
            packet.ent1_unk = Some(reader.read_u16::<LittleEndian>()?);
        }
        if flags & 0x8 != 0 {
            packet.ent2_id = Some(reader.read_u64::<LittleEndian>()?);
        }
        if flags & 0x10 != 0 {
            packet.ent2_type = Some(reader.read_u16::<LittleEndian>()?);
        }
        if flags & 0x20 != 0 {
            packet.ent2_unk = Some(reader.read_u16::<LittleEndian>()?);
        }
        if flags & 0x40 != 0 {
            packet.timestamp = Some(Duration::from_secs(
                reader.read_u32::<LittleEndian>()? as u64
            ));
        }
        if flags & 0x80 != 0 {
            packet.rot_x = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
        }
        if flags & 0x100 != 0 {
            packet.rot_y = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
        }
        if flags & 0x200 != 0 {
            packet.rot_z = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
        }
        if flags & 0x400 != 0 {
            packet.rot_w = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
        }
        if flags & 0x800 != 0 {
            packet.cur_x = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
        }
        if flags & 0x1000 != 0 {
            packet.cur_y = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
        }
        if flags & 0x2000 != 0 {
            packet.cur_z = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
        }
        if flags & 0x4000 != 0 {
            packet.unk1 = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
        }
        if flags & 0x8000 != 0 {
            packet.unk_x = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
        }
        if flags & 0x10000 != 0 {
            packet.unk_y = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
        }
        if flags & 0x20000 != 0 {
            packet.unk_z = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
        }
        if flags & 0x40000 != 0 {
            packet.unk2 = Some(f16::from_bits(reader.read_u16::<LittleEndian>()?));
        }
        if flags & 0x80000 != 0 {
            if flags & 0x100000 != 0 {
                packet.unk4 = Some(reader.read_u8()?);
            } else {
                packet.unk3 = Some(reader.read_u32::<LittleEndian>()?);
            }
        }
        Ok(packet)
    }
    fn write(&self, is_ngs: bool) -> Vec<u8> {
        let mut tmp_buf = vec![];
        let mut flags = 0u32;
        if let Some(n) = self.ent1_id {
            tmp_buf.write_u64::<LittleEndian>(n).unwrap();
            flags += 0x1;
        }
        if let Some(n) = self.ent1_type {
            tmp_buf.write_u16::<LittleEndian>(n).unwrap();
            flags += 0x2;
        }
        if let Some(n) = self.ent1_unk {
            tmp_buf.write_u16::<LittleEndian>(n).unwrap();
            flags += 0x4;
        }
        if let Some(n) = self.ent2_id {
            tmp_buf.write_u64::<LittleEndian>(n).unwrap();
            flags += 0x8;
        }
        if let Some(n) = self.ent2_type {
            tmp_buf.write_u16::<LittleEndian>(n).unwrap();
            flags += 0x10;
        }
        if let Some(n) = self.ent2_unk {
            tmp_buf.write_u16::<LittleEndian>(n).unwrap();
            flags += 0x20;
        }
        if let Some(x) = self.timestamp {
            tmp_buf
                .write_u32::<LittleEndian>(x.as_secs() as u32)
                .unwrap();
            flags += 0x40;
        }
        if let Some(n) = self.rot_x {
            tmp_buf.write_u16::<LittleEndian>(n.to_bits()).unwrap();
            flags += 0x80;
        }
        if let Some(n) = self.rot_y {
            tmp_buf.write_u16::<LittleEndian>(n.to_bits()).unwrap();
            flags += 0x100;
        }
        if let Some(n) = self.rot_z {
            tmp_buf.write_u16::<LittleEndian>(n.to_bits()).unwrap();
            flags += 0x200;
        }
        if let Some(n) = self.rot_w {
            tmp_buf.write_u16::<LittleEndian>(n.to_bits()).unwrap();
            flags += 0x400;
        }
        if let Some(n) = self.cur_x {
            tmp_buf.write_u16::<LittleEndian>(n.to_bits()).unwrap();
            flags += 0x800;
        }
        if let Some(n) = self.cur_y {
            tmp_buf.write_u16::<LittleEndian>(n.to_bits()).unwrap();
            flags += 0x1000;
        }
        if let Some(n) = self.cur_z {
            tmp_buf.write_u16::<LittleEndian>(n.to_bits()).unwrap();
            flags += 0x2000;
        }
        if let Some(n) = self.unk1 {
            tmp_buf.write_u16::<LittleEndian>(n.to_bits()).unwrap();
            flags += 0x4000;
        }
        if let Some(n) = self.unk_x {
            tmp_buf.write_u16::<LittleEndian>(n.to_bits()).unwrap();
            flags += 0x8000;
        }
        if let Some(n) = self.unk_y {
            tmp_buf.write_u16::<LittleEndian>(n.to_bits()).unwrap();
            flags += 0x10000;
        }
        if let Some(n) = self.unk_z {
            tmp_buf.write_u16::<LittleEndian>(n.to_bits()).unwrap();
            flags += 0x20000;
        }
        if let Some(n) = self.unk2 {
            tmp_buf.write_u16::<LittleEndian>(n.to_bits()).unwrap();
            flags += 0x40000;
        }
        if let Some(n) = self.unk4 {
            tmp_buf.write_u8(n).unwrap();
            flags += 0x180000;
        } else if let Some(n) = self.unk3 {
            tmp_buf.write_u32::<LittleEndian>(n).unwrap();
            flags += 0x80000;
        }
        let mut buf = if flags == 0xFFFFF {
            PacketHeader::new(
                0x04,
                0x07,
                Flags {
                    object_related: true,
                    full_movement: true,
                    flag10: true,
                    ..Default::default()
                },
            )
            .write(is_ngs)
        } else {
            PacketHeader::new(
                0x04,
                0x07,
                Flags {
                    object_related: true,
                    flag10: true,
                    ..Default::default()
                },
            )
            .write(is_ngs)
        };
        buf.write_all(&self.unk).unwrap();
        if flags != 0xFFFFF {
            buf.write_u24::<LittleEndian>(flags).unwrap();
        }
        buf.append(&mut tmp_buf);
        buf
    }
}
