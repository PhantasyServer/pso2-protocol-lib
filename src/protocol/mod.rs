pub mod login;
pub mod models;
pub mod objects;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use login::*;
use objects::*;
use packetlib_impl::{HelperReadWrite, PacketReadWrite};
use std::{
    io::{Cursor, Read, Seek, Write},
    time::Duration,
};

use self::models::{character::Character, Position};

pub(crate) trait PacketReadWrite: Sized {
    /// Read a packet from stream.
    fn read(reader: &mut (impl Read + Seek), flags: Flags) -> std::io::Result<Self>;
    /// Write a packet to a Vec.
    fn write(self, is_ngs: bool) -> Vec<u8>;
}

pub(crate) trait HelperReadWrite: Sized {
    fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self>;
    fn write(self, writer: &mut impl Write) -> std::io::Result<()>;
}

#[derive(Debug, Default, Clone, PartialEq)]
#[non_exhaustive]
pub enum Packet {
    #[default]
    None,
    // 0x03, 0x03
    InitialLoad,
    // 0x03, 0x04
    LoadingScreenTransition,
    // 0x03, 0x08
    ServerHello(ServerHelloPacket),
    // 0x03, 0x0B
    ServerPing,
    // 0x03, 0x0C
    ServerPong,
    // 0x03, 0x23
    FinishLoading,

    // 0x04, 0x07
    Movement(MovementPacket),
    // 0x04, 0x15
    SetTag(SetTagPacket),

    // 0x06, 0x00
    SetPlayerID(SetPlayerIDPacket),

    // 0x07, 0x00
    ChatMessage(ChatMessage),

    // Spawn packets [0x08]
    // 0x08, 0x04
    CharacterSpawn(CharacterSpawnPacket),
    // 0x08, 0x09
    EventSpawn(EventSpawnPacket),
    // 0x08, 0x0B
    ObjectSpawn(ObjectSpawnPacket),
    // 0x08, 0x0C
    NPCSpawn(NPCSpawnPacket),

    // 0x0F, 0x00
    FileTransfer(FileTransferPacket),

    // Login packets [0x11]
    // 0x11, 0x00
    SegaIDLogin(SegaIDLoginPacket),
    // 0x11, 0x01
    LoginResponse(LoginResponsePacket),
    // 0x11, 0x02
    CharacterListRequest,
    // 0x11, 0x03
    CharacterListResponse(CharacterListPacket),
    // 0x11, 0x04
    StartGame(StartGamePacket),
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
    // 0x11, 0x2C
    BlockBalance(BlockBalancePacket),
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
    // 0x11, 0x66
    SalonEntryRequest,
    // 0x11, 0x67
    SalonEntryResponse(SalonResponse),
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
    Unknown((PacketHeader, Vec<u8>)),
}

