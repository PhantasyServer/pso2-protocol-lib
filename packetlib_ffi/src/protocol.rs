use pso2packetlib::protocol::{Packet, PacketType as PacketTypeEX, ProtocolRW};
use std::{error::Error, ffi::CString};

/// Packet types.
#[repr(C)]
#[derive(Clone, Copy)]
pub enum PacketType {
    NGS,
    Classic,
    NA,
    JP,
    Vita,
    Raw,
}

/// Serialized packet format
#[repr(C)]
#[derive(Clone, Copy)]
pub enum SerializedFormat {
    JSON,
    MessagePack,
    MessagePackNamed,
}

/// Fat pointer to data.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DataBuffer {
    pub ptr: *const u8,
    pub size: usize,
}
const NULL_BUF: DataBuffer = DataBuffer {
    ptr: std::ptr::null(),
    size: 0,
};

pub struct PacketWorker {
    err_str: Option<CString>,
    data: Vec<u8>,
    packets: Vec<Packet>,
    packet_type: PacketTypeEX,
    serde_format: SerializedFormat,
}

/// Creates a new packet worker.
#[no_mangle]
pub extern "C" fn new_worker(
    packet_type: PacketType,
    serde_format: SerializedFormat,
) -> Box<PacketWorker> {
    Box::new(PacketWorker {
        err_str: None,
        data: vec![],
        packets: vec![],
        packet_type: packet_type.into(),
        serde_format,
    })
}

/// Destroys a packet worker.
#[no_mangle]
pub extern "C" fn free_worker(_worker: Option<Box<PacketWorker>>) {}

/// Sets a new packet type.
#[no_mangle]
pub extern "C" fn set_packet_type(worker: Option<&mut PacketWorker>, packet_type: PacketType) {
    if let Some(worker) = worker {
        worker.packet_type = packet_type.into();
    }
}

/// Sets a new serde format.
#[no_mangle]
pub extern "C" fn set_serde_format(worker: Option<&mut PacketWorker>, format: SerializedFormat) {
    if let Some(worker) = worker {
        worker.serde_format = format;
    }
}

/// Checks if the specified serde format is supported.
#[no_mangle]
pub extern "C" fn serde_supported(serde_format: SerializedFormat) -> bool {
    serde_format.is_supported()
}

/// Parses packet data and returns a fat pointer to the serialized packet or a null pointer if
/// an error occurred.
///
/// # Safety
/// The returned pointer is only valid until the next data-returning function call.
/// If the returned array is empty, the pointer might be non-null but still invalid. This is not
/// considered an error.
#[no_mangle]
pub extern "C" fn parse_packet(
    worker: Option<&mut PacketWorker>,
    data_ptr: *const u8,
    size: usize,
) -> DataBuffer {
    let null = NULL_BUF;
    let Some(worker) = worker else {
        return null;
    };
    worker.err_str = None;
    if worker.get_last_packet() && !worker.data.is_empty() {
        return DataBuffer {
            ptr: worker.data.as_ptr(),
            size: worker.data.len(),
        };
    } else if worker.err_str.is_some() {
        return null;
    }
    if data_ptr.is_null() {
        return null;
    }
    let data = unsafe { std::slice::from_raw_parts(data_ptr, size) };
    if worker.parse_packet(data) {
        return DataBuffer {
            ptr: worker.data.as_ptr(),
            size: worker.data.len(),
        };
    }
    null
}

/// Deserializes packet and returns a fat pointer to the packet data or a null pointer if an error
/// occured.
///
/// # Safety
/// The returned pointer is only valid until the next data-returning function call.
/// If the returned array is empty, the pointer might be non-null but still invalid. This is not
/// considered an error.
#[no_mangle]
pub extern "C" fn create_packet(
    worker: Option<&mut PacketWorker>,
    data_ptr: *const u8,
    size: usize,
) -> DataBuffer {
    let null = NULL_BUF;
    let Some(worker) = worker else {
        return null;
    };
    worker.err_str = None;
    if data_ptr.is_null() {
        return null;
    }
    let data = unsafe { std::slice::from_raw_parts(data_ptr, size) };
    if worker.create_packet(data) {
        return DataBuffer {
            ptr: worker.data.as_ptr(),
            size: worker.data.len(),
        };
    }
    null
}

/// Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
/// occurred.
///
/// # Safety
/// The returned pointer is only valid until the next failable function call.
#[no_mangle]
pub extern "C" fn get_error(worker: Option<&mut PacketWorker>) -> *const u8 {
    let null = std::ptr::null();
    let Some(worker) = worker else {
        return null;
    };
    match worker.err_str {
        Some(ref str) => str.as_ptr() as *const u8,
        None => null,
    }
}

