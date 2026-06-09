use crate::{
    asciistring::StringRW,
    protocol::{read_magic, write_magic, HelperReadWrite, PacketError},
    AsciiString,
};
use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    time::Duration,
};

// ----------------------------------------------------------------
// Type definitions
// ----------------------------------------------------------------

/// Opaque type for fixed sized string.
#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct FixedString<const N: usize> {
    // might change in the future
    string: String,
}

/// Opaque type for fixed sized ascii string.
#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct FixedAsciiString<const N: usize> {
    // might change in the future
    string: AsciiString,
}

/// Opaque type for windows FILETIME (previously attribute `#[PSOTime]`).
#[derive(Clone, Default, Debug, Hash, PartialEq, PartialOrd)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct WinTime {
    time: Duration,
}

/// Opaque type for fixed sized array.
#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct FixedVec<const N: usize, T> {
    // might change in the future
    data: Vec<T>,
}

trait SizeProvider {
    fn to_size(reader: &mut &[u8]) -> Result<u32, PacketError>;
    fn to_data(size: usize) -> Vec<u8>;
}

/// Opaque type for integer sized array (with no magic).
#[derive(Clone, Default, Debug, Hash, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct VecUSize<S, T> {
    // might change in the future
    data: Vec<T>,
    _p_data: PhantomData<S>,
}

/// Opaque type for dynamic byte array.
#[derive(Clone, Default, Debug, Hash, PartialEq, PartialOrd)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Bytes<const NO_PADDING: bool = false> {
    #[cfg_attr(feature = "serde", serde(with = "serde_bytes"))]
    bytes: Vec<u8>,
}

/// Opaque type for fixed sized byte array.
#[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
#[repr(transparent)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct FixedBytes<const N: usize, const NO_PADDING: bool = false> {
    bytes: Vec<u8>,
}

// ----------------------------------------------------------------
// Trait implementations
// ----------------------------------------------------------------

macro_rules! helper_int {
    ($name:ty) => {
        impl SizeProvider for $name {
            fn to_size(reader: &mut &[u8]) -> Result<u32, PacketError> {
                Ok(<$name>::read(reader, crate::protocol::PacketType::Classic, 0, 0)? as u32)
            }
            fn to_data(size: usize) -> Vec<u8> {
                (size as $name).to_le_bytes().to_vec()

            }
        }

    };
    ($name:ty, $($name_r:ty),+) => {
        helper_int!($name);
        helper_int!($($name_r),+);
    };
}

helper_int!(u8, u16, u32, u64);

impl<const N: usize> Deref for FixedString<N> {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.string
    }
}
impl<const N: usize> DerefMut for FixedString<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.string
    }
}
impl<const N: usize> From<String> for FixedString<N> {
    fn from(value: String) -> Self {
        if cfg!(test) {
            let value: String = value.chars().chain("\0".chars().cycle()).take(N).collect();
            Self { string: value }
        } else {
            Self { string: value }
        }
    }
}
impl<const N: usize> Display for FixedString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.string.fmt(f)
    }
}
impl<const N: usize> HelperReadWrite for FixedString<N> {
    fn read(
        reader: &mut &[u8],
        _: crate::protocol::PacketType,
        _: u32,
        _: u32,
    ) -> Result<Self, crate::protocol::PacketError> {
        Ok(Self {
            string: <String as StringRW>::read_fixed(reader, N as _)?,
        })
    }

    fn write(&self, writer: &mut Vec<u8>, _: crate::protocol::PacketType, _: u32, _: u32) {
        writer.extend_from_slice(&StringRW::write_fixed(&self.string, N as _));
    }
}
#[cfg(feature = "serde")]
impl<'de, const N: usize> serde::Deserialize<'de> for FixedString<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

impl<const N: usize> Deref for FixedAsciiString<N> {
    type Target = AsciiString;

