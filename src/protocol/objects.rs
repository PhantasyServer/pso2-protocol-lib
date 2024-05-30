//! Object related packets. \[0x04\]
use super::{
    models::Position, Flags, ObjectHeader, PacketError, PacketHeader, PacketReadWrite, PacketType,
};
use crate::AsciiString;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use half::f16;
use std::{
    io::{Read, Seek, Write},
    time::Duration,
};

// ----------------------------------------------------------------
// Object related packets
// ----------------------------------------------------------------

/// (0x04, 0x02) Object Teleport Location
///
/// (S -> C) Sent when the client is teleported.
///
/// Response to: [`crate::protocol::Packet::Interact`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x02)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct TeleportTransferPacket {
    pub unk1: [u8; 0xC],
    /// Object that started the teleportation.
    pub source_tele: ObjectHeader,
    /// New location.
    pub location: Position,
    pub unk2: u16,
}

/// (0x04, 0x06) Item Picked Up.
///
/// (S -> C) Sent when players pickup items.
///
/// Response to: [`crate::protocol::Packet::ItemPickupRequest`] and ???
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x06)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct ItemPickedUpPacket {
    /// Player that picked up the item. (?)
    pub player: ObjectHeader,
    /// Item object that was picked up.
    pub item: ObjectHeader,
}

/// (0x04, 0x07) Object Movement. (broadcast)
///
/// (Bidirectional) Sent when players (or objects?) move.
///
/// Response to: [`crate::protocol::Packet::Movement`] (C->S)
/// Respond with: [`crate::protocol::Packet::Movement`] (S->C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MovementPacket {
    pub unk: [u8; 0x6],
    pub ent1_id: Option<u64>,
    pub ent1_type: Option<u16>,
    pub ent1_unk: Option<u16>,
    pub ent2_id: Option<u64>,
    pub ent2_type: Option<u16>,
    pub ent2_unk: Option<u16>,
    /// Timestamp of action.
    pub timestamp: Option<Duration>,
    /// X quaternion rotation.
    pub rot_x: Option<f16>,
    /// Y quaternion rotation.
    pub rot_y: Option<f16>,
    /// Z quaternion rotation.
    pub rot_z: Option<f16>,
    /// W quaternion rotation.
    pub rot_w: Option<f16>,
    /// Current x position (i.e. new position).
    pub cur_x: Option<f16>,
    /// Current y position (i.e. new position).
    pub cur_y: Option<f16>,
    /// Current z position (i.e. new position).
    pub cur_z: Option<f16>,
    pub unk1: Option<f16>,
    pub unk_x: Option<f16>,
    pub unk_y: Option<f16>,
    pub unk_z: Option<f16>,
    pub unk2: Option<f16>,
    pub unk3: Option<u32>,
    pub unk4: Option<u8>,
}

/// (0x04, 0x08) Client Movement Action.
///
/// (C -> S) Sent when players does some action (e.g. jumping or attacking).
///
/// Respond with: [`crate::protocol::Packet::MovementActionServer`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x08)]
#[Flags(Flags::PACKED | Flags::OBJECT_RELATED)]
#[Magic(0x922D, 0x45)]
pub struct MovementActionPacket {
    pub unk1: ObjectHeader,
    /// Object that performed an action.
    pub performer: ObjectHeader,
    pub unk3: u32,
    pub unk4: [u8; 0x10],
    pub unk5: [u8; 0x8],
    pub unk6: [u8; 0xC],
    /// Name of performed action.
    pub action: AsciiString,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: Vec<u32>,
    pub unk10: u32,
}

/// (0x04, 0x0F) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x0F)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct Unk040FPacket {
    pub unk1: ObjectHeader,
    pub unk2: ObjectHeader,
    pub unk3: ObjectHeader,
    pub unk4: u16,
    pub unk5: u16,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: f16,
    pub unk10: f16,
    pub unk11: f16,
    pub unk12: u16,
    pub unk13: u16,
    pub unk14: u16,
    pub unk15: u16,
    pub unk16: u16,
    pub unk17: u32,
    pub unk18: u32,
}

/// (0x04, 0x13) Unknown.
///
/// (C -> S)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x13)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct Unk0413Packet {
    pub unk1: [u8; 0xC],
    pub unk2: ObjectHeader,
    pub unk3: ObjectHeader,
    pub unk4: u32,
}

