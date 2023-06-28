use crate::protocol::{read_utf16, write_utf16};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Seek, Write};

// ----------------------------------------------------------------
// Structures
// ----------------------------------------------------------------

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[derive(Debug, Default, Clone, PartialEq)]
pub struct HSVColor {
    pub hue: u16,
    pub saturation: u16,
    pub value: u16,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
// I'm unsure if we need to name these fields
pub struct Figure(pub u16, pub u16, pub u16);

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct AccessoryData(pub u8, pub u8, pub u8);

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum Race {
    #[default]
    Human,
    Newman,
    Cast,
    Deuman,
    Unknown = 0xFFFF,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(u16)]
pub enum Gender {
    #[default]
    Male,
    Female,
    Unknown = 0xFFFF,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
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
#[derive(Debug, Default, Clone, Copy, PartialEq)]
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
    Unknown = 0xFF,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ClassFlags {
    //TODO: figure out other classes
    pub hunter: bool,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct ClassLevel {
    pub level1: u16,
    pub level2: u16,
    pub exp: u32,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq)]
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

impl Character {
    pub(crate) fn read(reader: &mut (impl Read + Seek)) -> std::io::Result<Self> {
        let character_id = reader.read_u32::<LittleEndian>()?;
        let player_id = reader.read_u32::<LittleEndian>()?;
        let unk1 = reader.read_u32::<LittleEndian>()?;
        let voice_type = reader.read_u32::<LittleEndian>()?;
        let unk2 = reader.read_u16::<LittleEndian>()?;
        let voice_pitch = reader.read_u16::<LittleEndian>()?;
        let name = read_utf16(reader, 16);

        let is_global = reader.read_u8()? != 0;
        reader.seek(std::io::SeekFrom::Current(-1))?;
        if !is_global {
            reader.seek(std::io::SeekFrom::Current(4))?;
        }

        let look = Look::read(reader)?;
        let classes = ClassInfo::read(reader)?;

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
    pub(crate) fn write(&self, writer: &mut impl Write, is_global: bool) -> std::io::Result<()> {
        writer.write_u32::<LittleEndian>(self.character_id)?;
        writer.write_u32::<LittleEndian>(self.player_id)?;
        writer.write_u32::<LittleEndian>(self.unk1)?;
        writer.write_u32::<LittleEndian>(self.voice_type)?;
        writer.write_u16::<LittleEndian>(self.unk2)?;
        writer.write_u16::<LittleEndian>(self.voice_pitch)?;
        writer.write_all(&write_utf16(&self.name, 16))?;

        if !is_global {
            writer.write_u32::<LittleEndian>(0)?;
        }
        self.look.write(writer)?;
        self.classes.write(writer)?;

        writer.write_all(&[0u8; 0x96])?;
        if is_global {
            writer.write_u32::<LittleEndian>(0)?;
        }

        Ok(())
    }
}

impl HSVColor {
    pub(crate) fn read(reader: &mut impl Read) -> std::io::Result<Self> {
        let hue = reader.read_u16::<LittleEndian>()?;
        let saturation = reader.read_u16::<LittleEndian>()?;
        let value = reader.read_u16::<LittleEndian>()?;
        Ok(Self {
            hue,
            saturation,
            value,
        })
    }
    pub(crate) fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write_u16::<LittleEndian>(self.hue)?;
        writer.write_u16::<LittleEndian>(self.saturation)?;
        writer.write_u16::<LittleEndian>(self.value)?;
        Ok(())
    }
}

impl Figure {
    pub(crate) fn read(reader: &mut impl Read) -> std::io::Result<Self> {
        let x = reader.read_u16::<LittleEndian>()?;
        let y = reader.read_u16::<LittleEndian>()?;
        let z = reader.read_u16::<LittleEndian>()?;
        Ok(Self(x, y, z))
    }
    pub(crate) fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write_u16::<LittleEndian>(self.0)?;
        writer.write_u16::<LittleEndian>(self.1)?;
        writer.write_u16::<LittleEndian>(self.2)?;
        Ok(())
    }
}

impl AccessoryData {
    pub(crate) fn read(reader: &mut impl Read) -> std::io::Result<Self> {
        let x = reader.read_u8()?;
        let y = reader.read_u8()?;
        let z = reader.read_u8()?;
        Ok(Self(x, y, z))
    }
    pub(crate) fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write_u8(self.0)?;
        writer.write_u8(self.1)?;
        writer.write_u8(self.2)?;
        Ok(())
    }
}

