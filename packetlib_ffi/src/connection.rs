use crate::protocol::{DataBuffer, Packet};
use pso2packetlib::connection::ConnectionError;
use std::{
    error::Error,
    ffi::{CStr, CString},
    net::{TcpListener, TcpStream},
};

/// Result of socket operations.
#[repr(C)]
pub enum SocketResult {
    /// Socket/Data is ready.
    Ready,
    /// Socket would block.
    Blocked,
    /// No socket is actually open.
    NoSocket,
    /// Socket operation produced an error, call [`get_sf_error`] or [`get_conn_error`] to get an
    /// error message.
    SocketError,
}

/// Factory for [`Connection`]. Handles error messages, listen sockets and temporarily stores
/// accepted connections.
pub struct SocketFactory {
    err_str: Option<CString>,
    listener: Option<TcpListener>,
    stream: Option<TcpStream>,
}

/// Connection between a client and a server.
pub struct Connection {
    err_str: Option<CString>,
    con: Option<pso2packetlib::Connection<pso2packetlib::protocol::Packet>>,
    data: Option<Packet>,
}

/// Wrapper for [`pso2packetlib::PublicKey`]
pub struct PublicKey {
    key: pso2packetlib::PublicKey,
}

/// Wrapper for [`pso2packetlib::PrivateKey`]
pub struct PrivateKey {
    key: pso2packetlib::PrivateKey,
}

//------------------------------------------
// Helper functions for working with sockets
//------------------------------------------

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
///
/// # Safety
/// `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
#[no_mangle]
pub extern "C" fn free_factory(_factory: Option<Box<SocketFactory>>) {}

/// Creates a new listener on the specified address.
///
/// # Safety
/// - `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
/// - `addr` must be a valid NULL terminated string in the form of "ip:port"
#[no_mangle]
pub unsafe extern "C" fn create_listener(
    factory: Option<&mut SocketFactory>,
    addr: *const i8,
) -> bool {
    let Some(factory) = factory else { return false };
    factory.err_str = None;
    // SAFETY: `addr` is a valid NULL terminated string (caller contract)
    match unsafe { create_listener_failable(addr) } {
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
///
/// # Safety
/// `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
#[no_mangle]
pub extern "C" fn listener_nonblocking(factory: Option<&SocketFactory>, nonblocking: bool) {
    let _ = factory
        .and_then(|f| f.listener.as_ref())
        .and_then(|s| s.set_nonblocking(nonblocking).ok());
}

/// Accepts a new incoming connection from installed listener. To collect the resulting connection
/// call [`get_connection`] or [`stream_into_fd`].
///
/// # Safety
/// `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
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
/// call [`get_connection`] or [`stream_into_fd`].
///
/// # Safety
/// - `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
/// - `addr` must be a valid NULL terminated string in the form of "ip:port"
#[no_mangle]
pub unsafe extern "C" fn create_stream(
    factory: Option<&mut SocketFactory>,
    addr: *const i8,
) -> bool {
    let Some(factory) = factory else { return false };
    factory.err_str = None;
    // SAFETY: `addr` is a valid NULL terminated string (caller contract)
    match unsafe { create_stream_failable(addr) } {
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
///
/// # Safety
/// `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
#[no_mangle]
pub extern "C" fn stream_nonblocking(factory: Option<&mut SocketFactory>, nonblocking: bool) {
    let _ = factory
        .and_then(|f| f.stream.as_ref())
        .and_then(|s| s.set_nonblocking(nonblocking).ok());
}

/// Returns the IP address of the stream.
///
/// # Safety
/// `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
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
/// - `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
/// - `packet_type` must be a valid variant of `PacketType`.
/// - 'in_key' must either be null or it must point to a valid [`PrivateKey`] strucure
/// - 'out_key' must either be null or it must point to a valid [`PublicKey`] structure.
///
/// # Note
/// This function takes ownership of `in_key` and `out_key`.
#[no_mangle]
pub extern "C" fn get_connection(
    factory: Option<&mut SocketFactory>,
    packet_type: crate::protocol::PacketType,
    in_key: Option<Box<PrivateKey>>,
    out_key: Option<Box<PublicKey>>,
) -> Option<Box<Connection>> {
    let con = factory?.stream.take()?;
    let in_key = match in_key {
        Some(k) => k.key,
        None => pso2packetlib::PrivateKey::None,
    };
    let out_key = match out_key {
        Some(k) => k.key,
        None => pso2packetlib::PublicKey::None,
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
    }))
}

/// Returns an incoming connection descriptor. Caller is responsible for closing the returned descriptor.
/// If no stream was opened, returns -1.
///
/// # Safety
/// `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
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
///
/// # Safety
/// - `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
/// - `fd` must be a valid descriptor.
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
///
/// # Safety
/// `fd` must be a valid descriptor.
#[no_mangle]
pub extern "C" fn close_fd(fd: i64) {
    if fd == i64::MAX {
        return;
    }
    #[cfg(windows)]
    {
        use std::os::windows::io::{FromRawSocket, OwnedSocket, RawSocket};
        // SAFETY: fd is a valid descriptor that is owned (caller contract)
        unsafe { OwnedSocket::from_raw_socket(fd as RawSocket) };
    }
    #[cfg(not(windows))]
    {
        use std::os::fd::{FromRawFd, OwnedFd, RawFd};
        // SAFETY: fd is a valid descriptor that is owned (caller contract)
        unsafe { OwnedFd::from_raw_fd(fd as RawFd) };
    }
}

/// Returns an owned socket descriptor. Caller is responsible for closing the returned descriptor.
/// If no listener was opened, returns -1.
///
/// # Safety
/// `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
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
/// - `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
/// - `fd` must be a valid descriptor.
///
/// # Notes
/// This function takes ownership of `fd`.
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
        // SAFETY: fd is a valid descriptor that is owned (caller contract)
        factory.listener = Some(unsafe { TcpListener::from_raw_socket(fd as RawSocket) });
        true
    }
    #[cfg(not(windows))]
    {
        use std::os::fd::{FromRawFd, RawFd};
        // SAFETY: fd is a valid descriptor that is owned (caller contract)
        factory.listener = Some(unsafe { TcpListener::from_raw_fd(fd as RawFd) });
        true
    }
}

/// Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
/// occurred.
///
/// # Safety
/// - `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
/// - The returned pointer is only valid until the next failable function call.
#[no_mangle]
pub extern "C" fn get_sf_error(factory: Option<&SocketFactory>) -> *const u8 {
    match factory.and_then(|f| f.err_str.as_ref()) {
        Some(e) => e.as_ptr() as *const u8,
        None => std::ptr::null(),
    }
}

unsafe fn create_listener_failable(str: *const i8) -> Result<TcpListener, Box<dyn Error>> {
    // SAFETY: `str` points to a valid NULL terminated string.
    let str = unsafe { CStr::from_ptr(str) }.to_str()?;
    let listener = TcpListener::bind(str)?;
    Ok(listener)
}

unsafe fn create_stream_failable(str: *const i8) -> Result<TcpStream, Box<dyn Error>> {
    // SAFETY: `str` points to a valid NULL terminated string.
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
        // SAFETY: `fd` is a valid descriptor that will outlive this function call
        let socket = unsafe { BorrowedSocket::borrow_raw(fd as RawSocket) };
        let socket = socket.try_clone_to_owned()?;
        Ok(socket.into_raw_socket() as i64)
    }
    #[cfg(not(windows))]
    {
        use std::os::fd::{BorrowedFd, IntoRawFd, RawFd};
        // SAFETY: `fd` is a valid descriptor that will outlive this function call
        let fd = unsafe { BorrowedFd::borrow_raw(fd as RawFd) };
        let fd = fd.try_clone_to_owned()?;
        Ok(fd.into_raw_fd() as i64)
    }
}

/// Creates a new connection from owned socket descriptor.
///
/// # Safety
/// - `fd` must be a valid descriptor.
/// - `packet_type` must be a valid variant of `PacketType`.
/// - 'in_key' must either be null or it must point to a valid [`PrivateKey`] strucure
/// - 'out_key' must either be null or it must point to a valid [`PublicKey`] structure.
///
/// # Note
/// This function takes ownership of `in_key`, `out_key` and `fd`.
#[no_mangle]
pub extern "C" fn new_connection(
    fd: i64,
    packet_type: crate::protocol::PacketType,
    in_key: Option<Box<PrivateKey>>,
    out_key: Option<Box<PublicKey>>,
) -> Box<Connection> {
    let con = {
        #[cfg(windows)]
        {
            use std::os::windows::io::{FromRawSocket, RawSocket};
            // SAFETY: fd is a valid descriptor that is owned (caller contract)
            unsafe { TcpStream::from_raw_socket(fd as RawSocket) }
        }
        #[cfg(not(windows))]
        {
            use std::os::fd::{FromRawFd, RawFd};
            // SAFETY: fd is a valid descriptor that is owned (caller contract)
            unsafe { TcpStream::from_raw_fd(fd as RawFd) }
        }
    };
    let in_key = match in_key {
        Some(k) => k.key,
        None => pso2packetlib::PrivateKey::None,
    };
    let out_key = match out_key {
        Some(k) => k.key,
        None => pso2packetlib::PublicKey::None,
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
    })
}

