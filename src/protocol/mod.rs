pub mod login;
pub mod models;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use login::*;
use packetlib_impl::PacketReadWrite;
use std::{
    io::{Cursor, Read, Seek, Write},
    time::Duration,
};

pub(crate) trait PacketReadWrite {
    /// Read a packet from stream.
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self>
    where
        Self: Sized;
    /// Write a packet to a Vec.
    fn write(self, is_ngs: bool) -> Vec<u8>;
}

pub(crate) trait HelperReadWrite {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self>
    where
        Self: Sized;
    fn write(self, writer: &mut impl Write) -> std::io::Result<()>;
}

#[derive(Debug, Default, Clone, PartialEq)]
#[non_exhaustive]
pub enum Packet {
    #[default]
    None,
    // 0x03, 0x08
    ServerHello(ServerHelloPacket),
    // 0x03, 0x0B
    ServerPing,
    // 0x03, 0x0C
    ServerPong,

    // Login packets [0x11]
    // 0x11, 0x00
    SegaIDLogin(SegaIDLoginPacket),
    // 0x11, 0x01
    LoginResponse(LoginResponsePacket),
    // 0x11, 0x02
    CharacterListRequest,
    // 0x11, 0x03
    CharacterListResponse(CharacterListPacket),
    // 0x11, 0x05
    CharacterCreate(CharacterCreatePacket),
    // 0x11, 0x0B
    EncryptionRequest(EncryptionRequestPacket),
    // 0x11, 0x0C
    EncryptionResponse(EncryptionResponsePacket),
    // 0x11, 0x0D
    ClientPing(ClientPingPacket),
    // 0x11, 0x0E
    ClientPong(ClientPongPacket),
    // 0x11, 0x1E
    NicknameRequest(NicknameRequestPacket),
    // 0x11, 0x1D
    NicknameResponse(NicknameResponsePacket),
    // 0x11, 0x2B
    ClientGoodbye,
    // 0x11, 0x2D
    SystemInformation(SystemInformationPacket),
    // 0x11, 0x3D
    ShipList(ShipListPacket),
    // 0x11, 0x41
    CreateCharacter1,
    // 0x11, 0x42
    CreateCharacter1Response(CreateCharacter1ResponsePacket),
    // 0x11, 0x54
    CreateCharacter2,
    // 0x11, 0x55
    CreateCharacter2Response(CreateCharacter2ResponsePacket),
    // 0x11, 0x63
    VitaLogin(VitaLoginPacket),
    // 0x11, 0x6B
    SegaIDInfoRequest,
    // 0x11, 0x86
    LoginHistoryRequest,
    // 0x11, 0x87
    LoginHistoryResponse(LoginHistoryPacket),
    // 0x11, 0xEA
    NicknameError(NicknameErrorPacket),
    // 0x11, 0xEE
    EmailCodeRequest(EmailCodeRequestPacket),
    // 0x11, 0xFF
    Unk1(Unk1Packet),

    // 0x19, 0x01
    SystemMessage(SystemMessagePacket),

    //Settings packets [0x2B]
    // 0x2B, 0x00
    SettingsRequest,
    // 0x2B, 0x01
    SaveSettings(SaveSettingsPacket),
    // 0x2B, 0x02
    LoadSettings(LoadSettingsPacket),

    //Other packets
    Encrypted(Vec<u8>),
    Unknown((PacketHeader, Vec<u8>)),
    UnknownNGS((PacketHeader, Vec<u8>)),
}