/// (0x04, 0x14) Client Interaction.
///
/// (C -> S) Sent when players interacts with some object.
///
/// Respond with: [`crate::protocol::Packet::SetTag`] (optional)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x14)]
#[Flags(Flags::PACKED | Flags::OBJECT_RELATED)]
#[Magic(0xD711, 0xCA)]
pub struct InteractPacket {
    pub unk1: [u8; 0xC],
    /// Target. (?)
    pub object1: ObjectHeader,
    pub unk2: [u8; 0x4],
    pub object3: ObjectHeader,
    pub object4: [u8; 0x10],
    /// Name of the action.
    pub action: AsciiString,
}

/// (0x04, 0x15) Object Action or Set Object Tag. (unicast or broadcast)
///
/// (S -> C) Sent when object performs some action or it has new tag (usually after interaction).
///
/// Response to: [`crate::protocol::Packet::Interact`] (optional)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x15)]
#[Flags(Flags::PACKED | Flags::OBJECT_RELATED)]
#[Magic(0x5CCF, 0x15)]
pub struct SetTagPacket {
    /// Player that receives this packet.
    pub receiver: ObjectHeader,
    /// Target object.
    pub target: ObjectHeader,
    pub unk1: u32,
    pub object3: ObjectHeader,
    pub object4: ObjectHeader,
    pub unk2: u8,
    pub unk3: u8,
    pub unk4: u8,
    pub unk5: u8,
    /// Name of the action or tag.
    pub attribute: AsciiString,
}

/// (0x04, 0x22) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x22)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct Unk0422Packet {
    pub unk1: ObjectHeader,
    pub unk2: ObjectHeader,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: [u8; 0x20],
}

/// (0x04, 0x23) Unknown.
///
/// (C -> S)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x23)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct Unk0423Packet {
    pub unk1: [u8; 0xC],
    pub unk2: ObjectHeader,
    pub unk3: ObjectHeader,
    pub unk4: u32,
}

/// (0x04, 0x24) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x24)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct Unk0424Packet {
    pub unk1: ObjectHeader,
    pub unk2: ObjectHeader,
    pub unk3: ObjectHeader,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: [u8; 0xC],
    pub unk7: [u8; 0xC],
}

/// (0x04, 0x25) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x25)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct Unk0425Packet {
    pub unk1: ObjectHeader,
    pub unk2: ObjectHeader,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
}

/// (0x04, 0x2B) Unknown.
///
/// (C -> S)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x2B)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct Unk042BPacket {
    pub unk1: [u8; 0xC],
    pub unk2: ObjectHeader,
}

/// (0x04, 0x2E) Load Learned Photon Arts. (broadcast)
///
/// (S -> C) Sent on any character spawning to list learned photon arts.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x2E)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct LoadPAsPacket {
    /// Player that receives this packet.
    pub receiver: ObjectHeader,
    /// Player that has this data.
    pub target: ObjectHeader,
    /// Levels for 0xEE(?) PAs.
    #[FixedLen(0xEE)]
    pub levels: Vec<u8>,
    #[FixedLen(0x40)]
    pub unk: Vec<u8>,
}

/// (0x04, 0x3B) Remove Object. (broadcast)
///
/// (S -> C) Sent when object gets deleted (e.g player disconnects).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x3B)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct RemoveObjectPacket {
    /// Player that receives this packet.
    pub receiver: ObjectHeader,
    /// Object that got removed.
    pub removed_object: ObjectHeader,
}

/// (0x04, 0x3C) Client Action Update.
///
/// (C -> S) Sent when player wants to update action data.
///
/// Respond with: [`crate::protocol::Packet::ActionUpdateServer`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x3C)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct ActionUpdatePacket {
    pub unk1: ObjectHeader,
    /// Object that performed this action.
    pub performer: ObjectHeader,
    pub unk2: [u8; 0x20],
}

/// (0x04, 0x52) Damage Received.
///
/// (S -> C) Sent when object receives damage (including healing and selfdamage).
///
/// Response to: [`crate::protocol::Packet::DealDamage`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x52)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct DamageReceivePacket {
    /// Player that receives this packet.
    pub receiver: ObjectHeader,
    /// Object that receives this damage.
    pub dmg_target: ObjectHeader,
    /// Object that deals this damage.
    pub dmg_inflicter: ObjectHeader,
    /// Inflicted damage ID.
    pub damage_id: u32,
    /// How much damage was inflicted.
    pub dmg_amount: i32,
    /// New HP.
    pub new_hp: u32,
    /// Hitbox ID (?).
    pub hitbox_id: u32,
    /// Hit x position.
    pub x_pos: f16,
    /// Hit y position.
    pub y_pos: f16,
    /// Hit z position.
    pub z_pos: f16,
    pub unk4: [u8; 0xE],
    pub unk5: u32,
}