/// Destroys a connection.
///
/// # Safety
/// `conn` must either be NULL or it must point to a valid [`Connection`] structure.
#[no_mangle]
pub extern "C" fn free_connection(_conn: Option<Box<Connection>>) {}

/// Returns the IP address of the connection.
///
/// # Safety
/// `conn` must either be NULL or it must point to a valid [`Connection`] structure.
#[no_mangle]
pub extern "C" fn get_conn_ip(conn: Option<&Connection>) -> u32 {
    conn.and_then(|c| c.con.as_ref())
        .and_then(|c| c.get_ip().ok())
        .map(|i| u32::from_be_bytes(i.octets()))
        .unwrap_or(0)
}

/// Changes the connection's packet type.
///
/// # Safety
/// - `conn` must either be NULL or it must point to a valid [`Connection`] structure.
/// - `packet_type` must be a valid variant of `PacketType`.
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
/// - `conn` must either be NULL or it must point to a valid [`Connection`] structure.
/// - The returned pointer is only valid until the next data-returning function call.
#[no_mangle]
pub extern "C" fn conn_get_data(conn: Option<&mut Connection>) -> Option<Box<Packet>> {
    match conn {
        Some(c) => c.data.take().map(Box::new),
        None => None,
    }
}

/// Reads a packet from the connection and stores it in the internal buffer. Call [`conn_get_data`]
/// to access it.
///
/// # Safety
/// `conn` must either be NULL or it must point to a valid [`Connection`] structure.
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
/// # Safety
/// - `conn` must either be NULL or it must point to a valid [`Connection`] structure.
/// - `packet` must either be NULL or it must point to a valid [`Packet`] structure.
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
///
/// # Safety
/// `conn` must either be NULL or it must point to a valid [`Connection`] structure.
#[no_mangle]
pub extern "C" fn conn_get_key(conn: Option<&mut Connection>) -> DataBuffer {
    let Some(conn) = conn else {
        return crate::protocol::NULL_BUF;
    };
    match conn.con.as_mut().map(|c| c.get_key()) {
        Some(key) => {
            let key = std::mem::ManuallyDrop::new(key);
            DataBuffer {
                ptr: key.as_ptr(),
                size: key.len(),
                _cap: key.capacity(),
            }
        }
        None => crate::protocol::NULL_BUF,
    }
}

/// Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
/// occurred.
///
/// # Safety
/// - `conn` must either be NULL or it must point to a valid [`Connection`] structure.
/// - The returned pointer is only valid until the next failable function call.
#[no_mangle]
pub extern "C" fn get_conn_error(conn: Option<&Connection>) -> *const u8 {
    match conn.and_then(|c| c.err_str.as_ref()) {
        Some(e) => e.as_ptr() as *const u8,
        None => std::ptr::null(),
    }
}

/// Creates a new public key from PEM-encoded PKCS#8 file.
///
/// # Safety
/// `path` must either be NULL or it must point to a valid NULL terminated string.
#[no_mangle]
pub unsafe extern "C" fn new_pub_key_file(path: *const i8) -> Option<Box<PublicKey>> {
    if path.is_null() {
        return None;
    }
    // SAFETY: `path` is a valid NULL terminated string (caller contract)
    let path = unsafe { CStr::from_ptr(path) }.to_str().ok()?;
    Some(Box::new(PublicKey {
        key: pso2packetlib::PublicKey::Path(path.into()),
    }))
}

/// Creates a new private key from PEM-encoded PKCS#8 file.
///
/// # Safety
/// `path` must either be NULL or it must point to a valid NULL terminated string.
#[no_mangle]
pub unsafe extern "C" fn new_priv_key_file(path: *const i8) -> Option<Box<PrivateKey>> {
    if path.is_null() {
        return None;
    }
    // SAFETY: `path` is a valid NULL terminated string (caller contract)
    let path = unsafe { CStr::from_ptr(path) }.to_str().ok()?;
    Some(Box::new(PrivateKey {
        key: pso2packetlib::PrivateKey::Path(path.into()),
    }))
}