impl SerializedFormat {
    fn serialize(self, packet: &Packet) -> Result<Vec<u8>, Box<dyn Error>> {
        match self {
            #[cfg(feature = "json")]
            SerializedFormat::JSON => {
                let mut packet_data = serde_json::to_vec(packet)?;
                packet_data.push(0);
                Ok(packet_data)
            }
            #[cfg(not(feature = "json"))]
            SerializedFormat::JSON => Err("Unsupported serde format".into()),
            #[cfg(feature = "messagepack")]
            SerializedFormat::MessagePack => Ok(rmp_serde::to_vec(packet)?),
            #[cfg(not(feature = "messagepack"))]
            SerializedFormat::MessagePack => Err("Unsupported serde format".into()),
            #[cfg(feature = "messagepack")]
            SerializedFormat::MessagePackNamed => Ok(rmp_serde::to_vec_named(packet)?),
            #[cfg(not(feature = "messagepack"))]
            SerializedFormat::MessagePackNamed => Err("Unsupported serde format".into()),
        }
    }
    fn deserialize(self, data: &[u8]) -> Result<Packet, Box<dyn Error>> {
        match self {
            #[cfg(feature = "json")]
            SerializedFormat::JSON => {
                use std::ffi::CStr;
                let packet_data = CStr::from_bytes_until_nul(data)?;
                Ok(serde_json::from_str(packet_data.to_str()?)?)
            }
            #[cfg(not(feature = "json"))]
            SerializedFormat::JSON => Err("Unsupported serde format".into()),
            #[cfg(feature = "messagepack")]
            SerializedFormat::MessagePack => Ok(rmp_serde::from_slice(data)?),
            #[cfg(not(feature = "messagepack"))]
            SerializedFormat::MessagePack => Err("Unsupported serde format".into()),
            #[cfg(feature = "messagepack")]
            SerializedFormat::MessagePackNamed => Ok(rmp_serde::from_slice(data)?),
            #[cfg(not(feature = "messagepack"))]
            SerializedFormat::MessagePackNamed => Err("Unsupported serde format".into()),
        }
    }
    fn is_supported(self) -> bool {
        match self {
            #[cfg(feature = "json")]
            SerializedFormat::JSON => true,
            #[cfg(not(feature = "json"))]
            SerializedFormat::JSON => false,
            #[cfg(feature = "messagepack")]
            SerializedFormat::MessagePack => true,
            #[cfg(not(feature = "messagepack"))]
            SerializedFormat::MessagePack => false,
            #[cfg(feature = "messagepack")]
            SerializedFormat::MessagePackNamed => true,
            #[cfg(not(feature = "messagepack"))]
            SerializedFormat::MessagePackNamed => false,
        }
    }
}

impl From<PacketType> for PacketTypeEX {
    fn from(value: PacketType) -> Self {
        match value {
            PacketType::NGS => Self::NGS,
            PacketType::Classic => Self::Classic,
            PacketType::NA => Self::NA,
            PacketType::JP => Self::JP,
            PacketType::Vita => Self::Vita,
            PacketType::Raw => Self::Raw,
        }
    }
}

impl PacketWorker {
    fn parse_packet(&mut self, data: &[u8]) -> bool {
        match self.parse_packet_failable(data) {
            Ok(_) => {
                self.err_str = None;
                true
            }

            Err(e) => {
                self.err_str = Some(CString::new(format!("{}", e)).unwrap_or_default());
                false
            }
        }
    }
    fn parse_packet_failable(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        self.packets
            .append(&mut Packet::read(data, self.packet_type)?);
        self.get_last_packet_failable()
    }
    fn get_last_packet(&mut self) -> bool {
        match self.get_last_packet_failable() {
            Ok(_) => {
                self.err_str = None;
                true
            }

            Err(e) => {
                self.err_str = Some(CString::new(format!("{}", e)).unwrap_or_default());
                false
            }
        }
    }
    fn get_last_packet_failable(&mut self) -> Result<(), Box<dyn Error>> {
        self.data = vec![];
        if let Some(packet) = self.packets.pop() {
            self.data = self.serde_format.serialize(&packet)?;
        }
        Ok(())
    }
    fn create_packet(&mut self, data: &[u8]) -> bool {
        match self.create_packet_failable(data) {
            Ok(_) => {
                self.err_str = None;
                true
            }

            Err(e) => {
                self.err_str = Some(CString::new(format!("{}", e)).unwrap_or_default());
                false
            }
        }
    }
    fn create_packet_failable(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        self.data = self.serde_format.deserialize(data)?.write(self.packet_type);

        Ok(())
    }
}