impl Race {
    pub(crate) fn read(num: u16) -> Self {
        match num {
            0 => Self::Human,
            1 => Self::Newman,
            2 => Self::Cast,
            3 => Self::Deuman,
            _ => Self::Unknown,
        }
    }
}

impl Gender {
    pub(crate) fn read(num: u16) -> Self {
        match num {
            0 => Self::Male,
            1 => Self::Female,
            _ => Self::Unknown,
        }
    }
}

impl Look {
    pub(crate) fn read(reader: &mut impl Read) -> std::io::Result<Self> {
        let running_animation = reader.read_u16::<LittleEndian>()?;
        let race = Race::read(reader.read_u16::<LittleEndian>()?);
        let gender = Gender::read(reader.read_u16::<LittleEndian>()?);
        let muscule = reader.read_u16::<LittleEndian>()?;
        let body = Figure::read(reader)?;
        let arms = Figure::read(reader)?;
        let legs = Figure::read(reader)?;
        let chest = Figure::read(reader)?;
        let face_shape = Figure::read(reader)?;
        let face_parts = Figure::read(reader)?;
        let eyes = Figure::read(reader)?;
        let nose_size = Figure::read(reader)?;
        let nose_height = Figure::read(reader)?;
        let mouth = Figure::read(reader)?;
        let ears = Figure::read(reader)?;
        let neck = Figure::read(reader)?;
        let waist = Figure::read(reader)?;
        let body2 = Figure::read(reader)?;
        let arms2 = Figure::read(reader)?;
        let legs2 = Figure::read(reader)?;
        let chest2 = Figure::read(reader)?;
        let neck2 = Figure::read(reader)?;
        let waist2 = Figure::read(reader)?;
        let mut unk1 = [0u8; 0x20];
        reader.read_exact(&mut unk1)?;
        let mut unk2 = [0u8; 0x0A];
        reader.read_exact(&mut unk2)?;
        let acc1_location = AccessoryData::read(reader)?;
        let acc2_location = AccessoryData::read(reader)?;
        let acc3_location = AccessoryData::read(reader)?;
        let acc4_location = AccessoryData::read(reader)?;
        let unk_color = HSVColor::read(reader)?;
        let costume_color = HSVColor::read(reader)?;
        let main_color = HSVColor::read(reader)?;
        let sub1_color = HSVColor::read(reader)?;
        let sub2_color = HSVColor::read(reader)?;
        let sub3_color = HSVColor::read(reader)?;
        let eye_color = HSVColor::read(reader)?;
        let hair_color = HSVColor::read(reader)?;
        let mut unk3 = [0u8; 0x20];
        reader.read_exact(&mut unk3)?;
        let mut unk4 = [0u8; 0x10];
        reader.read_exact(&mut unk4)?;
        let costume_id = reader.read_u16::<LittleEndian>()?;
        let body_paint1 = reader.read_u16::<LittleEndian>()?;
        let sticker_id = reader.read_u16::<LittleEndian>()?;
        let right_eye_id = reader.read_u16::<LittleEndian>()?;
        let eyebrow_id = reader.read_u16::<LittleEndian>()?;
        let eyelash_id = reader.read_u16::<LittleEndian>()?;
        let face_id1 = reader.read_u16::<LittleEndian>()?;
        let face_id2 = reader.read_u16::<LittleEndian>()?;
        let facemakeup1_id = reader.read_u16::<LittleEndian>()?;
        let hairstyle_id = reader.read_u16::<LittleEndian>()?;
        let acc1_id = reader.read_u16::<LittleEndian>()?;
        let acc2_id = reader.read_u16::<LittleEndian>()?;
        let acc3_id = reader.read_u16::<LittleEndian>()?;
        let facemakeup2_id = reader.read_u16::<LittleEndian>()?;
        let leg_id = reader.read_u16::<LittleEndian>()?;
        let arm_id = reader.read_u16::<LittleEndian>()?;
        let acc4_id = reader.read_u16::<LittleEndian>()?;
        let mut unk5 = [0u8; 0x4];
        reader.read_exact(&mut unk5)?;
        let body_paint2 = reader.read_u16::<LittleEndian>()?;
        let left_eye_id = reader.read_u16::<LittleEndian>()?;
        let mut unk6 = [0u8; 0x12];
        reader.read_exact(&mut unk6)?;
        let acc1_size = AccessoryData::read(reader)?;
        let acc2_size = AccessoryData::read(reader)?;
        let acc3_size = AccessoryData::read(reader)?;
        let acc4_size = AccessoryData::read(reader)?;
        let acc1_rotation = AccessoryData::read(reader)?;
        let acc2_rotation = AccessoryData::read(reader)?;
        let acc3_rotation = AccessoryData::read(reader)?;
        let acc4_rotation = AccessoryData::read(reader)?;
        let unk7 = reader.read_u16::<LittleEndian>()?;
        let mut unk8 = [0u8; 0x8];
        reader.read_exact(&mut unk8)?;
        let skin_color_type = reader.read_u8()?;
        let eyebrow_thickness = reader.read_u8()?;
        Ok(Self {
            running_animation,
            race,
            gender,
            muscule,
            body,
            arms,
            legs,
            chest,
            face_shape,
            face_parts,
            eyes,
            nose_size,
            nose_height,
            mouth,
            ears,
            neck,
            waist,
            body2,
            arms2,
            legs2,
            chest2,
            neck2,
            waist2,
            unk1,
            unk2,
            acc1_location,
            acc2_location,
            acc3_location,
            acc4_location,
            unk_color,
            costume_color,
            main_color,
            sub1_color,
            sub2_color,
            sub3_color,
            eye_color,
            hair_color,
            unk3,
            unk4,
            costume_id,
            body_paint1,
            sticker_id,
            right_eye_id,
            eyebrow_id,
            eyelash_id,
            face_id1,
            face_id2,
            facemakeup1_id,
            hairstyle_id,
            acc1_id,
            acc2_id,
            acc3_id,
            facemakeup2_id,
            leg_id,
            arm_id,
            acc4_id,
            unk5,
            body_paint2,
            left_eye_id,
            unk6,
            acc1_size,
            acc2_size,
            acc3_size,
            acc4_size,
            acc1_rotation,
            acc2_rotation,
            acc3_rotation,
            acc4_rotation,
            unk7,
            unk8,
            skin_color_type,
            eyebrow_thickness,
        })
    }
    pub(crate) fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write_u16::<LittleEndian>(self.running_animation)?;
        writer.write_u16::<LittleEndian>(self.race as u16)?;
        writer.write_u16::<LittleEndian>(self.gender as u16)?;
        writer.write_u16::<LittleEndian>(self.muscule)?;
        self.body.write(writer)?;
        self.arms.write(writer)?;
        self.legs.write(writer)?;
        self.chest.write(writer)?;
        self.face_shape.write(writer)?;
        self.face_parts.write(writer)?;
        self.eyes.write(writer)?;
        self.nose_size.write(writer)?;
        self.nose_height.write(writer)?;
        self.mouth.write(writer)?;
        self.ears.write(writer)?;
        self.neck.write(writer)?;
        self.waist.write(writer)?;
        self.body2.write(writer)?;
        self.arms2.write(writer)?;
        self.legs2.write(writer)?;
        self.chest2.write(writer)?;
        self.neck2.write(writer)?;
        self.waist2.write(writer)?;
        writer.write_all(&self.unk1)?;
        writer.write_all(&self.unk2)?;
        self.acc1_location.write(writer)?;
        self.acc2_location.write(writer)?;
        self.acc3_location.write(writer)?;
        self.acc4_location.write(writer)?;
        self.unk_color.write(writer)?;
        self.costume_color.write(writer)?;
        self.main_color.write(writer)?;
        self.sub1_color.write(writer)?;
        self.sub2_color.write(writer)?;
        self.sub3_color.write(writer)?;
        self.eye_color.write(writer)?;
        self.hair_color.write(writer)?;
        writer.write_all(&self.unk3)?;
        writer.write_all(&self.unk4)?;
        writer.write_u16::<LittleEndian>(self.costume_id)?;
        writer.write_u16::<LittleEndian>(self.body_paint1)?;
        writer.write_u16::<LittleEndian>(self.sticker_id)?;
        writer.write_u16::<LittleEndian>(self.right_eye_id)?;
        writer.write_u16::<LittleEndian>(self.eyebrow_id)?;
        writer.write_u16::<LittleEndian>(self.eyelash_id)?;
        writer.write_u16::<LittleEndian>(self.face_id1)?;
        writer.write_u16::<LittleEndian>(self.face_id2)?;
        writer.write_u16::<LittleEndian>(self.facemakeup1_id)?;
        writer.write_u16::<LittleEndian>(self.hairstyle_id)?;
        writer.write_u16::<LittleEndian>(self.acc1_id)?;
        writer.write_u16::<LittleEndian>(self.acc2_id)?;
        writer.write_u16::<LittleEndian>(self.acc3_id)?;
        writer.write_u16::<LittleEndian>(self.facemakeup2_id)?;
        writer.write_u16::<LittleEndian>(self.leg_id)?;
        writer.write_u16::<LittleEndian>(self.arm_id)?;
        writer.write_u16::<LittleEndian>(self.acc4_id)?;
        writer.write_all(&self.unk5)?;
        writer.write_u16::<LittleEndian>(self.body_paint2)?;
        writer.write_u16::<LittleEndian>(self.left_eye_id)?;
        writer.write_all(&self.unk6)?;
        self.acc1_size.write(writer)?;
        self.acc2_size.write(writer)?;
        self.acc3_size.write(writer)?;
        self.acc4_size.write(writer)?;
        self.acc1_rotation.write(writer)?;
        self.acc2_rotation.write(writer)?;
        self.acc3_rotation.write(writer)?;
        self.acc4_rotation.write(writer)?;
        writer.write_u16::<LittleEndian>(self.unk7)?;
        writer.write_all(&self.unk8)?;
        writer.write_u8(self.skin_color_type)?;
        writer.write_u8(self.eyebrow_thickness)?;
        Ok(())
    }
}

