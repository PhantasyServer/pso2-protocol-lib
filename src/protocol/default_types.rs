use crate::{asciistring::StringRW, AsciiString};

use super::{read_magic, write_magic, HelperReadWrite, PacketError};
use half::f16;
use std::{net::Ipv4Addr, time::Duration};

macro_rules! helper_int {
    ($name:ty; $read:ident, $write:ident) => {
        impl HelperReadWrite for $name {
            fn read(
                reader: &mut (impl std::io::Read + std::io::Seek),
                _: super::PacketType,
                _: u32,
                _: u32,
            ) -> Result<Self, super::PacketError> {
                let mut buf = [0; std::mem::size_of::<$name>()];
                reader
                    .read_exact(&mut buf)
                    .map_err(|e| PacketError::FieldError {
                        packet_name: stringify!($name),
                        field_name: "value",
                        error: e,
                    })?;
                Ok(<$name>::$read(buf))
            }

            fn write(
                &self,
                writer: &mut impl std::io::Write,
                _: super::PacketType,
                _: u32,
                _: u32,
            ) -> Result<(), super::PacketError> {
                let buf = self.$write();
                writer.write_all(&buf).map_err(|e| PacketError::FieldError {
                    packet_name: stringify!($name),
                    field_name: "value",
                    error: e,
                })
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
        reader: &mut (impl std::io::Read + std::io::Seek),
        packet_type: super::PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<Self, PacketError> {
        T::read(reader, packet_type, xor, sub).map(Box::new)
    }

    fn write(
        &self,
        writer: &mut impl std::io::Write,
        packet_type: super::PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<(), PacketError> {
        self.as_ref().write(writer, packet_type, xor, sub)
    }
}

impl<T: HelperReadWrite, const N: usize> HelperReadWrite for [T; N] {
    fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
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

    fn write(
        &self,
        writer: &mut impl std::io::Write,
        packet_type: super::PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<(), PacketError> {
        for i in self {
            i.write(writer, packet_type, xor, sub).map_err(|e| {
                PacketError::CompositeFieldError {
                    packet_name: "array",
                    field_name: "value",
                    error: e.into(),
                }
            })?;
        }
        Ok(())
    }
}

impl HelperReadWrite for Duration {
    fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
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
        writer: &mut impl std::io::Write,
        packet_type: crate::protocol::PacketType,
        _: u32,
        _: u32,
    ) -> Result<(), crate::protocol::PacketError> {
        (self.as_secs() as u32)
            .write(writer, packet_type, 0, 0)
            .map_err(|e| PacketError::CompositeFieldError {
                packet_name: "WinTime",
                field_name: "time",
                error: e.into(),
            })
    }
}

impl HelperReadWrite for String {
    fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        _: crate::protocol::PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<Self, crate::protocol::PacketError> {
        <String as StringRW>::read_variable(reader, sub, xor).map_err(|e| PacketError::FieldError {
            packet_name: "String",
            field_name: "str",
            error: e,
        })
    }

    fn write(
        &self,
        writer: &mut impl std::io::Write,
        _: crate::protocol::PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<(), crate::protocol::PacketError> {
        writer
            .write_all(&self.write_variable(sub, xor))
            .map_err(|e| PacketError::FieldError {
                packet_name: "String",
                field_name: "str",
                error: e,
            })
    }
}

impl HelperReadWrite for AsciiString {
    fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        _: crate::protocol::PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<Self, crate::protocol::PacketError> {
        <AsciiString as StringRW>::read_variable(reader, sub, xor).map_err(|e| {
            PacketError::FieldError {
                packet_name: "AsciiString",
                field_name: "str",
                error: e,
            }
        })
    }

    fn write(
        &self,
        writer: &mut impl std::io::Write,
        _: crate::protocol::PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<(), crate::protocol::PacketError> {
        writer
            .write_all(&self.write_variable(sub, xor))
            .map_err(|e| PacketError::FieldError {
                packet_name: "AsciiString",
                field_name: "str",
                error: e,
            })
    }
}

impl<T: HelperReadWrite> HelperReadWrite for Vec<T> {
    fn read(
        reader: &mut (impl std::io::Read + std::io::Seek),
        packet_type: crate::protocol::PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<Self, crate::protocol::PacketError> {
        let len = read_magic(reader, sub, xor).map_err(|e| PacketError::FieldError {
            packet_name: "Vec",
            field_name: "len",
            error: e,
        })?;
        let mut data = vec![];
        data.reserve_exact(len as usize);

        let seek1 = reader
            .stream_position()
            .map_err(|e| PacketError::PaddingError {
                packet_name: "Vec",
                field_name: "pre_read",
                error: e,
            })?;
        for _ in 0..len {
            data.push(T::read(reader, packet_type, xor, sub).map_err(|e| {
                PacketError::CompositeFieldError {
                    packet_name: "Vec",
                    field_name: "value",
                    error: e.into(),
                }
            })?);
        }
        let seek2 = reader
            .stream_position()
            .map_err(|e| PacketError::PaddingError {
                packet_name: "Vec",
                field_name: "post_read",
                error: e,
            })?;
        let len = (seek2 - seek1) as usize;
        reader
            .seek(std::io::SeekFrom::Current(
                (len.next_multiple_of(4) - len) as i64,
            ))
            .map_err(|e| PacketError::PaddingError {
                packet_name: "Vec",
                field_name: "padding",
                error: e,
            })?;
        Ok(data)
    }

    fn write(
        &self,
        writer: &mut impl std::io::Write,
        packet_type: crate::protocol::PacketType,
        xor: u32,
        sub: u32,
    ) -> Result<(), crate::protocol::PacketError> {
        (write_magic(self.len() as u32, sub, xor))
            .write(writer, packet_type, xor, sub)
            .map_err(|e| PacketError::CompositeFieldError {
                packet_name: "Vec",
                field_name: "len",
                error: e.into(),
            })?;
        let mut buf = vec![];
        for i in self.iter() {
            i.write(&mut buf, packet_type, xor, sub).map_err(|e| {
                PacketError::CompositeFieldError {
                    packet_name: "Vec",
                    field_name: "value",
                    error: e.into(),
                }
            })?;
        }
        let len = buf.len();
        writer
            .write_all(&buf)
            .map_err(|e| PacketError::FieldError {
                packet_name: "Vec",
                field_name: "value",
                error: e,
            })?;
        writer
            .write_all(&vec![0; len.next_multiple_of(4) - len])
            .map_err(|e| PacketError::PaddingError {
                packet_name: "Vec",
                field_name: "padding",
                error: e,
            })?;

        Ok(())
    }
}
