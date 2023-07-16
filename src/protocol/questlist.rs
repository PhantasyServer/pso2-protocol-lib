use super::{HelperReadWrite, PacketReadWrite};

// 0x0B, 0x09
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x09)]
pub struct Unk0B09Packet {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
}

// 0x0B, 0x15
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x15)]
pub struct AvailableQuestsRequestPacket {
    pub unk1: u32,
}

// 0x0B, 0x16
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x16)]
pub struct AvailableQuestsPacket {
    pub unk1: u16,
    pub extreme_count: u16,
    pub unk2: u16,
    pub arks_count: u16,
    pub limited_time_count: u16,
    pub extreme_debug_count: u16,
    pub blank1_count: u16,
    pub unk3: u16,
    pub net_cafe_count: u16,
    pub warming_debug_count: u16,
    pub blank2_count: u16,
    pub advance_count: u16,
    pub expedition_count: u16,
    pub expedition_debug_count: u16,
    pub arks_debug_count: u16,
    pub unk4_count: u16,
    pub challenge_count: u16,
    pub urgent_count: u16,
    pub urgent_debug_count: u16,
    pub time_attack_count: u16,
    pub time_attack_debug_count: u16,
    pub arks_debug2_count: [u16; 9],
    pub blank3_count: u16,
    pub unk5: u16,
    pub recommended_count: u16,
    pub unk6: u16,
    pub ultimate_debug_count: u16,
    pub agp_count: u16,
    pub bonus_count: u16,
    pub unk7: u16,
    pub training_count: [u16; 10],
    pub unk8: [u16; 30],
    pub unk9: u64,
    pub unk10: u64,
    pub unk11: u64,
    pub unk12: u64,
    pub unk13: u64,
}

// 0x0B, 0x17
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x17)]
pub struct QuestCategoryRequestPacket {
    pub unk1: u32,
    pub unk2: u32,
}

// 0x0B, 0x18
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0x18)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct QuestCategoryPacket {
    #[Magic(0x1DB0, 0xC5)]
    pub quests: Vec<Quest>,
}

// 0x0B, 0xAF
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x0B, 0xAF)]
pub struct Unk0BAFPacket {
    pub unk1: u32,
    pub unk2: u32,
}

// copied from polaris server
#[derive(Debug, /* Default, */ Clone, PartialEq, HelperReadWrite)]
pub struct Quest {
    #[FixedAscii(0x18)]
    pub date: String,
    pub unk1: [u32; 4],
    pub unk2: [u16; 2],
    pub name_id: u32,
    pub unk3: [u32; 27],
    pub unk4: u16,
    pub unk5: u8,
    pub unk6: u8,
    pub unk7: [u32; 20],
    pub unk8: [u16; 3],
    pub length: u8,
    pub party_type: u8,
    pub difficulties: u8,
    pub difficulties_completed: u8,
    pub unk9: u8,
    pub req_level: u8,
    pub sub_class_req_level: u8,
    pub recommended_level: u8,
    pub unk10: [u8; 8],
    pub unk11: u16,
    pub unk12: [u32; 2],
    pub unk13: u16,
    pub unk14: [u8; 2],
    // pub unk15: [QuestThing; 16],
    pub unk15: [u8; 0x320],
}

// should this be copy?
#[derive(Debug, Default, Copy, Clone, PartialEq, HelperReadWrite)]
pub struct QuestThing {
    pub unk1: [u32; 2],
    pub unk2: [u8; 2],
    pub unk3: u16,
}
