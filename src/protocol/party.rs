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
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x00)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct AddMemberPacket {
    pub new_member: ObjectHeader,
    pub unk1: u32,
    pub level: u32,
    pub sublevel: u32,
    #[SeekAfter(3)]
    pub class: Class,
    pub subclass: Class,
    pub padding: [u8; 3],
    #[VariableStr(0xCCE7, 0x13)]
    pub nickname: String,
    #[VariableStr(0xCCE7, 0x13)]
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
    #[VariableStr(0xCCE7, 0x13)]
    pub unk15: String,
    #[VariableStr(0xCCE7, 0x13)]
    pub unk16: AsciiString,
}

#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x00)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct AddMemberNGSPacket {
    pub new_member: ObjectHeader,
    pub unk1: u32,
    pub level: u32,
    pub sublevel: u32,
    #[SeekAfter(3)]
    pub class: Class,
    pub subclass: Class,
    pub padding: [u8; 3],
    #[VariableStr(0xCCE7, 0x13)]
    pub nickname: String,
    #[VariableStr(0xCCE7, 0x13)]
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
    #[VariableStr(0xCCE7, 0x13)]
    pub unk15: String,
    #[VariableStr(0xCCE7, 0x13)]
    pub unk16: AsciiString,
    #[VariableStr(0xCCE7, 0x13)]
    pub unk17: AsciiString,
}

// 0x0E, 0x01
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
pub struct PartyInitPacket {
    pub party_object: ObjectHeader,
    pub leader: ObjectHeader,
    pub people_amount: u32,
    pub entries: [PartyEntry; 4],
    #[VariableStr(0xD863, 0xA9)]
    pub unk2: AsciiString,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x02)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct PartyInitNGSPacket {
    pub party_object: ObjectHeader,
    pub leader: ObjectHeader,
    pub people_amount: u32,
    pub entries: [PartyEntryNGS; 4],
    #[VariableStr(0xD863, 0xA9)]
    pub unk2: AsciiString,
}

// 0x0E, 0x04
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x04)]
pub struct PartyInviteResultPacket {
    pub unk1: u32,
    pub status: u32,
}

// 0x0E, 0x05
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x05)]
pub struct PartyInviteRequestPacket {
    pub invitee: ObjectHeader,
}

// 0x0E, 0x06
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x06)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct NewInvitePacket {
    pub party_object: ObjectHeader,
    pub inviter: ObjectHeader,
    #[VariableStr(0xEF59, 0xD5)]
    pub name: String,
    #[VariableStr(0xEF59, 0xD5)]
    pub inviter_name: String,
    #[VariableStr(0xEF59, 0xD5)]
    pub questname: String,
}

// 0x0E, 0x07
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x07)]
pub struct AcceptInvitePacket {
    pub party_object: ObjectHeader,
    pub inviter: ObjectHeader,
}

// 0x0E, 0x0C
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x0C)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct NewPartySettingsPacket {
    #[VariableStr(0x11CB, 0x98)]
    pub name: String,
    #[VariableStr(0x11CB, 0x98)]
    pub password: String,
    #[VariableStr(0x11CB, 0x98)]
    pub comments: String,
    #[VariableStr(0x11CB, 0x98)]
    pub questname: String,
    pub min_level: u8,
    pub max_level: u8,
    pub playstyle: u8,
    pub flags: PartyFlags,
    pub unk: u64,
}

// 0x0E, 0x0D
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x0D)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct PartySettingsPacket {
    #[VariableStr(0x9789, 0xe3)]
    pub name: String,
    #[VariableStr(0x9789, 0xe3)]
    pub password: String,
    #[VariableStr(0x9789, 0xe3)]
    pub comments: String,
    pub min_level: u8,
    pub max_level: u8,
    pub playstyle: u8,
    pub flags: PartyFlags,
    pub unk: u64,
}

// 0x0E, 0x0E
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x0E)]
pub struct TransferLeaderPacket {
    pub target: ObjectHeader,
}

// 0x0E, 0x0F
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x0F)]
pub struct NewLeaderPacket {
    pub leader: ObjectHeader,
}

// 0x0E, 0x10
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x10)]
pub struct KickMemberPacket {
    pub member: ObjectHeader,
}

// 0x0E, 0x11
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x11)]
pub struct KickedMemberPacket {
    pub member: ObjectHeader,
}

