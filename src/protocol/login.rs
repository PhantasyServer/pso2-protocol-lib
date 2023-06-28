use super::{
    duration_to_psotime, models::character::Character, psotime_to_duration, read_magic, read_utf16,
    read_utf8, read_variable_utf16, read_variable_utf8, write_magic, write_utf16, write_utf8,
    write_variable_utf16, write_variable_utf8, EntityType, Flags, ObjectHeader, PacketHeader,
    PacketReadWrite,
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    io::{Read, Seek, Write},
    net::Ipv4Addr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

// ----------------------------------------------------------------
// Login packets
// ----------------------------------------------------------------

// 0x11, 0x00
#[derive(Debug, Clone, PartialEq)]
pub struct SegaIDLoginPacket {
    //FIXME: fix data sizes
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub ver_id: [u8; 0x20],
    pub interfaces: Vec<NetInterface>,
    pub unk4: [u8; 0x90],
    pub unk5: [u8; 0x10],
    pub flag1: u32,
    pub flag2: u32,
    pub flag3: u32,
    pub flag4: u32,
    pub language: String,
    pub unk6: u32,
    pub unk7: u32,
    pub magic1: u32,
    pub unk8: [u8; 0x20],
    pub unk9: [u8; 0x44],
    pub username: String,
    pub password: String,
    pub unk10: u32,
    pub unk11: u32,
}

// 0x11, 0x01
#[derive(Debug, Clone, PartialEq)]
pub struct LoginResponsePacket {
    pub status: bool,
    pub error: String,
    pub player_id: u32,
    pub blockname: String,
    pub unk1: f32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: f32,
    pub unk6: f32,
    pub unk7: u32,
    pub unk8: f32,
    pub unk9: f32,
    pub unk10: u32,
    pub unk11: f32,
    pub unk12: u32,
    pub unk13: f32,
    pub unk14: [f32; 0xA],
    pub unk15: [f32; 0x15],
    pub unk16: u32,
    pub unk17: u32,
}

// 0x11, 0x03
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CharacterListPacket {
    pub is_global: bool,
    pub characters: Vec<Character>,
    pub play_times: [u32; 30],
    pub deletion_flags: [(u32, u32); 30],
    pub transfer_flags: [(u32, u32); 30],
    pub account_accessory: u16,
    pub login_survey: u32,
    pub ad: u32,
}

// 0x11, 0x05
#[derive(Debug, Default, Clone, PartialEq)]
pub struct CharacterCreatePacket {
    pub character: Character,
    pub is_global: bool,
}

// 0x11, 0x0B
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EncryptionRequestPacket {
    pub rsa_data: Vec<u8>,
}

// 0x11, 0x0C
#[derive(Debug, Default, Clone, PartialEq)]
pub struct EncryptionResponsePacket {
    pub data: Vec<u8>,
}

// 0x11, 0x0D
#[derive(Debug, Clone, PartialEq)]
pub struct ClientPingPacket {
    pub time: Duration,
}

// 0x11, 0x0E
#[derive(Debug, Clone, PartialEq)]
pub struct ClientPongPacket {
    pub client_time: Duration,
    pub server_time: Duration,
}

// 0x11, 0x1E
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NicknameRequestPacket {
    // Judging by Polaris Server this packet contains 0x44 byte long array of something
    pub error: u16,
}

// 0x11, 0x1D
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NicknameResponsePacket {
    pub nickname: String,
}

// 0x11, 0x2D
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SystemInformationPacket {
    pub cpu_info: String,
    pub video_info: String,
    pub vram: u64,
    pub total_ram: u64,
    pub unk1: u32,
    pub unk2: u32,
    pub windows_version: String,
    pub window_size: String,
    pub unk3: String,
    pub unk4: String,
    pub video_driver: String,
    pub total_disk_space: u64,
    pub free_disk_space: u64,
}

// 0x11, 0x3D
#[derive(Debug, Clone, PartialEq)]
pub struct ShipListPacket {
    pub ships: Vec<ShipEntry>,
    pub timestamp: Duration,
}

// 0x11, 0x42
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CreateCharacter1ResponsePacket {
    pub status: u32,
    pub unk2: u32,
    pub used_smth: u32,
    pub req_ac: u32,
}

// 0x11, 0x55
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CreateCharacter2ResponsePacket {
    pub unk: u32,
}

// 0x11, 0x63
#[derive(Debug, Clone, PartialEq)]
pub struct VitaLoginPacket {
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u16,
    pub unk4: u32,
    pub unk5: u32,
    pub ver_id: [u8; 0x20],
    pub interfaces: Vec<NetInterface>,
    pub unk6: [u8; 0x10],
    pub unk7: [u8; 0x90],
    pub unk8: [u8; 0x10],
    pub flag1: u32,
    pub flag2: u32,
    pub flag3: u32,
    pub flag4: u32,
    pub flag5: u32,
    pub flag6: u32,
    pub language: String,
    pub unk9: u32,
    pub unk10: u32,
    pub magic1: u32,
    pub unk11: [u8; 0x20],
    pub unk12: [u8; 0x44],
    pub username: String,
    pub password: String,
    pub unk13: u8,
    pub unk14: u8,
    pub unk15: u16,
    pub unk16: String,
    pub unk17: Vec<u8>,
    pub unk18: [u8; 0x10],
}

// 0x11, 0x87
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LoginHistoryPacket {
    pub attempts: Vec<LoginAttempt>,
}

// 0x11, 0xEA
#[derive(Debug, Clone, PartialEq)]
pub struct NicknameErrorPacket {
    pub unk1: u32,
    pub nickname: String,
}

// 0x11, 0xEE
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EmailCodeRequestPacket {
    pub unk1: u32,
    pub message: String,
}

// 0x11, 0xFF
#[derive(Debug, Clone, PartialEq)]
pub struct Unk1Packet {
    pub unk1: u8,
    pub unk2: u8,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: String,
    pub unk6: [u8; 0xC],
    pub unk7: [u8; 0x40],
    pub unk8: [u8; 0x20],
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[derive(Debug, Default, Clone, PartialEq)]
pub struct NetInterface {
    pub state: u32,
    pub mac: String,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ShipEntry {
    pub id: u32,
    pub name: String,
    pub ip: [u8; 4],
    pub status: ShipStatus,
    pub order: u16,
}

#[repr(u16)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum ShipStatus {
    #[default]
    Unknown,
    Online,
    Busy,
    Full,
    Offline,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct LoginAttempt {
    pub ip: Ipv4Addr,
    pub status: LoginResult,
    pub timestamp: Duration,
    pub unk: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(u32)]
pub enum LoginResult {
    #[default]
    Successful,
    EmailConfirmed,
    LoginError,
    EmailAuthError,
    AuthEmailSent,
    OTPError,
    InMaintenance,
    GenericError,
}

// ----------------------------------------------------------------
// Read/Write implementations
// ----------------------------------------------------------------

impl PacketReadWrite for SegaIDLoginPacket {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let unk1 = reader.read_u32::<LittleEndian>()?;
        let unk2 = reader.read_u32::<LittleEndian>()?;
        let unk3 = reader.read_u32::<LittleEndian>()?;
        let mut ver_id = [0u8; 0x20];
        reader.read_exact(&mut ver_id)?;
        let mac_amount = read_magic(reader, 0x6B, 0x5E6)? as usize;
        let mut interfaces = Vec::with_capacity(mac_amount);
        for _ in 0..mac_amount {
            interfaces.push(NetInterface::read(reader)?);
        }
        reader.seek(std::io::SeekFrom::Current(0x14))?;
        let mut unk4 = [0u8; 0x90];
        reader.read_exact(&mut unk4)?;
        reader.seek(std::io::SeekFrom::Current(0x10))?;
        let mut unk5 = [0u8; 0x10];
        reader.read_exact(&mut unk5)?;
        reader.seek(std::io::SeekFrom::Current(0x10))?;
        let flag1 = reader.read_u32::<LittleEndian>()?;
        let flag2 = reader.read_u32::<LittleEndian>()?;
        let flag3 = reader.read_u32::<LittleEndian>()?;
        let flag4 = reader.read_u32::<LittleEndian>()?;
        reader.seek(std::io::SeekFrom::Current(0x8))?;
        let language = read_utf16(reader, 0x10);
        let unk6 = reader.read_u32::<LittleEndian>()?;
        let unk7 = reader.read_u32::<LittleEndian>()?;
        let magic1 = reader.read_u32::<LittleEndian>()?;
        let mut unk8 = [0u8; 0x20];
        reader.read_exact(&mut unk8)?;
        let mut unk9 = [0u8; 0x44];
        reader.read_exact(&mut unk9)?;
        unk9.iter_mut().map(|x| *x ^= 0x15).count();
        reader.seek(std::io::SeekFrom::Current(0x104))?;
        let username = read_utf8(reader, 0x40);
        reader.seek(std::io::SeekFrom::Current(0x20))?;
        let password = read_utf8(reader, 0x40);
        reader.seek(std::io::SeekFrom::Current(0x4))?;
        let unk10 = reader.read_u32::<LittleEndian>()?;
        let unk11 = reader.read_u32::<LittleEndian>()?;
        Ok(Self {
            unk1,
            unk2,
            unk3,
            ver_id,
            interfaces,
            unk4,
            unk5,
            flag1,
            flag2,
            flag3,
            flag4,
            language,
            unk6,
            unk7,
            magic1,
            unk8,
            unk9,
            username,
            password,
            unk10,
            unk11,
        })
    }
    fn write(mut self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(
            0x11,
            0x00,
            Flags {
                packed: true,
                ..Default::default()
            },
        )
        .write(is_ngs);
        buf.write_u32::<LittleEndian>(self.unk1).unwrap();
        buf.write_u32::<LittleEndian>(self.unk2).unwrap();
        buf.write_u32::<LittleEndian>(self.unk3).unwrap();
        buf.write_all(&self.ver_id).unwrap();
        buf.write_u32::<LittleEndian>(write_magic(self.interfaces.len() as u32, 107, 0x5E6))
            .unwrap();
        for interface in self.interfaces.iter_mut() {
            interface.write(&mut buf).unwrap();
        }
        buf.write_all(&[0u8; 0x14]).unwrap();
        buf.write_all(&self.unk4).unwrap();
        buf.write_all(&[0u8; 0x10]).unwrap();
        buf.write_all(&self.unk5).unwrap();
        buf.write_all(&[0u8; 0x10]).unwrap();
        buf.write_u32::<LittleEndian>(self.flag1).unwrap();
        buf.write_u32::<LittleEndian>(self.flag2).unwrap();
        buf.write_u32::<LittleEndian>(self.flag3).unwrap();
        buf.write_u32::<LittleEndian>(self.flag4).unwrap();
        buf.write_all(&[0u8; 0x8]).unwrap();
        buf.write_all(&write_utf16(&self.language, 0x10)).unwrap();
        buf.write_u32::<LittleEndian>(self.unk6).unwrap();
        buf.write_u32::<LittleEndian>(self.unk7).unwrap();
        buf.write_u32::<LittleEndian>(self.magic1).unwrap();
        buf.write_all(&self.unk8).unwrap();
        self.unk9.iter_mut().map(|x| *x ^= 0x15).count();
        buf.write_all(&self.unk9).unwrap();
        buf.write_all(&[0u8; 0x104]).unwrap();
        buf.write_all(&write_utf8(&self.username, 0x40)).unwrap();
        buf.write_all(&[0u8; 0x20]).unwrap();
        buf.write_all(&write_utf8(&self.password, 0x40)).unwrap();
        buf.write_all(&[0u8; 0x4]).unwrap();
        buf.write_u32::<LittleEndian>(self.unk10).unwrap();
        buf.write_u32::<LittleEndian>(self.unk11).unwrap();
        buf.write_u32::<LittleEndian>(0).unwrap();
        buf
    }
}

impl PacketReadWrite for LoginResponsePacket {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let status = matches!(reader.read_u32::<LittleEndian>()?, 0);
        let error = read_variable_utf16(reader, 0xB6, 0x8BA4);
        let player_id = ObjectHeader::read(reader)?.id;
        let blockname = read_utf16(reader, 0x20);
        let unk1 = reader.read_f32::<LittleEndian>()?;
        let unk2 = reader.read_u32::<LittleEndian>()?;
        let unk3 = reader.read_u32::<LittleEndian>()?;
        let unk4 = reader.read_u32::<LittleEndian>()?;
        let unk5 = reader.read_f32::<LittleEndian>()?;
        let unk6 = reader.read_f32::<LittleEndian>()?;
        let unk7 = reader.read_u32::<LittleEndian>()?;
        let unk8 = reader.read_f32::<LittleEndian>()?;
        let unk9 = reader.read_f32::<LittleEndian>()?;
        let unk10 = reader.read_u32::<LittleEndian>()?;
        let unk11 = reader.read_f32::<LittleEndian>()?;
        let unk12 = reader.read_u32::<LittleEndian>()?;
        let unk13 = reader.read_f32::<LittleEndian>()?;
        let mut unk14 = [0f32; 0xA];
        for num in &mut unk14 {
            *num = reader.read_f32::<LittleEndian>()?;
        }
        let mut unk15 = [0f32; 0x15];
        for num in &mut unk15 {
            *num = reader.read_f32::<LittleEndian>()?;
        }
        let unk16 = reader.read_u32::<LittleEndian>()?;
        let unk17 = reader.read_u32::<LittleEndian>()?;
        Ok(Self {
            status,
            error,
            player_id,
            blockname,
            unk1,
            unk2,
            unk3,
            unk4,
            unk5,
            unk6,
            unk7,
            unk8,
            unk9,
            unk10,
            unk11,
            unk12,
            unk13,
            unk14,
            unk15,
            unk16,
            unk17,
        })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(
            0x11,
            0x01,
            Flags {
                packed: true,
                ..Default::default()
            },
        )
        .write(is_ngs);
        buf.write_u32::<LittleEndian>(!self.status as u32).unwrap();
        buf.write_all(&write_variable_utf16(&self.error, 0xB6, 0x8BA4))
            .unwrap();
        ObjectHeader {
            id: self.player_id,
            entity_type: EntityType::Player,
        }
        .write(&mut buf)
        .unwrap();
        buf.write_all(&write_utf16(&self.blockname, 0x20)).unwrap();
        buf.write_f32::<LittleEndian>(self.unk1).unwrap();
        buf.write_u32::<LittleEndian>(self.unk2).unwrap();
        buf.write_u32::<LittleEndian>(self.unk3).unwrap();
        buf.write_u32::<LittleEndian>(self.unk4).unwrap();
        buf.write_f32::<LittleEndian>(self.unk5).unwrap();
        buf.write_f32::<LittleEndian>(self.unk6).unwrap();
        buf.write_u32::<LittleEndian>(self.unk7).unwrap();
        buf.write_f32::<LittleEndian>(self.unk8).unwrap();
        buf.write_f32::<LittleEndian>(self.unk9).unwrap();
        buf.write_u32::<LittleEndian>(self.unk10).unwrap();
        buf.write_f32::<LittleEndian>(self.unk11).unwrap();
        buf.write_u32::<LittleEndian>(self.unk12).unwrap();
        buf.write_f32::<LittleEndian>(self.unk13).unwrap();
        for num in self.unk14 {
            buf.write_f32::<LittleEndian>(num).unwrap();
        }
        for num in self.unk15 {
            buf.write_f32::<LittleEndian>(num).unwrap();
        }
        buf.write_u32::<LittleEndian>(self.unk16).unwrap();
        buf.write_u32::<LittleEndian>(self.unk17).unwrap();
        buf.write_all(&[0; 0xC]).unwrap();
        buf
    }
}

impl PacketReadWrite for CharacterListPacket {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let char_amount = reader.read_u32::<LittleEndian>()?.clamp(0, 30);
        reader.seek(std::io::SeekFrom::Current(4))?;
        let mut characters = vec![];
        for i in 0..30 {
            reader.seek(std::io::SeekFrom::Current(4))?;
            let character = Character::read(reader)?;
            if i < char_amount {
                characters.push(character);
            }
        }
        // ???
        reader.seek(std::io::SeekFrom::Current(0x41A4))?;
        let mut play_times = [0u32; 30];
        for item in &mut play_times {
            *item = reader.read_u32::<LittleEndian>()?;
        }
        reader.seek(std::io::SeekFrom::Current(32))?;
        let mut deletion_flags = [(0u32, 0u32); 30];
        for item in &mut deletion_flags {
            item.0 = reader.read_u32::<LittleEndian>()?;
            item.1 = reader.read_u32::<LittleEndian>()?;
        }
        let mut transfer_flags = [(0u32, 0u32); 30];
        for item in &mut transfer_flags {
            item.0 = reader.read_u32::<LittleEndian>()?;
            item.1 = reader.read_u32::<LittleEndian>()?;
        }
        let account_accessory = reader.read_u16::<LittleEndian>()?;
        reader.seek(std::io::SeekFrom::Current(6))?;
        let login_survey = reader.read_u32::<LittleEndian>()?;
        let ad = reader.read_u32::<LittleEndian>()?;

        Ok(Self {
            is_global: false,
            characters,
            play_times,
            deletion_flags,
            transfer_flags,
            account_accessory,
            login_survey,
            ad,
        })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(0x11, 0x03, Flags::default()).write(is_ngs);
        buf.write_u32::<LittleEndian>((self.characters.len() as u32).clamp(0, 30))
            .unwrap();
        buf.write_u32::<LittleEndian>(0).unwrap();

        let mut characters = self.characters;
        if characters.is_empty() {
            characters.push(Character::default());
        }

        for character in characters.iter().cycle().take(30) {
            buf.write_u32::<LittleEndian>(0).unwrap();
            character.write(&mut buf, self.is_global).unwrap();
        }
        // ???
        for _ in 0..0x41A4 {
            buf.write_u8(0).unwrap();
        }
        for i in 0..30 {
            buf.write_u32::<LittleEndian>(self.play_times[i]).unwrap();
        }
        // ???
        for _ in 0..32 {
            buf.write_u8(0).unwrap();
        }
        for i in 0..30 {
            // deletion flag
            buf.write_u32::<LittleEndian>(self.deletion_flags[i].0)
                .unwrap();
            // timestamp
            buf.write_u32::<LittleEndian>(self.deletion_flags[i].1)
                .unwrap();
        }
        for i in 0..30 {
            // transfer flag
            buf.write_u32::<LittleEndian>(self.transfer_flags[i].0)
                .unwrap();
            // ??? prob target ship
            buf.write_u32::<LittleEndian>(self.transfer_flags[i].1)
                .unwrap();
        }
        buf.write_u16::<LittleEndian>(self.account_accessory)
            .unwrap();
        // ???
        buf.write_all(&[0u8; 6]).unwrap();
        buf.write_u32::<LittleEndian>(self.login_survey).unwrap();
        buf.write_u32::<LittleEndian>(self.ad).unwrap();
        // ???
        buf.write_u32::<LittleEndian>(0x00_00_00_00).unwrap();
        // ???
        buf.write_u32::<LittleEndian>(0x00_00_00_00).unwrap();
        buf
    }
}

