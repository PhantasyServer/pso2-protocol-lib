use crate::{
    asciistring::StringRW,
    protocol::{HelperReadWrite, PacketType},
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Seek, Write};

// ----------------------------------------------------------------
// Structures
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Character {
    pub character_id: u32,
    pub player_id: u32,
    pub unk1: u32,
    pub voice_type: u32,
    pub unk2: u16,
    pub voice_pitch: u16,
    pub name: String,
    pub look: Look,
    pub classes: ClassInfo,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct HSVColor {
    pub hue: u16,
    pub saturation: u16,
    pub value: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
// I'm unsure if we need to name these fields
pub struct Figure(pub u16, pub u16, pub u16);

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct AccessoryData(pub i8, pub i8, pub i8);

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u16)]
pub enum Race {
    #[default]
    Human,
    Newman,
    Cast,
    Deuman,

    #[Read_default]
    Unknown = 0xFFFF,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u16)]
pub enum Gender {
    #[default]
    Male,
    Female,

    #[Read_default]
    Unknown = 0xFFFF,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct Look {
    pub running_animation: u16,
    pub race: Race,
    pub gender: Gender,
    pub muscule: u16,
    pub body: Figure,
    pub arms: Figure,
    pub legs: Figure,
    pub chest: Figure,
    pub face_shape: Figure,
    pub face_parts: Figure,
    pub eyes: Figure,
    pub nose_size: Figure,
    pub nose_height: Figure,
    pub mouth: Figure,
    pub ears: Figure,
    pub neck: Figure,
    pub waist: Figure,
    pub body2: Figure,
    pub arms2: Figure,
    pub legs2: Figure,
    pub chest2: Figure,
    pub neck2: Figure,
    pub waist2: Figure,
    pub unk1: [u8; 0x20],
    pub unk2: [u8; 0x0A],
    pub acc1_location: AccessoryData,
    pub acc2_location: AccessoryData,
    pub acc3_location: AccessoryData,
    pub acc4_location: AccessoryData,
    pub unk_color: HSVColor,
    pub costume_color: HSVColor,
    pub main_color: HSVColor,
    pub sub1_color: HSVColor,
    pub sub2_color: HSVColor,
    pub sub3_color: HSVColor,
    pub eye_color: HSVColor,
    pub hair_color: HSVColor,
    pub unk3: [u8; 0x20],
    pub unk4: [u8; 0x10],
    pub costume_id: u16,
    pub body_paint1: u16,
    pub sticker_id: u16,
    pub right_eye_id: u16,
    pub eyebrow_id: u16,
    pub eyelash_id: u16,
    pub face_id1: u16,
    pub face_id2: u16,
    pub facemakeup1_id: u16,
    pub hairstyle_id: u16,
    pub acc1_id: u16,
    pub acc2_id: u16,
    pub acc3_id: u16,
    pub facemakeup2_id: u16,
    pub leg_id: u16,
    pub arm_id: u16,
    pub acc4_id: u16,
    pub unk5: [u8; 0x4],
    pub body_paint2: u16,
    pub left_eye_id: u16,
    pub unk6: [u8; 0x12],
    pub acc1_size: AccessoryData,
    pub acc2_size: AccessoryData,
    pub acc3_size: AccessoryData,
    pub acc4_size: AccessoryData,
    pub acc1_rotation: AccessoryData,
    pub acc2_rotation: AccessoryData,
    pub acc3_rotation: AccessoryData,
    pub acc4_rotation: AccessoryData,
    pub unk7: u16,
    pub unk8: [u8; 0x8],
    pub skin_color_type: u8,
    pub eyebrow_thickness: u8,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u8)]
pub enum Class {
    #[default]
    Hunter,
    Ranger,
    Force,
    Fighter,
    Gunner,
    Techer,
    Braver,
    Bouncer,
    Challenger,
    Summoner,
    BattleWarrior,
    Hero,
    Phantom,
    Etole,
    Luster,

