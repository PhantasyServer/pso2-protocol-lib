//! Common packet structures.
pub mod character;
#[cfg(feature = "item_attrs")]
#[cfg_attr(docsrs, doc(cfg(feature = "item_attrs")))]
pub mod item_attrs;

use super::{PacketError, PacketType};
use crate::protocol::HelperReadWrite;
use half::f16;

// ----------------------------------------------------------------
// Structures
// ----------------------------------------------------------------

/// Generic object position. Almost always followed by one [`u16`] padding.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Copy, Clone, PartialEq, HelperReadWrite)]
pub struct Position {
    /// X rotation quaternion.
    pub rot_x: f16,
    /// Y rotation quaternion.
    pub rot_y: f16,
    /// Z rotation quaternion.
    pub rot_z: f16,
    /// W rotation quaternion.
    pub rot_w: f16,
    /// X position.
    pub pos_x: f16,
    /// Y position.
    pub pos_y: f16,
    /// Z position.
    pub pos_z: f16,
}

/// Euler type position. Not used in game, just for printing.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct EulerPosition {
    /// Roll angle in radians.
    pub roll: f32,
    /// Pitch angle in radians.
    pub pitch: f32,
    /// Yaw angle in radians.
    pub yaw: f32,
    /// X position.
    pub x: f32,
    /// Y position.
    pub y: f32,
    /// Z position.
    pub z: f32,
}

// For implementation details look at the HelperReadWrite impl.
/// SG currency value.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SGValue(pub f32);

// For implementation details look at the HelperReadWrite impl.
/// Fun currency value.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct FunValue(pub u32);

// ----------------------------------------------------------------
// Read/Write implementations
// ----------------------------------------------------------------

impl HelperReadWrite for EulerPosition {
    fn read(
        reader: &mut &[u8],
        packet_type: PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<Self, PacketError> {
        let pos = Position::read(reader, packet_type, xor, sub)?;
        Ok(pos.into())
    }

    fn write(&self, writer: &mut Vec<u8>, packet_type: PacketType, xor: u32, sub: u32) {
        let pos: Position = (*self).into();
        pos.write(writer, packet_type, xor, sub)
    }
}

impl HelperReadWrite for SGValue {
    fn read(reader: &mut &[u8], pt: PacketType, _: u32, _: u32) -> Result<Self, PacketError> {
        let mut buf = u32::read(reader, pt, 0, 0)
            .map_err(|e| PacketError::CompositeFieldError {
                packet_name: "SGValue",
                field_name: "value",
                error: Box::new(e),
            })?
            .to_le_bytes();
        buf.swap(2, 0);
        buf.swap(3, 1);
        let value = u32::from_le_bytes(buf);
        let value = value as f32 / 5.0;
        Ok(Self(value))
    }

    fn write(&self, writer: &mut Vec<u8>, _: PacketType, _: u32, _: u32) {
        let value = (self.0 * 5.0) as u32;
        let mut buf = value.to_le_bytes();
        buf.swap(2, 0);
        buf.swap(3, 1);
        writer.extend_from_slice(&buf);
    }
}

impl HelperReadWrite for FunValue {
    fn read(
        reader: &mut &[u8],
        pt: PacketType,
        _: u32,
        _: u32,
    ) -> Result<Self, PacketError> {
        let mut buf = u32::read(reader, pt, 0, 0)
            .map_err(|e| PacketError::CompositeFieldError {
                packet_name: "FunValue",
                field_name: "value",
                error: Box::new(e),
            })?
            .to_le_bytes();
        buf.swap(2, 0);
        buf.swap(3, 1);
        let value = u32::from_le_bytes(buf);
        Ok(Self(value))
    }