impl PacketReadWrite for CharacterCreatePacket {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let character = Character::read(reader)?;
        Ok(Self {
            character,
            is_global: false,
        })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(0x11, 0x05, Flags::default()).write(is_ngs);
        self.character.write(&mut buf, self.is_global).unwrap();
        buf
    }
}

impl PacketReadWrite for EncryptionRequestPacket {
    fn read(reader: &mut impl Read) -> std::io::Result<Self> {
        let mut rsa_data = vec![];
        reader.read_to_end(&mut rsa_data)?;
        let mut tmp_data = vec![];
        let mut iter = rsa_data.into_iter().rev().skip(4);
        if let Some(x) = iter.find(|x| *x != 0x00) {
            tmp_data.push(x);
            tmp_data.extend(iter);
        }
        Ok(Self { rsa_data: tmp_data })
    }
    fn write(mut self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(0x11, 0x0B, Flags::default()).write(is_ngs);
        self.rsa_data.reverse();
        self.rsa_data.resize(0x104, 0);
        buf.extend(self.rsa_data);
        buf
    }
}

impl PacketReadWrite for EncryptionResponsePacket {
    fn read(reader: &mut impl Read) -> std::io::Result<Self> {
        let mut data = vec![];
        reader.read_to_end(&mut data)?;

        Ok(Self { data })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(0x11, 0x0C, Flags::default()).write(is_ngs);
        buf.extend(self.data);
        buf
    }
}

