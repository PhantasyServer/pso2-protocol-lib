use super::{HelperReadWrite, PacketReadWrite};

// ----------------------------------------------------------------
// ARKS Missions packets
// ----------------------------------------------------------------

// 0x4A, 0x01
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x4A, 0x01)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct MissionListPacket {
    pub unk1: u32,
    #[Magic(0xC691, 0x47)]
    pub missions: Vec<Mission>,
    pub daily_update: u32,
    pub weekly_update: u32,
    pub tier_update: u32,
}

// 0x4A, 0x03
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x4A, 0x03)]
#[Flags(Flags {packed: true, ..Default::default()})]
pub struct Unk4A03Packet {
    pub unk1: u32,
    #[Magic(0xD20D, 0xDD)]
    pub unk2: Vec<Mission>,
    #[Magic(0xD20D, 0xDD)]
    pub unk3: Vec<u32>,
    #[Magic(0xD20D, 0xDD)]
    pub unk4: Vec<Unk2Struct>,
    pub unk5: u32,
}

// 0x4A, 0x0C
#[derive(Debug, Clone, Default, PartialEq, PacketReadWrite)]
#[Id(0x4A, 0x0C)]
pub struct SetTrackedMissionPacket {
    pub id: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct Mission {
    /*5 - main
    1 - daily
    2 - weekly
    7 - tier */
    pub mission_type: u32,
    pub start_date: u32,
    pub end_date: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub completion_date: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
    pub unk14: u32,
    pub unk15: u32,
}

#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct Unk2Struct {
    pub unk1: [u32; 0x40],
}
