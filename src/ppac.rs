use crate::protocol::{Packet, PacketType, ProtocolRW};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    io::{BufReader, ErrorKind, Read, Seek, SeekFrom, Write},
    time::Duration,
};
use zstd::stream::{Decoder, Encoder};

/// Possible types of packet data output.
pub enum OutputType {
    /// Output only parsed packet data.
    Packet,
    /// Output only raw packet data.
    Raw,
    /// Output both types.
    Both,
}

/// Direction of the packet.
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

enum ReaderWrapper<R: Read> {
    NoEnc(R),
    Zstd(Decoder<'static, BufReader<R>>),
}

enum WriterWrapper<W: Write> {
    NoEnc(W),
    Zstd(Encoder<'static, W>),
}

/// Reader for the `ppac` packet files.
pub struct PPACReader<R: Read> {
    reader: ReaderWrapper<R>,
    version: u8,
    packet_buffer: Vec<Packet>,
    data_buffer: Vec<Vec<u8>>,
    protocol_type: PacketType,
    last_header: Header,
    out_type: OutputType,
}

/// Writer of the `ppac` packet files.
#[derive(Debug)]
pub struct PPACWriter<W: Write> {
    writer: Option<WriterWrapper<W>>,
    packet_type: PacketType,
}

/// Packet data.
pub struct PacketData {
    /// When was the packet stored.
    pub time: Duration,
    /// Where the packet was heading.
    pub direction: Direction,
    /// Which client version produced this packet.
    pub protocol_type: PacketType,
    /// Parsed packet (if requested).
    pub packet: Option<Packet>,
    /// Unparsed packet (if requested).
    pub data: Option<Vec<u8>>,
}

//--------------------------------------
// Reader/Writer wrapper implementation
//--------------------------------------

impl<R: Read> ReaderWrapper<R> {
    fn into_inner(self) -> R {
        match self {
            ReaderWrapper::NoEnc(r) => r,
            ReaderWrapper::Zstd(e) => e.finish().into_inner(),
        }
    }
}

impl<R: Read> Read for ReaderWrapper<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self {
            ReaderWrapper::NoEnc(r) => r.read(buf),
            ReaderWrapper::Zstd(d) => d.read(buf),
        }
    }
}

impl<W: Write> WriterWrapper<W> {
    fn into_inner(self) -> std::io::Result<W> {
        match self {
            WriterWrapper::NoEnc(w) => Ok(w),
            WriterWrapper::Zstd(e) => e.finish(),
        }
    }
    fn write_u8_raw(&mut self, byte: u8) -> std::io::Result<()> {
        match self {
            WriterWrapper::NoEnc(w) => w.write_u8(byte),
            WriterWrapper::Zstd(e) => e.get_mut().write_u8(byte),
        }
    }
}

impl<W: Write> Write for WriterWrapper<W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            WriterWrapper::NoEnc(w) => w.write(buf),
            WriterWrapper::Zstd(e) => e.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            WriterWrapper::NoEnc(w) => w.flush(),
            WriterWrapper::Zstd(e) => e.flush(),
        }
    }
}

impl<W: Write + Seek> Seek for WriterWrapper<W> {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match self {
            WriterWrapper::NoEnc(w) => w.seek(pos),
            WriterWrapper::Zstd(e) => e.get_mut().seek(pos),
        }
    }
}

impl<W: Write> std::fmt::Debug for WriterWrapper<W> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

//--------------------------------------
// PPAC reader wrapper implementation
//--------------------------------------