impl PacketReadWrite for ClientPingPacket {
    fn read(reader: &mut impl Read) -> std::io::Result<Self> {
        let timestamp = reader.read_u64::<LittleEndian>()?;
        let time = psotime_to_duration(timestamp);
        Ok(Self { time })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(0x11, 0x0D, Flags::default()).write(is_ngs);
        buf.write_u64::<LittleEndian>(duration_to_psotime(self.time))
            .unwrap();
        buf
    }
}

impl PacketReadWrite for ClientPongPacket {
    fn read(reader: &mut impl Read) -> std::io::Result<Self> {
        let timestamp = reader.read_u64::<LittleEndian>()?;
        let client_time = psotime_to_duration(timestamp);
        let timestamp = reader.read_u64::<LittleEndian>()?;
        let server_time = psotime_to_duration(timestamp);
        Ok(Self {
            client_time,
            server_time,
        })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(0x11, 0x0E, Flags::default()).write(is_ngs);
        buf.write_u64::<LittleEndian>(duration_to_psotime(self.client_time))
            .unwrap();
        buf.write_u64::<LittleEndian>(duration_to_psotime(self.server_time))
            .unwrap();
        buf
    }
}

impl PacketReadWrite for NicknameRequestPacket {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let error = reader.read_u16::<LittleEndian>()?;