    #[Read_default]
    Unknown = 0xFF,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
#[Flags(u16)]
pub struct ClassFlags {
    pub hunter: bool,
    pub ranger: bool,
    pub force: bool,
    pub fighter: bool,
    pub gunner: bool,
    pub techer: bool,
    pub braver: bool,
    pub bouncer: bool,
    pub challenger: bool,
    pub summoner: bool,
    pub battlewarrior: bool,
    pub hero: bool,
    pub phantom: bool,
    pub etole: bool,
    pub luster: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Copy, Clone, PartialEq, HelperReadWrite)]
pub struct ClassLevel {
    pub level1: u16,
    pub level2: u16,
    pub exp: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct ClassInfo {
    pub unk1: u32,
    pub main_class: Class,
    pub sub_class: Class,
    pub unk2: u16,
    pub enabled_classes: ClassFlags,
    pub unk3: u16,
    pub hunter_info: ClassLevel,
    pub ranger_info: ClassLevel,
    pub force_info: ClassLevel,
    pub fighter_info: ClassLevel,
    pub gunner_info: ClassLevel,
    pub techer_info: ClassLevel,
    pub braver_info: ClassLevel,
    pub bouncer_info: ClassLevel,
    pub challenger_info: ClassLevel,
    pub summoner_info: ClassLevel,
    pub battle_warrior_info: ClassLevel,
    pub hero_info: ClassLevel,
    pub phantom_info: ClassLevel,
    pub etole_info: ClassLevel,
    pub luster_info: ClassLevel,
    pub unk16_info: ClassLevel,
    pub unk17_info: ClassLevel,
    pub unk18_info: ClassLevel,
    pub unk19_info: ClassLevel,
    pub unk20_info: ClassLevel,
    pub unk21_info: ClassLevel,
    pub unk22_info: ClassLevel,
    pub unk23_info: ClassLevel,
    pub unk24_info: ClassLevel,
    pub unk1_maxlevel: u16,
    pub unk2_maxlevel: u16,
    pub unk3_maxlevel: u16,
    pub unk4_maxlevel: u16,
    pub unk5_maxlevel: u16,
    pub unk6_maxlevel: u16,
    pub unk7_maxlevel: u16,
    pub unk8_maxlevel: u16,
    pub unk9_maxlevel: u16,
    pub unk10_maxlevel: u16,
    pub unk11_maxlevel: u16,
    pub unk12_maxlevel: u16,
    pub unk13_maxlevel: u16,
    pub unk14_maxlevel: u16,
    pub unk15_maxlevel: u16,
}

// ----------------------------------------------------------------
// Read/Write implementations
// ----------------------------------------------------------------

impl HelperReadWrite for Character {
    fn read(reader: &mut (impl Read + Seek), packet_type: PacketType) -> std::io::Result<Self> {
        let character_id = reader.read_u32::<LittleEndian>()?;
        let player_id = reader.read_u32::<LittleEndian>()?;
        let unk1 = reader.read_u32::<LittleEndian>()?;
        let voice_type = reader.read_u32::<LittleEndian>()?;
        let unk2 = reader.read_u16::<LittleEndian>()?;
        let voice_pitch = reader.read_u16::<LittleEndian>()?;
        let name = String::read(reader, 16);

        let is_global = matches!(packet_type, PacketType::NA);
        if !is_global {
            reader.seek(std::io::SeekFrom::Current(4))?;
        }

        let look = Look::read(reader, packet_type)?;
        let classes = ClassInfo::read(reader, packet_type)?;

        reader.seek(std::io::SeekFrom::Current(0x96))?;
        if is_global {
            reader.seek(std::io::SeekFrom::Current(4))?;
        }

        Ok(Self {
            character_id,
            player_id,
            unk1,
            voice_type,
            unk2,
            voice_pitch,
            name,
            look,
            classes,
        })
    }
    fn write(&self, writer: &mut impl Write, packet_type: PacketType) -> std::io::Result<()> {
        writer.write_u32::<LittleEndian>(self.character_id)?;
        writer.write_u32::<LittleEndian>(self.player_id)?;
        writer.write_u32::<LittleEndian>(self.unk1)?;
        writer.write_u32::<LittleEndian>(self.voice_type)?;
        writer.write_u16::<LittleEndian>(self.unk2)?;
        writer.write_u16::<LittleEndian>(self.voice_pitch)?;
        writer.write_all(&self.name.write(16))?;

        let is_global = matches!(packet_type, PacketType::NA);

        if !is_global {
            writer.write_u32::<LittleEndian>(0)?;
        }
        self.look.write(writer, packet_type)?;
        self.classes.write(writer, packet_type)?;

        writer.write_all(&[0u8; 0x96])?;
        if is_global {
            writer.write_u32::<LittleEndian>(0)?;
        }

        Ok(())
    }
}

// ----------------------------------------------------------------
// Other implementations
// ----------------------------------------------------------------

impl Character {
    pub fn get_level(&self) -> &ClassLevel {
        match self.classes.main_class {
            Class::Hunter => &self.classes.hunter_info,
            Class::Ranger => &self.classes.ranger_info,
            Class::Force => &self.classes.force_info,
            Class::Fighter => &self.classes.fighter_info,
            Class::Gunner => &self.classes.gunner_info,
            Class::Techer => &self.classes.techer_info,
            Class::Braver => &self.classes.braver_info,
            Class::Bouncer => &self.classes.bouncer_info,
            Class::Challenger => &self.classes.challenger_info,
            Class::Summoner => &self.classes.summoner_info,
            Class::BattleWarrior => &self.classes.battle_warrior_info,
            Class::Hero => &self.classes.hero_info,
            Class::Phantom => &self.classes.phantom_info,
            Class::Etole => &self.classes.etole_info,
            Class::Luster => &self.classes.luster_info,
            Class::Unknown => &self.classes.unk16_info,
        }
    }
    pub fn get_level_mut(&mut self) -> &mut ClassLevel {
        match self.classes.main_class {
            Class::Hunter => &mut self.classes.hunter_info,
            Class::Ranger => &mut self.classes.ranger_info,
            Class::Force => &mut self.classes.force_info,
            Class::Fighter => &mut self.classes.fighter_info,
            Class::Gunner => &mut self.classes.gunner_info,
            Class::Techer => &mut self.classes.techer_info,
            Class::Braver => &mut self.classes.braver_info,
            Class::Bouncer => &mut self.classes.bouncer_info,
            Class::Challenger => &mut self.classes.challenger_info,
            Class::Summoner => &mut self.classes.summoner_info,
            Class::BattleWarrior => &mut self.classes.battle_warrior_info,
            Class::Hero => &mut self.classes.hero_info,
            Class::Phantom => &mut self.classes.phantom_info,
            Class::Etole => &mut self.classes.etole_info,
            Class::Luster => &mut self.classes.luster_info,
            Class::Unknown => &mut self.classes.unk16_info,
        }
    }
    pub fn get_sublevel(&self) -> &ClassLevel {
        match self.classes.sub_class {
            Class::Hunter => &self.classes.hunter_info,
            Class::Ranger => &self.classes.ranger_info,
            Class::Force => &self.classes.force_info,
            Class::Fighter => &self.classes.fighter_info,
            Class::Gunner => &self.classes.gunner_info,
            Class::Techer => &self.classes.techer_info,
            Class::Braver => &self.classes.braver_info,
            Class::Bouncer => &self.classes.bouncer_info,
            Class::Challenger => &self.classes.challenger_info,
            Class::Summoner => &self.classes.summoner_info,
            Class::BattleWarrior => &self.classes.battle_warrior_info,
            Class::Hero => &self.classes.hero_info,
            Class::Phantom => &self.classes.phantom_info,
            Class::Etole => &self.classes.etole_info,
            Class::Luster => &self.classes.luster_info,
            Class::Unknown => &self.classes.unk16_info,
        }
    }
    pub fn get_sublevel_mut(&mut self) -> &mut ClassLevel {
        match self.classes.sub_class {
            Class::Hunter => &mut self.classes.hunter_info,
            Class::Ranger => &mut self.classes.ranger_info,
            Class::Force => &mut self.classes.force_info,
            Class::Fighter => &mut self.classes.fighter_info,
            Class::Gunner => &mut self.classes.gunner_info,
            Class::Techer => &mut self.classes.techer_info,
            Class::Braver => &mut self.classes.braver_info,
            Class::Bouncer => &mut self.classes.bouncer_info,
            Class::Challenger => &mut self.classes.challenger_info,
            Class::Summoner => &mut self.classes.summoner_info,
            Class::BattleWarrior => &mut self.classes.battle_warrior_info,
            Class::Hero => &mut self.classes.hero_info,
            Class::Phantom => &mut self.classes.phantom_info,
            Class::Etole => &mut self.classes.etole_info,
            Class::Luster => &mut self.classes.luster_info,
            Class::Unknown => &mut self.classes.unk16_info,
        }
    }
}

// ----------------------------------------------------------------
// Default implementations
// ----------------------------------------------------------------

impl Default for Look {
    fn default() -> Self {
        Self {
            running_animation: 9,
            race: Race::Human,
            gender: Gender::Male,
            muscule: 0,
            body: Figure(0, 359, 120),
            arms: Figure(65177, 65296, 65177),
            legs: Figure(64396, 65117, 1570),
            chest: Figure(0, 0, 0),
            face_shape: Figure(63936, 62870, 64470),
            face_parts: Figure(3200, 63937, 61270),
            eyes: Figure(64470, 5866, 55536),
            nose_size: Figure(64470, 29, 218),
            nose_height: Figure(0, 60, 60),
            mouth: Figure(606, 4266, 60203),
            ears: Figure(0, 0, 0),
            neck: Figure(0, 0, 0),
            waist: Figure(0, 0, 0),
            body2: Figure(0, 359, 120),
            arms2: Figure(65177, 65296, 65177),
            legs2: Figure(64396, 65117, 1570),
            chest2: Figure(0, 0, 0),
            neck2: Figure(0, 0, 0),
            waist2: Figure(0, 0, 0),
            unk1: [0u8; 0x20],
            unk2: [0u8; 0x0A],
            acc1_location: AccessoryData(0, 0, 0),
            acc2_location: AccessoryData(0, 0, 0),
            acc3_location: AccessoryData(0, 0, 0),
            acc4_location: AccessoryData(0, 0, 0),
            unk_color: HSVColor {
                hue: 0,
                saturation: 0,
                value: 0,
            },
            costume_color: HSVColor {
                hue: 37424,
                saturation: 43810,
                value: 2000,
            },
            main_color: HSVColor {
                hue: 0,
                saturation: 0,
                value: 0,
            },
            sub1_color: HSVColor {
                hue: 0,
                saturation: 0,
                value: 0,
            },
            sub2_color: HSVColor {
                hue: 0,
                saturation: 24751,
                value: 10000,
            },
            sub3_color: HSVColor {
                hue: 0,
                saturation: 0,
                value: 0,
            },
            eye_color: HSVColor {
                hue: 11814,
                saturation: 4272,
                value: 10000,
            },
            hair_color: HSVColor {
                hue: 38526,
                saturation: 45526,
                value: 1000,
            },
            unk3: [0u8; 0x20],
            unk4: [0u8; 0x10],
            costume_id: 9,
            body_paint1: 0,
            sticker_id: 0,
            right_eye_id: 1,
            eyebrow_id: 0,
            eyelash_id: 0,
            face_id1: 0,
            face_id2: 0,
            facemakeup1_id: 0,
            hairstyle_id: 5,
            acc1_id: 0,
            acc2_id: 0,
            acc3_id: 0,
            facemakeup2_id: 0,
            leg_id: 0,
            arm_id: 0,
            acc4_id: 0,
            unk5: [33, 78, 33, 78],
            body_paint2: 0,
            left_eye_id: 0,
            unk6: [0u8; 0x12],
            acc1_size: AccessoryData(0, 0, 0),
            acc2_size: AccessoryData(0, 0, 0),
            acc3_size: AccessoryData(0, 0, 0),
            acc4_size: AccessoryData(0, 0, 0),
            acc1_rotation: AccessoryData(0, 0, 0),
            acc2_rotation: AccessoryData(0, 0, 0),
            acc3_rotation: AccessoryData(0, 0, 0),
            acc4_rotation: AccessoryData(0, 0, 0),
            unk7: 0,
            unk8: [2, 1, 0, 0, 0, 0, 0, 0],
            skin_color_type: 0,
            eyebrow_thickness: 0,
        }
    }
}

impl Default for ClassLevel {
    fn default() -> Self {
        Self {
            level1: 1,
            level2: 1,
            exp: 0,
        }
    }
}