    fn deref(&self) -> &Self::Target {
        &self.string
    }
}
impl<const N: usize> DerefMut for FixedAsciiString<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.string
    }
}
impl<const N: usize> From<String> for FixedAsciiString<N> {
    fn from(value: String) -> Self {
        if cfg!(test) {
            let value: String = value
                .chars()
                .filter(char::is_ascii)
                .chain("\0".chars().cycle())
                .take(N)
                .collect();
            Self {
                string: value.into(),
            }
        } else {
            Self {
                string: value.into(),
            }
        }
    }
}
impl<const N: usize> From<AsciiString> for FixedAsciiString<N> {
    fn from(value: AsciiString) -> Self {
        Self { string: value }
    }
}
impl<const N: usize> Display for FixedAsciiString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.string.fmt(f)
    }
}
impl<const N: usize> HelperReadWrite for FixedAsciiString<N> {
    fn read(
        reader: &mut &[u8],
        _: crate::protocol::PacketType,
        _: u32,
        _: u32,
    ) -> Result<Self, crate::protocol::PacketError> {
        Ok(Self {
            string: <AsciiString as StringRW>::read_fixed(reader, N as _)?,
        })
    }

    fn write(&self, writer: &mut Vec<u8>, _: crate::protocol::PacketType, _: u32, _: u32) {
        writer.extend_from_slice(&self.string.write_fixed(N as _));
    }
}
#[cfg(feature = "serde")]
impl<'de, const N: usize> serde::Deserialize<'de> for FixedAsciiString<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(AsciiString::deserialize(deserializer)?.into())
    }
}

impl Deref for WinTime {
    type Target = Duration;

    fn deref(&self) -> &Self::Target {
        &self.time
    }
}
impl DerefMut for WinTime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.time
    }
}
impl From<Duration> for WinTime {
    fn from(value: Duration) -> Self {
        let value = Duration::from_secs(value.as_secs());
        Self { time: value }
    }
}
// this is the constant for convering between windows FILETIME
// (modified to use miliseconds instead of 100ns ticks)
// and unix timestamp
const WIN_FT_TIME_TO_TIMESTAMP: u64 = 0x0A97_30B6_6800;
impl HelperReadWrite for WinTime {
    fn read(
        reader: &mut &[u8],
        packet_type: crate::protocol::PacketType,
        _: u32,
        _: u32,
    ) -> Result<Self, crate::protocol::PacketError> {
        Ok(Self {
            time: Duration::from_millis(
                u64::read(reader, packet_type, 0, 0)
                    .map_err(|e| PacketError::CompositeFieldError {
                        packet_name: "WinTime",
                        field_name: "time",
                        error: e.into(),
                    })?
                    .saturating_sub(WIN_FT_TIME_TO_TIMESTAMP),
            ),
        })
    }

    fn write(
        &self,
        writer: &mut Vec<u8>,
        packet_type: crate::protocol::PacketType,
        _: u32,
        _: u32,
    ) {
        (self.time.as_millis() as u64 + WIN_FT_TIME_TO_TIMESTAMP).write(writer, packet_type, 0, 0);
    }
}

impl<const N: usize, T> Deref for FixedVec<N, T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<const N: usize, T> DerefMut for FixedVec<N, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
impl<const N: usize, T> From<Vec<T>> for FixedVec<N, T> {
    fn from(value: Vec<T>) -> Self {
        Self { data: value }
    }
}
impl<const N: usize, T> From<FixedVec<N, T>> for Vec<T> {
    fn from(value: FixedVec<N, T>) -> Self {
        value.data
    }
}
impl<const N: usize, T: HelperReadWrite + Default> HelperReadWrite for FixedVec<N, T> {
    fn read(
        reader: &mut &[u8],
        packet_type: crate::protocol::PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<Self, crate::protocol::PacketError> {
        let mut data = vec![];
        data.reserve_exact(N);

        for _ in 0..N {
            data.push(T::read(reader, packet_type, xor, sub).map_err(|e| {
                PacketError::CompositeFieldError {
                    packet_name: "FixedVec",
                    field_name: "value",
                    error: e.into(),
                }
            })?);
        }
        Ok(Self { data })
    }

    fn write(
        &self,
        writer: &mut Vec<u8>,
        packet_type: crate::protocol::PacketType,
        xor: u32,
        sub: u32,
    ) {
        for i in self.iter().chain(std::iter::repeat(&T::default())).take(N) {
            i.write(writer, packet_type, xor, sub);
        }
    }
}
#[cfg(feature = "serde")]
impl<'de, const N: usize, T: serde::Deserialize<'de>> serde::Deserialize<'de> for FixedVec<N, T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Vec::<T>::deserialize(deserializer)?.into())
    }
}

