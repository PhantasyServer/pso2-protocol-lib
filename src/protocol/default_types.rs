use crate::{asciistring::StringRW, AsciiString};

use super::{read_magic, write_magic, HelperReadWrite, PacketError};
use half::f16;
use std::{net::Ipv4Addr, time::Duration};

macro_rules! helper_int {
    ($name:ty; $read:ident, $write:ident) => {
        impl HelperReadWrite for $name {
            fn read(
                reader: &mut &[u8],
                _: super::PacketType,
                _: u32,
                _: u32,
            ) -> Result<Self, super::PacketError> {
                const BUF_SIZE: usize = core::mem::size_of::<$name>();
                let mut buf = [0; BUF_SIZE];
                let read_len = reader.len();
                if read_len < BUF_SIZE {
                    return Err(PacketError::FieldError {
                        packet_name: stringify!($name),
                        field_name: "value",
                        expected: BUF_SIZE,
                        got: read_len,
                    });
                }
                buf.copy_from_slice(&reader[..BUF_SIZE]);
                *reader = &reader[BUF_SIZE..];
                Ok(<$name>::$read(buf))
            }

            fn write(
                &self,
                writer: &mut Vec<u8>,
                _: super::PacketType,
                _: u32,
                _: u32,
            ) {
                writer.extend_from_slice(&self.$write());
            }
        }
    };
    ($name:ty, $($name_r:ty),+;$read:ident, $write:ident) => {
        helper_int!($name; $read, $write);
        helper_int!($($name_r),+; $read, $write);
    };
}

helper_int!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f16, f32, f64; from_le_bytes, to_le_bytes);
helper_int!(Ipv4Addr; from, octets);

impl<T: HelperReadWrite> HelperReadWrite for Box<T> {
    fn read(
        reader: &mut &[u8],
        packet_type: super::PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<Self, PacketError> {
        T::read(reader, packet_type, xor, sub).map(Box::new)
    }

    fn write(&self, writer: &mut Vec<u8>, packet_type: super::PacketType, xor: u32, sub: u32) {
        self.as_ref().write(writer, packet_type, xor, sub)
    }
}

impl<T: HelperReadWrite, const N: usize> HelperReadWrite for [T; N] {
    fn read(
        reader: &mut &[u8],
        packet_type: super::PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<Self, PacketError> {
        let mut arr = vec![];
        arr.reserve_exact(N);

        for _ in 0..N {
            arr.push(T::read(reader, packet_type, xor, sub).map_err(|e| {
                PacketError::CompositeFieldError {
                    packet_name: "array",
                    field_name: "value",
                    error: e.into(),
                }
            })?);
        }

        if let Ok(arr) = arr.try_into() {
            Ok(arr)
        } else {
            unreachable!()
        }
    }

    fn write(&self, writer: &mut Vec<u8>, packet_type: super::PacketType, xor: u32, sub: u32) {
        for i in self {
            i.write(writer, packet_type, xor, sub);
        }
    }
}

impl HelperReadWrite for Duration {
    fn read(
        reader: &mut &[u8],
        packet_type: crate::protocol::PacketType,
        _: u32,
        _: u32,
    ) -> Result<Self, crate::protocol::PacketError> {
        Ok(Duration::from_secs(
            u32::read(reader, packet_type, 0, 0).map_err(|e| PacketError::CompositeFieldError {
                packet_name: "WinTime",
                field_name: "time",
                error: e.into(),
            })? as u64,
        ))
    }

    fn write(
        &self,
        writer: &mut Vec<u8>,
        packet_type: crate::protocol::PacketType,
        _: u32,
        _: u32,
    ) {
        (self.as_secs() as u32).write(writer, packet_type, 0, 0);
    }
}

impl HelperReadWrite for String {
    fn read(
        reader: &mut &[u8],
        _: crate::protocol::PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<Self, crate::protocol::PacketError> {
        <String as StringRW>::read_variable(reader, sub, xor)
    }

    fn write(&self, writer: &mut Vec<u8>, _: crate::protocol::PacketType, xor: u32, sub: u32) {
        writer.extend_from_slice(&self.write_variable(sub, xor));
    }
}

impl HelperReadWrite for AsciiString {
    fn read(
        reader: &mut &[u8],
        _: crate::protocol::PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<Self, crate::protocol::PacketError> {
        <AsciiString as StringRW>::read_variable(reader, sub, xor)
    }

    fn write(&self, writer: &mut Vec<u8>, _: crate::protocol::PacketType, xor: u32, sub: u32) {
        writer.extend_from_slice(&self.write_variable(sub, xor));
    }
}

impl<T: HelperReadWrite> HelperReadWrite for Vec<T> {
    fn read(
        reader: &mut &[u8],
        packet_type: crate::protocol::PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<Self, crate::protocol::PacketError> {
        let len = read_magic(reader, sub, xor).map_err(|e| PacketError::CompositeFieldError {
            packet_name: "Vec",
            field_name: "len",
            error: Box::new(e),
        })?;
        let mut data = vec![];
        data.reserve_exact(len as usize);

        let seek1 = reader.as_ptr() as usize;
        for _ in 0..len {
            data.push(T::read(reader, packet_type, xor, sub).map_err(|e| {
                PacketError::CompositeFieldError {
                    packet_name: "Vec",
                    field_name: "value",
                    error: e.into(),
                }
            })?);
        }
        let seek2 = reader.as_ptr() as usize;
        let len = (seek2 - seek1) as usize;
        let padding = len.next_multiple_of(4) - len;
        if reader.len() < padding {
            return Err(PacketError::PaddingError {
                packet_name: "Vec",
                field_name: "padding",
                expected: padding,
                got: reader.len(),
            });
        }
        *reader = &reader[padding..];
        Ok(data)
    }

    fn write(
        &self,
        writer: &mut Vec<u8>,
        packet_type: crate::protocol::PacketType,
        xor: u32,
        sub: u32,
    ) {
        (write_magic(self.len() as u32, sub, xor)).write(writer, packet_type, xor, sub);
        let len1 = writer.len();
        for i in self.iter() {
            i.write(writer, packet_type, xor, sub);
        }
        let len2 = writer.len();
        let wrote_len = len2 - len1;
        let padded_len = len2 + (wrote_len.next_multiple_of(4) - wrote_len);
        writer.resize(padded_len, 0);
    }
}
