//! Ascii only string.
use crate::protocol::{read_magic, write_magic};
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::{Read, Seek, SeekFrom, Write};

/// Helper read/write trait for strings.
pub trait StringRW: Sized + Default + std::ops::Deref<Target = str> {
    /// Reads a fixed length string from a reader.
    fn read_fixed(reader: &mut impl Read, len: u64) -> std::io::Result<Self>;
    /// Writes a fixed length string to a writer.
    fn write_fixed(&self, len: usize) -> Vec<u8>;
    /// Returns number of bytes needed to pad a string to align it to a 4 byte boundary.
    fn get_padding(len: u64) -> u64;
    /// Reads a variable length string from a reader.
    fn read_variable(reader: &mut (impl Read + Seek), sub: u32, xor: u32) -> std::io::Result<Self> {
        let magic_len = read_magic(reader, sub, xor)? as u64;
        if magic_len == 0 {
            return Ok(Default::default());
        }
        let len = magic_len;
        let padding = Self::get_padding(len);
        let string = Self::read_fixed(reader, len)?;
        reader.seek(SeekFrom::Current(padding as i64))?;
        Ok(string)
    }
    /// Writes a variable length string to a writer.
    fn write_variable(&self, sub: u32, xor: u32) -> Vec<u8> {
        let mut buf = vec![];
        if self.is_empty() {
            buf.write_u32::<LittleEndian>(write_magic(0, sub, xor))
                .unwrap();
            return buf;
        }
        #[cfg(not(test))]
        let len = self.chars().count() + 1;
        #[cfg(test)]
        let len = self.chars().count();
        let padding = Self::get_padding(len as u64) as usize;
        buf.write_u32::<LittleEndian>(write_magic(len as u32, sub, xor))
            .unwrap();
        buf.write_all(&self.write_fixed(len)).unwrap();
        buf.write_all(&vec![0; padding]).unwrap();
        buf
    }
}

impl StringRW for String {
    fn read_fixed(reader: &mut impl Read, len: u64) -> std::io::Result<Self> {
        let len = len * 2;
        let mut buf = vec![];
        reader.take(len).read_to_end(&mut buf)?;
        let buf = &buf;
        let mut words = vec![];
        for word in buf.chunks(2) {
            words.push(u16::from_le_bytes(word.try_into().unwrap()))
        }
        let mut string = String::from_utf16_lossy(&words);
        #[cfg(not(test))]
        if let Some(x) = string.find('\0') {
            string.replace_range(x.., "");
        }
        Ok(string)
    }

    fn write_fixed(&self, len: usize) -> Vec<u8> {
        let mut buf = vec![];
        let string = self
            .chars()
            .take(len - 1)
            .chain("\0".chars().cycle())
            .take(len)
            .collect::<String>();
        for word in string.encode_utf16() {
            buf.extend(word.to_le_bytes())
        }
        buf
    }

    fn get_padding(len: u64) -> u64 {
        2 * (len & 1)
    }
}

// newtype for ascii strings
/// Ascii only string.
/// # Usage
/// ```
/// # use pso2packetlib::AsciiString;
/// # fn main() {
/// let string: AsciiString = "Hello❤️".into();
/// assert_eq!(string.len(), 5);
/// println!("{string}"); //prints "Hello"
/// let string: String = string.into();
/// assert_eq!(string, String::from("Hello"));
/// # }
/// ```
///
#[derive(Default, Clone, PartialEq, PartialOrd, Hash)]
#[repr(transparent)]
pub struct AsciiString(String);

impl std::fmt::Display for AsciiString {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl std::fmt::Debug for AsciiString {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl From<AsciiString> for String {
    fn from(value: AsciiString) -> Self {
        value.0
    }
}

impl From<String> for AsciiString {
    /// Convert from [`String`] into [`AsciiString`].
    /// All non-ascii characters are discarded.
    fn from(value: String) -> Self {
        let string = value.chars().filter(char::is_ascii).collect();
        Self(string)
    }
}

impl From<&str> for AsciiString {
    /// Convert from [`str`] into [`AsciiString`].
    /// All non-ascii characters are discarded.
    fn from(value: &str) -> Self {
        let string = value.chars().filter(char::is_ascii).collect();
        Self(string)
    }
}

impl PartialEq<&str> for AsciiString {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

impl PartialEq<String> for AsciiString {
    fn eq(&self, other: &String) -> bool {
        self.0 == *other
    }
}

impl std::ops::Deref for AsciiString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for AsciiString {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(ser)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for AsciiString {
    fn deserialize<D>(deser: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deser)?;
        if string.chars().any(|c| !c.is_ascii()) {
            return Err(serde::de::Error::custom(
                "String contains non-ascii charactes",
            ));
        }

        Ok(Self(string))
    }
}

impl StringRW for AsciiString {
    fn read_fixed(reader: &mut impl Read, len: u64) -> std::io::Result<Self> {
        let mut buf = vec![];
        reader.take(len).read_to_end(&mut buf)?;
        let mut string = String::from_utf8_lossy(&buf).to_string();
        #[cfg(not(test))]
        if let Some(x) = string.find('\0') {
            string.replace_range(x.., "");
        }
        #[cfg(not(test))]
        let string = string.chars().filter(char::is_ascii).collect();
        Ok(Self(string))
    }

    #[cfg(not(test))]
    fn write_fixed(&self, len: usize) -> Vec<u8> {
        self.chars()
            .take(len - 1)
            .map(|c| c as u8)
            .chain([0].into_iter().cycle())
            .take(len)
            .collect()
    }
    // sega pls clear your buffers
    #[cfg(test)]
    fn write_fixed(&self, len: usize) -> Vec<u8> {
        self.chars()
            .map(|c| c as u8)
            .chain([0].into_iter().cycle())
            .take(len)
            .collect()
    }

    #[inline]
    fn get_padding(len: u64) -> u64 {
        3 - ((len - 1) & 3)
    }
}

impl AsciiString {
    /// Create an [`AsciiString`] without checking for non-ascii characters.
    ///
    /// # Safety
    /// The caller must ensure that the contents of the string are valid ascii characters.
    pub fn from_string_unchecked(other: String) -> Self {
        Self(other)
    }

    pub fn as_str(&self) -> &str {
        self
    }
}