        Ok(Self { error })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(0x11, 0x1E, Flags::default()).write(is_ngs);
        buf.write_u16::<LittleEndian>(self.error).unwrap();
        buf.extend(std::iter::once(0).cycle().take(0x42));
        buf
    }
}

impl PacketReadWrite for NicknameResponsePacket {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let nickname = read_utf16(reader, 0x10);

        Ok(Self { nickname })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(0x11, 0x1D, Flags::default()).write(is_ngs);
        buf.write_all(&write_utf16(&self.nickname, 0x20)).unwrap();
        buf.extend(std::iter::once(0).cycle().take(0x20));
        buf
    }
}

impl PacketReadWrite for SystemInformationPacket {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let cpu_info = read_variable_utf8(reader, 0x9F, 0x883D);
        let video_info = read_variable_utf8(reader, 0x9F, 0x883D);
        let vram = reader.read_u64::<LittleEndian>()?;
        let total_ram = reader.read_u64::<LittleEndian>()?;
        let unk1 = reader.read_u32::<LittleEndian>()?;
        let unk2 = reader.read_u32::<LittleEndian>()?;
        let windows_version = read_variable_utf16(reader, 0x9F, 0x883D);
        let window_size = read_variable_utf8(reader, 0x9F, 0x883D);
        let unk3 = read_variable_utf16(reader, 0x9F, 0x883D);
        let unk4 = read_variable_utf16(reader, 0x9F, 0x883D);
        let video_driver = read_variable_utf16(reader, 0x9F, 0x883D);
        let total_disk_space = reader.read_u64::<LittleEndian>()?;
        let free_disk_space = reader.read_u64::<LittleEndian>()?;
        Ok(Self {
            cpu_info,
            video_info,
            vram,
            total_ram,
            unk1,
            unk2,
            windows_version,
            window_size,
            unk3,
            unk4,
            video_driver,
            total_disk_space,
            free_disk_space,
        })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(
            0x11,
            0x2D,
            Flags {
                packed: true,
                ..Default::default()
            },
        )
        .write(is_ngs);
        buf.write_all(&write_variable_utf8(&self.cpu_info, 0x9F, 0x883D))
            .unwrap();
        buf.write_all(&write_variable_utf8(&self.video_info, 0x9F, 0x883D))
            .unwrap();
        buf.write_u64::<LittleEndian>(self.vram).unwrap();
        buf.write_u64::<LittleEndian>(self.total_ram).unwrap();
        buf.write_u32::<LittleEndian>(self.unk1).unwrap();
        buf.write_u32::<LittleEndian>(self.unk2).unwrap();
        buf.write_all(&write_variable_utf16(&self.windows_version, 0x9F, 0x883D))
            .unwrap();
        buf.write_all(&write_variable_utf8(&self.window_size, 0x9F, 0x883D))
            .unwrap();
        buf.write_all(&write_variable_utf16(&self.unk3, 0x9F, 0x883D))
            .unwrap();
        buf.write_all(&write_variable_utf16(&self.unk4, 0x9F, 0x883D))
            .unwrap();
        buf.write_all(&write_variable_utf16(&self.video_driver, 0x9F, 0x883D))
            .unwrap();
        buf.write_u64::<LittleEndian>(self.total_disk_space)
            .unwrap();
        buf.write_u64::<LittleEndian>(self.free_disk_space).unwrap();
        buf
    }
}