impl<S, T> Deref for VecUSize<S, T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
impl<S, T> DerefMut for VecUSize<S, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
impl<S, T> From<Vec<T>> for VecUSize<S, T> {
    fn from(value: Vec<T>) -> Self {
        Self {
            data: value,
            _p_data: PhantomData,
        }
    }
}
impl<S, T> From<VecUSize<S, T>> for Vec<T> {
    fn from(value: VecUSize<S, T>) -> Self {
        value.data
    }
}
impl<S: SizeProvider, T: HelperReadWrite> HelperReadWrite for VecUSize<S, T> {
    fn read(
        reader: &mut &[u8],
        packet_type: crate::protocol::PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<Self, crate::protocol::PacketError> {
        let len = S::to_size(reader).map_err(|e| PacketError::CompositeFieldError {
            packet_name: "VecUSize",
            field_name: "len",
            error: e.into(),
        })?;
        let mut data = vec![];
        data.reserve_exact(len as usize);

        for _ in 0..len {
            data.push(T::read(reader, packet_type, xor, sub).map_err(|e| {
                PacketError::CompositeFieldError {
                    packet_name: "VecUSize",
                    field_name: "value",
                    error: e.into(),
                }
            })?);
        }
        Ok(Self {
            data,
            _p_data: PhantomData,
        })
    }

    fn write(
        &self,
        writer: &mut Vec<u8>,
        packet_type: crate::protocol::PacketType,
        xor: u32,
        sub: u32,
    ) {
        writer.extend_from_slice(&S::to_data(self.data.len()));
        for i in self.iter() {
            i.write(writer, packet_type, xor, sub);
        }
    }
}

impl<const NO_PADDING: bool> Deref for Bytes<NO_PADDING> {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}
impl<const NO_PADDING: bool> DerefMut for Bytes<NO_PADDING> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}
impl<const NO_PADDING: bool> From<Vec<u8>> for Bytes<NO_PADDING> {
    fn from(value: Vec<u8>) -> Self {
        Self { bytes: value }
    }
}
impl<const NO_PADDING: bool> From<Bytes<NO_PADDING>> for Vec<u8> {
    fn from(value: Bytes<NO_PADDING>) -> Self {
        value.bytes
    }
}
impl<const NO_PADDING: bool> HelperReadWrite for Bytes<NO_PADDING> {
    fn read(
        reader: &mut &[u8],
        _: crate::protocol::PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<Self, crate::protocol::PacketError> {
        let len = read_magic(reader, sub, xor).map_err(|e| PacketError::CompositeFieldError {
            packet_name: "Bytes",
            field_name: "len",
            error: Box::new(e),
        })? as usize;
        if reader.len() < len {
            return Err(PacketError::FieldError {
                packet_name: "Bytes",
                field_name: "value",
                expected: len,
                got: reader.len(),
            });
        }
        let bytes = reader[..len].to_vec();
        *reader = &reader[len..];
        if !NO_PADDING {
            let padding = len.next_multiple_of(4) - len;
            if reader.len() < padding {
                return Err(PacketError::PaddingError {
                    packet_name: "Bytes",
                    field_name: "padding",
                    expected: padding,
                    got: reader.len(),
                });
            }
            *reader = &reader[padding..];
        }
        Ok(Self { bytes })
    }

    fn write(
        &self,
        writer: &mut Vec<u8>,
        packet_type: crate::protocol::PacketType,
        xor: u32,
        sub: u32,
    ) {
        write_magic(self.bytes.len() as _, sub, xor).write(writer, packet_type, xor, sub);
        writer.extend_from_slice(&self.bytes);
        if !NO_PADDING {
            let len = self.bytes.len();
            let padded_len = writer.len() + (len.next_multiple_of(4) - len);
            writer.resize(padded_len, 0);
        }
    }
}
#[cfg(feature = "serde")]
impl<'de, const NO_PADDING: bool> serde::Deserialize<'de> for Bytes<NO_PADDING> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Vec::<u8>::deserialize(deserializer)?.into())
    }
}

