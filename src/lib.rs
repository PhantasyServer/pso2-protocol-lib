#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(unsafe_code)]

pub(crate) mod asciistring;
#[cfg(feature = "connection")]
pub(crate) mod connection;
#[cfg(feature = "connection")]
pub(crate) mod encryption;
#[cfg(any(feature = "ppac", test))]
#[cfg_attr(docsrs, doc(cfg(feature = "ppac")))]
pub mod ppac;
pub mod protocol;
#[cfg(all(feature = "connection", feature = "proxy"))]
pub(crate) mod proxy_connection;

#[cfg(feature = "connection")]
#[cfg_attr(docsrs, doc(cfg(feature = "connection")))]
pub use connection::{Connection, PrivateKey};
#[cfg(all(feature = "connection", feature = "proxy"))]
#[cfg_attr(docsrs, doc(cfg(all(feature = "connection", feature = "proxy"))))]
pub use proxy_connection::{ProxyConnection, PublicKey};
#[cfg(all(feature = "connection", feature = "proxy", feature = "tokio"))]
#[cfg_attr(
    docsrs,
    doc(cfg(all(feature = "connection", feature = "proxy", feature = "tokio")))
)]
pub use proxy_connection::{ProxyRead, ProxyWrite};

pub use asciistring::AsciiString;