impl PacketReadWrite for ShipListPacket {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let ship_count = read_magic(reader, 0x51, 0xE418)?;
        let mut ships = vec![];
        for _ in 0..ship_count {
            ships.push(ShipEntry::read(reader)?);
        }
        let timestamp = Duration::from_secs(reader.read_u32::<LittleEndian>()? as u64);
        let _ = reader.read_u32::<LittleEndian>()?;
        Ok(Self { ships, timestamp })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(
            0x11,
            0x3D,
            Flags {
                packed: true,
                ..Default::default()
            },
        )
        .write(is_ngs);
        buf.write_u32::<LittleEndian>(write_magic(self.ships.len() as u32, 0x51, 0xE418))
            .unwrap();
        for ship in self.ships {
            ship.write(&mut buf).unwrap()
        }
        buf.write_u32::<LittleEndian>(self.timestamp.as_secs() as u32)
            .unwrap();
        buf.write_u32::<LittleEndian>(1).unwrap();
        buf
    }
}

impl PacketReadWrite for CreateCharacter1ResponsePacket {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let status = reader.read_u32::<LittleEndian>()?;
        let unk2 = reader.read_u32::<LittleEndian>()?;
        let used_smth = reader.read_u32::<LittleEndian>()?;
        let req_ac = reader.read_u32::<LittleEndian>()?;
        Ok(Self {
            status,
            unk2,
            used_smth,
            req_ac,
        })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(0x11, 0x42, Flags::default()).write(is_ngs);
        buf.write_u32::<LittleEndian>(self.status).unwrap();
        buf.write_u32::<LittleEndian>(self.unk2).unwrap();
        buf.write_u32::<LittleEndian>(self.used_smth).unwrap();
        buf.write_u32::<LittleEndian>(self.req_ac).unwrap();
        buf
    }
}

impl PacketReadWrite for CreateCharacter2ResponsePacket {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let unk = reader.read_u32::<LittleEndian>()?;
        Ok(Self { unk })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(0x11, 0x55, Flags::default()).write(is_ngs);
        buf.write_u32::<LittleEndian>(self.unk).unwrap();
        buf
    }
}

