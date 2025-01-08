use pso2packetlib::protocol::{Packet as PacketEX, PacketType as PacketTypeEX, ProtocolRW};
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

/// Serialized packet format.
#[repr(C)]
#[derive(Clone, Copy)]
pub enum SerializedFormat {
    /// Packets are serialized in JSON format.
    JSON,
    /// Packets are serialized in MessagePack format (all fields are arrays).
    MessagePack,
    /// Packets are serialized in MessagePack format (fields are named).
    MessagePackNamed,
}

/// Fat pointer to data.
#[repr(C)]
pub struct DataBuffer {
    pub ptr: *const u8,
    pub size: usize,
    /// INTERNAL: vector capacity
    pub _cap: usize,
}
pub(crate) const NULL_BUF: DataBuffer = DataBuffer {
    ptr: std::ptr::null(),
    size: 0,
    _cap: 0,
};

/// Factory for [`Packet`]. Handles error messages, stores packets and current serialization
/// format.
pub struct PacketWorker {
    err_str: Option<CString>,
    packets: Vec<PacketEX>,
    packet_type: PacketTypeEX,
    serde_format: SerializedFormat,
}

/// Wrapper type for [`pso2packetlib::protocol::Packet`].
#[derive(Clone)]
pub struct Packet(PacketEX);

/// Creates a new packet worker.
///
/// # Safety
/// - `packet_type` must be a valid variant of `PacketType`.
/// - `serde_format` must be a valid variant of [`SerializedFormat`].
#[no_mangle]
pub extern "C" fn new_worker(
    packet_type: PacketType,
    serde_format: SerializedFormat,
) -> Box<PacketWorker> {
    Box::new(PacketWorker {
        err_str: None,
        packets: vec![],
        packet_type: packet_type.into(),
        serde_format,
    })
}

/// Destroys a packet worker.
///
/// # Safety
/// `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
#[no_mangle]
pub extern "C" fn free_worker(_worker: Option<Box<PacketWorker>>) {}

/// Destroys a packet.
///
/// # Safety
/// `packet` must either be NULL or it must point to a valid [`Packet`] structure.
#[no_mangle]
pub extern "C" fn free_packet(_packet: Option<Box<Packet>>) {}

/// Destroys a data pointer and deallocates pointed at memory.
///
/// # Safety
/// - `data` must be a valid [`DataBuffer`] structure with valid data pointer.
#[no_mangle]
pub extern "C" fn free_data(data: DataBuffer) {
    if data.ptr.is_null() {
        return;
    }
    // SAFETY:
    // 1) ptr is allocated by the global allocator (Vec::new contract)
    // 2) deallocation allignment is the same as allocation allignment
    // 3) deallocation size is the same as allocation size
    // 4) length <= capacity (caller contract)
    // 5) data is properly initialized (Vec::new contract, caller contract)
    // 6) capacity is the same as allocation capacity (called contract)
    let _data = unsafe { Vec::from_raw_parts(data.ptr as *mut u8, data.size, data._cap) };
}

/// Clones the data pointer.
///
/// # Safety
/// - `data` must be a valid [`DataBuffer`] structure with valid data pointer.
#[no_mangle]
pub extern "C" fn clone_data(data: DataBuffer) -> DataBuffer {
    if data.ptr.is_null() {
        return NULL_BUF;
    }
    // SAFETY:
    // 1) ptr is allocated by the global allocator (Vec::new contract)
    // 2) deallocation allignment is the same as allocation allignment
    // 3) deallocation size is the same as allocation size
    // 4) length <= capacity (caller contract)
    // 5) data is properly initialized (Vec::new contract, caller contract)
    // 6) capacity is the same as allocation capacity (called contract)
    let data = std::mem::ManuallyDrop::new(unsafe {
        Vec::from_raw_parts(data.ptr as *mut u8, data.size, data._cap)
    });
    let data = std::mem::ManuallyDrop::new(data.clone());
    DataBuffer {
        ptr: data.as_ptr(),
        size: data.len(),
        _cap: data.capacity(),
    }
}

/// Clones the packet.
///
/// # Safety
/// `packet` must either be NULL or it must point to a valid [`Packet`] structure.
#[no_mangle]
pub extern "C" fn clone_packet(packet: Option<&Packet>) -> Option<Box<Packet>> {
    packet.cloned().map(Box::new)
}