impl Class {
    pub(crate) fn read(num: u8) -> Self {
        match num {
            0 => Self::Hunter,
            1 => Self::Ranger,
            2 => Self::Force,
            3 => Self::Fighter,
            4 => Self::Gunner,
            5 => Self::Techer,
            6 => Self::Braver,
            7 => Self::Bouncer,
            8 => Self::Challenger,
            9 => Self::Summoner,
            10 => Self::BattleWarrior,
            11 => Self::Hero,
            12 => Self::Phantom,
            13 => Self::Etole,
            14 => Self::Luster,
            _ => Self::Unknown,
        }
    }
}

impl ClassFlags {
    fn read(mut num: u16) -> Self {
        let mut flags = Self::default();
        if num & 0b0000_0001 != 0 {
            flags.hunter = true;
            num -= 0b0000_0001;
        }

        if num != 0 {
            println!("Unknown flags: {num}");
        }
        flags
    }
    fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        let mut num = 0;
        if self.hunter {
            num += 0b0000_0001;
        }
        writer.write_u16::<LittleEndian>(num)?;
        Ok(())
    }
}

impl ClassLevel {
    pub(crate) fn read(reader: &mut impl Read) -> std::io::Result<Self> {
        let level1 = reader.read_u16::<LittleEndian>()?;
        let level2 = reader.read_u16::<LittleEndian>()?;
        let exp = reader.read_u32::<LittleEndian>()?;
        Ok(Self {
            level1,
            level2,
            exp,
        })
    }
    pub(crate) fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write_u16::<LittleEndian>(self.level1)?;
        writer.write_u16::<LittleEndian>(self.level2)?;
        writer.write_u32::<LittleEndian>(self.exp)?;
        Ok(())
    }
}

