#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::missing_const_for_fn)]

pub mod protocol;

#[cfg(feature = "connection")]
pub mod connection;
#[cfg(feature = "ppac")]
pub mod ppac;

/// Current library version.
pub const LIBRARY_VERSION: u32 = 4;

/// Returns the compiled library version.
#[no_mangle]
pub const extern "C" fn get_library_version() -> u32 {
    LIBRARY_VERSION
}

/// Returns whether the library is built with connection support.
#[no_mangle]
pub const extern "C" fn have_connection() -> bool {
    cfg!(feature = "connection")
}

/// Returns whether the library is built with PPAC support.
#[no_mangle]
pub const extern "C" fn have_ppac() -> bool {
    cfg!(feature = "ppac")
}