impl<const N: usize, const NO_PADDING: bool> Deref for FixedBytes<N, NO_PADDING> {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}
impl<const N: usize, const NO_PADDING: bool> DerefMut for FixedBytes<N, NO_PADDING> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.bytes
    }
}
impl<const N: usize, const NO_PADDING: bool> From<Vec<u8>> for FixedBytes<N, NO_PADDING> {
    fn from(value: Vec<u8>) -> Self {
        Self { bytes: value }
    }
}
impl<const N: usize, const NO_PADDING: bool> From<FixedBytes<N, NO_PADDING>> for Vec<u8> {
    fn from(value: FixedBytes<N, NO_PADDING>) -> Self {
        value.bytes
    }
}
impl<const N: usize, const NO_PADDING: bool> HelperReadWrite for FixedBytes<N, NO_PADDING> {
    fn read(
        reader: &mut &[u8],
        _: crate::protocol::PacketType,
        _: u32,
        _: u32,
    ) -> Result<Self, crate::protocol::PacketError> {
        if reader.len() < N {
            return Err(PacketError::FieldError {
                packet_name: "FixedBytes",
                field_name: "value",
                expected: N,
                got: reader.len(),
            });
        }
        let bytes = reader[..N].to_vec();
        *reader = &reader[N..];
        if !NO_PADDING {
            let padding = N.next_multiple_of(4) - N;
            if reader.len() < padding {
                return Err(PacketError::PaddingError {
                    packet_name: "FixedBytes",
                    field_name: "padding",
                    expected: padding,
                    got: reader.len(),
                });
            }
            *reader = &reader[padding..];
        }
        Ok(Self { bytes })
    }

    fn write(&self, writer: &mut Vec<u8>, _: crate::protocol::PacketType, _: u32, _: u32) {
        let size = self.bytes.len().min(N);
        writer.extend_from_slice(&self.bytes[..size]);
        let len = writer.len() + (N - size);
        writer.resize(len, 0);
        if !NO_PADDING {
            let padded_len = writer.len() + (N.next_multiple_of(4) - N);
            writer.resize(padded_len, 0);
        }
    }
}
#[cfg(feature = "serde")]
impl<'de, const N: usize, const NO_PADDING: bool> serde::Deserialize<'de>
    for FixedBytes<N, NO_PADDING>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(Vec::<u8>::deserialize(deserializer)?.into())
    }
}

impl<const N: usize> Default for FixedString<N> {
    fn default() -> Self {
        Self {
            string: {
                if cfg!(test) {
                    let mut str = String::with_capacity(N);
                    for _ in 0..N {
                        str.push('\0');
                    }
                    str
                } else {
                    Default::default()
                }
            },
        }
    }
}

impl<const N: usize> Default for FixedAsciiString<N> {
    fn default() -> Self {
        Self {
            string: {
                if cfg!(test) {
                    let mut str = String::with_capacity(N);
                    for _ in 0..N {
                        str.push('\0');
                    }
                    str.into()
                } else {
                    Default::default()
                }
            },
        }
    }
}

impl<const N: usize, T: Default> Default for FixedVec<N, T> {
    fn default() -> Self {
        Self {
            data: {
                if cfg!(test) {
                    let mut vec = Vec::with_capacity(N);
                    for _ in 0..N {
                        vec.push(Default::default());
                    }
                    vec
                } else {
                    Default::default()
                }
            },
        }
    }
}

impl<const N: usize, const NO_PADDING: bool> Default for FixedBytes<N, NO_PADDING> {
    fn default() -> Self {
        Self {
            bytes: {
                if cfg!(test) {
                    vec![0; N]
                } else {
                    Default::default()
                }
            },
        }
    }
}
