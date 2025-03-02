//! Character related structures.
use crate::{
    asciistring::StringRW,
    protocol::{HelperReadWrite, PacketError, PacketType},
};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Seek, Write};

// ----------------------------------------------------------------
// Structures
// ----------------------------------------------------------------

/// Character data. (Classic only)
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Character {
    pub character_id: u32,
    pub player_id: u32,
    pub unk1: u32,
    /// Voice type ID.
    pub voice_type: u32,
    pub unk2: u16,
    pub voice_pitch: i16,
    pub name: String,
    pub look: Look,
    pub unk3: u32,
    pub classes: ClassInfo,
    pub unk4: String,
}

/// HSV color data.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct HSVColor {
    /// 0-60000
    pub hue: u16,
    /// 0-60000
    pub saturation: u16,
    /// 0-10000
    pub value: u16,
}

/// Character's figure data.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
// I'm unsure if we need to name these fields
pub struct Figure(pub u16, pub u16, pub u16);

/// Character's accessory data. Represented by three sliders during editing.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct AccessoryData(pub i8, pub i8, pub i8);

/// Character race.
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

/// Character gender.
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

/// Character's look data.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Clone, PartialEq, HelperReadWrite)]
pub struct Look {
    /// Running animation ID.
    pub running_animation: RunAnimation,
    pub race: Race,
    pub gender: Gender,
    /// Muscule mass.
    pub muscule: u16,
    /// Body proportions.
    pub body: Figure,
    /// Arm proportions.
    pub arms: Figure,
    /// Leg proportions.
    pub legs: Figure,
    /// Chest proportions.
    pub chest: Figure,
    pub face_shape: Figure,
    /// Face parts position.
    pub face_parts: Figure,
    /// Eye shape.
    pub eyes: Figure,
    pub nose_size: Figure,
    pub nose_height: Figure,
    /// Mouth shape and size.
    pub mouth: Figure,
    /// Ear shape for newmans, horn shape for deumans.
    pub ears: Figure,
    /// Head proportions.
    pub neck: Figure,
    /// Waist proportions.
    pub waist: Figure,
    /// Duplicate of `body`.
    pub body2: Figure,
    /// Duplicate of `arms`.
    pub arms2: Figure,
    /// Duplicate of `legs`.
    pub legs2: Figure,
    /// Duplicate of `chest`.
    pub chest2: Figure,
    /// Duplicate of `neck`.
    pub neck2: Figure,
    /// Duplicate of `waist`.
    pub waist2: Figure,
    pub unk1: [u8; 0x20],
    pub unk2: [u8; 0x0A],
    /// Accessory 1 position.
    pub acc1_location: AccessoryData,
    /// Accessory 2 position.
    pub acc2_location: AccessoryData,
    /// Accessory 3 position.
    pub acc3_location: AccessoryData,
    /// Accessory 4 position.
    pub acc4_location: AccessoryData,
    pub unk_color: HSVColor,
    /// Outfit color.
    pub costume_color: HSVColor,
    /// Main color for CASTs.
    pub main_color: HSVColor,
    /// Sub-color 1 for CASTs.
    pub sub1_color: HSVColor,
    /// Skin color for non-CASTs, sub-color 2 for CASTs.
    pub sub2_color: HSVColor,
    /// Left eye color for deumans, sub-color 3 for CASTs.
    pub sub3_color: HSVColor,
    /// Character eye color (right eye color for deumans).
    pub eye_color: HSVColor,
    pub hair_color: HSVColor,
    pub unk3: [u8; 0x20],
    pub unk4: [u8; 0x10],
    /// Outfit ID for non-CASTs, body part ID for CASTs.
    pub costume_id: u16,
    /// Body paint 1 ID.
    pub body_paint1: u16,
    /// Outfit decal ID.
    pub sticker_id: u16,
    /// Eye iris ID (right eye for deumans).
    pub right_eye_id: u16,
    /// Eyebrow ID and color.
    pub eyebrow_id: u16,
    pub eyelash_id: u16,
    // I have no idea why there are 2 face ids.
    /// Face ID 1.
    pub face_id1: u16,
    /// Face ID 2.
    pub face_id2: u16,
    /// Face makeup pattern 1 ID.
    pub facemakeup1_id: u16,
    pub hairstyle_id: u16,
    /// Accessory 1 ID.
    pub acc1_id: u16,
    /// Accessory 2 ID.
    pub acc2_id: u16,
    /// Accessory 3 ID.
    pub acc3_id: u16,
    /// Face makeup pattern 2 ID.
    pub facemakeup2_id: u16,
    /// Leg part ID for CASTs.
    pub leg_id: u16,
    /// Arm part ID for CASTs.
    pub arm_id: u16,
    /// Accessory 4 ID.
    pub acc4_id: u16,
    pub unk5: [u8; 0x4],
    /// Body paint 2 ID.
    pub body_paint2: u16,
    /// Left eye iris ID (matters only for deumans).
    pub left_eye_id: u16,
    pub unk6: [u8; 0x12],
    /// Accessory 1 size.
    pub acc1_size: AccessoryData,
    /// Accessory 2 size.
    pub acc2_size: AccessoryData,
    /// Accessory 3 size.
    pub acc3_size: AccessoryData,
    /// Accessory 4 size.
    pub acc4_size: AccessoryData,
    /// Accessory 1 angle.
    pub acc1_rotation: AccessoryData,
    /// Accessory 2 angle.
    pub acc2_rotation: AccessoryData,
    /// Accessory 3 angle.
    pub acc3_rotation: AccessoryData,
    /// Accessory 4 angle.
    pub acc4_rotation: AccessoryData,
    pub unk7: u16,
    pub unk8: [u8; 0x8],
    pub skin_color_type: SkinColor,
    pub eyebrow_thickness: i8,
}

