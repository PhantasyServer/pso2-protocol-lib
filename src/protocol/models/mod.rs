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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct SGValue(pub f32);
impl HelperReadWrite for SGValue {
    fn read(reader: &mut (impl std::io::Read + std::io::Seek)) -> std::io::Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf[2..4])?;
        reader.read_exact(&mut buf[0..2])?;
        let value = u32::from_le_bytes(buf);
        let value = value as f32 / 5.0;
        Ok(Self(value))
    }

    fn write(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        let value = (self.0 * 5.0) as u32;
        let buf = value.to_le_bytes();
        writer.write(&buf[2..4])?;
        writer.write(&buf[0..2])?;
        Ok(())
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct FunValue(pub u32);
impl HelperReadWrite for FunValue {
    fn read(reader: &mut (impl std::io::Read + std::io::Seek)) -> std::io::Result<Self> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf[2..4])?;
        reader.read_exact(&mut buf[0..2])?;
        let value = u32::from_le_bytes(buf);
        Ok(Self(value))
    }

    fn write(&self, writer: &mut impl std::io::Write) -> std::io::Result<()> {
        let buf = self.0.to_le_bytes();
        writer.write(&buf[2..4])?;
        writer.write(&buf[0..2])?;
        Ok(())
    }
}
