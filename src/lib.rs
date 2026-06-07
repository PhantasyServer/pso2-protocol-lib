#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(unsafe_code)]
#![warn(clippy::future_not_send)]

pub mod asciistring;
#[cfg(feature = "connection")]
pub mod connection;
#[cfg(feature = "connection")]
pub(crate) mod encryption;
pub mod fixed_types;
#[cfg(feature = "ppac")]
#[cfg_attr(docsrs, doc(cfg(feature = "ppac")))]
pub mod ppac;
pub mod protocol;

#[doc(hidden)]
pub mod derive_reexports;

#[cfg(feature = "connection")]
#[cfg_attr(docsrs, doc(cfg(feature = "connection")))]
pub use connection::{Connection, PrivateKey, PublicKey};

pub use asciistring::AsciiString;

/// Derive macro for [`protocol::ProtocolRW`].
///
/// # Note
/// This macro makes few assumtions about the protocol enum:
/// - All packet must either have no fields or only one with a type that implements
///   [`protocol::PacketReadWrite`].
/// - Raw packet must either have no fields or only one with a [`Vec<u8>`] inside.
/// - Unknown packet must either have no fields or only one with a tuple of
///   ([`protocol::PacketHeader`], [`Vec<u8>`]) inside.
///
/// # Enum attribute explanation
/// - `#[pso2packet(gen_tests)]` generates a test function for each packet that tests that writing
/// and then reading produces the same packet.
///
/// # Field attribute explanation
/// - `#[pso2packet(id(_id_, _subid_))]` sets the ID and subID of the packet variant.
/// - `#[pso2packet(empty)]` marks the variant as empty, i.e. it will always return an empty vec.
/// - `#[pso2packet(raw)]` marks the variant that will receive raw data if requested.
/// - `#[pso2packet(unknown)]` marks the variant that will receive unknown packets.
/// - `#[pso2packet(NGS)]` marks the packet as NGS-only.
/// - `#[pso2packet(Classic)]` marks the packet as classic only, i.e. non-NGS packet (Vita, JP, NA).
/// - `#[pso2packet(NA)]` marks the packet as NA classic only.
/// - `#[pso2packet(JP)]` marks the packet as JP classic only.
/// - `#[pso2packet(Vita)]` marks the packet as Vita only.
/// - `#[pso2packet(category(_category_))]` sets the category of all the packets following this attribute.
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use pso2packetlib_impl::ProtocolRW;

/// Derive macro for [`protocol::PacketReadWrite`].
///
/// # Note
/// This macro makes few assumtions about the packet struct:
/// - All types used must implement [`protocol::HelperReadWrite`] or use
///   `#[pso2packet(manual_rw(..))]` attribute.
///
/// # Attribute explanation
/// ## Container attributes
/// - `#[pso2packet(id(_id_, _subid_))]` sets the ID and subID of the packet.
/// - `#[pso2packet(flags(_`[`protocol::Flags`]`_))]` sets the flags of the packet.
/// - `#[pso2packet(magic(_xor_, _sub_))]`. If the `packed` flag is set, then this attribute sets
///   the deciphering xor and sub for variable length types.
/// ## Field attributes
/// - `#[pso2packet(only_on(_`[`protocol::PacketType`]`_))]`. If set then the field will only be
///   read/written if the reader packet type matches the specified packet type.
/// - `#[pso2packet(not_on(_`[`protocol::PacketType`]`_))]`. If set then the field will only be
///   read/written if the reader packet type differs from the specified packet type.
/// - `#[pso2packet(manual_rw(_readfn_, _writefn_))]` sets the read/write functions for the field.
///   Specified functions must have the same prototype as the [`protocol::HelperReadWrite`]
///   functions.
/// - `#[pso2packet(seek(_seek-amount_))]` sets the padding before the field data.
/// - `#[pso2packet(seek_after(_seek-amount_))]` sets the padding after the field data.
/// - `#[pso2packet(const_u16(_const-int_))]` sets the constant u16 before the field data.
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use pso2packetlib_impl::PacketRW;

/// Derive macro for [`protocol::HelperReadWrite`].
///
/// # Note
/// This macro makes few assumtions about
/// 1) the packet struct:
/// - All types used must implement [`protocol::HelperReadWrite`] or use
///   `#[pso2packet(manual_rw(..))]` attribute.
/// attribute.
/// 2) the variant enum:
/// - None of the fields must contain any data.
/// - `#[repr(_)]` must be set to an integer.
/// - Enum must implement [`Copy`].
///
/// # Attribute explanation
/// ## Container attributes
/// - `#[pso2packet(bitflags(u*))]` adds read/write support for [`bitflags`] flags containers.
/// ## Struct field attributes
/// - `#[pso2packet(only_on(_`[`protocol::PacketType`]`_))]`. If set then the field will only be
///   read/written if the reader packet type matches the specified packet type.
/// - `#[pso2packet(not_on(_`[`protocol::PacketType`]`_))]`. If set then the field will only be
///   read/written if the reader packet type differs from the specified packet type.
/// - `#[pso2packet(manual_rw(_readfn_, _writefn_))]` sets the read/write functions for the field.
///   Specified functions must have the same prototype as the [`protocol::HelperReadWrite`]
///   functions.
/// - `#[pso2packet(seek(_seek-amount_))]` sets the padding before the field data.
/// - `#[pso2packet(seek_after(_seek-amount_))]` sets the padding after the field data.
/// - `#[pso2packet(const_u16(_const-int_))]` sets the constant u16 before the field data.
/// ## Enum field attributes
/// - `#[pso2packet(read_default)]` sets the default enum variant for reading.
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use pso2packetlib_impl::HelperRW;

#[cfg(docsrs)]
pub mod protocol_docs;
