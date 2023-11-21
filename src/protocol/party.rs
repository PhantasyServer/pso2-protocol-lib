use crate::{
    protocol::{
        models::character::Class, HelperReadWrite, ObjectHeader, PacketReadWrite, PacketType,
    },
    AsciiString,
};

// ----------------------------------------------------------------
// Party packets
// ----------------------------------------------------------------

// 0x0E, 0x00
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x00)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xCCE7, 0x13)]
pub struct AddMemberPacket {
    pub new_member: ObjectHeader,
    pub unk1: u32,
    pub level: u32,
    pub sublevel: u32,
    #[SeekAfter(3)]
    pub class: Class,
    pub subclass: Class,
    pub padding: [u8; 3],
    pub nickname: String,
    pub char_name: String,
    pub unk5: [u8; 0xC],
    pub unk6: u16,
    pub unk7: [u8; 2],
    pub hp: [u32; 3],
    pub map_id: u16,
    pub unk10: [u8; 4],
    pub unk11: u16,
    pub unk12: u32,
    pub unk13: [u8; 0xC],
    pub unk14: [u32; 3],
    pub unk15: String,
    pub unk16: AsciiString,
}

#[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x00)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xCCE7, 0x13)]
pub struct AddMemberNGSPacket {
    pub new_member: ObjectHeader,
    pub unk1: u32,
    pub level: u32,
    pub sublevel: u32,
    #[SeekAfter(3)]
    pub class: Class,
    pub subclass: Class,
    pub padding: [u8; 3],
    pub nickname: String,
    pub char_name: String,
    pub unk5: [u8; 0xC],
    pub unk6: u16,
    pub unk7: u8,
    pub language: ShortLanguage,
    pub hp: [u32; 3],
    pub map_id: u16,
    pub unk10: [u8; 4],
    pub unk11: u16,
    pub unk12: u32,
    pub unk13: [u8; 0xC],
    pub unk14: [u32; 3],
    pub unk15: String,
    pub unk16: AsciiString,
    pub unk17: AsciiString,
}

// 0x0E, 0x01
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x01)]
pub struct RemoveMemberPacket {
    pub removed_member: ObjectHeader,
    pub receiver: ObjectHeader,
}

// 0x0E, 0x02
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xD863, 0xA9)]
pub struct PartyInitPacket {
    pub party_object: ObjectHeader,
    pub leader: ObjectHeader,
    pub people_amount: u32,
    pub entries: [PartyEntry; 4],
    pub unk2: AsciiString,
}

#[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xD863, 0xA9)]
pub struct PartyInitNGSPacket {
    pub party_object: ObjectHeader,
    pub leader: ObjectHeader,
    pub people_amount: u32,
    pub entries: [PartyEntryNGS; 4],
    pub unk2: AsciiString,
}

// 0x0E, 0x04
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x04)]
pub struct PartyInviteResultPacket {
    pub unk1: u32,
    pub status: u32,
}

// 0x0E, 0x05
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x05)]
pub struct PartyInviteRequestPacket {
    pub invitee: ObjectHeader,
}

// 0x0E, 0x06
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x06)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xEF59, 0xD5)]
pub struct NewInvitePacket {
    pub party_object: ObjectHeader,
    pub inviter: ObjectHeader,
    pub name: String,
    pub inviter_name: String,
    pub questname: String,
}

// 0x0E, 0x07
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x07)]
pub struct AcceptInvitePacket {
    pub party_object: ObjectHeader,
    pub inviter: ObjectHeader,
}

// 0x0E, 0x0C
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x0C)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x11CB, 0x98)]
pub struct NewPartySettingsPacket {
    pub name: String,
    pub password: String,
    pub comments: String,
    pub questname: String,
    pub min_level: u8,
    pub max_level: u8,
    pub playstyle: u8,
    pub flags: PartyFlags,
    pub unk: u64,
}

// 0x0E, 0x0D
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x0D)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x9789, 0xE3)]
pub struct PartySettingsPacket {
    pub name: String,
    pub password: String,
    pub comments: String,
    pub min_level: u8,
    pub max_level: u8,
    pub playstyle: u8,
    pub flags: PartyFlags,
    pub unk: u64,
}

// 0x0E, 0x0E
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x0E)]
pub struct TransferLeaderPacket {
    pub target: ObjectHeader,
}

// 0x0E, 0x0F
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x0F)]
pub struct NewLeaderPacket {
    pub leader: ObjectHeader,
}

// 0x0E, 0x10
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x10)]
pub struct KickMemberPacket {
    pub member: ObjectHeader,
}

// 0x0E, 0x11
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x11)]
pub struct KickedMemberPacket {
    pub member: ObjectHeader,
}

// 0x0E, 0x17
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x17)]
pub struct DisbandPartyPacket {
    pub party: ObjectHeader,
}

// 0x0E, 0x19
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x19)]
#[Flags(Flags {object_related: true, ..Default::default()})]
pub struct ChatStatusPacket {
    pub object: ObjectHeader,
    pub status: u32,
}

// 0x0E, 0x1A
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x1A)]
pub struct Unk0E1APacket {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
}

