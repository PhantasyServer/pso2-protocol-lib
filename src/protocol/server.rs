use super::{HelperReadWrite, PacketReadWrite};

// ----------------------------------------------------------------
// Server packets
// ----------------------------------------------------------------

// 0x03, 0x08
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x08)]
pub struct ServerHelloPacket {
    #[Const_u16(0x03)]
    #[SeekAfter(8)]
    pub version: u16,
}

// 0x03, 0x24
#[derive(Debug, Clone, PartialEq, PacketReadWrite)]
#[Id(0x03, 0x24)]
#[Flags(Flags {packed: true, ..Default::default()})]
// This was somewhat checked against NGS's implementation, however PSO2 seems to have extra fields?(they are commented out)
pub struct LoadLevelPacket {
    pub unk1: [u8; 0xC],
    pub unk2: [u8; 0xC],
    pub unk3: [u8; 0x34],
    pub unk4: [u8; 0xC],
    pub unk5: [u8; 0xC],
    pub unk6: [u8; 0xC],
    #[VariableAscii(0x7542, 0x5E)]
    pub unk7: String,
    #[Magic(0x7542, 0x5E)]
    pub unk8: Vec<LoadLevelThing1>,
    #[Magic(0x7542, 0x5E)]
    pub unk9: Vec<LoadLevelThing2>,
    #[Magic(0x7542, 0x5E)]
    pub unk10: Vec<LoadLevelThing3>,
    #[Magic(0x7542, 0x5E)]
    pub unk11: Vec<LoadLevelThing4>,
    #[Magic(0x7542, 0x5E)]
    pub unk12: Vec<LoadLevelThing5>,
    #[Magic(0x7542, 0x5E)]
    pub unk13: Vec<LoadLevelThing6>,
    #[Magic(0x7542, 0x5E)]
    pub unk14: Vec<LoadLevelThing7>,
    #[Magic(0x7542, 0x5E)]
    pub unk15: Vec<LoadLevelThing8>,
    #[Magic(0x7542, 0x5E)]
    pub unk16: Vec<UnkThing1>,
    #[VariableAscii(0x7542, 0x5E)]
    pub unk17: String,
    pub unk18: u32,
    pub unk19: u32,
    pub unk20: u32,
    pub unk21: [u8; 0x3C],
    pub unk22: u32,
    pub unk23: [u8; 0x10],
    pub unk24: [u8; 0x10],
    #[Magic(0x7542, 0x5E)]
    pub unk25: Vec<u32>,
    pub unk26: [u8; 0x200],
    #[Magic(0x7542, 0x5E)]
    pub unk27: Vec<UnkThing2>,
    #[VariableAscii(0x7542, 0x5E)]
    pub unk28: String,
    #[VariableAscii(0x7542, 0x5E)]
    pub unk29: String,
    pub unk30: u64,
    pub unk31: u64,
    pub unk32: u8,
    pub unk33: u8,
    pub unk34: u8,
    pub unk35: u8,
    pub unk36: u32,
    pub unk37: [u8; 0x14],
    pub unk38: u64,
    pub unk39: u32,
    pub unk40: u32,
    pub unk41: [u8; 0x12],
    pub unk42: u64,
    pub unk43: u8,
    pub unk44: u8,
    // i'm unsure if those are correct fields to comment out
    // problematic 48 bytes are between unk29 and unk48

    // pub unk45: u8,
    // pub unk46: u8,
    // pub unk47: u32,
    #[Magic(0x7542, 0x5E)]
    pub unk48: Vec<LoadLevelThing9>,
    #[VariableAscii(0x7542, 0x5E)]
    pub unk49: String,
    #[Magic(0x7542, 0x5E)]
    pub unk50: Vec<LoadLevelThing10>,
    pub unk51: u32,
    pub unk52: [u8; 0x14],
    pub unk53: [u8; 0x14],
    pub unk54: u32,
}