/// (0x04, 0x71) Object Movement End. (broadcast)
///
/// (Bidirectional) Sent when players (or objects?) stop moving.
///
/// Response to: [`crate::protocol::Packet::MovementEnd`] (C->S)
/// Respond with: [`crate::protocol::Packet::MovementEnd`] (S->C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x71)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct MovementEndPacket {
    pub unk1: ObjectHeader,
    pub unk2: ObjectHeader,
    pub unk3: u32,
    /// Current position of the object.
    pub cur_pos: Position,
    pub unk5: u16,
    pub unk_x: f16,
    pub unk_y: f16,
    pub unk_z: f16,
    pub unk7: u16,
    pub unk8: u32,
}

/// (0x04, 0x75) Action End. (broadcast)
///
/// (Bidirectional) Sent when objects stop an action.
///
/// Response to: [`crate::protocol::Packet::ActionEnd`] (C->S)
/// Respond with: [`crate::protocol::Packet::ActionEnd`] (S->C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x75)]
#[Flags(Flags::PACKED | Flags::OBJECT_RELATED)]
#[Magic(0x83EF, 0x40)]
pub struct ActionEndPacket {
    pub unk1: [u8; 0xC],
    /// Object that was performing an action.
    pub performer: ObjectHeader,
    pub unk2: u32,
    pub unk3: ObjectHeader,
    pub unk4: ObjectHeader,
    pub unk5: [u8; 4],
    /// Name of an action.
    pub action: AsciiString,
}

/// (0x04, 0x79) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x79)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct Unk0479Packet {
    pub unk1: ObjectHeader,
    pub unk2: ObjectHeader,
    pub unk3: u32,
}

/// (0x04, 0x80) Movement Action Response. (broadcast)
///
/// (S -> C) Sent when players does some action (e.g. jumping or attacking).
///
/// Response to: [`crate::protocol::Packet::MovementAction`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x80)]
#[Flags(Flags::PACKED | Flags::OBJECT_RELATED)]
#[Magic(0x4315, 0x7A)]
pub struct MovementActionServerPacket {
    /// Player that receives this packet.
    pub receiver: ObjectHeader,
    /// Object that performed this action.
    pub performer: ObjectHeader,
    pub unk3: u32,
    pub unk4: [u8; 0x10],
    pub unk5: [u8; 0x8],
    pub unk6: [u8; 0xC],
    /// Name of an action.
    pub action: AsciiString,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: Vec<u32>,
    pub unk10: u32,
}

/// (0x04, 0x81) Action Update Response. (broadcast)
///
/// (S -> C) Sent when player wants to update action data.
///
/// Response to: [`crate::protocol::Packet::ActionUpdate`]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x81)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct ActionUpdateServerPacket {
    /// Player that receives this packet.
    pub receiver: ObjectHeader,
    /// Object that performed this action.
    pub performer: ObjectHeader,
    pub unk2: [u8; 0x20],
}

/// (0x04, 0x86) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0x86)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct Unk0486Packet {
    pub unk1: ObjectHeader,
    pub unk2: ObjectHeader,
}

/// (0x04, 0xB0) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0xB0)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct Unk04B0Packet {
    pub unk1: ObjectHeader,
    pub unk2: ObjectHeader,
    pub unk3: [u8; 0xC],
    pub unk4: u32,
}

/// (0x04, 0xEA) Unknown.
///
/// (S -> C)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x04, 0xEA)]
#[Flags(Flags::OBJECT_RELATED)]
pub struct Unk04EAPacket {
    pub unk1: ObjectHeader,
    pub unk2: ObjectHeader,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: [u8; 0xC],
    pub unk7: u32,
    pub unk8: [u8; 0x14],
    pub unk9: u16,
    pub unk10: u16,
    pub unk11: u32,
}

// ----------------------------------------------------------------
// Read/Write implementations
// ----------------------------------------------------------------