/// Checks if the packet is empty.
///
/// # Safety
/// `packet` must either be NULL or it must point to a valid [`Packet`] structure.
#[no_mangle]
pub extern "C" fn packet_is_empty(packet: Option<&Packet>) -> bool {
    let Some(packet) = packet else { return false };
    let packet: &PacketEX = packet;
    matches!(packet, PacketEX::None)
}

/// Sets a new packet type.
///
/// # Safety
/// - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
/// - `packet_type` must be a valid variant of `PacketType`.
#[no_mangle]
pub extern "C" fn set_packet_type(worker: Option<&mut PacketWorker>, packet_type: PacketType) {
    if let Some(worker) = worker {
        worker.packet_type = packet_type.into();
    }
}

/// Sets a new serde format.
///
/// # Safety
/// - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
/// - `format` must be a valid variant of [`SerializedFormat`].
#[no_mangle]
pub extern "C" fn set_serde_format(worker: Option<&mut PacketWorker>, format: SerializedFormat) {
    if let Some(worker) = worker {
        worker.serde_format = format;
    }
}

/// Checks if the specified serde format is supported.
///
/// # Safety
/// `format` must be a valid variant of [`SerializedFormat`].
#[no_mangle]
pub const extern "C" fn serde_supported(serde_format: SerializedFormat) -> bool {
    serde_format.is_supported()
}

/// Parses raw packet data and returns a [`Packet`] type or a null pointer if an error occured.
///
/// # Safety
/// - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
/// - `data_ptr' must point to valid packet data up to `size` bytes.
#[no_mangle]
pub unsafe extern "C" fn raw_to_packet(
    worker: Option<&mut PacketWorker>,
    data_ptr: *const u8,
    size: usize,
) -> Option<Box<Packet>> {
    let worker = worker?;
    worker.err_str = None;
    if let Some(packet) = worker.packets.pop() {
        return Some(Box::new(packet.into()));
    }
    if data_ptr.is_null() {
        worker.err_str = Some(CString::new("No data provided").unwrap_or_default());
        return None;
    }
    // SAFETY: data is not null and valid for `size` bytes (caller contract)
    let data = unsafe { std::slice::from_raw_parts(data_ptr, size) };
    match worker.parse_packet_failable(data) {
        Ok(p) => Some(Box::new(p.into())),
        Err(e) => {
            worker.err_str = Some(CString::new(format!("{}", e)).unwrap_or_default());
            None
        }
    }
}

/// Parses serialized packet data and returns a [`Packet`] type or a null pointer if an error
/// occurred.
///
/// # Safety
/// - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
/// - `data_ptr' must point to valid serialied data up to `size` bytes.
#[no_mangle]
pub unsafe extern "C" fn ser_to_packet(
    worker: Option<&mut PacketWorker>,
    data_ptr: *const u8,
    size: usize,
) -> Option<Box<Packet>> {
    let worker = worker?;
    worker.err_str = None;
    if data_ptr.is_null() {
        worker.err_str = Some(CString::new("No data provided").unwrap_or_default());
        return None;
    }
    // SAFETY: data is not null and valid for `size` bytes (caller contract)
    let data = unsafe { std::slice::from_raw_parts(data_ptr, size) };
    match worker.serde_format.deserialize(data) {
        Ok(p) => Some(Box::new(p.into())),
        Err(e) => {
            worker.err_str = Some(CString::new(format!("{}", e)).unwrap_or_default());
            None
        }
    }
}

/// Parses [`Packet`] and returns raw packet data.
///
/// # Safety
/// - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
/// - If the returned array is empty, the pointer might be non-null but still invalid. This is not
///   considered an error.
#[no_mangle]
pub extern "C" fn packet_to_raw(
    worker: Option<&mut PacketWorker>,
    packet: Option<&Packet>,
) -> DataBuffer {
    let null = NULL_BUF;
    let Some(worker) = worker else {
        return null;
    };
    worker.err_str = None;
    let Some(packet) = packet else {
        worker.err_str = Some(CString::new("No packet provided").unwrap_or_default());
        return null;
    };
    let data = std::mem::ManuallyDrop::new(packet.write(worker.packet_type));
    DataBuffer {
        ptr: data.as_ptr(),
        size: data.len(),
        _cap: data.capacity(),
    }
}