impl PacketReadWrite for VitaLoginPacket {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let unk1 = reader.read_u8()?;
        let unk2 = reader.read_u8()?;
        let unk3 = reader.read_u16::<LittleEndian>()?;
        let unk4 = reader.read_u32::<LittleEndian>()?;
        let unk5 = reader.read_u32::<LittleEndian>()?;
        let mut ver_id = [0u8; 0x20];
        reader.read_exact(&mut ver_id)?;
        let mac_amount = read_magic(reader, 0x77, 0xBE3F)? as usize;
        let mut interfaces = Vec::with_capacity(mac_amount);
        for _ in 0..mac_amount {
            interfaces.push(NetInterface::read(reader)?);
        }
        let mut unk6 = [0u8; 0x10];
        reader.read_exact(&mut unk6)?;
        reader.seek(std::io::SeekFrom::Current(0x4))?;
        let mut unk7 = [0u8; 0x90];
        reader.read_exact(&mut unk7)?;
        reader.seek(std::io::SeekFrom::Current(0x10))?;
        let mut unk8 = [0u8; 0x10];
        reader.read_exact(&mut unk8)?;
        reader.seek(std::io::SeekFrom::Current(0x10))?;
        let flag1 = reader.read_u32::<LittleEndian>()?;
        let flag2 = reader.read_u32::<LittleEndian>()?;
        let flag3 = reader.read_u32::<LittleEndian>()?;
        let flag4 = reader.read_u32::<LittleEndian>()?;
        let flag5 = reader.read_u32::<LittleEndian>()?;
        let flag6 = reader.read_u32::<LittleEndian>()?;
        let language = read_utf16(reader, 0x10);
        let unk9 = reader.read_u32::<LittleEndian>()?;
        let unk10 = reader.read_u32::<LittleEndian>()?;
        let magic1 = reader.read_u32::<LittleEndian>()?;
        let mut unk11 = [0u8; 0x20];
        reader.read_exact(&mut unk11)?;
        let mut unk12 = [0u8; 0x44];
        reader.read_exact(&mut unk12)?;
        unk12.iter_mut().map(|x| *x ^= 0x15).count();
        reader.seek(std::io::SeekFrom::Current(0xFC))?;
        let username = read_utf8(reader, 0x40);
        reader.seek(std::io::SeekFrom::Current(0x20))?;
        let password = read_utf8(reader, 0x40);
        reader.seek(std::io::SeekFrom::Current(0x4))?;
        let unk13 = reader.read_u8()?;
        let unk14 = reader.read_u8()?;
        let unk15 = reader.read_u16::<LittleEndian>()?;
        let unk16 = read_variable_utf8(reader, 0x77, 0xBE3F);
        let unk17_magic = read_magic(reader, 0x77, 0xBE3F)? as usize;
        let mut unk17 = vec![0u8; unk17_magic];
        reader.read_exact(&mut unk17)?;
        let mut unk18 = [0u8; 0x10];
        reader.read_exact(&mut unk18)?;
        Ok(Self {
            unk1,
            unk2,
            unk3,
            unk4,
            unk5,
            ver_id,
            interfaces,
            unk6,
            unk7,
            unk8,
            flag1,
            flag2,
            flag3,
            flag4,
            flag5,
            flag6,
            language,
            unk9,
            unk10,
            magic1,
            unk11,
            unk12,
            username,
            password,
            unk13,
            unk14,
            unk15,
            unk16,
            unk17,
            unk18,
        })
    }
    fn write(mut self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(
            0x11,
            0x63,
            Flags {
                packed: true,
                ..Default::default()
            },
        )
        .write(is_ngs);
        buf.write_u8(self.unk1).unwrap();
        buf.write_u8(self.unk2).unwrap();
        buf.write_u16::<LittleEndian>(self.unk3).unwrap();
        buf.write_u32::<LittleEndian>(self.unk4).unwrap();
        buf.write_u32::<LittleEndian>(self.unk5).unwrap();
        buf.write_all(&self.ver_id).unwrap();
        buf.write_u32::<LittleEndian>(write_magic(self.interfaces.len() as u32, 0x77, 0xBE3F))
            .unwrap();
        for interface in self.interfaces.iter_mut() {
            interface.write(&mut buf).unwrap();
        }
        buf.write_all(&self.unk6).unwrap();
        buf.write_all(&[0u8; 0x4]).unwrap();
        buf.write_all(&self.unk7).unwrap();
        buf.write_all(&[0u8; 0x10]).unwrap();
        buf.write_all(&self.unk8).unwrap();
        buf.write_all(&[0u8; 0x10]).unwrap();
        buf.write_u32::<LittleEndian>(self.flag1).unwrap();
        buf.write_u32::<LittleEndian>(self.flag2).unwrap();
        buf.write_u32::<LittleEndian>(self.flag3).unwrap();
        buf.write_u32::<LittleEndian>(self.flag4).unwrap();
        buf.write_u32::<LittleEndian>(self.flag5).unwrap();
        buf.write_u32::<LittleEndian>(self.flag6).unwrap();
        buf.write_all(&write_utf16(&self.language, 0x10)).unwrap();
        buf.write_u32::<LittleEndian>(self.unk9).unwrap();
        buf.write_u32::<LittleEndian>(self.unk10).unwrap();
        buf.write_u32::<LittleEndian>(self.magic1).unwrap();
        buf.write_all(&self.unk11).unwrap();
        self.unk12.iter_mut().map(|x| *x ^= 0x15).count();
        buf.write_all(&self.unk12).unwrap();
        buf.write_all(&[0u8; 0xFC]).unwrap();
        buf.write_all(&write_utf8(&self.username, 0x40)).unwrap();
        buf.write_all(&[0u8; 0x20]).unwrap();
        buf.write_all(&write_utf8(&self.password, 0x40)).unwrap();
        buf.write_all(&[0u8; 0x4]).unwrap();
        buf.write_u8(self.unk13).unwrap();
        buf.write_u8(self.unk14).unwrap();
        buf.write_u16::<LittleEndian>(self.unk15).unwrap();
        buf.write_all(&write_variable_utf8(&self.unk16, 0x77, 0xBE3F))
            .unwrap();
        buf.write_u32::<LittleEndian>(write_magic(self.unk17.len() as u32, 0x77, 0xBE3F))
            .unwrap();
        buf.write_all(&self.unk17).unwrap();
        buf.write_all(&self.unk18).unwrap();
        buf
    }
}

impl PacketReadWrite for LoginHistoryPacket {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let num = read_magic(reader, 8, 0x8ceb)?;
        let mut attempts = vec![];
        for _ in 0..num {
            attempts.push(LoginAttempt::read(reader)?);
        }

        Ok(Self { attempts })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(
            0x11,
            0x87,
            Flags {
                packed: true,
                ..Default::default()
            },
        )
        .write(is_ngs);
        buf.write_u32::<LittleEndian>(write_magic(
            (self.attempts.len() as u32).clamp(0, 50),
            8,
            0x8ceb,
        ))
        .unwrap();
        for attempt in self.attempts.iter().take(50) {
            attempt.write(&mut buf).unwrap();
        }
        buf
    }
}

impl PacketReadWrite for NicknameErrorPacket {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let unk1 = reader.read_u32::<LittleEndian>()?;
        let nickname = read_variable_utf16(reader, 0x14, 0x4544);