impl Packet {
    pub fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = vec![];
        buf.write_u32::<LittleEndian>(0).unwrap();
        buf.extend(match self {
            Self::None => {
                return vec![];
            }

            Self::ServerHello(packet) => packet.write(is_ngs),
            Self::ServerPing => PacketHeader::new(0x03, 0x0B, Flags::default()).write(is_ngs),
            Self::ServerPong => PacketHeader::new(0x03, 0x0C, Flags::default()).write(is_ngs),

            //Login packets
            Self::SegaIDLogin(packet) => packet.write(is_ngs),
            Self::LoginResponse(packet) => packet.write(is_ngs),
            Self::CharacterListRequest => {
                PacketHeader::new(0x11, 0x02, Flags::default()).write(is_ngs)
            }
            Self::CharacterListResponse(packet) => packet.write(is_ngs),
            Self::CharacterCreate(packet) => packet.write(is_ngs),
            Self::EncryptionRequest(packet) => packet.write(is_ngs),
            Self::EncryptionResponse(packet) => packet.write(is_ngs),
            Self::ClientPing(packet) => packet.write(is_ngs),
            Self::ClientPong(packet) => packet.write(is_ngs),
            Self::NicknameRequest(packet) => packet.write(is_ngs),
            Self::NicknameResponse(packet) => packet.write(is_ngs),
            Self::ClientGoodbye => PacketHeader::new(0x11, 0x2B, Flags::default()).write(is_ngs),
            Self::SystemInformation(packet) => packet.write(is_ngs),
            Self::ShipList(packet) => packet.write(is_ngs),
            Self::CreateCharacter1 => PacketHeader::new(0x11, 0x41, Flags::default()).write(is_ngs),
            Self::CreateCharacter1Response(packet) => packet.write(is_ngs),
            Self::CreateCharacter2 => PacketHeader::new(0x11, 0x54, Flags::default()).write(is_ngs),
            Self::CreateCharacter2Response(packet) => packet.write(is_ngs),
            Self::VitaLogin(packet) => packet.write(is_ngs),
            Self::SegaIDInfoRequest => {
                PacketHeader::new(0x11, 0x6B, Flags::default()).write(is_ngs)
            }
            Self::LoginHistoryRequest => {
                PacketHeader::new(0x11, 0x86, Flags::default()).write(is_ngs)
            }
            Self::LoginHistoryResponse(packet) => packet.write(is_ngs),
            Self::NicknameError(packet) => packet.write(is_ngs),
            Self::EmailCodeRequest(packet) => packet.write(is_ngs),
            Self::Unk1(packet) => packet.write(is_ngs),

            Self::SystemMessage(packet) => packet.write(is_ngs),

            //Settings packets
            Self::SettingsRequest => PacketHeader::new(0x2B, 0x00, Flags::default()).write(is_ngs),
            Self::SaveSettings(packet) => packet.write(is_ngs),
            Self::LoadSettings(packet) => packet.write(is_ngs),

            //Other packets
            Self::Encrypted(data) => return data,
            Self::Unknown(data) => {
                let mut out_data = data.0.write(is_ngs);
                out_data.extend_from_slice(&data.1);
                out_data
            }
            Self::UnknownNGS(data) => {
                let mut out_data = vec![];
                out_data.extend_from_slice(&data.1);
                out_data
            }
        });
        let len = (buf.len() as u32).to_le_bytes();
        buf[..4].copy_from_slice(&len);
        buf
    }
    pub fn read(input: &[u8], is_ngs: bool) -> std::io::Result<(usize, Vec<Self>)> {
        let mut packets: Vec<Self> = vec![];
        let buffer_length = input.len();
        let mut pointer = 0;
        loop {
            if input[pointer..].len() <= 4 {
                return Ok((pointer, packets));
            }
            if pointer >= buffer_length {
                break;
            }
            let len = (&input[pointer..pointer + 4]).read_u32::<LittleEndian>()? as usize - 4;
            pointer += 4;
            if input[pointer..].len() < len {
                return Ok((pointer - 4, packets));
            }
            let mut buf_tmp = Cursor::new(&input[pointer..pointer + len]);
            let header = PacketHeader::read(&mut buf_tmp, is_ngs)?;
            pointer += len;
            // println!("{:?}", (header.id, header.subid));
            match (header.id, header.subid) {
                (0x11, 0x0B) => {
                    packets.push(Self::EncryptionRequest(EncryptionRequestPacket::read(
                        &mut buf_tmp,
                    )?));
                }
                (0x11, 0x0C) => {
                    packets.push(Self::EncryptionResponse(EncryptionResponsePacket::read(
                        &mut buf_tmp,
                    )?));
                }
                (_, _) if is_ngs => {
                    packets.push(Self::UnknownNGS({
                        let mut data = vec![];
                        buf_tmp.set_position(0);
                        buf_tmp.read_to_end(&mut data)?;
                        (header, data)
                    }));
                }

                (0x03, 0x08) => {
                    packets.push(Self::ServerHello(ServerHelloPacket::read(&mut buf_tmp)?))
                }
                (0x03, 0x0B) => {
                    packets.push(Self::ServerPing);
                }
                (0x03, 0x0C) => {
                    packets.push(Self::ServerPong);
                }

                //Login packets
                (0x11, 0x00) => {
                    packets.push(Self::SegaIDLogin(SegaIDLoginPacket::read(&mut buf_tmp)?));
                }
                (0x11, 0x01) => {
                    if is_ngs {
                        packets.push(Self::Unknown({
                            let mut data = vec![];
                            buf_tmp.read_to_end(&mut data)?;
                            (header, data)
                        }));
                    } else {
                        packets.push(Self::LoginResponse(LoginResponsePacket::read(
                            &mut buf_tmp,
                        )?));
                    }
                }
                (0x11, 0x02) => {
                    packets.push(Self::CharacterListRequest);
                }
                (0x11, 0x03) => {
                    packets.push(Self::CharacterListResponse(CharacterListPacket::read(
                        &mut buf_tmp,
                    )?));
                }
                (0x11, 0x05) => {
                    packets.push(Self::CharacterCreate(CharacterCreatePacket::read(
                        &mut buf_tmp,
                    )?));
                }
                (0x11, 0x0D) => {
                    packets.push(Self::ClientPing(ClientPingPacket::read(&mut buf_tmp)?));
                }
                (0x11, 0x0E) => {
                    packets.push(Self::ClientPong(ClientPongPacket::read(&mut buf_tmp)?));
                }
                (0x11, 0x1E) => {
                    packets.push(Self::NicknameRequest(NicknameRequestPacket::read(
                        &mut buf_tmp,
                    )?));
                }
                (0x11, 0x1D) => {
                    packets.push(Self::NicknameResponse(NicknameResponsePacket::read(
                        &mut buf_tmp,
                    )?));
                }
                (0x11, 0x2B) => {
                    packets.push(Self::ClientGoodbye);
                }
                (0x11, 0x2D) => {
                    packets.push(Self::SystemInformation(SystemInformationPacket::read(
                        &mut buf_tmp,
                    )?));
                }
                (0x11, 0x3D) => {
                    packets.push(Self::ShipList(ShipListPacket::read(&mut buf_tmp)?));
                }
                (0x11, 0x41) => {
                    packets.push(Self::CreateCharacter1);
                }
                (0x11, 0x42) => {
                    packets.push(Self::CreateCharacter1Response(
                        CreateCharacter1ResponsePacket::read(&mut buf_tmp)?,
                    ));
                }
                (0x11, 0x54) => {
                    packets.push(Self::CreateCharacter2);
                }
                (0x11, 0x55) => {
                    packets.push(Self::CreateCharacter2Response(
                        CreateCharacter2ResponsePacket::read(&mut buf_tmp)?,
                    ));
                }
                (0x11, 0x63) => {
                    packets.push(Self::VitaLogin(VitaLoginPacket::read(&mut buf_tmp)?));
                }
                (0x11, 0x6B) => {
                    packets.push(Self::SegaIDInfoRequest);
                }
                (0x11, 0x86) => {
                    packets.push(Self::LoginHistoryRequest);
                }
                (0x11, 0x87) => {
                    packets.push(Self::LoginHistoryResponse(LoginHistoryPacket::read(
                        &mut buf_tmp,
                    )?));
                }
                (0x11, 0xEA) => {
                    packets.push(Self::NicknameError(NicknameErrorPacket::read(
                        &mut buf_tmp,
                    )?));
                }
                (0x11, 0xEE) => {
                    packets.push(Self::EmailCodeRequest(EmailCodeRequestPacket::read(
                        &mut buf_tmp,
                    )?));
                }
                (0x11, 0xFF) => {
                    packets.push(Self::Unk1(Unk1Packet::read(&mut buf_tmp)?));
                }

                (0x19, 0x01) => {
                    packets.push(Self::SystemMessage(SystemMessagePacket::read(
                        &mut buf_tmp,
                    )?));
                }

                //Settings packets
                (0x2B, 0x00) => packets.push(Self::SettingsRequest),
                (0x2B, 0x01) => {
                    packets.push(Self::SaveSettings(SaveSettingsPacket::read(&mut buf_tmp)?));
                }
                (0x2B, 0x02) => {
                    packets.push(Self::LoadSettings(LoadSettingsPacket::read(&mut buf_tmp)?));
                }

                //Other packets
                _ => {
                    packets.push(Self::Unknown({
                        let mut data = vec![];
                        buf_tmp.read_to_end(&mut data)?;
                        (header, data)
                    }));
                }
            }
        }

        Ok((pointer, packets))
    }
}

