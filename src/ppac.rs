use crate::protocol::{Packet, PacketType, ProtocolRW};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    io::{ErrorKind, Read, Seek, SeekFrom, Write},
    time::Duration,
};

/// Possible types of packet data output
pub enum OutputType {
    /// Output only parsed packet data
    Packet,
    /// Output only raw packet data
    Raw,
    /// Output both types
    Both,
}

/// Direction of the packet
#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Direction {
    ToServer,
    ToClient,
}

struct Header {
    time: Duration,
    direction: Direction,
}

/// Reader for the `ppac` packet files
pub struct PPACReader<R: Read + Seek> {
    reader: R,
    version: u8,
    packet_buffer: Vec<Packet>,
    data_buffer: Vec<Vec<u8>>,
    protocol_type: PacketType,
    last_header: Header,
    out_type: OutputType,
}

/// Writer of the `ppac` packet files
#[derive(Debug)]
pub struct PPACWriter<W: Write> {
    writer: W,
    packet_type: PacketType,
}

/// Packet data
pub struct PacketData {
    /// When was the packet stored
    pub time: Duration,
    /// Where the packet was heading
    pub direction: Direction,
    /// Which client version produced this packet
    pub protocol_type: PacketType,
    /// Parsed packet (if requested)
    pub packet: Option<Packet>,
    /// Unparsed packet (if requested)
    pub data: Option<Vec<u8>>,
}

impl<R: Read + Seek> PPACReader<R> {
    /// Open a log file
    pub fn open(mut reader: R) -> std::io::Result<Self> {
        let mut header = [0u8; 4];
        reader.read_exact(&mut header)?;
        if &header != b"PPAC" {
            return Err(ErrorKind::InvalidData.into());
        }
        let version = reader.read_u8()?;
        let protocol_type = if version >= 3 {
            let tmp_proto = reader.read_u8()?;
            match tmp_proto {
                0 => PacketType::Classic,
                1 => PacketType::NGS,
                2 => PacketType::NA,
                3 => PacketType::JP,
                4 => PacketType::Vita,
                _ => return Err(ErrorKind::InvalidData.into()),
            }
        } else {
            PacketType::NGS
        };
        Ok(Self {
            reader,
            version,
            packet_buffer: vec![],
            data_buffer: vec![],
            protocol_type,
            last_header: Header {
                time: Duration::new(0, 0),
                direction: Direction::ToServer,
            },
            out_type: OutputType::Packet,
        })
    }

    /// Sets the output type
    pub fn set_out_type(&mut self, out_type: OutputType) {
        self.out_type = out_type;
    }