    fn write(&self, writer: &mut Vec<u8>, _: PacketType, _: u32, _: u32) {
        let mut buf = self.0.to_le_bytes();
        buf.swap(2, 0);
        buf.swap(3, 1);
        writer.extend_from_slice(&buf);
    }
}

// ----------------------------------------------------------------
// Transformation implementations
// ----------------------------------------------------------------

impl From<Position> for EulerPosition {
    fn from(value: Position) -> Self {
        let x = value.pos_x.to_f32();
        let y = value.pos_y.to_f32();
        let z = value.pos_z.to_f32();
        let qx = value.rot_x.to_f32();
        let qy = value.rot_y.to_f32();
        let qz = value.rot_z.to_f32();
        let qw = value.rot_w.to_f32();
        let (roll, pitch, yaw) = quat_to_euler(qx, qy, qz, qw);
        Self {
            roll,
            pitch,
            yaw,
            x,
            y,
            z,
        }
    }
}

impl From<EulerPosition> for Position {
    fn from(value: EulerPosition) -> Self {
        let pos_x = f16::from_f32(value.x);
        let pos_y = f16::from_f32(value.y);
        let pos_z = f16::from_f32(value.z);
        let (qx, qy, qz, qw) = euler_to_quat(value.roll, value.pitch, value.yaw);
        let rot_x = f16::from_f32(qx);
        let rot_y = f16::from_f32(qy);
        let rot_z = f16::from_f32(qz);
        let rot_w = f16::from_f32(qw);
        Self {
            rot_x,
            rot_y,
            rot_z,
            rot_w,
            pos_x,
            pos_y,
            pos_z,
        }
    }
}

//output angles are in radians
fn quat_to_euler(qx: f32, qy: f32, qz: f32, qw: f32) -> (f32, f32, f32) {
    let sqx = qx * qx;
    let sqy = qy * qy;
    let sqz = qz * qz;
    let sqw = qw * qw;
    let unit = sqx + sqy + sqz + sqw;
    let test = qx * qy + qz * qw;

    if test > 0.499 * unit {
        let yaw = 2.0 * qx.atan2(qw);
        let pitch = std::f32::consts::PI / 2.0;
        let roll = 0.0;
        return (roll, pitch, yaw);
    }
    if test < -0.499 * unit {
        let yaw = -2.0 * qx.atan2(qw);
        let pitch = -std::f32::consts::PI / 2.0;
        let roll = 0.0;
        return (roll, pitch, yaw);
    }

    let yaw_y = 2.0 * (qy * qw - qx * qz);
    let yaw_x = sqx - sqy - sqz + sqw;
    let yaw = yaw_y.atan2(yaw_x);

    let pitch = (2.0 * test / unit).asin();

    let roll_y = 2.0 * (qx * qw - qy * qz);
    let roll_x = -sqx + sqy - sqz + sqw;
    let roll = roll_y.atan2(roll_x);

    (roll, pitch, yaw)
}

// angles are in radians
fn euler_to_quat(roll: f32, pitch: f32, yaw: f32) -> (f32, f32, f32, f32) {
    let (sr, cr) = (roll * 0.5).sin_cos();
    let (sp, cp) = (pitch * 0.5).sin_cos();
    let (sy, cy) = (yaw * 0.5).sin_cos();

    let qx = sr * cp * cy - cr * sp * sy;
    let qy = cr * sp * cy + sr * cp * sy;
    let qz = cr * cp * sy - sr * sp * cy;
    let qw = cr * cp * cy + sr * sp * sy;

    (qx, qy, qz, qw)
}

// ----------------------------------------------------------------
// Other implementations
// ----------------------------------------------------------------

impl Position {
    pub fn dist(&self, o: &Self) -> f64 {
        let (x, y, z): (f64, f64, f64) = (self.pos_x.into(), self.pos_y.into(), self.pos_z.into());
        let (ox, oy, oz): (f64, f64, f64) = (o.pos_x.into(), o.pos_y.into(), o.pos_z.into());
        f64::sqrt((ox - x).powi(2) + (oy - y).powi(2) + (oz - z).powi(2))
    }
    pub fn dist_2d(&self, o: &Self) -> f64 {
        let (x, z): (f64, f64) = (self.pos_x.into(), self.pos_z.into());
        let (ox, oz): (f64, f64) = (o.pos_x.into(), o.pos_z.into());
        f64::sqrt((ox - x).powi(2) + (oz - z).powi(2))
    }
}

impl EulerPosition {
    pub fn dist(&self, o: &Self) -> f64 {
        let (x, y, z): (f64, f64, f64) = (self.x.into(), self.y.into(), self.z.into());
        let (ox, oy, oz): (f64, f64, f64) = (o.x.into(), o.y.into(), o.z.into());
        f64::sqrt((ox - x).powi(2) + (oy - y).powi(2) + (oz - z).powi(2))
    }
    pub fn dist_2d(&self, o: &Self) -> f64 {
        let (x, z): (f64, f64) = (self.x.into(), self.z.into());
        let (ox, oz): (f64, f64) = (o.x.into(), o.z.into());
        f64::sqrt((ox - x).powi(2) + (oz - z).powi(2))
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            rot_x: f16::from_f32(0.0),
            rot_y: f16::from_f32(0.0),
            rot_z: f16::from_f32(0.0),
            rot_w: f16::from_f32(1.0),
            pos_x: f16::from_f32(0.0),
            pos_y: f16::from_f32(0.0),
            pos_z: f16::from_f32(0.0),
        }
    }
}
