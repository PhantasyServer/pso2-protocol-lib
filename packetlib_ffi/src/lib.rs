#![deny(unsafe_op_in_unsafe_fn)]

pub mod protocol;

#[cfg(feature = "connection")]
pub mod connection;
#[cfg(feature = "ppac")]
pub mod ppac;

pub const API_VERSION: u32 = 5;
pub const PROTOCOL_VERSION: u32 = 4;

#[no_mangle]
pub extern "C" fn get_api_version() -> u32 {
    API_VERSION
}

#[no_mangle]
pub extern "C" fn get_protocol_version() -> u32 {
    PROTOCOL_VERSION
}

/// Returns whether the library is built with connection support.
#[no_mangle]
pub extern "C" fn have_connection() -> bool {
    #[cfg(feature = "connection")]
    {
        true
    }
    #[cfg(not(feature = "connection"))]
    {
        false
    }
}

/// Returns whether the library is built with PPAC support.
#[no_mangle]
pub extern "C" fn have_ppac() -> bool {
    #[cfg(feature = "ppac")]
    {
        true
    }
    #[cfg(not(feature = "ppac"))]
    {
        false
    }
}