// 0x0E, 0x17
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x17)]
pub struct DisbandPartyPacket {
    pub party: ObjectHeader,
}

// 0x0E, 0x19
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x19)]
#[Flags(Flags {object_related: true, ..Default::default()})]
pub struct Unk0E19Packet {
    pub object: ObjectHeader,
    pub unk: u32,
}

// 0x0E, 0x1B
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x1B)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct PartyInfoPacket {
    pub num_of_infos: u32,
    pub infos: [PartyInfo; 10],
}

// 0x0E, 0x1C
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x1C)]
pub struct PartyInfoStopperPacker {
    pub unk: u32,
}

// 0x0E, 0x1D
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x1D)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct GetPartyDetailsPacket {
    #[Magic(0xF364, 0x95)]
    pub parties: Vec<ObjectHeader>,
}

// 0x0E, 0x1E
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x1E)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct PartyDetailsPacket {
    pub num_of_details: u32,
    pub details: [PartyDetails; 0xC],
}

// 0x0E, 0x2B
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x2B)]
pub struct NewBusyStatePacket {
    pub object: ObjectHeader,
    pub state: u32,
}

// 0x0E, 0x2C
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x2C)]
pub struct InviteDeclinePacket {
    pub decline_status: RejectStatus,
}

// 0x0E, 0x2E
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x2E)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct GetPartyInfoPacket {
    #[Magic(0xD4FC, 0x92)]
    pub parties: Vec<ObjectHeader>,
}

// 0x0E, 0x4F
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0E, 0x4F)]
pub struct SetPartyColorPacket {
    pub target: ObjectHeader,
    pub unk: [u32; 3],
    pub in_party: u32,
}

// 0x0E, 0x67
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
    #[VariableStr(0xD863, 0xA9)]
    pub nickname: String,
    #[VariableStr(0xD863, 0xA9)]
    pub char_name: String,
    pub level: u8,
    pub sublevel: u8,
    pub class: Class,
    pub subclass: Class,
    pub unk1: [u8; 8],
    pub unk2: u32,
    pub hp: [u32; 3],
    pub map_id: u16,
    pub unk3: u16,
    pub unk4: [u8; 0xC],
    pub unk5: [u32; 3],
    #[VariableStr(0xD863, 0xA9)]
    pub unk6: String,
    #[OnlyOn(PacketType::Vita)]
    #[VariableStr(0xD863, 0xA9)]
    pub unk10: String,
    #[VariableStr(0xD863, 0xA9)]
    pub unk7: AsciiString,
    pub unk8: u8,
    pub unk9: [u8; 3],
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, Default, PartialEq, HelperReadWrite)]
pub struct PartyEntryNGS {
    pub id: ObjectHeader,
    #[VariableStr(0xD863, 0xA9)]
    pub nickname: String,
    #[VariableStr(0xD863, 0xA9)]
    pub char_name: String,
    pub level: u8,
    pub sublevel: u8,
    pub class: Class,
    pub subclass: Class,
    pub unk1: [u8; 8],
    pub unk2: u32,
    pub hp: [u32; 3],
    pub map_id: u16,
    pub unk3: u16,
    pub unk4: [u8; 0xC],
    pub unk5: [u32; 3],
    #[VariableStr(0xD863, 0xA9)]
    pub unk6: String,
    #[VariableStr(0xD863, 0xA9)]
    pub unk7: AsciiString,
    pub unk8: u8,
    pub unk9: [u8; 3],
    #[VariableStr(0xD863, 0xA9)]
    pub unk10: String,
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

#[derive(Debug, Clone, Default, PartialEq, HelperReadWrite)]
pub struct PartyInfo {
    pub unk1: [u8; 0xC],
    pub party_object: ObjectHeader,
    #[VariableStr(0xE7E8, 0xFF)]
    pub questname: String,
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
    #[VariableStr(0x7921, 0xE0)]
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
    #[VariableStr(0x7921, 0xE0)]
    pub char_name: String,
    #[VariableStr(0x7921, 0xE0)]
    pub nickname: String,
    pub id: ObjectHeader,
    pub class: Class,
    pub subclass: Class,
    pub level: u8,
    pub sublevel: u8,
    pub unk4: [u8; 5],
    pub unk5: [u8; 3],
}