// ----------------------------------------------------------------
// Common structures
// ----------------------------------------------------------------

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PacketHeader {
    pub id: u8,
    pub subid: u16,
    pub flag1: Flags,
    pub flag2: Flags,
}
impl PacketHeader {
    fn new(id: u8, subid: u16, flag1: Flags) -> Self {
        Self {
            id,
            subid,
            flag1,
            flag2: Flags::default(),
        }
    }
    fn read(reader: &mut (impl Read + Seek), is_ngs: bool) -> std::io::Result<Self> {
        let (id, subid, flag1, flag2) = if !is_ngs {
            let id = reader.read_u8()?;
            let subid = reader.read_u8()? as u16;
            let flag1 = Flags::read(reader)?;
            let flag2 = Flags::read(reader)?;
            (id, subid, flag1, flag2)
        } else {
            let flag1 = Flags::read(reader)?;
            let id = reader.read_u8()?;
            let subid = reader.read_u16::<LittleEndian>()?;
            let flag2 = Flags::default();
            (id, subid, flag1, flag2)
        };

        Ok(Self {
            id,
            subid,
            flag1,
            flag2,
        })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = vec![];
        if !is_ngs {
            buf.write_u8(self.id).unwrap();
            buf.write_u8(self.subid as u8).unwrap();
            self.flag1.write(&mut buf).unwrap();
            self.flag2.write(&mut buf).unwrap();
        } else {
            self.flag1.write(&mut buf).unwrap();
            buf.write_u8(self.id).unwrap();
            buf.write_u16::<LittleEndian>(self.subid).unwrap();
            // buf.write_u8(self.flag2 as u8).unwrap();
        }
        buf
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Flags {
    pub packed: bool,
    pub flag10: bool,
    pub full_movement: bool,
    pub object_related: bool,
}
impl HelperReadWrite for Flags {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let mut flags = Self::default();
        let mut num = reader.read_u8()?;
        if num & 0x40 != 0 {
            flags.object_related = true;
            num -= 0x40;
        }
        if num & 0x20 != 0 {
            flags.full_movement = true;
            num -= 0x20;
        }
        if num & 0x10 != 0 {
            flags.flag10 = true;
            num -= 0x10;
        }
        if num & 0x4 != 0 {
            flags.packed = true;
            num -= 0x4;
        }
        if num != 0 {
            println!("Unknown flags: {num}");
        }
        Ok(flags)
    }
    fn write(self, writer: &mut impl Write) -> std::io::Result<()> {
        let mut num = 0;
        if self.packed {
            num += 0x4;
        }
        if self.flag10 {
            num += 0x10;
        }
        if self.full_movement {
            num += 0x20;
        }
        if self.object_related {
            num += 0x40;
        }
        writer.write_u8(num)?;
        Ok(())
    }
}

#[repr(u16)]
#[derive(Debug, Default, Clone, PartialEq)]
pub enum EntityType {
    #[default]
    Unknown = 0,
    Player = 4,
    Map = 5,
    Object = 6,
}
impl EntityType {
    fn read(num: u16) -> Self {
        match num {
            4 => Self::Player,
            5 => Self::Map,
            6 => Self::Object,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ObjectHeader {
    pub id: u32,
    pub entity_type: EntityType,
}
impl ObjectHeader {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let id = reader.read_u32::<LittleEndian>()?;
        reader.seek(std::io::SeekFrom::Current(0x4))?;
        let entity_type = EntityType::read(reader.read_u16::<LittleEndian>()?);
        reader.seek(std::io::SeekFrom::Current(0x2))?;
        Ok(Self { id, entity_type })
    }
    fn write(self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write_u32::<LittleEndian>(self.id)?;
        writer.write_all(&[0u8; 0x4])?;
        writer.write_u16::<LittleEndian>(self.entity_type as u16)?;
        writer.write_all(&[0u8; 0x2])?;
        Ok(())
    }
}

// ----------------------------------------------------------------
// Packets
// ----------------------------------------------------------------

// 0x03, 0x08
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x08)]
pub struct ServerHelloPacket {
    #[Const_u16(0x03)]
    #[SeekAfter(8)]
    pub version: u16,
}
impl Default for ServerHelloPacket {
    fn default() -> Self {
        Self { version: 0xc9 }
    }
}

// 0x19, 0x01
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x19, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SystemMessagePacket {
    #[VariableUtf16(0x78F7, 0xA2)]
    pub message: String,
    #[VariableUtf16(0x78F7, 0xA2)]
    pub unk: String,
    pub msg_type: MessageType,
    pub unk2: u32,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum MessageType {
    GoldenTicker,
    AdminMessage,
    AdminMessageInstant,
    SystemMessage,
    #[default]
    GenericMessage,
}
impl MessageType {
    pub(crate) fn read(reader: &mut impl Read) -> std::io::Result<Self> {
        let num = reader.read_u32::<LittleEndian>()?;
        Ok(match num {
            0 => Self::GoldenTicker,
            1 => Self::AdminMessage,
            2 => Self::AdminMessageInstant,
            3 => Self::SystemMessage,
            4 => Self::GenericMessage,
            _ => Self::GenericMessage,
        })
    }
    pub(crate) fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write_u32::<LittleEndian>(*self as u32)?;
        Ok(())
    }
}

// ----------------------------------------------------------------
// Settings packets
// ----------------------------------------------------------------
//

// 0x2B, 0x01
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2B, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct SaveSettingsPacket {
    #[VariableAscii(0xCEF1, 0xB5)]
    pub settings: String,
}

// 0x2B, 0x02
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2B, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct LoadSettingsPacket {
    #[VariableAscii(0x54AF, 0x100)]
    pub settings: String,
}

// ----------------------------------------------------------------
// Utils
// ----------------------------------------------------------------
fn read_magic(reader: &mut impl Read, sub: u32, xor: u32) -> std::io::Result<u32> {
    let num = reader.read_u32::<LittleEndian>()?;
    Ok((num ^ xor) - sub)
}
fn write_magic(num: u32, sub: u32, xor: u32) -> u32 {
    (num + sub) ^ xor
}
fn read_utf16(reader: &mut impl Read, len: u64) -> String {
    let len = len * 2;
    let mut buf = vec![];
    reader.take(len).read_to_end(&mut buf).unwrap();
    let buf = match buf.len() % 2 {
        0 => &buf,
        _ => &buf[..buf.len() - 1],
    };
    let mut words = vec![];
    for word in buf.chunks(2) {
        words.push(u16::from_le_bytes(word.try_into().unwrap()))
    }
    let mut string = String::from_utf16_lossy(&words);
    if let Some(x) = string.find('\0') {
        string.replace_range(x.., "");
    }
    string
}
fn read_variable_utf16(reader: &mut impl Read, sub: u32, xor: u32) -> String {
    let magic_len = read_magic(reader, sub, xor).unwrap() as u64;
    if magic_len == 0 {
        return String::new();
    }
    let len = magic_len;
    let padding = magic_len & 1;
    read_utf16(reader, len + padding)
}
fn write_utf16(string: &str, len: usize) -> Vec<u8> {
    let mut buf = vec![];
    let string = string
        .chars()
        .take(len - 1)
        .chain("\0".chars().cycle())
        .take(len)
        .collect::<String>();
    for word in string.encode_utf16() {
        buf.extend(word.to_le_bytes())
    }
    buf
}
fn write_variable_utf16(string: &str, sub: u32, xor: u32) -> Vec<u8> {
    let mut buf = vec![];
    if string.is_empty() {
        buf.write_u32::<LittleEndian>(write_magic(0, sub, xor))
            .unwrap();
        return buf;
    }
    let length = string.len() + 1;
    let padding = length & 1;
    buf.write_u32::<LittleEndian>(write_magic(length as u32, sub, xor))
        .unwrap();
    buf.write_all(&write_utf16(string, length + padding))
        .unwrap();
    buf
}
fn read_utf8(reader: &mut impl Read, len: u64) -> String {
    let mut buf = vec![];
    reader.take(len).read_to_end(&mut buf).unwrap();
    let mut string = String::from_utf8_lossy(&buf).to_string();
    if let Some(x) = string.find('\0') {
        string.replace_range(x.., "");
    }
    string
}
fn read_variable_utf8(reader: &mut impl Read, sub: u32, xor: u32) -> String {
    let magic_len = read_magic(reader, sub, xor).unwrap() as u64;
    if magic_len == 0 {
        return String::new();
    }
    let len = magic_len - 1;
    let padding = 4 - (len & 3);
    read_utf8(reader, len + padding)
}
fn write_utf8(string: &str, len: usize) -> Vec<u8> {
    let string = string
        .chars()
        .filter(char::is_ascii)
        .take(len - 1)
        .chain("\0".chars().cycle())
        .take(len)
        .collect::<String>();
    let mut buf = vec![];
    buf.extend(string.bytes());
    buf
}
fn write_variable_utf8(string: &str, sub: u32, xor: u32) -> Vec<u8> {
    let mut buf = vec![];
    if string.is_empty() {
        buf.write_u32::<LittleEndian>(write_magic(0, sub, xor))
            .unwrap();
        return buf;
    }
    let length = string.len();
    let padding = 4 - (length & 3);
    buf.write_u32::<LittleEndian>(write_magic(length as u32 + 1, sub, xor))
        .unwrap();
    buf.write_all(&write_utf8(string, length + padding))
        .unwrap();
    buf
}
fn psotime_to_duration(timestamp: u64) -> Duration {
    const UNIX_TIME: u64 = 0x0029_5E96_4886_4000;
    let timestamp = timestamp * 1000;
    Duration::from_micros(timestamp - UNIX_TIME)
}
fn duration_to_psotime(time: Duration) -> u64 {
    const UNIX_TIME: u64 = 0x0029_5E96_4886_4000;
    let timestamp = time.as_micros() as u64 + UNIX_TIME;
    timestamp / 1000
}
