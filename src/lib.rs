#![cfg_attr(docsrs, feature(doc_cfg))]

pub(crate) mod asciistring;
#[cfg(feature = "connection")]
pub(crate) mod connection;
#[cfg(feature = "connection")]
pub(crate) mod encryption;
pub mod protocol;

#[cfg(feature = "connection")]
#[cfg_attr(docsrs, doc(cfg(feature = "connection")))]
pub use connection::Connection;

pub use asciistring::AsciiString;