        Ok(Self { unk1, nickname })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(
            0x11,
            0xEA,
            Flags {
                packed: true,
                ..Default::default()
            },
        )
        .write(is_ngs);
        buf.write_u32::<LittleEndian>(self.unk1).unwrap();
        buf.write_all(&write_variable_utf16(&self.nickname, 0x14, 0x4544))
            .unwrap();
        buf
    }
}

impl PacketReadWrite for EmailCodeRequestPacket {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let unk1 = reader.read_u32::<LittleEndian>()?;
        let message = read_variable_utf16(reader, 0x40, 0x5C3B);

        Ok(Self { unk1, message })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(
            0x11,
            0xEE,
            Flags {
                packed: true,
                ..Default::default()
            },
        )
        .write(is_ngs);
        buf.write_u32::<LittleEndian>(self.unk1).unwrap();
        buf.write_all(&write_variable_utf16(&self.message, 0x40, 0x5C3B))
            .unwrap();
        buf
    }
}

impl PacketReadWrite for Unk1Packet {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let unk1 = reader.read_u8()?;
        let unk2 = reader.read_u8()?;
        let unk3 = reader.read_u8()?;
        let unk4 = reader.read_u8()?;
        let unk5 = read_variable_utf16(reader, 0x3D, 0x3DD3);
        let mut unk6 = [0; 0xC];
        reader.read_exact(&mut unk6)?;
        let mut unk7 = [0; 0x40];
        reader.read_exact(&mut unk7)?;
        let mut unk8 = [0; 0x20];
        reader.read_exact(&mut unk8)?;

        Ok(Self {
            unk1,
            unk2,
            unk3,
            unk4,
            unk5,
            unk6,
            unk7,
            unk8,
        })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(
            0x11,
            0xFF,
            Flags {
                packed: true,
                ..Default::default()
            },
        )
        .write(is_ngs);
        buf.write_u8(self.unk1).unwrap();
        buf.write_u8(self.unk2).unwrap();
        buf.write_u8(self.unk3).unwrap();
        buf.write_u8(self.unk4).unwrap();
        buf.write_all(&write_variable_utf16(&self.unk5, 0x3D, 0x3DD3))
            .unwrap();
        buf.write_all(&self.unk6).unwrap();
        buf.write_all(&self.unk7).unwrap();
        buf.write_all(&self.unk8).unwrap();

        buf
    }
}

impl NetInterface {
    fn read(reader: &mut impl Read) -> std::io::Result<Self> {
        let state = reader.read_u32::<LittleEndian>()?;
        let mac = read_utf8(reader, 0x18);
        Ok(Self { state, mac })
    }
    fn write(&mut self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write_u32::<LittleEndian>(self.state)?;
        writer.write_all(&write_utf8(&self.mac, 0x18))?;
        Ok(())
    }
}

impl ShipEntry {
    pub(crate) fn read(reader: &mut impl Read) -> std::io::Result<Self> {
        let id = reader.read_u32::<LittleEndian>()?;
        let name = read_utf16(reader, 0x10);
        let mut ip = [0u8; 4];
        reader.read_exact(&mut ip)?;
        let _ = reader.read_u32::<LittleEndian>()?;
        let status = ShipStatus::read(reader.read_u16::<LittleEndian>()?);
        let order = reader.read_u16::<LittleEndian>()?;
        let _ = reader.read_u32::<LittleEndian>()?;

        Ok(Self {
            id,
            name,
            ip,
            status,
            order,
        })
    }
    pub(crate) fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write_u32::<LittleEndian>(self.id)?;
        writer.write_all(&write_utf16(&self.name, 0x10))?;
        writer.write_all(&self.ip)?;
        writer.write_u32::<LittleEndian>(0)?;
        writer.write_u16::<LittleEndian>(self.status as u16)?;
        writer.write_u16::<LittleEndian>(self.order)?;
        writer.write_u32::<LittleEndian>(0)?;

        Ok(())
    }
}

impl ShipStatus {
    fn read(num: u16) -> Self {
        match num {
            1 => Self::Online,
            2 => Self::Busy,
            3 => Self::Full,
            4 => Self::Offline,
            _ => Self::Unknown,
        }
    }
}

impl LoginAttempt {
    pub(crate) fn read(reader: &mut impl Read) -> std::io::Result<Self> {
        let mut ip_buf = [0u8; 4];
        reader.read_exact(&mut ip_buf)?;
        let ip = Ipv4Addr::from(ip_buf);
        let status = LoginResult::read(reader.read_u32::<LittleEndian>()?);
        let timestamp = Duration::from_secs(reader.read_u32::<LittleEndian>()? as u64);
        let unk = reader.read_u32::<LittleEndian>()?;
        Ok(Self {
            ip,
            status,
            timestamp,
            unk,
        })
    }
    pub(crate) fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write_all(&self.ip.octets())?;
        writer.write_u32::<LittleEndian>(self.status as u32)?;
        writer.write_u32::<LittleEndian>(self.timestamp.as_secs() as u32)?;
        writer.write_u32::<LittleEndian>(self.unk)?;

        Ok(())
    }
}

impl LoginResult {
    pub(crate) fn read(num: u32) -> Self {
        match num {
            0 => Self::Successful,
            1 => Self::EmailConfirmed,
            2 => Self::LoginError,
            3 => Self::EmailAuthError,
            4 => Self::AuthEmailSent,
            5 => Self::OTPError,
            6 => Self::InMaintenance,
            _ => Self::GenericError,
        }
    }
}

// ----------------------------------------------------------------
// Default implementations
// ----------------------------------------------------------------