// ----------------------------------------------------------------
// Additional structs
// ----------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing1 {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
}
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing2 {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
}
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing3 {
    pub unk1: u32,
    pub unk2: u32,
}
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing4 {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
    pub unk14: u32,
    pub unk15: u32,
    pub unk16: u32,
    pub unk17: u32,
    pub unk18: u32,
    pub unk19: u32,
    pub unk20: u32,
    pub unk21: u32,
    pub unk22: u32,
    pub unk23: u32,
    pub unk24: u32,
    pub unk25: u32,
    pub unk26: u32,
    pub unk27: u32,
    pub unk28: u32,
    pub unk29: u32,
    pub unk30: u32,
    pub unk31: u32,
    pub unk32: u32,
    pub unk33: u32,
    pub unk34: u32,
}
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing5 {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
    pub unk14: u32,
    pub unk15: u32,
    pub unk16: u32,
    pub unk17: u32,
    pub unk18: u32,
    pub unk19: u32,
    pub unk20: u32,
    pub unk21: u32,
    pub unk22: u32,
    pub unk23: u32,
    pub unk24: u32,
    pub unk25: u32,
    pub unk26: u32,
    pub unk27: u32,
    pub unk28: u32,
    pub unk29: u32,
    pub unk30: u32,
    pub unk31: u32,
    pub unk32: u32,
    pub unk33: u32,
    pub unk34: u32,
    pub unk35: u32,
    pub unk36: u32,
    pub unk37: u32,
    pub unk38: u32,
    pub unk39: u32,
    pub unk40: u32,
    pub unk41: u32,
    pub unk42: u32,
    pub unk43: u32,
    pub unk44: u32,
    pub unk45: u32,
    pub unk46: u32,
    pub unk47: u32,
    pub unk48: u32,
    pub unk49: u32,
    pub unk50: u32,
    pub unk51: u32,
    pub unk52: u32,
    pub unk53: u32,
    pub unk54: u32,
    pub unk55: u32,
    pub unk56: u32,
}
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing6 {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
    pub unk14: u32,
    pub unk15: u32,
    pub unk16: u32,
    pub unk17: u32,
    pub unk18: u32,
    pub unk19: u32,
    pub unk20: u32,
    pub unk21: u32,
    pub unk22: u32,
    pub unk23: u32,
    pub unk24: u32,
    pub unk25: u32,
    pub unk26: u32,
    pub unk27: u32,
    pub unk28: u32,
    pub unk29: u32,
    pub unk30: u32,
    pub unk31: u32,
    pub unk32: u32,
    pub unk33: u32,
    pub unk34: u32,
    pub unk35: u32,
    pub unk36: u32,
    pub unk37: u32,
    pub unk38: u32,
    pub unk39: u32,
    pub unk40: u32,
}
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing7 {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
    pub unk14: u32,
    pub unk15: u32,
    pub unk16: u32,
    pub unk17: u32,
    pub unk18: u32,
    pub unk19: u32,
    pub unk20: u32,
    pub unk21: u32,
    pub unk22: u32,
    pub unk23: u32,
    pub unk24: u32,
    pub unk25: u32,
    pub unk26: u32,
    pub unk27: u32,
    pub unk28: u32,
    pub unk29: u32,
    pub unk30: u32,
    pub unk31: u32,
    pub unk32: u32,
    pub unk33: u32,
    pub unk34: u32,
    pub unk35: u32,
    pub unk36: u32,
    pub unk37: u32,
    pub unk38: u32,
    pub unk39: u32,
    pub unk40: u32,
    pub unk41: u32,
    pub unk42: u32,
    pub unk43: u32,
    pub unk44: u32,
    pub unk45: u32,
    pub unk46: u32,
    pub unk47: u32,
    pub unk48: u32,
    pub unk49: u32,
    pub unk50: u32,
    pub unk51: u32,
    pub unk52: u32,
    pub unk53: u32,
    pub unk54: u32,
    pub unk55: u32,
    pub unk56: u32,
    pub unk57: u32,
    pub unk58: u32,
    pub unk59: u32,
    pub unk60: u32,
}
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing8 {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
    pub unk14: u32,
    pub unk15: u32,
    pub unk16: u32,
    pub unk17: u32,
    pub unk18: u32,
    pub unk19: u32,
    pub unk20: u32,
    pub unk21: u32,
    pub unk22: u32,
    pub unk23: u32,
    pub unk24: u32,
    pub unk25: u32,
    pub unk26: u32,
    pub unk27: u32,
    pub unk28: u32,
    pub unk29: u32,
    pub unk30: u32,
    pub unk31: u32,
    pub unk32: u32,
    pub unk33: u32,
    pub unk34: u32,
    pub unk35: u32,
    pub unk36: u32,
    pub unk37: u32,
    pub unk38: u32,
    pub unk39: u32,
    pub unk40: u32,
    pub unk41: u32,
}
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing9 {
    pub unk1: u32,
    pub unk2: u32,
}
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct LoadLevelThing10 {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,
    pub unk13: u32,
}
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct UnkThing1 {
    pub unk1: u32,
    pub unk2: u32,
    pub unk3: u32,
    pub unk4: u32,
    pub unk5: u32,
    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,
    pub unk9: u32,
    pub unk10: u32,
}
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct UnkThing2 {
    pub unk1: u32,
    pub unk2: u32,
}

// ----------------------------------------------------------------
// Default implementations
// ----------------------------------------------------------------

impl Default for ServerHelloPacket {
    fn default() -> Self {
        Self { version: 0xc9 }
    }
}