/// Creates a new public key from RSA parameters.
///
/// # Arguments
/// - `n` - RSA modulus
/// - `e` - RSA public exponent
///
/// # Safety
/// - `n` must either be NULL or it must point to a valid byte array up to `n_size` bytes.
/// - `e` must either be NULL or it must point to a valid byte array up to `e_size` bytes.
#[no_mangle]
pub unsafe extern "C" fn new_pub_key_params(
    n: *const u8,
    n_size: usize,
    e: *const u8,
    e_size: usize,
) -> Option<Box<PublicKey>> {
    let n = if !n.is_null() {
        // SAFETY: `n` points to a valid array up to `n_size` bytes (caller contract)
        unsafe { std::slice::from_raw_parts(n, n_size) }.to_vec()
    } else {
        vec![]
    };
    let e = if !e.is_null() {
        // SAFETY: `e` points to a valid array up to `e_size` bytes (caller contract)
        unsafe { std::slice::from_raw_parts(e, e_size) }.to_vec()
    } else {
        vec![]
    };
    Some(Box::new(PublicKey {
        key: pso2packetlib::PublicKey::Params { n, e },
    }))
}

/// Creates a new private key from RSA parameters.
///
/// # Arguments
/// - `n` - RSA modulus
/// - `e` - RSA public exponent
/// - `d` - RSA private exponent
/// - `p` - RSA first prime
/// - `q` - RSA second prime
///
/// # Safety
/// - `n` must either be NULL or it must point to a valid byte array up to `n_size` bytes.
/// - `e` must either be NULL or it must point to a valid byte array up to `e_size` bytes.
/// - `d` must either be NULL or it must point to a valid byte array up to `d_size` bytes.
/// - `p` must either be NULL or it must point to a valid byte array up to `p_size` bytes.
/// - `q` must either be NULL or it must point to a valid byte array up to `q_size` bytes.
#[no_mangle]
pub unsafe extern "C" fn new_priv_key_params(
    n: *const u8,
    n_size: usize,
    e: *const u8,
    e_size: usize,
    d: *const u8,
    d_size: usize,
    p: *const u8,
    p_size: usize,
    q: *const u8,
    q_size: usize,
) -> Option<Box<PrivateKey>> {
    let n = if !n.is_null() {
        // SAFETY: `n` points to a valid array up to `n_size` bytes (caller contract)
        unsafe { std::slice::from_raw_parts(n, n_size) }.to_vec()
    } else {
        vec![]
    };
    let e = if !e.is_null() {
        // SAFETY: `e` points to a valid array up to `e_size` bytes (caller contract)
        unsafe { std::slice::from_raw_parts(e, e_size) }.to_vec()
    } else {
        vec![]
    };
    let d = if !d.is_null() {
        // SAFETY: `d` points to a valid array up to `d_size` bytes (caller contract)
        unsafe { std::slice::from_raw_parts(d, d_size) }.to_vec()
    } else {
        vec![]
    };
    let p = if !p.is_null() {
        // SAFETY: `p` points to a valid array up to `p_size` bytes (caller contract)
        unsafe { std::slice::from_raw_parts(p, p_size) }.to_vec()
    } else {
        vec![]
    };
    let q = if !q.is_null() {
        // SAFETY: `q` points to a valid array up to `q_size` bytes (caller contract)
        unsafe { std::slice::from_raw_parts(q, q_size) }.to_vec()
    } else {
        vec![]
    };
    Some(Box::new(PrivateKey {
        key: pso2packetlib::PrivateKey::Params { n, e, d, p, q },
    }))
}

/// Destroys a public key
///
/// # Safety
/// `key` must either be NULL or it must point to a valid [`PublicKey`] structure.
#[no_mangle]
pub extern "C" fn free_pub_key(_key: Option<Box<PublicKey>>) {}

/// Destroys a private key
///
/// # Safety
/// `key` must either be NULL or it must point to a valid [`PrivateKey`] structure.
#[no_mangle]
pub extern "C" fn free_priv_key(_key: Option<Box<PrivateKey>>) {}

fn conn_read_packet_failable(conn: &mut Connection) -> Result<SocketResult, Box<dyn Error>> {
    let packet = match conn.con.as_mut().unwrap().read_packet() {
        Ok(p) => p,
        Err(ConnectionError::Io(e)) if e.kind() == std::io::ErrorKind::WouldBlock => {
            return Ok(SocketResult::Blocked)
        }
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
        Err(ConnectionError::Io(e)) if e.kind() == std::io::ErrorKind::WouldBlock => {
            return Ok(SocketResult::Blocked)
        }
        Err(e) => return Err(e.into()),
    };
    Ok(SocketResult::Ready)
}