impl Default for SegaIDLoginPacket {
    fn default() -> Self {
        Self {
            unk1: 0,
            unk2: 9,
            unk3: 0,
            ver_id: [0u8; 0x20],
            interfaces: vec![],
            unk4: [0u8; 0x90],
            unk5: [0u8; 0x10],
            flag1: 0,
            flag2: 0,
            flag3: 0,
            flag4: 0,
            language: String::new(),
            unk6: 7,
            unk7: 7,
            magic1: 0x0419,
            unk8: [0u8; 0x20],
            unk9: [0u8; 0x44],
            username: String::new(),
            password: String::new(),
            unk10: 512,
            unk11: 0x058A,
        }
    }
}

impl Default for LoginResponsePacket {
    fn default() -> Self {
        Self {
            status: true,
            error: String::new(),
            player_id: 0,
            blockname: String::new(),
            unk1: 60.0,
            unk2: 7,
            unk3: 0xA,
            unk4: 1,
            unk5: 10.0,
            unk6: 5.0,
            unk7: 11,
            unk8: 1.0,
            unk9: 75.0,
            unk10: 40,
            unk11: 10.0,
            unk12: 1,
            unk13: 100.0,
            unk14: [1.0; 0xA],
            unk15: [100.0; 0x15],
            unk16: 0x91A2B,
            unk17: 0x91A2B,
        }
    }
}

impl Default for ClientPingPacket {
    fn default() -> Self {
        Self {
            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
        }
    }
}

impl Default for ClientPongPacket {
    fn default() -> Self {
        Self {
            client_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
            server_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
        }
    }
}

impl Default for ShipListPacket {
    fn default() -> Self {
        Self {
            ships: vec![],
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap(),
        }
    }
}

impl Default for VitaLoginPacket {
    fn default() -> Self {
        Self {
            unk1: 0,
            unk2: 0,
            unk3: 0,
            unk4: 9,
            unk5: 0,
            ver_id: [0u8; 0x20],
            interfaces: vec![],
            unk6: [0u8; 0x10],
            unk7: [0u8; 0x90],
            unk8: [0u8; 0x10],
            flag1: 0,
            flag2: 0,
            flag3: 0,
            flag4: 0,
            flag5: 0,
            flag6: 0,
            language: String::new(),
            unk9: 0,
            unk10: 0,
            magic1: 0,
            unk11: [0u8; 0x20],
            unk12: [0u8; 0x44],
            username: String::new(),
            password: String::new(),
            unk13: 0,
            unk14: 2,
            unk15: 0,
            unk16: String::new(),
            unk17: vec![],
            unk18: [0u8; 0x10],
        }
    }
}

impl Default for NicknameErrorPacket {
    fn default() -> Self {
        Self {
            unk1: 2,
            nickname: String::new(),
        }
    }
}

impl Default for Unk1Packet {
    fn default() -> Self {
        Self {
            unk1: 0,
            unk2: 0,
            unk3: 0,
            unk4: 0,
            unk5: String::new(),
            unk6: [0; 0xC],
            unk7: [0; 0x40],
            unk8: [0; 0x20],
        }
    }
}

impl Default for LoginAttempt {
    fn default() -> Self {
        Self {
            ip: Ipv4Addr::UNSPECIFIED,
            status: LoginResult::Successful,
            timestamp: Duration::new(0, 0),
            unk: 9,
        }
    }
}

// ----------------------------------------------------------------
// Tests
// ----------------------------------------------------------------

// Probably macroable
#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::protocol::{
        login::{
            ClientPingPacket, ClientPongPacket, LoginAttempt, LoginHistoryPacket, ShipEntry,
            ShipListPacket,
        },
        Packet,
    };
    #[test]
    fn check_11_00() {
        let packet = Packet::SegaIDLogin(Default::default());
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_01() {
        let packet = Packet::LoginResponse(Default::default());
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_02() {
        let packet = Packet::CharacterListRequest;
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_03() {
        let packet = Packet::CharacterListResponse(Default::default());
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_05() {
        let packet = Packet::CharacterCreate(Default::default());
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_0b() {
        let packet = Packet::EncryptionRequest(Default::default());
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_0c() {
        let packet = Packet::EncryptionResponse(Default::default());
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_0d() {
        let packet = Packet::ClientPing(ClientPingPacket {
            time: Duration::from_secs(100),
        });
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_0e() {
        let packet = Packet::ClientPong(ClientPongPacket {
            client_time: Duration::from_secs(200),
            server_time: Duration::from_secs(300),
        });
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_1e() {
        let packet = Packet::NicknameRequest(Default::default());
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_1d() {
        let packet = Packet::NicknameResponse(Default::default());
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_2b() {
        let packet = Packet::ClientGoodbye;
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_2d() {
        let packet = Packet::SystemInformation(Default::default());
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_3d() {
        let packet = Packet::ShipList(ShipListPacket {
            ships: vec![ShipEntry::default(); 4],
            timestamp: Duration::from_secs(200),
        });
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_41() {
        let packet = Packet::CreateCharacter1;
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_42() {
        let packet = Packet::CreateCharacter1Response(Default::default());
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_54() {
        let packet = Packet::CreateCharacter2;
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_55() {
        let packet = Packet::CreateCharacter2Response(Default::default());
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_63() {
        let packet = Packet::VitaLogin(Default::default());
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_6b() {
        let packet = Packet::SegaIDInfoRequest;
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_86() {
        let packet = Packet::LoginHistoryRequest;
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_87() {
        let packet = Packet::LoginHistoryResponse(LoginHistoryPacket {
            attempts: vec![LoginAttempt::default(); 50],
        });
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_ea() {
        let packet = Packet::NicknameError(Default::default());
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_ee() {
        let packet = Packet::EmailCodeRequest(Default::default());
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
    #[test]
    fn check_11_ff() {
        let packet = Packet::Unk1(Default::default());
        let data = packet.clone().write(false);
        let packet2 = Packet::read(&data, false).unwrap();
        assert_eq!(packet, packet2.1[0]);
    }
}
