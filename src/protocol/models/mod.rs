use crate::protocol::HelperReadWrite;

pub mod character;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Position {
    pub rot_x: u16,
    pub rot_y: u16,
    pub rot_z: u16,
    pub rot_w: u16,
    pub pos_x: u16,
    pub pos_y: u16,
    pub pos_z: u16,
}