/// Character's run animation.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u16)]
pub enum RunAnimation {
    #[default]
    #[Read_default]
    Walking = 9,
    /// Hover animation (only for CASTs).
    Hovering = 11,
}

/// Character's skin color type
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq, HelperReadWrite)]
#[repr(u8)]
pub enum SkinColor {
    #[default]
    #[Read_default]
    RaceDefined,
    Human,
    Deuman,
    Cast,
}

/// Character class.
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

bitflags::bitflags! {
    /// Enabled classes flags.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
    #[BitFlags(u16)]
    pub struct ClassFlags: u16 {
        const Hunter = 1 << 0;
        const Ranger = 1 << 1;
        const Force = 1 << 2;
        const Fighter = 1 << 3;
        const Gunner = 1 << 4;
        const Techer = 1 << 5;
        const Braver = 1 << 6;
        const Bouncer = 1 << 7;
        const Challenger = 1 << 8;
        const Summoner = 1 << 9;
        const BattleWarrior = 1 << 10;
        const Hero = 1 << 11;
        const Phantom = 1 << 12;
        const Etole = 1 << 13;
        const Luster = 1 << 14;
    }
}

/// Character class level.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Copy, Clone, PartialEq, HelperReadWrite)]
pub struct ClassLevel {
    /// Main level.
    pub level1: u16,
    pub level2: u16,
    /// Current EXP.
    pub exp: u32,
}

