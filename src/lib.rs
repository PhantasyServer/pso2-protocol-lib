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
/// [`protocol::PacketReadWrite`].
/// - Raw packet must either have no fields or only one with a [`Vec<u8>`] inside.
/// - Unknown packet must either have no fields or only one with a tuple of
/// ([`protocol::PacketHeader`], [`Vec<u8>`]) inside.
///
/// # Attribute explanation
/// - `#[Id(_id_, _subid_)]` sets the ID and subID of the packet variant.
/// - `#[Empty]` marks the variant as empty, i.e. it will always return an empty vec.
/// - `#[Raw]` marks the variant that will receive raw data if requested.
/// - `#[Unknown]` marks the variant that will receive unknown packets.
/// - `#[NGS]` marks the packet as NGS-only.
/// - `#[Classic]` marks the packet as classic only, i.e. non-NGS packet (Vita, JP, NA).
/// - `#[NA]` marks the packet as NA classic only.
/// - `#[JP]` marks the packet as JP classic only.
/// - `#[Vita]` marks the packet as Vita only.
/// - `#[Category(_category_)]` sets the category of all the packets following this attribute.
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use packetlib_impl::ProtocolRW;

/// Derive macro for [`protocol::PacketReadWrite`].
///
/// # Note
/// This macro makes few assumtions about the packet struct:
/// - The only container type currently allowed is [`Vec<T>`].
/// - Any type that is not hardcoded (i.e integers, floats, [`half::f16`], [`std::net::Ipv4Addr`],
/// [`std::time::Duration`], [`String`], [`AsciiString`]) must implement
/// [`protocol::HelperReadWrite`] or have the `read`, `write` functions with the same prototype.
///
/// # Attribute explanation
/// ## Container attributes
/// - `#[Id(_id_, _subid_)]` sets the ID and subID of the packet.
/// - `#[Flags(_`[`protocol::Flags`]`_)]` sets the flags of the packet.
/// - `#[Magic(_xor_, _sub_)]`. If the `packed` flag is set, then this attribute sets the
/// deciphering xor and sub for variable length types.
/// ## Field attributes
/// - `#[Seek(_seek-amount_)]` sets the padding before the field data.
/// - `#[SeekAfter(_seek-amount_)]` sets the padding after the field data.
/// - `#[Const_u16(_const-int_)]` sets the constant u16 before the field data.
/// - `#[PSOTime]`. Assumes the following [`std::time::Duration`] is stored as a Windows filetime.
/// - `#[Len_u16]`. Assumes that the length for the following variable length type is stored in a
/// u16 preceding the actual data.
/// - `#[Len_u32]`. Assumes that the length for the following variable length type is stored in a
/// u32 preceding the actual data.
/// - `#[FixedLen(_len_)]` sets the length for the following variable length type.
/// - `#[OnlyOn(_`[`protocol::PacketType`]`_)]`. If set then the field will only be read/written if
/// the reader packet type matches the specified packet type.
/// - `#[NotOn(_`[`protocol::PacketType`]`_)]`. If set then the field will only be read/written if
/// the reader packet type differs from the specified packet type.
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use packetlib_impl::PacketRW;

/// Derive macro for [`protocol::HelperReadWrite`].
///
/// # Note
/// This macro makes few assumtions about
/// 1) the packet struct:
/// - The only container type currently allowed is [`Vec<T>`].
/// - Any type that is not hardcoded (i.e integers, floats, [`half::f16`], [`std::net::Ipv4Addr`],
/// [`std::time::Duration`], [`String`], [`AsciiString`]) must implement
/// [`protocol::HelperReadWrite`] or have the `read`, `write` functions with the same prototype.
/// 2) the flags struct:
/// - All fields must be of type [`bool`]
/// 3) the variant enum:
/// - None of the fields must contain any data.
/// - `#[repr(_)]` must be set to an integer.
/// - Enum must implement [`Copy`].
///
/// # Attribute explanation
/// ## Container attributes
/// - `#[Flags(u*)]` makes the struct into a flags struct with the specified length.
/// - `#[NoPadding]` disables the 4-byte alligning padding after the type.
/// ## Field attributes
/// - `#[Seek(_seek-amount_)]` sets the padding before the field data.
/// - `#[SeekAfter(_seek-amount_)]` sets the padding after the field data.
/// - `#[Const_u16(_const-int_)]` sets the constant u16 before the field data.
/// - `#[PSOTime]`. Assumes the following [`std::time::Duration`] is stored as a Windows filetime.
/// - `#[Len_u16]`. Assumes that the length for the following variable length type is stored in a
/// u16 preceding the actual data.
/// - `#[Len_u32]`. Assumes that the length for the following variable length type is stored in a
/// u32 preceding the actual data.
/// - `#[FixedLen(_len_)]` sets the length for the following variable length type.
/// - `#[Read_default]` sets the default enum variant for reading.
/// - `#[Skip]`. If applied to a field struct field, then this attribute will skip one bit of the
/// flags.
/// - `#[ManualRW(_readfn_, _writefn_)]` sets the read/write functions for the variant. Specified
/// functions must have the same prototype as the [`protocol::HelperReadWrite`] functions.
/// - `#[OnlyOn(_`[`protocol::PacketType`]`_)]`. If set then the field will only be read/written if
/// the reader packet type matches the specified packet type.
/// - `#[NotOn(_`[`protocol::PacketType`]`_)]`. If set then the field will only be read/written if
/// the reader packet type differs from the specified packet type.
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use packetlib_impl::HelperRW;
