use std::{
    error::Error,
    ffi::{CStr, CString},
    net::{TcpListener, TcpStream},
};

use pso2packetlib::{PrivateKey, PublicKey};

use crate::protocol::{DataBuffer, Packet};

#[repr(C)]
pub enum SocketResult {
    Ready,
    Blocked,
    NoSocket,
    SocketError,
}

//------------------------------------------
// Helper functions for working with sockets
//------------------------------------------

pub struct SocketFactory {
    err_str: Option<CString>,
    listener: Option<TcpListener>,
    stream: Option<TcpStream>,
}

/// Creates a new socket factory.
#[no_mangle]
pub extern "C" fn new_factory() -> Box<SocketFactory> {
    Box::new(SocketFactory {
        err_str: None,
        listener: None,
        stream: None,
    })
}

/// Destroys a socket factory.
#[no_mangle]
pub extern "C" fn free_factory(_factory: Option<Box<SocketFactory>>) {}

/// Creates a new listener on the specified address.
#[no_mangle]
pub extern "C" fn create_listener(factory: Option<&mut SocketFactory>, addr: *const i8) -> bool {
    let Some(factory) = factory else { return false };
    factory.err_str = None;
    match create_listener_failable(addr) {
        Ok(listener) => {
            factory.listener = Some(listener);
            true
        }
        Err(e) => {
            factory.err_str = Some(CString::new(e.to_string()).unwrap_or_default());
            false
        }
    }
}

/// Sets the blocking mode of the listener.
#[no_mangle]
pub extern "C" fn listener_nonblocking(factory: Option<&SocketFactory>, nonblocking: bool) {
    let _ = factory
        .and_then(|f| f.listener.as_ref())
        .and_then(|s| s.set_nonblocking(nonblocking).ok());
}

/// Accepts a new incoming connection from installed listener. To collect the resulting connection
/// call `get_connection` or `stream_into_fd".
#[no_mangle]
pub extern "C" fn accept_listener(factory: Option<&mut SocketFactory>) -> SocketResult {
    let Some(factory) = factory else {
        return SocketResult::SocketError;
    };
    factory.err_str = None;
    let Some(listener) = factory.listener.as_ref() else {
        return SocketResult::NoSocket;
    };
    match listener.accept() {
        Ok((s, _)) => {
            factory.stream = Some(s);
            SocketResult::Ready
        }
        Err(x) if x.kind() == std::io::ErrorKind::WouldBlock => SocketResult::Blocked,
        Err(e) => {
            factory.err_str = Some(CString::new(e.to_string()).unwrap_or_default());
            SocketResult::SocketError
        }
    }
}

/// Creates a new stream to the specified address. To collect the resulting stream
/// call `get_connection` or `stream_into_fd".
#[no_mangle]
pub extern "C" fn create_stream(factory: Option<&mut SocketFactory>, addr: *const i8) -> bool {
    let Some(factory) = factory else { return false };
    factory.err_str = None;
    match create_stream_failable(addr) {
        Ok(stream) => {
            factory.stream = Some(stream);
            true
        }
        Err(e) => {
            factory.err_str = Some(CString::new(e.to_string()).unwrap_or_default());
            false
        }
    }
}

/// Sets the blocking mode of the stream.
#[no_mangle]
pub extern "C" fn stream_nonblocking(factory: Option<&mut SocketFactory>, nonblocking: bool) {
    let _ = factory
        .and_then(|f| f.stream.as_ref())
        .and_then(|s| s.set_nonblocking(nonblocking).ok());
}

/// Returns the IP address of the stream.
#[no_mangle]
pub extern "C" fn get_stream_ip(factory: Option<&SocketFactory>) -> u32 {
    let Some(addr) = factory
        .and_then(|f| f.stream.as_ref())
        .and_then(|s| s.peer_addr().ok())
    else {
        return 0;
    };
    match addr.ip() {
        std::net::IpAddr::V4(x) => u32::from_be_bytes(x.octets()),
        std::net::IpAddr::V6(_) => 0,
    }
}