//yikes
impl PacketReadWrite for MovementPacket {
    fn read(
        reader: &mut (impl Read + Seek),
        flags: &Flags,
        _: PacketType,
    ) -> Result<Self, PacketError> {
        let mut packet = Self::default();
        reader
            .read_exact(&mut packet.unk)
            .map_err(|e| PacketError::FieldError {
                packet_name: "MovementPacket",
                field_name: "unk",
                error: e,
            })?;
        if flags.contains(Flags::FULL_MOVEMENT) {
            packet.ent1_id =
                Some(
                    reader
                        .read_u64::<LittleEndian>()
                        .map_err(|e| PacketError::FieldError {
                            packet_name: "MovementPacket",
                            field_name: "ent1_id",
                            error: e,
                        })?,
                );
            packet.ent1_type =
                Some(
                    reader
                        .read_u16::<LittleEndian>()
                        .map_err(|e| PacketError::FieldError {
                            packet_name: "MovementPacket",
                            field_name: "ent1_type",
                            error: e,
                        })?,
                );
            packet.ent1_unk =
                Some(
                    reader
                        .read_u16::<LittleEndian>()
                        .map_err(|e| PacketError::FieldError {
                            packet_name: "MovementPacket",
                            field_name: "ent1_unk",
                            error: e,
                        })?,
                );
            packet.ent2_id =
                Some(
                    reader
                        .read_u64::<LittleEndian>()
                        .map_err(|e| PacketError::FieldError {
                            packet_name: "MovementPacket",
                            field_name: "ent2_id",
                            error: e,
                        })?,
                );
            packet.ent2_type =
                Some(
                    reader
                        .read_u16::<LittleEndian>()
                        .map_err(|e| PacketError::FieldError {
                            packet_name: "MovementPacket",
                            field_name: "ent2_type",
                            error: e,
                        })?,
                );
            packet.ent2_unk =
                Some(
                    reader
                        .read_u16::<LittleEndian>()
                        .map_err(|e| PacketError::FieldError {
                            packet_name: "MovementPacket",
                            field_name: "ent2_unk",
                            error: e,
                        })?,
                );
            packet.timestamp = Some(Duration::from_secs(
                reader
                    .read_u32::<LittleEndian>()
                    .map_err(|e| PacketError::FieldError {
                        packet_name: "MovementPacket",
                        field_name: "timestamp",
                        error: e,
                    })? as u64,
            ));
            packet.rot_x = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "rot_x",
                    error: e,
                },
            )?));
            packet.rot_y = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "rot_y",
                    error: e,
                },
            )?));
            packet.rot_z = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "rot_z",
                    error: e,
                },
            )?));
            packet.rot_w = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "rot_w",
                    error: e,
                },
            )?));
            packet.cur_x = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "cur_x",
                    error: e,
                },
            )?));
            packet.cur_y = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "cur_y",
                    error: e,
                },
            )?));
            packet.cur_z = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "cur_z",
                    error: e,
                },
            )?));
            packet.unk1 = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "unk1",
                    error: e,
                },
            )?));
            packet.unk_x = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "unk_x",
                    error: e,
                },
            )?));
            packet.unk_y = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "unk_y",
                    error: e,
                },
            )?));
            packet.unk_z = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "unk_z",
                    error: e,
                },
            )?));
            packet.unk2 = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "unk2",
                    error: e,
                },
            )?));
            packet.unk3 =
                Some(
                    reader
                        .read_u32::<LittleEndian>()
                        .map_err(|e| PacketError::FieldError {
                            packet_name: "MovementPacket",
                            field_name: "unk3",
                            error: e,
                        })?,
                );
            return Ok(packet);
        }
        let flags = reader
            .read_u24::<LittleEndian>()
            .map_err(|e| PacketError::FieldError {
                packet_name: "MovementPacket",
                field_name: "flags",
                error: e,
            })?;
        if flags & 0x1 != 0 {
            packet.ent1_id =
                Some(
                    reader
                        .read_u64::<LittleEndian>()
                        .map_err(|e| PacketError::FieldError {
                            packet_name: "MovementPacket",
                            field_name: "ent1_id",
                            error: e,
                        })?,
                );
        }
        if flags & 0x2 != 0 {
            packet.ent1_type =
                Some(
                    reader
                        .read_u16::<LittleEndian>()
                        .map_err(|e| PacketError::FieldError {
                            packet_name: "MovementPacket",
                            field_name: "ent1_type",
                            error: e,
                        })?,
                );
        }
        if flags & 0x4 != 0 {
            packet.ent1_unk =
                Some(
                    reader
                        .read_u16::<LittleEndian>()
                        .map_err(|e| PacketError::FieldError {
                            packet_name: "MovementPacket",
                            field_name: "ent1_unk",
                            error: e,
                        })?,
                );
        }
        if flags & 0x8 != 0 {
            packet.ent2_id =
                Some(
                    reader
                        .read_u64::<LittleEndian>()
                        .map_err(|e| PacketError::FieldError {
                            packet_name: "MovementPacket",
                            field_name: "ent2_id",
                            error: e,
                        })?,
                );
        }
        if flags & 0x10 != 0 {
            packet.ent2_type =
                Some(
                    reader
                        .read_u16::<LittleEndian>()
                        .map_err(|e| PacketError::FieldError {
                            packet_name: "MovementPacket",
                            field_name: "ent2_type",
                            error: e,
                        })?,
                );
        }
        if flags & 0x20 != 0 {
            packet.ent2_unk =
                Some(
                    reader
                        .read_u16::<LittleEndian>()
                        .map_err(|e| PacketError::FieldError {
                            packet_name: "MovementPacket",
                            field_name: "ent2_unk",
                            error: e,
                        })?,
                );
        }
        if flags & 0x40 != 0 {
            packet.timestamp = Some(Duration::from_secs(
                reader
                    .read_u32::<LittleEndian>()
                    .map_err(|e| PacketError::FieldError {
                        packet_name: "MovementPacket",
                        field_name: "timestamp",
                        error: e,
                    })? as u64,
            ));
        }
        if flags & 0x80 != 0 {
            packet.rot_x = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "rot_x",
                    error: e,
                },
            )?));
        }
        if flags & 0x100 != 0 {
            packet.rot_y = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "rot_y",
                    error: e,
                },
            )?));
        }
        if flags & 0x200 != 0 {
            packet.rot_z = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "rot_z",
                    error: e,
                },
            )?));
        }
        if flags & 0x400 != 0 {
            packet.rot_w = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "rot_w",
                    error: e,
                },
            )?));
        }
        if flags & 0x800 != 0 {
            packet.cur_x = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "cur_x",
                    error: e,
                },
            )?));
        }
        if flags & 0x1000 != 0 {
            packet.cur_y = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "cur_y",
                    error: e,
                },
            )?));
        }
        if flags & 0x2000 != 0 {
            packet.cur_z = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "cur_z",
                    error: e,
                },
            )?));
        }
        if flags & 0x4000 != 0 {
            packet.unk1 = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "unk1",
                    error: e,
                },
            )?));
        }
        if flags & 0x8000 != 0 {
            packet.unk_x = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "unk_x",
                    error: e,
                },
            )?));
        }
        if flags & 0x10000 != 0 {
            packet.unk_y = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "unk_y",
                    error: e,
                },
            )?));
        }
        if flags & 0x20000 != 0 {
            packet.unk_z = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "unk_z",
                    error: e,
                },
            )?));
        }
        if flags & 0x40000 != 0 {
            packet.unk2 = Some(f16::from_bits(reader.read_u16::<LittleEndian>().map_err(
                |e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "unk2",
                    error: e,
                },
            )?));
        }
        if flags & 0x80000 != 0 {
            if flags & 0x100000 != 0 {
                packet.unk4 = Some(reader.read_u8().map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "unk4",
                    error: e,
                })?);
            } else {
                packet.unk3 = Some(reader.read_u32::<LittleEndian>().map_err(|e| {
                    PacketError::FieldError {
                        packet_name: "MovementPacket",
                        field_name: "unk3",
                        error: e,
                    }
                })?);
            }
        }
        Ok(packet)
    }
    fn write(&self, packet_type: PacketType) -> Result<Vec<u8>, PacketError> {
        let mut tmp_buf = vec![];
        let mut flags = 0u32;
        if let Some(n) = self.ent1_id {
            tmp_buf
                .write_u64::<LittleEndian>(n)
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "ent1_id",
                    error: e,
                })?;
            flags += 0x1;
        }
        if let Some(n) = self.ent1_type {
            tmp_buf
                .write_u16::<LittleEndian>(n)
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "ent1_type",
                    error: e,
                })?;
            flags += 0x2;
        }
        if let Some(n) = self.ent1_unk {
            tmp_buf
                .write_u16::<LittleEndian>(n)
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "ent1_unk",
                    error: e,
                })?;
            flags += 0x4;
        }
        if let Some(n) = self.ent2_id {
            tmp_buf
                .write_u64::<LittleEndian>(n)
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "ent2_id",
                    error: e,
                })?;
            flags += 0x8;
        }
        if let Some(n) = self.ent2_type {
            tmp_buf
                .write_u16::<LittleEndian>(n)
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "ent2_type",
                    error: e,
                })?;
            flags += 0x10;
        }
        if let Some(n) = self.ent2_unk {
            tmp_buf
                .write_u16::<LittleEndian>(n)
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "ent2_unk",
                    error: e,
                })?;
            flags += 0x20;
        }
        if let Some(x) = self.timestamp {
            tmp_buf
                .write_u32::<LittleEndian>(x.as_secs() as u32)
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "timestamp",
                    error: e,
                })?;
            flags += 0x40;
        }
        if let Some(n) = self.rot_x {
            tmp_buf
                .write_u16::<LittleEndian>(n.to_bits())
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "rot_x",
                    error: e,
                })?;
            flags += 0x80;
        }
        if let Some(n) = self.rot_y {
            tmp_buf
                .write_u16::<LittleEndian>(n.to_bits())
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "rot_y",
                    error: e,
                })?;
            flags += 0x100;
        }
        if let Some(n) = self.rot_z {
            tmp_buf
                .write_u16::<LittleEndian>(n.to_bits())
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "rot_z",
                    error: e,
                })?;
            flags += 0x200;
        }
        if let Some(n) = self.rot_w {
            tmp_buf
                .write_u16::<LittleEndian>(n.to_bits())
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "rot_w",
                    error: e,
                })?;
            flags += 0x400;
        }
        if let Some(n) = self.cur_x {
            tmp_buf
                .write_u16::<LittleEndian>(n.to_bits())
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "cur_x",
                    error: e,
                })?;
            flags += 0x800;
        }
        if let Some(n) = self.cur_y {
            tmp_buf
                .write_u16::<LittleEndian>(n.to_bits())
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "cur_y",
                    error: e,
                })?;
            flags += 0x1000;
        }
        if let Some(n) = self.cur_z {
            tmp_buf
                .write_u16::<LittleEndian>(n.to_bits())
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "cur_z",
                    error: e,
                })?;
            flags += 0x2000;
        }
        if let Some(n) = self.unk1 {
            tmp_buf
                .write_u16::<LittleEndian>(n.to_bits())
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "unk1",
                    error: e,
                })?;
            flags += 0x4000;
        }
        if let Some(n) = self.unk_x {
            tmp_buf
                .write_u16::<LittleEndian>(n.to_bits())
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "unk_x",
                    error: e,
                })?;
            flags += 0x8000;
        }
        if let Some(n) = self.unk_y {
            tmp_buf
                .write_u16::<LittleEndian>(n.to_bits())
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "unk_y",
                    error: e,
                })?;
            flags += 0x10000;
        }
        if let Some(n) = self.unk_z {
            tmp_buf
                .write_u16::<LittleEndian>(n.to_bits())
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "unk_z",
                    error: e,
                })?;
            flags += 0x20000;
        }
        if let Some(n) = self.unk2 {
            tmp_buf
                .write_u16::<LittleEndian>(n.to_bits())
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "unk2",
                    error: e,
                })?;
            flags += 0x40000;
        }
        if let Some(n) = self.unk4 {
            tmp_buf.write_u8(n).map_err(|e| PacketError::FieldError {
                packet_name: "MovementPacket",
                field_name: "unk4",
                error: e,
            })?;
            flags += 0x180000;
        } else if let Some(n) = self.unk3 {
            tmp_buf
                .write_u32::<LittleEndian>(n)
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "unk3",
                    error: e,
                })?;
            flags += 0x80000;
        }
        let mut buf = if flags == 0xFFFFF {
            PacketHeader::new(
                0x04,
                0x07,
                Flags::OBJECT_RELATED | Flags::FLAG_10 | Flags::FULL_MOVEMENT,
            )
            .write(packet_type)
        } else {
            PacketHeader::new(0x04, 0x07, Flags::OBJECT_RELATED | Flags::FLAG_10).write(packet_type)
        };
        buf.write_all(&self.unk)
            .map_err(|e| PacketError::FieldError {
                packet_name: "MovementPacket",
                field_name: "unk",
                error: e,
            })?;
        if flags != 0xFFFFF {
            buf.write_u24::<LittleEndian>(flags)
                .map_err(|e| PacketError::FieldError {
                    packet_name: "MovementPacket",
                    field_name: "flags",
                    error: e,
                })?;
        }
        buf.append(&mut tmp_buf);
        Ok(buf)
    }
}