impl Packet {
    pub fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = vec![];
        buf.write_u32::<LittleEndian>(0).unwrap();
        buf.extend(match self {
            Self::None => {
                return vec![];
            }

            Self::InitialLoad => PacketHeader::new(0x03, 0x03, Flags::default()).write(is_ngs),
            Self::LoadingScreenTransition => {
                PacketHeader::new(0x03, 0x04, Flags::default()).write(is_ngs)
            }
            Self::ServerHello(packet) => packet.write(is_ngs),
            Self::ServerPing => PacketHeader::new(0x03, 0x0B, Flags::default()).write(is_ngs),
            Self::ServerPong => PacketHeader::new(0x03, 0x0C, Flags::default()).write(is_ngs),
            Self::FinishLoading => PacketHeader::new(0x03, 0x23, Flags::default()).write(is_ngs),

            Self::Movement(packet) => packet.write(is_ngs),
            Self::SetTag(packet) => packet.write(is_ngs),

            Self::SetPlayerID(packet) => packet.write(is_ngs),

            Self::ChatMessage(packet) => packet.write(is_ngs),

            Self::CharacterSpawn(packet) => packet.write(is_ngs),
            Self::EventSpawn(packet) => packet.write(is_ngs),
            Self::ObjectSpawn(packet) => packet.write(is_ngs),
            Self::NPCSpawn(packet) => packet.write(is_ngs),

            Self::FileTransfer(packet) => packet.write(is_ngs),

            //Login packets
            Self::SegaIDLogin(packet) => packet.write(is_ngs),
            Self::LoginResponse(packet) => packet.write(is_ngs),
            Self::CharacterListRequest => {
                PacketHeader::new(0x11, 0x02, Flags::default()).write(is_ngs)
            }
            Self::CharacterListResponse(packet) => packet.write(is_ngs),
            Self::StartGame(packet) => packet.write(is_ngs),
            Self::CharacterCreate(packet) => packet.write(is_ngs),
            Self::EncryptionRequest(packet) => packet.write(is_ngs),
            Self::EncryptionResponse(packet) => packet.write(is_ngs),
            Self::ClientPing(packet) => packet.write(is_ngs),
            Self::ClientPong(packet) => packet.write(is_ngs),
            Self::NicknameRequest(packet) => packet.write(is_ngs),
            Self::NicknameResponse(packet) => packet.write(is_ngs),
            Self::ClientGoodbye => PacketHeader::new(0x11, 0x2B, Flags::default()).write(is_ngs),
            Self::BlockBalance(packet) => packet.write(is_ngs),
            Self::SystemInformation(packet) => packet.write(is_ngs),
            Self::ShipList(packet) => packet.write(is_ngs),
            Self::CreateCharacter1 => PacketHeader::new(0x11, 0x41, Flags::default()).write(is_ngs),
            Self::CreateCharacter1Response(packet) => packet.write(is_ngs),
            Self::CreateCharacter2 => PacketHeader::new(0x11, 0x54, Flags::default()).write(is_ngs),
            Self::CreateCharacter2Response(packet) => packet.write(is_ngs),
            Self::VitaLogin(packet) => packet.write(is_ngs),
            Self::SalonEntryRequest => {
                PacketHeader::new(0x11, 0x66, Flags::default()).write(is_ngs)
            }
            Self::SalonEntryResponse(packet) => packet.write(is_ngs),
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
            Self::Unknown(data) => {
                let mut out_data = data.0.write(is_ngs);
                out_data.extend_from_slice(&data.1);
                out_data
            }
        });
        let len = (buf.len() + 3) & (usize::MAX ^ 3);
        buf.resize(len, 0);
        let len = (len as u32).to_le_bytes();
        buf[..4].copy_from_slice(&len);
        buf
    }
    pub fn read(input: &[u8], is_ngs: bool) -> std::io::Result<Vec<Self>> {
        let mut packets: Vec<Self> = vec![];
        let buffer_length = input.len();
        let mut pointer = 0;
        loop {
            if pointer >= buffer_length {
                break;
            }
            if input[pointer..].len() <= 4 {
                break;
            }
            let len = (&input[pointer..pointer + 4]).read_u32::<LittleEndian>()? as usize - 4;
            pointer += 4;
            if input[pointer..].len() < len {
                return Err(std::io::ErrorKind::UnexpectedEof.into());
            }
            let mut buf_tmp = Cursor::new(&input[pointer..pointer + len]);
            let header = PacketHeader::read(&mut buf_tmp, is_ngs)?;
            let flags = header.flag1.clone();
            pointer += len;
            match (header.id, header.subid, is_ngs) {
                (0x03, 0x03, _) => packets.push(Self::InitialLoad),
                (0x03, 0x04, _) => packets.push(Self::LoadingScreenTransition),
                (0x03, 0x08, _) => packets.push(Self::ServerHello(ServerHelloPacket::read(
                    &mut buf_tmp,
                    flags,
                )?)),
                (0x03, 0x0B, _) => {
                    packets.push(Self::ServerPing);
                }
                (0x03, 0x0C, _) => {
                    packets.push(Self::ServerPong);
                }
                (0x03, 0x23, _) => packets.push(Self::FinishLoading),

                (0x04, 0x07, _) => {
                    packets.push(Self::Movement(MovementPacket::read(&mut buf_tmp, flags)?))
                }
                (0x04, 0x15, _) => {
                    packets.push(Self::SetTag(SetTagPacket::read(&mut buf_tmp, flags)?))
                }

                (0x06, 0x00, _) => packets.push(Self::SetPlayerID(SetPlayerIDPacket::read(
                    &mut buf_tmp,
                    flags,
                )?)),

                (0x07, 0x00, _) => {
                    packets.push(Self::ChatMessage(ChatMessage::read(&mut buf_tmp, flags)?))
                }

                (0x08, 0x04, false) => packets.push(Self::CharacterSpawn(
                    CharacterSpawnPacket::read(&mut buf_tmp, flags)?,
                )),
                (0x08, 0x09, _) => packets.push(Self::EventSpawn(EventSpawnPacket::read(
                    &mut buf_tmp,
                    flags,
                )?)),
                (0x08, 0x0B, _) => packets.push(Self::ObjectSpawn(ObjectSpawnPacket::read(
                    &mut buf_tmp,
                    flags,
                )?)),
                (0x08, 0x0C, _) => {
                    packets.push(Self::NPCSpawn(NPCSpawnPacket::read(&mut buf_tmp, flags)?))
                }

                (0x0F, 0x00, _) => {
                    packets.push(Self::FileTransfer(FileTransferPacket::read(
                    &mut buf_tmp, flags
                )?))},

                //Login packets
                (0x11, 0x00, _) => {
                    packets.push(Self::SegaIDLogin(SegaIDLoginPacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }
                (0x11, 0x01, false) => {
                    packets.push(Self::LoginResponse(LoginResponsePacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }
                (0x11, 0x02, _) => {
                    packets.push(Self::CharacterListRequest);
                }
                (0x11, 0x03, false) => {
                    packets.push(Self::CharacterListResponse(CharacterListPacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }
                (0x11, 0x04, false) => {
                    packets.push(Self::StartGame(StartGamePacket::read(&mut buf_tmp, flags)?));
                }
                (0x11, 0x05, false) => {
                    packets.push(Self::CharacterCreate(CharacterCreatePacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }
                (0x11, 0x0B, _) => {
                    packets.push(Self::EncryptionRequest(EncryptionRequestPacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }
                (0x11, 0x0C, _) => {
                    packets.push(Self::EncryptionResponse(EncryptionResponsePacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }
                (0x11, 0x0D, _) => {
                    packets.push(Self::ClientPing(ClientPingPacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }
                (0x11, 0x0E, _) => {
                    packets.push(Self::ClientPong(ClientPongPacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }
                (0x11, 0x1E, _) => {
                    packets.push(Self::NicknameRequest(NicknameRequestPacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }
                (0x11, 0x1D, _) => {
                    packets.push(Self::NicknameResponse(NicknameResponsePacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }
                (0x11, 0x2B, _) => {
                    packets.push(Self::ClientGoodbye);
                }
                (0x11, 0x2C, false) => {
                    packets.push(Self::BlockBalance(BlockBalancePacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }
                (0x11, 0x2D, _) => {
                    packets.push(Self::SystemInformation(SystemInformationPacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }
                (0x11, 0x3D, _) => {
                    packets.push(Self::ShipList(ShipListPacket::read(&mut buf_tmp, flags)?));
                }
                (0x11, 0x41, _) => {
                    packets.push(Self::CreateCharacter1);
                }
                (0x11, 0x42, _) => {
                    packets.push(Self::CreateCharacter1Response(
                        CreateCharacter1ResponsePacket::read(&mut buf_tmp, flags)?,
                    ));
                }
                (0x11, 0x54, _) => {
                    packets.push(Self::CreateCharacter2);
                }
                (0x11, 0x55, _) => {
                    packets.push(Self::CreateCharacter2Response(
                        CreateCharacter2ResponsePacket::read(&mut buf_tmp, flags)?,
                    ));
                }
                (0x11, 0x63, false) => {
                    packets.push(Self::VitaLogin(VitaLoginPacket::read(&mut buf_tmp, flags)?));
                }
                (0x11, 0x66, _) => {
                    packets.push(Self::SalonEntryRequest);
                }
                (0x11, 0x67, false) => {
                    packets.push(Self::SalonEntryResponse(SalonResponse::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }
                (0x11, 0x6B, false) => {
                    packets.push(Self::SegaIDInfoRequest);
                }
                (0x11, 0x86, _) => {
                    packets.push(Self::LoginHistoryRequest);
                }
                (0x11, 0x87, _) => {
                    packets.push(Self::LoginHistoryResponse(LoginHistoryPacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }
                (0x11, 0xEA, _) => {
                    packets.push(Self::NicknameError(NicknameErrorPacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }
                (0x11, 0xEE, _) => {
                    packets.push(Self::EmailCodeRequest(EmailCodeRequestPacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }
                (0x11, 0xFF, _) => {
                    packets.push(Self::Unk1(Unk1Packet::read(&mut buf_tmp, flags)?));
                }

                (0x19, 0x01, _) => {
                    packets.push(Self::SystemMessage(SystemMessagePacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }

                //Settings packets
                (0x2B, 0x00, _) => packets.push(Self::SettingsRequest),
                (0x2B, 0x01, _) => {
                    packets.push(Self::SaveSettings(SaveSettingsPacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }
                (0x2B, 0x02, _) => {
                    packets.push(Self::LoadSettings(LoadSettingsPacket::read(
                        &mut buf_tmp,
                        flags,
                    )?));
                }

                //Other packets
                (_, _, _) => {
                    packets.push(Self::Unknown({
                        let mut data = vec![];
                        buf_tmp.read_to_end(&mut data)?;
                        (header, data)
                    }));
                }
            }
        }

        Ok(packets)
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

#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[repr(u16)]
pub enum EntityType {
    #[default]
    Unknown = 0,
    Player = 4,
    Map = 5,
    Object = 6,
}

#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct ObjectHeader {
    pub id: u32,
    #[Seek(4)]
    #[SeekAfter(2)]
    pub entity_type: EntityType,
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

// 0x06, 0x00
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x06, 0x00)]
pub struct SetPlayerIDPacket {
    pub player_id: u32,
    pub unk1: u32,
    pub unk2: u32,
}

// 0x07, 0x00
#[derive(Debug, Default, Clone, PartialEq, PacketReadWrite)]
#[Id(0x07, 0x00)]
#[Flags(Flags {packed: true, object_related: true, ..Default::default()})]
pub struct ChatMessage {
    pub unk1: [u8; 0xC],
    pub area: u8,
    pub unk3: u8,
    pub unk4: u16,
    #[VariableUtf16(0x9D3F, 0x44)]
    pub unk5: String,
    #[VariableUtf16(0x9D3F, 0x44)]
    pub message: String,
}

//0x08, 0x04
#[derive(Debug, Clone, PartialEq)]
pub struct CharacterSpawnPacket {
    // unsure about real structure
    pub player_obj: ObjectHeader,
    pub position: Position,
    pub unk1: u16, // padding?
    pub unk2: String,
    pub unk3: u16,
    pub unk4: u16,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub is_me: u32, //47 - me, 39 - otherwise
    pub character: Character,
    pub is_global: bool,
    pub unk9: String, // title?
    pub unk10: u32,
    pub unk11: u32, // gmflag?
    pub nickname: String,
    pub unk12: [u8; 0x40],
}
impl PacketReadWrite for CharacterSpawnPacket {
    fn read(reader: &mut (impl Read + Seek), _: Flags) -> std::io::Result<Self> {
        let player_obj = ObjectHeader::read(reader)?;
        let position = Position::read(reader)?;
        let unk1 = reader.read_u16::<LittleEndian>()?;
        let unk2 = read_utf8(reader, 0x20);
        let unk3 = reader.read_u16::<LittleEndian>()?;
        let unk4 = reader.read_u16::<LittleEndian>()?;
        let unk5 = reader.read_u32::<LittleEndian>()?;
        let unk6 = reader.read_u32::<LittleEndian>()?;
        let unk7 = reader.read_u32::<LittleEndian>()?;
        let unk8 = reader.read_u32::<LittleEndian>()?;
        let is_me = reader.read_u32::<LittleEndian>()?;
        let character = Character::read(reader)?;
        let unk9 = read_utf16(reader, 0x20);
        let unk10 = reader.read_u32::<LittleEndian>()?;
        let unk11 = reader.read_u32::<LittleEndian>()?;
        let nickname = read_utf16(reader, 0x10);
        let mut unk12 = [0u8; 0x40];
        reader.read_exact(&mut unk12)?;
        Ok(Self {
            player_obj,
            position,
            unk1,
            unk2,
            unk3,
            unk4,
            unk5,
            unk6,
            unk7,
            unk8,
            is_me,
            character,
            is_global: false,
            unk9,
            unk10,
            unk11,
            nickname,
            unk12,
        })
    }
    fn write(self, is_ngs: bool) -> Vec<u8> {
        let mut buf = PacketHeader::new(0x08, 0x04, Flags::default()).write(is_ngs);
        self.player_obj.write(&mut buf).unwrap();
        self.position.write(&mut buf).unwrap();
        buf.write_u16::<LittleEndian>(self.unk1).unwrap();
        buf.write_all(&write_utf8(&self.unk2, 0x20)).unwrap();
        buf.write_u16::<LittleEndian>(self.unk3).unwrap();
        buf.write_u16::<LittleEndian>(self.unk4).unwrap();
        buf.write_u32::<LittleEndian>(self.unk5).unwrap();
        buf.write_u32::<LittleEndian>(self.unk6).unwrap();
        buf.write_u32::<LittleEndian>(self.unk7).unwrap();
        buf.write_u32::<LittleEndian>(self.unk8).unwrap();
        buf.write_u32::<LittleEndian>(self.is_me).unwrap();
        self.character.write(&mut buf, self.is_global).unwrap();
        buf.write_all(&write_utf16(&self.unk9, 0x20)).unwrap();
        buf.write_u32::<LittleEndian>(self.unk10).unwrap();
        buf.write_u32::<LittleEndian>(self.unk11).unwrap();
        buf.write_all(&write_utf16(&self.nickname, 0x10)).unwrap();
        buf.write_all(&self.unk12).unwrap();
        buf
    }
}
impl Default for CharacterSpawnPacket {
    fn default() -> Self {
        Self {
            player_obj: ObjectHeader {
                id: 0,
                entity_type: EntityType::Player,
            },
            position: Position {
                rot_x: half::f16::from_bits(0),
                rot_y: half::f16::from_bits(15360),
                rot_z: half::f16::from_bits(0),
                rot_w: half::f16::from_bits(0),
                pos_x: half::f16::from_bits(14892),
                pos_y: half::f16::from_bits(0),
                pos_z: half::f16::from_bits(22589),
            },
            unk1: 0,
            unk2: "Character".to_string(),
            unk3: 1,
            unk4: 0,
            unk5: 602,
            unk6: 1,
            unk7: 53,
            unk8: 0,
            is_me: 47,
            character: Character::default(),
            is_global: true,
            unk9: String::new(),
            unk10: 0,
            unk11: 0,
            nickname: String::new(),
            unk12: [0u8; 0x40],
        }
    }
}

// 0x08, 0x09
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x09)]
pub struct EventSpawnPacket {
    pub object: ObjectHeader,
    pub position: Position,
    pub unk1: u16,
    #[FixedAscii(0x20)]
    pub name: String,
    pub unk2: u32,
    pub unk3: [u8; 0xC],
    pub unk4: u16,
    pub unk5: u16,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
    pub flags: u32,
    #[Len_u32]
    pub data: Vec<u32>,
}

// 0x08, 0x0B
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x0B)]
pub struct ObjectSpawnPacket {
    pub object: ObjectHeader,
    pub position: Position,
    pub unk1: u16,
    #[FixedAscii(0x34)]
    pub name: String,
    pub flags: u32,
    #[Len_u32]
    pub data: Vec<u32>,
}

// 0x08, 0x0C
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x08, 0x0C)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct NPCSpawnPacket {
    pub object: ObjectHeader,
    pub position: Position,
    pub unk1: u16,
    #[FixedAscii(0x20)]
    pub name: String,
    pub unk2: u32,
    pub unk3: [u8; 0xC],
    pub unk4: u16,
    pub unk5: u16,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    #[VariableAscii(0x9FCD, 0xE7)]
    pub unk13: String,
}

// 0x0F, 0x00
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0F, 0x00)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct FileTransferPacket {
    pub id: u16,
    pub segment: u16,
    pub total_size: u32,
    #[Magic(0x8A92, 0x30)]
    pub data: Vec<u8>,
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
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
pub enum MessageType {
    GoldenTicker,
    AdminMessage,
    AdminMessageInstant,
    SystemMessage,
    #[default]
    GenericMessage,
}

// ----------------------------------------------------------------
// Settings packets
// ----------------------------------------------------------------

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