/// Creates a new connection from incoming connection.
///
/// # Safety
/// 'in_key' must either be null or it must point to a UTF-8-encoded, zero-terminated
/// path to a PKCS#8 file containing a private key for decryption.
/// 'out_key' must either be null or it must point to a UTF-8-encoded, zero-terminated
/// path to a PKCS#8 file containing a public key for encryption.
#[no_mangle]
pub unsafe extern "C" fn get_connection(
    factory: Option<&mut SocketFactory>,
    packet_type: crate::protocol::PacketType,
    in_key: *const i8,
    out_key: *const i8,
) -> Option<Box<Connection>> {
    let Some(factory) = factory else {
        return None;
    };
    let Some(con) = factory.stream.take() else {
        return None;
    };
    let in_key = if !in_key.is_null() {
        PrivateKey::Path(
            unsafe { CStr::from_ptr(in_key) }
                .to_string_lossy()
                .to_string()
                .into(),
        )
    } else {
        PrivateKey::None
    };
    let out_key = if !out_key.is_null() {
        PublicKey::Path(
            unsafe { CStr::from_ptr(out_key) }
                .to_string_lossy()
                .to_string()
                .into(),
        )
    } else {
        PublicKey::None
    };
    Some(Box::new(Connection {
        err_str: None,
        con: Some(pso2packetlib::Connection::new(
            con,
            packet_type.into(),
            in_key,
            out_key,
        )),
        data: None,
        key: vec![],
    }))
}

/// Returns an incoming connection descriptor. Caller is responsible for closing the returned descriptor.
/// If no stream was opened, returns -1.
#[no_mangle]
pub extern "C" fn stream_into_fd(factory: Option<&mut SocketFactory>) -> i64 {
    let Some(factory) = factory else {
        return -1;
    };
    match factory.stream.take() {
        Some(stream) => {
            #[cfg(windows)]
            {
                use std::os::windows::io::IntoRawSocket;
                stream.into_raw_socket() as i64
            }
            #[cfg(not(windows))]
            {
                use std::os::fd::IntoRawFd;
                stream.into_raw_fd() as i64
            }
        }
        None => -1,
    }
}

/// Clones the descriptor. Returns the cloned descriptor or -1 if an error occurred.
#[no_mangle]
pub extern "C" fn clone_fd(factory: Option<&mut SocketFactory>, fd: i64) -> i64 {
    let Some(factory) = factory else {
        return -1;
    };
    factory.err_str = None;
    match copy_fd_failable(fd) {
        Ok(fd) => fd,
        Err(e) => {
            factory.err_str = Some(CString::new(e.to_string()).unwrap_or_default());
            -1
        }
    }
}

/// Closes the file descriptor.
#[no_mangle]
pub extern "C" fn close_fd(fd: i64) {
    if fd == i64::MAX {
        return;
    }
    #[cfg(windows)]
    {
        use std::os::windows::io::{FromRawSocket, OwnedSocket, RawSocket};
        unsafe { OwnedSocket::from_raw_socket(fd as RawSocket) };
    }
    #[cfg(not(windows))]
    {
        use std::os::fd::{FromRawFd, OwnedFd, RawFd};
        unsafe { OwnedFd::from_raw_fd(fd as RawFd) };
    }
}

/// Returns an owned socket descriptor. Caller is responsible for closing the returned descriptor.
/// If no listener was opened, returns -1.
#[no_mangle]
pub extern "C" fn listener_into_fd(factory: Option<&mut SocketFactory>) -> i64 {
    let Some(factory) = factory else {
        return -1;
    };
    match factory.listener.take() {
        Some(listener) => {
            #[cfg(windows)]
            {
                use std::os::windows::io::IntoRawSocket;
                listener.into_raw_socket() as i64
            }
            #[cfg(not(windows))]
            {
                use std::os::fd::IntoRawFd;
                listener.into_raw_fd() as i64
            }
        }
        None => -1,
    }
}

/// Installs the provided listener. This function takes ownership of the descriptor.
///
/// # Safety
/// `fd` must be a valid descriptor.
#[no_mangle]
pub unsafe extern "C" fn listener_from_fd(factory: Option<&mut SocketFactory>, fd: i64) -> bool {
    let Some(factory) = factory else {
        return false;
    };
    if fd == i64::MAX {
        return false;
    }
    #[cfg(windows)]
    {
        use std::os::windows::io::{FromRawSocket, RawSocket};
        factory.listener = Some(unsafe { TcpListener::from_raw_socket(fd as RawSocket) });
        true
    }
    #[cfg(not(windows))]
    {
        use std::os::fd::{FromRawFd, RawFd};
        factory.listener = Some(unsafe { TcpListener::from_raw_fd(fd as RawFd) });
        true
    }
}

/// Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
/// occurred.
///
/// # Safety
/// The returned pointer is only valid until the next failable function call.
#[no_mangle]
pub extern "C" fn get_sf_error(factory: Option<&SocketFactory>) -> *const u8 {
    match factory.and_then(|f| f.err_str.as_ref()) {
        Some(e) => e.as_ptr() as *const u8,
        None => std::ptr::null(),
    }
}

fn create_listener_failable(str: *const i8) -> Result<TcpListener, Box<dyn Error>> {
    let str = unsafe { CStr::from_ptr(str) }.to_str()?;
    let listener = TcpListener::bind(str)?;
    Ok(listener)
}

fn create_stream_failable(str: *const i8) -> Result<TcpStream, Box<dyn Error>> {
    let str = unsafe { CStr::from_ptr(str) }.to_str()?;
    let listener = TcpStream::connect(str)?;
    Ok(listener)
}

fn copy_fd_failable(fd: i64) -> Result<i64, Box<dyn Error>> {
    if fd == -1 {
        return Ok(-1);
    }
    #[cfg(windows)]
    {
        use std::os::windows::io::{BorrowedSocket, IntoRawSocket, RawSocket};
        let socket = unsafe { BorrowedSocket::borrow_raw(fd as RawSocket) };
        let socket = socket.try_clone_to_owned()?;
        Ok(socket.into_raw_socket() as i64)
    }
    #[cfg(not(windows))]
    {
        use std::os::fd::{BorrowedFd, IntoRawFd, RawFd};
        let fd = unsafe { BorrowedFd::borrow_raw(fd as RawFd) };
        let fd = fd.try_clone_to_owned()?;
        Ok(fd.into_raw_fd() as i64)
    }
}

pub struct Connection {
    err_str: Option<CString>,
    con: Option<pso2packetlib::Connection<pso2packetlib::protocol::Packet>>,
    data: Option<Packet>,
    key: Vec<u8>,
}

/// Creates a new connection from owned socket descriptor.
///
/// # Safety
/// `fd` must be a valid descriptor.
///
/// 'in_key' must either be null or it must point to a UTF-8-encoded, zero-terminated
/// path to a PKCS#8 file containing a private key for decryption.
/// 'out_key' must either be null or it must point to a UTF-8-encoded, zero-terminated
/// path to a PKCS#8 file containing a public key for encryption.
#[no_mangle]
pub unsafe extern "C" fn new_connection(
    fd: i64,
    packet_type: crate::protocol::PacketType,
    in_key: *const i8,
    out_key: *const i8,
) -> Box<Connection> {
    let con = {
        #[cfg(windows)]
        {
            use std::os::windows::io::{FromRawSocket, RawSocket};
            unsafe { TcpStream::from_raw_socket(fd as RawSocket) }
        }
        #[cfg(not(windows))]
        {
            use std::os::fd::{FromRawFd, RawFd};
            unsafe { TcpStream::from_raw_fd(fd as RawFd) }
        }
    };
    let in_key = if !in_key.is_null() {
        PrivateKey::Path(
            unsafe { CStr::from_ptr(in_key) }
                .to_string_lossy()
                .to_string()
                .into(),
        )
    } else {
        PrivateKey::None
    };
    let out_key = if !out_key.is_null() {
        PublicKey::Path(
            unsafe { CStr::from_ptr(out_key) }
                .to_string_lossy()
                .to_string()
                .into(),
        )
    } else {
        PublicKey::None
    };
    Box::new(Connection {
        err_str: None,
        con: Some(pso2packetlib::Connection::new(
            con,
            packet_type.into(),
            in_key,
            out_key,
        )),
        data: None,
        key: vec![],
    })
}

/// Destroys a connection.
#[no_mangle]
pub extern "C" fn free_connection(_conn: Option<Box<Connection>>) {}

