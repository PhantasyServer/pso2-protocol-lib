#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(unsafe_code)]
#![warn(clippy::future_not_send)]

pub mod asciistring;
#[cfg(feature = "connection")]
pub mod connection;
#[cfg(feature = "connection")]
pub(crate) mod encryption;
#[cfg(feature = "ppac")]
#[cfg_attr(docsrs, doc(cfg(feature = "ppac")))]
pub mod ppac;
pub mod protocol;

#[cfg(all(feature = "connection", feature = "proxy"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "connection", feature = "proxy"))))]
pub use connection::proxy::{ProxyConnection, PublicKey};
#[cfg(all(feature = "proxy", feature = "split_connection"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "proxy", feature = "split_connection"))))]
pub use connection::proxy::{ProxyRead, ProxyWrite};
#[cfg(feature = "connection")]
#[cfg_attr(docsrs, doc(cfg(feature = "connection")))]
pub use connection::{Connection, PrivateKey};

pub use asciistring::AsciiString;
