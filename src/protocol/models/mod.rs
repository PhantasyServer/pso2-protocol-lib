pub mod character;
use crate::protocol::HelperReadWrite;
use half::f16;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Clone, PartialEq, HelperReadWrite)]
pub struct Position {
    pub rot_x: f16,
    pub rot_y: f16,
    pub rot_z: f16,
    pub rot_w: f16,
    pub pos_x: f16,
    pub pos_y: f16,
    pub pos_z: f16,
}
