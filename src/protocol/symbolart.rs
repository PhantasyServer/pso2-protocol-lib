//! Symbol Art related packets. \[0x2F\]
use super::{HelperReadWrite, MessageChannel, ObjectHeader, PacketReadWrite};

// ----------------------------------------------------------------
// Symbol Art packets
// ----------------------------------------------------------------

/// (0x2F, 0x00) Request Symbol Art Data (client).
///
/// (C -> S) Sent when the client encounters an unknown symbol art and wants to receive it's data.
///
/// Respond with: [`crate::protocol::Packet::SymbolArtClientData`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x00)]
pub struct SymbolArtClientDataRequestPacket {
    /// Symbol Art UUID.
    pub uuid: u128,
}

/// (0x2F, 0x01) Request Symbol Art Data (server).
///
/// (S -> C) Sent when the server encounters an unknown symbol art and wants to receive it's data.
///
/// Respond with: [`crate::protocol::Packet::SymbolArtData`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x01)]
pub struct SymbolArtDataRequestPacket {
    /// Symbol Art UUID.
    pub uuid: u128,
}

/// (0x2F, 0x02) Symbol Art Data (serverbound).
///
/// (C -> S) Sent in response to the reqeust
///
/// Response to: [`crate::protocol::Packet::SymbolArtDataRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x4B58, 0x76)]
pub struct SymbolArtDataPacket {
    /// Symbol Art UUID.
    pub uuid: u128,
    /// Symbol Art data.
    pub data: Vec<u8>,
    /// Symbol Art name.
    pub name: String,
}

/// (0x2F, 0x03) Symbol Art Data (clientbound).
///
/// (S -> C) Sent in response to the reqeust
///
/// Response to: [`crate::protocol::Packet::SymbolArtClientDataRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x03)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xD116, 0xC1)]
pub struct SymbolArtClientDataPacket {
    /// Symbol Art UUID.
    pub uuid: u128,
    /// Symbol Art data.
    pub data: Vec<u8>,
}

/// (0x2F, 0x04) Change Symbol Art Slot.
///
/// (C -> S) Sent when the client puts a new symbol art in a save slot.
///
/// Respond with: [`crate::protocol::Packet::SymbolArtResult`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x04)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x56D3, 0x0C)]
pub struct ChangeSymbolArtPacket {
    /// Symbol Art UUIDs and slot indexes.
    pub uuids: Vec<SlottedSymbolArt>,
}

/// (0x2F, 0x05) Change Symbol Art Slot Result.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::ChangeSymbolArt`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x05)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xDC91, 0x57)]
pub struct SymbolArtResultPacket {
    pub unk1: u32,
    pub uuids: Vec<u128>,
}

/// (0x2F, 0x07) Saved Symbol Art List.
///
/// (S -> C) Sent in response to the request.
///
/// Response to: [`crate::protocol::Packet::SymbolArtListRequest`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x07)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xE80C, 0xED)]
pub struct SymbolArtListPacket {
    /// Player object.
    pub object: ObjectHeader,
    /// Currenly selected character ID.
    pub character_id: u32,
    /// Saved UUIDs.
    pub uuids: Vec<u128>,
}

/// (0x2F, 0x08) Send Symbol Art.
///
/// (C -> S) Sent when a client sends a symbol art.
///
/// Respond with: [`crate::protocol::Packet::ReceiveSymbolArt`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x08)]
#[Flags(Flags {object_related: true, ..Default::default()})]
pub struct SendSymbolArtPacket {
    /// Sender object (unset).
    pub object: ObjectHeader,
    /// Symbol Art UUID.
    pub uuid: u128,
    /// Symbol Art channel.
    pub area: MessageChannel,
    pub unk1: u8,
    pub unk2: u16,
    pub unk3: u32,
}

/// (0x2F, 0x09) Received Symbol Art.
///
/// (S -> C) Sent when a client receives a symbol art.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x2F, 0x09)]
pub struct ReceiveSymbolArtPacket {
    /// Sender object.
    pub object: ObjectHeader,
    /// Symbol Art UUID.
    pub uuid: u128,
    /// Symbol Art channel.
    pub area: MessageChannel,
    pub unk1: u8,
    pub unk2: u16,
    pub unk3: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

/// Symbol Art in a saved SA list.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
pub struct SlottedSymbolArt {
    /// UUID of the Symbol Art.
    pub uuid: u128,
    /// Slot index.
    pub slot: u32,
}