impl ClassInfo {
    pub(crate) fn read(reader: &mut impl Read) -> std::io::Result<Self> {
        let unk1 = reader.read_u32::<LittleEndian>()?;
        let main_class = Class::read(reader.read_u8()?);
        let sub_class = Class::read(reader.read_u8()?);
        let unk2 = reader.read_u16::<LittleEndian>()?;
        let enabled_classes = ClassFlags::read(reader.read_u16::<LittleEndian>()?);
        let unk3 = reader.read_u16::<LittleEndian>()?;
        let hunter_info = ClassLevel::read(reader)?;
        let ranger_info = ClassLevel::read(reader)?;
        let force_info = ClassLevel::read(reader)?;
        let fighter_info = ClassLevel::read(reader)?;
        let gunner_info = ClassLevel::read(reader)?;
        let techer_info = ClassLevel::read(reader)?;
        let braver_info = ClassLevel::read(reader)?;
        let bouncer_info = ClassLevel::read(reader)?;
        let challenger_info = ClassLevel::read(reader)?;
        let summoner_info = ClassLevel::read(reader)?;
        let battle_warrior_info = ClassLevel::read(reader)?;
        let hero_info = ClassLevel::read(reader)?;
        let phantom_info = ClassLevel::read(reader)?;
        let etole_info = ClassLevel::read(reader)?;
        let luster_info = ClassLevel::read(reader)?;
        let unk16_info = ClassLevel::read(reader)?;
        let unk17_info = ClassLevel::read(reader)?;
        let unk18_info = ClassLevel::read(reader)?;
        let unk19_info = ClassLevel::read(reader)?;
        let unk20_info = ClassLevel::read(reader)?;
        let unk21_info = ClassLevel::read(reader)?;
        let unk22_info = ClassLevel::read(reader)?;
        let unk23_info = ClassLevel::read(reader)?;
        let unk24_info = ClassLevel::read(reader)?;
        let unk1_maxlevel = reader.read_u16::<LittleEndian>()?;
        let unk2_maxlevel = reader.read_u16::<LittleEndian>()?;
        let unk3_maxlevel = reader.read_u16::<LittleEndian>()?;
        let unk4_maxlevel = reader.read_u16::<LittleEndian>()?;
        let unk5_maxlevel = reader.read_u16::<LittleEndian>()?;
        let unk6_maxlevel = reader.read_u16::<LittleEndian>()?;
        let unk7_maxlevel = reader.read_u16::<LittleEndian>()?;
        let unk8_maxlevel = reader.read_u16::<LittleEndian>()?;
        let unk9_maxlevel = reader.read_u16::<LittleEndian>()?;
        let unk10_maxlevel = reader.read_u16::<LittleEndian>()?;
        let unk11_maxlevel = reader.read_u16::<LittleEndian>()?;
        let unk12_maxlevel = reader.read_u16::<LittleEndian>()?;
        let unk13_maxlevel = reader.read_u16::<LittleEndian>()?;
        let unk14_maxlevel = reader.read_u16::<LittleEndian>()?;
        let unk15_maxlevel = reader.read_u16::<LittleEndian>()?;
        Ok(Self {
            unk1,
            main_class,
            sub_class,
            unk2,
            enabled_classes,
            unk3,
            hunter_info,
            ranger_info,
            force_info,
            fighter_info,
            gunner_info,
            techer_info,
            braver_info,
            bouncer_info,
            challenger_info,
            summoner_info,
            battle_warrior_info,
            hero_info,
            phantom_info,
            etole_info,
            luster_info,
            unk16_info,
            unk17_info,
            unk18_info,
            unk19_info,
            unk20_info,
            unk21_info,
            unk22_info,
            unk23_info,
            unk24_info,
            unk1_maxlevel,
            unk2_maxlevel,
            unk3_maxlevel,
            unk4_maxlevel,
            unk5_maxlevel,
            unk6_maxlevel,
            unk7_maxlevel,
            unk8_maxlevel,
            unk9_maxlevel,
            unk10_maxlevel,
            unk11_maxlevel,
            unk12_maxlevel,
            unk13_maxlevel,
            unk14_maxlevel,
            unk15_maxlevel,
        })
    }
    pub(crate) fn write(&self, writer: &mut impl Write) -> std::io::Result<()> {
        writer.write_u32::<LittleEndian>(self.unk1)?;
        writer.write_u8(self.main_class as u8)?;
        writer.write_u8(self.sub_class as u8)?;
        writer.write_u16::<LittleEndian>(self.unk2)?;
        self.enabled_classes.write(writer)?;
        writer.write_u16::<LittleEndian>(self.unk3)?;
        self.hunter_info.write(writer)?;
        self.ranger_info.write(writer)?;
        self.force_info.write(writer)?;
        self.fighter_info.write(writer)?;
        self.gunner_info.write(writer)?;
        self.techer_info.write(writer)?;
        self.braver_info.write(writer)?;
        self.bouncer_info.write(writer)?;
        self.challenger_info.write(writer)?;
        self.summoner_info.write(writer)?;
        self.battle_warrior_info.write(writer)?;
        self.hero_info.write(writer)?;
        self.phantom_info.write(writer)?;
        self.etole_info.write(writer)?;
        self.luster_info.write(writer)?;
        self.unk16_info.write(writer)?;
        self.unk17_info.write(writer)?;
        self.unk18_info.write(writer)?;
        self.unk19_info.write(writer)?;
        self.unk20_info.write(writer)?;
        self.unk21_info.write(writer)?;
        self.unk22_info.write(writer)?;
        self.unk23_info.write(writer)?;
        self.unk24_info.write(writer)?;
        writer.write_u16::<LittleEndian>(self.unk1_maxlevel)?;
        writer.write_u16::<LittleEndian>(self.unk2_maxlevel)?;
        writer.write_u16::<LittleEndian>(self.unk3_maxlevel)?;
        writer.write_u16::<LittleEndian>(self.unk4_maxlevel)?;
        writer.write_u16::<LittleEndian>(self.unk5_maxlevel)?;
        writer.write_u16::<LittleEndian>(self.unk6_maxlevel)?;
        writer.write_u16::<LittleEndian>(self.unk7_maxlevel)?;
        writer.write_u16::<LittleEndian>(self.unk8_maxlevel)?;
        writer.write_u16::<LittleEndian>(self.unk9_maxlevel)?;
        writer.write_u16::<LittleEndian>(self.unk10_maxlevel)?;
        writer.write_u16::<LittleEndian>(self.unk11_maxlevel)?;
        writer.write_u16::<LittleEndian>(self.unk12_maxlevel)?;
        writer.write_u16::<LittleEndian>(self.unk13_maxlevel)?;
        writer.write_u16::<LittleEndian>(self.unk14_maxlevel)?;
        writer.write_u16::<LittleEndian>(self.unk15_maxlevel)?;
        Ok(())
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
