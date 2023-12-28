#![cfg_attr(docsrs, feature(doc_cfg))]

pub(crate) mod asciistring;
#[cfg(feature = "connection")]
pub(crate) mod connection;
#[cfg(feature = "connection")]
pub(crate) mod encryption;
// #[cfg(test)]
// pub mod ppac;
#[cfg(feature = "ppac")]
#[cfg_attr(docsrs, doc(cfg(feature = "ppac")))]
pub mod ppac;
pub mod protocol;

#[cfg(feature = "connection")]
#[cfg_attr(docsrs, doc(cfg(feature = "connection")))]
pub use connection::{Connection, PrivateKey, PublicKey};

pub use asciistring::AsciiString;
