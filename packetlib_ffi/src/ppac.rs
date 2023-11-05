use pso2packetlib::ppac;
use pso2packetlib::ppac::PPACReader as PR;
use std::{
    error::Error,
    ffi::{CStr, CString},
    fs::File,
};

use crate::protocol::{PacketType, SerializedFormat};

#[repr(C)]
pub enum ReaderResult {
    Ok,
    RawOnly,
    ReaderEOF,
    PPACError,
}

#[repr(C)]
pub enum Direction {
    ToServer,
    ToClient,
}

#[repr(C)]
pub enum OutputType {
    /// Output only parsed packet.
    OutputPacket,
    /// Output only raw packet.
    OutputRaw,
    /// Output both packets.
    OutputBoth,
}

pub struct PPACReader {
    reader: Option<PR<File>>,
    err_str: Option<CString>,
    data: Option<ppac::PacketData>,
    data_parsed: Vec<u8>,
    serde_format: SerializedFormat,
}

#[repr(C)]
pub struct PacketData {
    /// When was the packet stored (in secs).
    pub time: u64,
    /// Where the packet was heading.
    pub direction: Direction,
    /// Which client version produced this packet.
    pub protocol_type: PacketType,
    /// Parsed packet (if requested)
    pub data_ptr: *const u8,
    pub data_size: usize,
    /// Raw packet (if requested)
    pub raw_ptr: *const u8,
    pub raw_size: usize,
}

/// Creates a new PPAC reader. After creation don't forget to check for errors.
#[no_mangle]
pub extern "C" fn new_reader(path: *const i8, serde_format: SerializedFormat) -> Box<PPACReader> {
    let mut reader = PPACReader {
        reader: None,
        err_str: None,
        data: None,
        data_parsed: vec![],
        serde_format,
    };
    if path.is_null() {
        reader.err_str = Some(CString::new("No path provided").unwrap_or_default());
        return Box::new(reader);
    }
    match new_reader_failable(path) {
        Ok(r) => reader.reader = Some(r),
        Err(e) => reader.err_str = Some(CString::new(e.to_string()).unwrap_or_default()),
    }
    Box::new(reader)
}

/// Destroys the reader.
#[no_mangle]
pub extern "C" fn free_reader(_reader: Option<Box<PPACReader>>) {}

/// Sets the output type.
#[no_mangle]
pub extern "C" fn set_out_type(reader: Option<&mut PPACReader>, out_type: OutputType) {
    let _ = reader
        .and_then(|r| r.reader.as_mut())
        .and_then(|r| Some(r.set_out_type(out_type.into())));
}

/// Reads the packet and returns if the function succeeded.
#[no_mangle]
pub extern "C" fn read_packet(reader: Option<&mut PPACReader>) -> ReaderResult {
    let Some(reader) = reader else {
        return ReaderResult::PPACError;
    };
    match read_packet_failable(reader) {
        Ok(p) => p,
        Err(e) => {
            reader.err_str = Some(CString::new(e.to_string()).unwrap_or_default());
            ReaderResult::PPACError
        }
    }
}

/// Returns a pointer to the packet data or a null pointer if no data exists.
///
/// # Safety
/// The returned pointer is only valid until the next data-returning function call.
/// If the returned array is empty, the pointer might be non-null but still invalid. This is not
/// considered an error.
#[no_mangle]
pub extern "C" fn get_reader_data(reader: Option<&PPACReader>) -> PacketData {
    let mut data = PacketData {
        time: 0,
        direction: Direction::ToServer,
        protocol_type: PacketType::Classic,
        data_ptr: std::ptr::null(),
        data_size: 0,
        raw_ptr: std::ptr::null(),
        raw_size: 0,
    };
    match reader.and_then(|r| r.data.as_ref()) {
        Some(c) => {
            data.time = c.time.as_secs();
            data.direction = c.direction.into();
            data.protocol_type = c.protocol_type.into();
            data.data_ptr = reader.unwrap().data_parsed.as_ptr();
            data.data_size = reader.unwrap().data_parsed.len();
            data.raw_ptr = c
                .data
                .as_ref()
                .and_then(|d| Some(d.as_ptr()))
                .unwrap_or(std::ptr::null());
            data.raw_size = c
                .data
                .as_ref()
                .and_then(|d| Some(d.len()))
                .unwrap_or_default();
        }
        None => {}
    }
    data
}

/// Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
/// occurred.
///
/// # Safety
/// The returned pointer is only valid until the next failable function call.
#[no_mangle]
pub extern "C" fn get_reader_error(reader: Option<&PPACReader>) -> *const u8 {
    match reader.and_then(|r| r.err_str.as_ref()) {
        Some(e) => e.as_ptr() as *const u8,
        None => std::ptr::null(),
    }
}

fn read_packet_failable(reader: &mut PPACReader) -> Result<ReaderResult, Box<dyn Error>> {
    reader.data_parsed = vec![];
    reader.data = None;
    let Some(pac_reader) = reader.reader.as_mut() else {
        return Err("Invalid reader state".into());
    };
    let mut packet_data = match pac_reader.read()? {
        Some(p) => p,
        None => return Ok(ReaderResult::ReaderEOF),
    };
    let is_raw = !packet_data.data.is_none() && packet_data.packet.is_none();
    let parsed = match packet_data.packet.take() {
        Some(p) => reader.serde_format.serialize(&p)?,
        None => vec![],
    };
    reader.data_parsed = parsed;
    reader.data = Some(packet_data);
    if is_raw {
        Ok(ReaderResult::RawOnly)
    } else {
        Ok(ReaderResult::Ok)
    }
}

fn new_reader_failable(path: *const i8) -> Result<PR<File>, Box<dyn Error>> {
    let str = unsafe { CStr::from_ptr(path) }.to_str()?;
    let file = File::open(str)?;
    Ok(PR::open(file)?)
}

impl From<ppac::Direction> for Direction {
    fn from(value: ppac::Direction) -> Self {
        match value {
            ppac::Direction::ToServer => Self::ToServer,
            ppac::Direction::ToClient => Self::ToClient,
        }
    }
}

impl From<OutputType> for ppac::OutputType {
    fn from(value: OutputType) -> Self {
        match value {
            OutputType::OutputPacket => Self::Packet,
            OutputType::OutputRaw => Self::Raw,
            OutputType::OutputBoth => Self::Both,
        }
    }
}