/// Parses [`Packet`] and returns serialized packet data or a null pointer if an error occured.
///
/// # Safety
/// - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
/// - If the returned array is empty, the pointer might be non-null but still invalid. This is not
///   considered an error.
#[no_mangle]
pub extern "C" fn packet_to_ser(
    worker: Option<&mut PacketWorker>,
    packet: Option<&Packet>,
) -> DataBuffer {
    let null = NULL_BUF;
    let Some(worker) = worker else {
        return null;
    };
    worker.err_str = None;
    let Some(packet) = packet else {
        worker.err_str = Some(CString::new("No packet provided").unwrap_or_default());
        return null;
    };
    match worker.serde_format.serialize(packet) {
        Ok(d) => {
            let data = std::mem::ManuallyDrop::new(d);
            DataBuffer {
                ptr: data.as_ptr(),
                size: data.len(),
                _cap: data.capacity(),
            }
        }
        Err(e) => {
            worker.err_str = Some(CString::new(format!("{}", e)).unwrap_or_default());
            null
        }
    }
}

/// Parses packet data and returns a fat pointer to the serialized packet or a null pointer if
/// an error occurred.
///
/// # Safety
/// - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
/// - `data_ptr' must point to valid packet data up to `size` bytes.
/// - If the returned array is empty, the pointer might be non-null but still invalid. This is not
///   considered an error.
#[no_mangle]
pub unsafe extern "C" fn parse_packet(
    worker: Option<&mut PacketWorker>,
    data_ptr: *const u8,
    size: usize,
) -> DataBuffer {
    let null = NULL_BUF;
    let Some(worker) = worker else {
        return null;
    };
    // SAFETY: 
    // 1) 'worker' is valid (caller contract)
    // 2) `data_ptr` points to valid data up to `size` bytes (caller contract)
    let packet = unsafe { raw_to_packet(Some(worker), data_ptr, size) };
    if packet.is_none() {
        return null;
    }
    let packet = packet.as_ref().unwrap().as_ref();
    packet_to_ser(Some(worker), Some(packet))
}

/// Deserializes packet and returns a fat pointer to the packet data or a null pointer if an error
/// occured.
///
/// # Safety
/// - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
/// - `data_ptr' must point to valid packet data up to `size` bytes.
/// - If the returned array is empty, the pointer might be non-null but still invalid. This is not
///   considered an error.
#[no_mangle]
pub unsafe extern "C" fn create_packet(
    worker: Option<&mut PacketWorker>,
    data_ptr: *const u8,
    size: usize,
) -> DataBuffer {
    let null = NULL_BUF;
    let Some(worker) = worker else {
        return null;
    };
    // SAFETY: 
    // 1) 'worker' is valid (caller contract)
    // 2) `data_ptr` points to valid data up to `size` bytes (caller contract)
    let packet = unsafe { ser_to_packet(Some(worker), data_ptr, size) };
    if packet.is_none() {
        return null;
    }
    let packet = packet.as_ref().unwrap().as_ref();
    packet_to_raw(Some(worker), Some(packet))
}

/// Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
/// occurred.
///
/// # Safety
/// - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
/// - The returned pointer is only valid until the next failable function call.
#[no_mangle]
pub extern "C" fn get_pw_error(worker: Option<&PacketWorker>) -> *const u8 {
    match worker.and_then(|w| w.err_str.as_ref()) {
        Some(e) => e.as_ptr() as *const u8,
        None => std::ptr::null(),
    }
}

impl SerializedFormat {
    pub(crate) fn serialize(self, packet: &PacketEX) -> Result<Vec<u8>, Box<dyn Error>> {
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
    pub(crate) fn deserialize(self, data: &[u8]) -> Result<PacketEX, Box<dyn Error>> {
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
    const fn is_supported(self) -> bool {
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

impl From<PacketTypeEX> for PacketType {
    fn from(value: PacketTypeEX) -> Self {
        match value {
            PacketTypeEX::NGS => Self::NGS,
            PacketTypeEX::Classic => Self::Classic,
            PacketTypeEX::NA => Self::NA,
            PacketTypeEX::JP => Self::JP,
            PacketTypeEX::Vita => Self::Vita,
            PacketTypeEX::Raw => Self::Raw,
        }
    }
}

impl From<PacketEX> for Packet {
    fn from(value: PacketEX) -> Self {
        Self(value)
    }
}
impl std::ops::Deref for Packet {
    type Target = PacketEX;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PacketWorker {
    fn parse_packet_failable(&mut self, data: &[u8]) -> Result<PacketEX, Box<dyn Error>> {
        self.packets
            .append(&mut PacketEX::read(data, self.packet_type)?);
        Ok(self.packets.pop().unwrap_or(PacketEX::None))
    }
}