/// Returns the IP address of the connection.
#[no_mangle]
pub extern "C" fn get_conn_ip(conn: Option<&Connection>) -> u32 {
    conn.and_then(|c| c.con.as_ref())
        .and_then(|c| c.get_ip().ok())
        .map(|i| u32::from_be_bytes(i.octets()))
        .unwrap_or(0)
}

/// Changes the connection's packet type.
#[no_mangle]
pub extern "C" fn conn_set_packet_type(
    conn: Option<&mut Connection>,
    packet_type: crate::protocol::PacketType,
) {
    if let Some(conn) = conn.and_then(|c| c.con.as_mut()) {
        conn.change_packet_type(packet_type.into());
    }
}

/// Returns a [`Packet`] or a null pointer if no connection was provided.
///
/// # Safety
/// The returned pointer is only valid until the next data-returning function call.
/// If the returned array is empty, the pointer might be non-null but still invalid. This is not
/// considered an error.
#[no_mangle]
pub extern "C" fn conn_get_data(conn: Option<&mut Connection>) -> Option<Box<Packet>> {
    match conn {
        Some(c) => c.data.take().map(Box::new),
        None => None,
    }
}

/// Reads a packet from the connection and stores it in the internal buffer. Call `conn_get_data`
/// to access it.
#[no_mangle]
pub extern "C" fn conn_read_packet(conn: Option<&mut Connection>) -> SocketResult {
    let Some(conn) = conn else {
        return SocketResult::NoSocket;
    };
    if conn.con.is_none() {
        return SocketResult::NoSocket;
    }
    match conn_read_packet_failable(conn) {
        Ok(r) => r,
        Err(e) => {
            conn.err_str = Some(CString::new(format!("{}", e)).unwrap_or_default());
            SocketResult::SocketError
        }
    }
}

/// Writes a packet to the connection. If `ptr` is null, flushes the buffer.
///
/// # Note
/// If this function returns [`SocketResult::Blocked`], then the data has been written to the
/// buffer.
#[no_mangle]
pub extern "C" fn conn_write_packet(
    conn: Option<&mut Connection>,
    packet: Option<&Packet>,
) -> SocketResult {
    let Some(conn) = conn else {
        return SocketResult::NoSocket;
    };
    if conn.con.is_none() {
        return SocketResult::NoSocket;
    }
    match conn_write_packet_failable(conn, packet) {
        Ok(r) => r,
        Err(e) => {
            conn.err_str = Some(CString::new(format!("{}", e)).unwrap_or_default());
            SocketResult::SocketError
        }
    }
}

/// Returns the encryption key (for [`Packet::EncryptionResponse`]).
#[no_mangle]
pub extern "C" fn conn_get_key(conn: Option<&mut Connection>) -> DataBuffer {
    let Some(conn) = conn else {
        return crate::protocol::NULL_BUF;
    };
    match conn.con.as_mut().map(|c| c.get_key()) {
        Some(key) => {
            conn.key = key;
            DataBuffer {
                ptr: conn.key.as_ptr(),
                size: conn.key.len(),
            }
        }
        None => crate::protocol::NULL_BUF,
    }
}

/// Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
/// occurred.
///
/// # Safety
/// The returned pointer is only valid until the next failable function call.
#[no_mangle]
pub extern "C" fn get_conn_error(conn: Option<&Connection>) -> *const u8 {
    match conn.and_then(|c| c.err_str.as_ref()) {
        Some(e) => e.as_ptr() as *const u8,
        None => std::ptr::null(),
    }
}

fn conn_read_packet_failable(conn: &mut Connection) -> Result<SocketResult, Box<dyn Error>> {
    let packet = match conn.con.as_mut().unwrap().read_packet() {
        Ok(p) => p,
        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => return Ok(SocketResult::Blocked),
        Err(e) => return Err(e.into()),
    };
    conn.data = Some(packet.into());
    Ok(SocketResult::Ready)
}

fn conn_write_packet_failable(
    conn: &mut Connection,
    packet: Option<&Packet>,
) -> Result<SocketResult, Box<dyn Error>> {
    let data = match packet {
        Some(p) => p,
        None => &pso2packetlib::protocol::Packet::None,
    };
    match conn.con.as_mut().unwrap().write_packet(data) {
        Ok(_) => {}
        Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => return Ok(SocketResult::Blocked),
        Err(e) => return Err(e.into()),
    };
    Ok(SocketResult::Ready)
}