/// Info about the character classes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct ClassInfo {
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
    fn read(
        reader: &mut (impl Read + Seek),
        packet_type: PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<Self, PacketError> {
        let character_id =
            reader
                .read_u32::<LittleEndian>()
                .map_err(|e| PacketError::FieldError {
                    packet_name: "Character",
                    field_name: "character_id",
                    error: e,
                })?;
        let player_id = reader
            .read_u32::<LittleEndian>()
            .map_err(|e| PacketError::FieldError {
                packet_name: "Character",
                field_name: "player_id",
                error: e,
            })?;
        let unk1 = reader
            .read_u32::<LittleEndian>()
            .map_err(|e| PacketError::FieldError {
                packet_name: "Character",
                field_name: "unk1",
                error: e,
            })?;
        let voice_type =
            reader
                .read_u32::<LittleEndian>()
                .map_err(|e| PacketError::FieldError {
                    packet_name: "Character",
                    field_name: "voice_type",
                    error: e,
                })?;
        let unk2 = reader
            .read_u16::<LittleEndian>()
            .map_err(|e| PacketError::FieldError {
                packet_name: "Character",
                field_name: "unk2",
                error: e,
            })?;
        let voice_pitch =
            reader
                .read_i16::<LittleEndian>()
                .map_err(|e| PacketError::FieldError {
                    packet_name: "Character",
                    field_name: "voice_pitch",
                    error: e,
                })?;
        let name = String::read_fixed(reader, 16).map_err(|e| PacketError::FieldError {
            packet_name: "Character",
            field_name: "name",
            error: e,
        })?;

        if matches!(packet_type, PacketType::Vita) {
            reader
                .seek(std::io::SeekFrom::Current(4))
                .map_err(|e| PacketError::PaddingError {
                    packet_name: "Character",
                    field_name: "look",
                    error: e,
                })?;
        }

        let look = Look::read(reader, packet_type, xor, sub).map_err(|e| {
            PacketError::CompositeFieldError {
                packet_name: "Character",
                field_name: "look",
                error: Box::new(e),
            }
        })?;
        let unk3 = reader
            .read_u32::<LittleEndian>()
            .map_err(|e| PacketError::FieldError {
                packet_name: "Character",
                field_name: "unk3",
                error: e,
            })?;
        let classes = ClassInfo::read(reader, packet_type, xor, sub).map_err(|e| {
            PacketError::CompositeFieldError {
                packet_name: "Character",
                field_name: "classes",
                error: Box::new(e),
            }
        })?;

        let unk4 = String::read_fixed(reader, 32).map_err(|e| PacketError::FieldError {
            packet_name: "Character",
            field_name: "unk4",
            error: e,
        })?;

        reader
            .seek(std::io::SeekFrom::Current(0x56))
            .map_err(|e| PacketError::PaddingError {
                packet_name: "Character",
                field_name: "unk3",
                error: e,
            })?;
        if matches!(packet_type, PacketType::NA) {
            reader
                .seek(std::io::SeekFrom::Current(4))
                .map_err(|e| PacketError::PaddingError {
                    packet_name: "Character",
                    field_name: "unk3",
                    error: e,
                })?;
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
            unk3,
            classes,
            unk4,
        })
    }
    fn write(
        &self,
        writer: &mut impl Write,
        packet_type: PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<(), PacketError> {
        writer
            .write_u32::<LittleEndian>(self.character_id)
            .map_err(|e| PacketError::FieldError {
                packet_name: "Character",
                field_name: "character_id",
                error: e,
            })?;
        writer
            .write_u32::<LittleEndian>(self.player_id)
            .map_err(|e| PacketError::FieldError {
                packet_name: "Character",
                field_name: "player_id",
                error: e,
            })?;
        writer
            .write_u32::<LittleEndian>(self.unk1)
            .map_err(|e| PacketError::FieldError {
                packet_name: "Character",
                field_name: "unk1",
                error: e,
            })?;
        writer
            .write_u32::<LittleEndian>(self.voice_type)
            .map_err(|e| PacketError::FieldError {
                packet_name: "Character",
                field_name: "voice_type",
                error: e,
            })?;
        writer
            .write_u16::<LittleEndian>(self.unk2)
            .map_err(|e| PacketError::FieldError {
                packet_name: "Character",
                field_name: "unk2",
                error: e,
            })?;
        writer
            .write_i16::<LittleEndian>(self.voice_pitch)
            .map_err(|e| PacketError::FieldError {
                packet_name: "Character",
                field_name: "voice_pitch",
                error: e,
            })?;
        writer
            .write_all(&self.name.write_fixed(16))
            .map_err(|e| PacketError::FieldError {
                packet_name: "Character",
                field_name: "name",
                error: e,
            })?;

        if matches!(packet_type, PacketType::Vita) {
            writer
                .write_u32::<LittleEndian>(0)
                .map_err(|e| PacketError::PaddingError {
                    packet_name: "Character",
                    field_name: "look",
                    error: e,
                })?;
        }
        self.look
            .write(writer, packet_type, xor, sub)
            .map_err(|e| PacketError::CompositeFieldError {
                packet_name: "Character",
                field_name: "look",
                error: Box::new(e),
            })?;
        writer
            .write_u32::<LittleEndian>(self.unk3)
            .map_err(|e| PacketError::FieldError {
                packet_name: "Character",
                field_name: "unk3",
                error: e,
            })?;
        self.classes
            .write(writer, packet_type, xor, sub)
            .map_err(|e| PacketError::CompositeFieldError {
                packet_name: "Character",
                field_name: "classes",
                error: Box::new(e),
            })?;
        writer
            .write_all(&self.unk4.write_fixed(32))
            .map_err(|e| PacketError::FieldError {
                packet_name: "Character",
                field_name: "unk4",
                error: e,
            })?;

        writer
            .write_all(&[0u8; 0x56])
            .map_err(|e| PacketError::PaddingError {
                packet_name: "Character",
                field_name: "unk3",
                error: e,
            })?;
        if matches!(packet_type, PacketType::NA) {
            writer
                .write_u32::<LittleEndian>(0)
                .map_err(|e| PacketError::PaddingError {
                    packet_name: "Character",
                    field_name: "unk3",
                    error: e,
                })?;
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
            running_animation: RunAnimation::Walking,
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
            skin_color_type: SkinColor::RaceDefined,
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