    /// Read a packet from logs
    pub fn read(&mut self) -> std::io::Result<Option<PacketData>> {
        let packet = if !self.packet_buffer.is_empty() {
            self.packet_buffer.drain(0..1).next()
        } else {
            None
        };
        let data = if !self.data_buffer.is_empty() {
            self.data_buffer.drain(0..1).next()
        } else {
            None
        };
        if packet.is_some() || data.is_some() {
            return Ok(Some(PacketData {
                time: self.last_header.time,
                direction: self.last_header.direction,
                protocol_type: self.protocol_type,
                packet,
                data,
            }));
        }
        let time = match self.read_time() {
            Ok(time) => time,
            Err(e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(None),
            Err(e) => return Err(e),
        };
        let direction = match self.reader.read_u8()? {
            0 => Direction::ToServer,
            _ => Direction::ToClient,
        };
        self.last_header = Header { time, direction };
        let len = self.reader.read_u64::<LittleEndian>()?;
        let (packet, data) = match self.out_type {
            OutputType::Packet => {
                self.read_packet(len)?;
                (self.packet_buffer.drain(0..1).next(), None)
            }
            OutputType::Raw => {
                self.read_data(len)?;
                (None, self.data_buffer.drain(0..1).next())
            }
            OutputType::Both => {
                let data_begin = self.reader.stream_position()?;
                let output = self.read_packet(len);
                let packet_data = match output {
                    Ok(_) => self.packet_buffer.drain(0..1).next(),
                    Err(_) => None,
                };
                self.reader.seek(std::io::SeekFrom::Start(data_begin))?;
                self.read_data(len)?;
                (packet_data, self.data_buffer.drain(0..1).next())
            }
        };
        Ok(Some(PacketData {
            time,
            direction,
            protocol_type: self.protocol_type,
            packet,
            data,
        }))
    }

    // Return the underlying reader
    pub fn into_inner(self) -> R {
        self.reader
    }

    fn read_packet(&mut self, len: u64) -> std::io::Result<()> {
        let mut data = vec![];
        self.reader.by_ref().take(len).read_to_end(&mut data)?;
        self.packet_buffer
            .append(&mut Packet::read(&data, self.protocol_type)?);
        Ok(())
    }

    fn read_data(&mut self, len: u64) -> std::io::Result<()> {
        let mut data = vec![];
        self.reader.by_ref().take(len).read_to_end(&mut data)?;
        let packets = Packet::read(&data, PacketType::Raw)?;
        for packet in packets {
            let Packet::Raw(raw_data) = packet else {
                unreachable!()
            };
            self.data_buffer.push(raw_data);
        }
        Ok(())
    }

    fn read_time(&mut self) -> std::io::Result<Duration> {
        if (2..).contains(&self.version) {
            Ok(Duration::from_nanos(
                self.reader.read_u128::<LittleEndian>()? as u64,
            ))
        } else {
            Ok(Duration::from_secs(self.reader.read_u64::<LittleEndian>()?))
        }
    }
}

impl<W: Write> PPACWriter<W> {
    /// Create a new log file
    pub fn new(mut writer: W, packet_type: PacketType) -> std::io::Result<PPACWriter<W>> {
        writer.write_all(b"PPAC")?;
        writer.write_u8(3)?;
        writer.write_u8(match packet_type {
            PacketType::Classic => 0,
            PacketType::NGS => 1,
            PacketType::NA => 2,
            PacketType::JP => 3,
            PacketType::Vita => 4,
            PacketType::Raw => return Err(ErrorKind::InvalidInput.into()),
        })?;
        Ok(Self {
            writer,
            packet_type,
        })
    }
    fn write_header(
        &mut self,
        time: Duration,
        direction: Direction,
        len: u64,
    ) -> std::io::Result<()> {
        self.writer.write_u128::<LittleEndian>(time.as_nanos())?;
        self.writer.write_u8(match direction {
            Direction::ToServer => 0,
            Direction::ToClient => 1,
        })?;
        self.writer.write_u64::<LittleEndian>(len)?;
        Ok(())
    }
    /// Write data without checking its length
    pub fn write_data_unchecked(
        &mut self,
        time: Duration,
        direction: Direction,
        input: &[u8],
    ) -> std::io::Result<()> {
        self.write_header(time, direction, input.len() as u64)?;
        self.writer.write_all(input)
    }
    /// Write data (must be valid packet data)
    pub fn write_data(
        &mut self,
        time: Duration,
        direction: Direction,
        input: &[u8],
    ) -> std::io::Result<()> {
        let buffer_length = input.len();
        let mut pointer = 0;
        loop {
            if pointer >= buffer_length {
                break;
            }
            if input[pointer..].len() <= 4 {
                break;
            }
            let len = (&input[pointer..pointer + 4]).read_u32::<LittleEndian>()? as usize;
            if input[pointer..].len() < len {
                return Err(std::io::ErrorKind::UnexpectedEof.into());
            }
            let data = &input[pointer..pointer + len];
            self.write_data_unchecked(time, direction, data)?;
            pointer += len;
        }
        Ok(())
    }
    /// Write a parsed packet
    pub fn write_packet(
        &mut self,
        time: Duration,
        direction: Direction,
        input: &Packet,
    ) -> std::io::Result<()> {
        let data = input.write(self.packet_type);
        self.write_header(time, direction, data.len() as u64)?;
        self.write_data_unchecked(time, direction, &data)?;
        Ok(())
    }

    // Return the underlying writer
    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W: Write + Seek> PPACWriter<W> {
    /// Change stored client type
    pub fn change_packet_type(&mut self, packet_type: PacketType) -> std::io::Result<()> {
        if matches!(packet_type, PacketType::Raw) {
            return Err(ErrorKind::InvalidInput.into());
        }
        let curr_pos = self.writer.stream_position()?;
        self.writer.seek(SeekFrom::Start(5))?;
        self.writer.write_u8(match packet_type {
            PacketType::Classic => 0,
            PacketType::NGS => 1,
            PacketType::NA => 2,
            PacketType::JP => 3,
            PacketType::Vita => 4,
            PacketType::Raw => unreachable!(),
        })?;
        self.writer.seek(SeekFrom::Start(curr_pos))?;
        self.packet_type = packet_type;
        Ok(())
    }
}

#[cfg(feature = "connection")]
pub(crate) fn get_now() -> Duration {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
}