impl<R: Read> PPACReader<R> {
    /// Opens a PPAC file.
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
        let reader = if version >= 4 {
            let enc_flag = reader.read_u8()?;
            match enc_flag {
                0 => ReaderWrapper::NoEnc(reader),
                _ => ReaderWrapper::Zstd(Decoder::new(reader)?),
            }
        } else {
            ReaderWrapper::NoEnc(reader)
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

    /// Sets the client type.
    pub fn set_out_type(&mut self, out_type: OutputType) {
        self.out_type = out_type;
    }

    /// Returns the readers protocol type..
    pub fn get_protocol_type(&self) -> PacketType {
        self.protocol_type
    }

    /// Reads a packet from the PPAC.
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
        let mut data = vec![];
        self.reader.by_ref().take(len).read_to_end(&mut data)?;
        let (packet, data) = match self.out_type {
            OutputType::Packet => {
                self.read_packet(&data)?;
                (self.packet_buffer.drain(0..1).next(), None)
            }
            OutputType::Raw => {
                self.read_data(&data)?;
                (None, self.data_buffer.drain(0..1).next())
            }
            OutputType::Both => {
                let output = self.read_packet(&data);
                let packet_data = match output {
                    Ok(_) => self.packet_buffer.drain(0..1).next(),
                    Err(_) => None,
                };
                self.read_data(&data)?;
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

    // Returns the underlying reader.
    pub fn into_inner(self) -> R {
        self.reader.into_inner()
    }

    fn read_packet(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.packet_buffer
            .append(&mut Packet::read(buf, self.protocol_type)?);
        Ok(())
    }

    fn read_data(&mut self, buf: &[u8]) -> std::io::Result<()> {
        let packets = Packet::read(buf, PacketType::Raw)?;
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

//--------------------------------------
// PPAC writer wrapper implementation
//--------------------------------------

impl<W: Write> PPACWriter<W> {
    /// Creates a new PPAC file.
    pub fn new(
        mut writer: W,
        packet_type: PacketType,
        is_enc: bool,
    ) -> std::io::Result<PPACWriter<W>> {
        writer.write_all(b"PPAC")?;
        writer.write_u8(4)?;
        writer.write_u8(match packet_type {
            PacketType::Classic => 0,
            PacketType::NGS => 1,
            PacketType::NA => 2,
            PacketType::JP => 3,
            PacketType::Vita => 4,
            PacketType::Raw => return Err(ErrorKind::InvalidInput.into()),
        })?;
        writer.write_u8(is_enc as u8)?;
        let writer = Some(match is_enc {
            true => WriterWrapper::Zstd(Encoder::new(writer, 3)?),
            false => WriterWrapper::NoEnc(writer),
        });
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
        let writer = self.writer.as_mut().unwrap();
        writer.write_u128::<LittleEndian>(time.as_nanos())?;
        writer.write_u8(match direction {
            Direction::ToServer => 0,
            Direction::ToClient => 1,
        })?;
        writer.write_u64::<LittleEndian>(len)?;
        Ok(())
    }
    /// Writes data without checking its length.
    pub fn write_data_unchecked(
        &mut self,
        time: Duration,
        direction: Direction,
        input: &[u8],
    ) -> std::io::Result<()> {
        self.write_header(time, direction, input.len() as u64)?;
        self.writer.as_mut().unwrap().write_all(input)
    }
    /// Writes data (must be valid packet data).
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
    /// Writes a parsed packet.
    pub fn write_packet(
        &mut self,
        time: Duration,
        direction: Direction,
        input: &Packet,
    ) -> std::io::Result<()> {
        let data = input.write(self.packet_type);
        self.write_data_unchecked(time, direction, &data)?;
        Ok(())
    }

    // Returns the underlying writer.
    pub fn into_inner(mut self) -> std::io::Result<W> {
        self.writer.take().unwrap().into_inner()
    }
}

impl<W: Write + Seek> PPACWriter<W> {
    /// Changes stored client type.
    pub fn change_packet_type(&mut self, packet_type: PacketType) -> std::io::Result<()> {
        if matches!(packet_type, PacketType::Raw) {
            return Err(ErrorKind::InvalidInput.into());
        }
        let writer = self.writer.as_mut().unwrap();
        let curr_pos = writer.stream_position()?;
        writer.seek(SeekFrom::Start(5))?;
        writer.write_u8_raw(match packet_type {
            PacketType::Classic => 0,
            PacketType::NGS => 1,
            PacketType::NA => 2,
            PacketType::JP => 3,
            PacketType::Vita => 4,
            PacketType::Raw => unreachable!(),
        })?;
        writer.seek(SeekFrom::Start(curr_pos))?;
        self.packet_type = packet_type;
        Ok(())
    }
}

impl<W: Write> Drop for PPACWriter<W> {
    fn drop(&mut self) {
        match self.writer.take() {
            Some(w) => {
                let _ = w.into_inner();
            }
            None => {}
        }
    }
}

#[cfg(feature = "connection")]
pub(crate) fn get_now() -> Duration {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
}
