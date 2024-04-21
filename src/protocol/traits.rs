use super::{Flags, PacketCategory, PacketError, PacketType};
use std::io::{Read, Seek, Write};

/// Trait for manipulating encryption data.
pub trait PacketEncryption {
    /// Returns `true` is the packet contains RSA data (i.e is [`crate::protocol::Packet::EncryptionRequest`]).
    fn is_enc_data(&self) -> bool;
    /// Returns a refrence to the RSA encrypted data.
    fn as_enc_data(&self) -> Option<&[u8]>;
    /// Returns a mutable refrence to the RSA encrypted data.
    fn mut_enc_data(&mut self) -> Option<&mut Vec<u8>>;
}

/// Read/Write trait for packet enums.
pub trait ProtocolRW: PacketEncryption + Sized {
    /// Reads packets from an input slice.
    fn read(input: &[u8], packet_type: PacketType) -> Result<Vec<Self>, PacketError>;
    /// Writes a packet to a byte vector.
    fn write(&self, packet_type: PacketType) -> Vec<u8>;
    /// Returns category of the packet.
    fn get_category(&self) -> PacketCategory;
}

/// Read/Write trait for packet data containing structs.
pub trait PacketReadWrite: Sized {
    /// Reads a packet from a stream.
    fn read(
        reader: &mut (impl Read + Seek),
        flags: &Flags,
        packet_type: PacketType,
    ) -> Result<Self, PacketError>;
    /// Writes a packet to a Vec.
    fn write(&self, packet_type: PacketType) -> Result<Vec<u8>, PacketError>;
}

/// Read/Write trait for aditional data structs/enums.
pub trait HelperReadWrite: Sized {
    /// Reads data from a stream.
    fn read(
        reader: &mut (impl Read + Seek),
        packet_type: PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<Self, PacketError>;
    /// Writes data to a stream.
    fn write(
        &self,
        writer: &mut impl Write,
        packet_type: PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<(), PacketError>;
}