// 0x0E, 0x1B
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x1B)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xE7E8, 0xFF)]
pub struct PartyInfoPacket {
    pub num_of_infos: u32,
    pub infos: [PartyInfo; 10],
}

// 0x0E, 0x1C
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x1C)]
pub struct PartyInfoStopperPacker {
    pub unk: u32,
}

// 0x0E, 0x1D
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x1D)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xF364, 0x95)]
pub struct GetPartyDetailsPacket {
    pub parties: Vec<ObjectHeader>,
}

// 0x0E, 0x1E
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x1E)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x7921, 0xE0)]
pub struct PartyDetailsPacket {
    pub num_of_details: u32,
    pub details: [PartyDetails; 0xC],
}

// 0x0E, 0x21
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x21)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x0A5A, 0xC1)]
pub struct Unk0E21Packet {
    pub people_amount: u32,
    pub entries: [PartyEntry; 4],
}

#[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x21)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0x0A5A, 0xC1)]
pub struct Unk0E21NGSPacket {
    pub people_amount: u32,
    pub entries: [PartyEntryNGS; 4],
}

// 0x0E, 0x2B
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x2B)]
pub struct NewBusyStatePacket {
    pub object: ObjectHeader,
    pub state: BusyState,
}

// 0x0E, 0x2C
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x2C)]
pub struct InviteDeclinePacket {
    pub decline_status: RejectStatus,
}

// 0x0E, 0x2E
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x2E)]
#[Flags(Flags {packed: true, ..Default::default()})]
#[Magic(0xD4FC, 0x92)]
pub struct GetPartyInfoPacket {
    pub parties: Vec<ObjectHeader>,
}

// 0x0E, 0x4F
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x4F)]
pub struct SetPartyColorPacket {
    pub target: ObjectHeader,
    pub unk: [u32; 3],
    pub in_party: u32,
}

// 0x0E, 0x67
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x67)]
pub struct PartySetupFinishPacket {
    pub unk: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, HelperReadWrite)]
pub struct PartyEntry {
    pub id: ObjectHeader,
    pub nickname: String,
    pub char_name: String,
    pub level: u8,
    pub sublevel: u8,
    pub class: Class,
    pub subclass: Class,
    pub color: Color,
    pub unk1: [u8; 7],
    pub unk2: u32,
    pub hp: [u32; 3],
    pub map_id: u16,
    pub unk3: u16,
    pub unk4: [u8; 0xC],
    pub unk5: [u32; 3],
    pub unk6: String,
    #[OnlyOn(PacketType::Vita)]
    pub unk10: String,
    pub unk7: AsciiString,
    pub lang: ShortLanguage,
    pub unk9: [u8; 3],
}

#[cfg(feature = "ngs_packets")]
#[cfg_attr(docsrs, doc(cfg(feature = "ngs_packets")))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, HelperReadWrite)]
pub struct PartyEntryNGS {
    pub id: ObjectHeader,
    pub nickname: String,
    pub char_name: String,
    pub level: u8,
    pub sublevel: u8,
    pub class: Class,
    pub subclass: Class,
    pub color: Color,
    pub unk1: [u8; 7],
    pub unk2: u32,
    pub hp: [u32; 3],
    pub map_id: u16,
    pub unk3: u16,
    pub unk4: [u8; 0xC],
    pub unk5: [u32; 3],
    pub unk6: String,
    pub unk7: AsciiString,
    pub lang: ShortLanguage,
    pub unk9: [u8; 3],
    pub unk10: String,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
#[repr(u8)]
pub enum Color {
    #[default]
    #[Read_default]
    Red,
    Green,
    Yellow,
    Blue,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
#[repr(u8)]
pub enum ShortLanguage {
    #[default]
    Japanese,
    English,

    #[Read_default]
    Unknown,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[Flags(u8)]
pub struct PartyFlags {
    pub friends_only: bool,
    pub alliance_only: bool,
    pub limit_others: bool,
    pub single_run: bool,
    pub open: bool,
    pub unk6: bool,
    pub vc_focus: bool,
    pub unk8: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, HelperReadWrite)]
pub struct PartyInfo {
    pub unk1: [u8; 0xC],
    pub party_object: ObjectHeader,
    pub name: String,
    pub unk2: [u8; 9],
    pub unk3: [u8; 3],
    pub unk4: u32,
    pub invite_time: u32,
    pub unk6: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum RejectStatus {
    #[default]
    #[Read_default]
    Allow,
    Reject,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, HelperReadWrite)]
pub struct PartyDetails {
    pub party_desc: String,
    pub party_id: ObjectHeader,
    pub unk3: [u8; 0x10],
    pub unk4: u64,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u16,
    pub unk8: u16,
    pub unk9: [u8; 12],
    pub unk10: [PartyMember; 4],
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, HelperReadWrite)]
pub struct PartyMember {
    pub char_name: String,
    pub nickname: String,
    pub id: ObjectHeader,
    pub class: Class,
    pub subclass: Class,
    pub level: u8,
    pub sublevel: u8,
    pub unk4: [u8; 5],
    pub unk5: [u8; 3],
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Copy, Default, PartialEq, HelperReadWrite)]
#[repr(u32)]
pub enum BusyState {
    #[default]
    #[Read_default]
    NotBusy,
    Busy,
}
