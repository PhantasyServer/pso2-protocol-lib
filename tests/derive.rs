use pso2packetlib::{
    fixed_types::{FixedString, VecUSize},
    protocol::{PacketType, ProtocolRW},
};

// we use pso2packetlib_impl instead of pso2packetlib to bypass `derive` feature requirement
#[derive(pso2packetlib_impl::ProtocolRW)]
enum Packet {
    #[Empty]
    None,
    #[Id(1, 1)]
    Numbers(Numbers),
    #[Id(1, 2)]
    Variables(Variables),
    #[Id(1, 3)]
    Misc(Misc),
    #[Id(1, 4)]
    Attributes(Attributes),
    #[Id(1, 5)]
    Helpers(Helpers),
    #[Unknown]
    Unknown((pso2packetlib::protocol::PacketHeader, Vec<u8>)),
}

#[derive(Debug, PartialEq, pso2packetlib_impl::PacketRW)]
#[Id(1, 1)]
struct Numbers {
    uint8: u8,
    int8: i8,
    uint16: u16,
    int16: i16,
    uint32: u32,
    int32: i32,
    uint64: u64,
    int64: i64,
    uint128: u128,
    int128: i128,
    float16: half::f16,
    float32: f32,
    float64: f64,
}

#[derive(Debug, PartialEq, pso2packetlib_impl::PacketRW)]
#[Id(1, 2)]
#[Flags(pso2packetlib::protocol::Flags::PACKED)]
#[Magic(0x10, 0x10)]
struct Variables {
    vec: pso2packetlib::fixed_types::Bytes,
    fixed_vec: pso2packetlib::fixed_types::FixedBytes<10>,
    str: String,
    fixed_str: FixedString<5>,
    astr: pso2packetlib::asciistring::AsciiString,
    fixed_astr: pso2packetlib::fixed_types::FixedAsciiString<10>,

    var_1: VecUSize<u16, u8>,
    var_2: VecUSize<u32, u8>,
}

#[derive(Debug, PartialEq, pso2packetlib_impl::PacketRW)]
#[Id(1, 3)]
struct Misc {
    ip: std::net::Ipv4Addr,
    time: std::time::Duration,
    pso2_time: pso2packetlib::fixed_types::WinTime,
}

#[derive(Debug, PartialEq, pso2packetlib_impl::PacketRW)]
#[Id(1, 4)]
struct Attributes {
    #[Seek(2)]
    #[SeekAfter(2)]
    a: u8,
    #[Const_u16(5)]
    b: u8,
    #[OnlyOn(pso2packetlib::protocol::PacketType::JP)]
    c: u8,
    #[NotOn(pso2packetlib::protocol::PacketType::JP)]
    d: u8,
}

#[derive(Debug, PartialEq, pso2packetlib_impl::PacketRW)]
#[Id(1, 5)]
struct Helpers {
    flags: HelperFlags,
    bitflags: HelperBitFlags,
    e: Enum,
}

#[derive(Debug, PartialEq, pso2packetlib_impl::HelperRW)]
#[Flags(u8)]
struct HelperFlags {
    #[Skip]
    a: bool,
    b: bool,
}

#[derive(Debug, PartialEq, Clone, Copy, pso2packetlib_impl::HelperRW)]
#[repr(u8)]
enum Enum {
    #[Read_default]
    A,
    B,
}

bitflags::bitflags! {
    #[derive(pso2packetlib_impl::HelperRW, PartialEq, Debug)]
    #[BitFlags(u16)]
    struct HelperBitFlags: u16 {
        const A = 1 << 0;
        const B = 1 << 1;
        const C = 1 << 2;
    }
}

impl pso2packetlib::protocol::PacketEncryption for Packet {
    fn is_enc_data(&self) -> bool {
        false
    }

    fn as_enc_data(&self) -> Option<&[u8]> {
        None
    }

    fn mut_enc_data(&mut self) -> Option<&mut Vec<u8>> {
        None
    }
}

#[test]
fn test_numbers() {
    let _ = Packet::None;
    let mut data = vec![
        0, 0, 0, 0, // len
        1, 1, 0, 0, // id
        1, // uint8,
        2, // int8,
        3, 0, // uint16,
        4, 0, // int16,
        5, 0, 0, 0, // uint32,
        6, 0, 0, 0, // int32,
        7, 0, 0, 0, 0, 0, 0, 0, // uint64,
        8, 0, 0, 0, 0, 0, 0, 0, // int64,
        9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // uint128,
        10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // int128,
        0x00, 0x3C, // f16,
        0x00, 0x00, 0x80, 0x3F, // f32,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x3F, // f64,
    ];
    let len = data.len() as u32;
    data[..4].copy_from_slice(&len.to_le_bytes());
    let packet = Packet::read(&data, PacketType::Classic)
        .expect("Failed to read the packet")
        .pop()
        .expect("Failed to extract the packet");
    let Packet::Numbers(packet) = packet else {
        panic!("Got incorrect packet")
    };
    let expected_packet = Numbers {
        uint8: 1,
        int8: 2,
        uint16: 3,
        int16: 4,
        uint32: 5,
        int32: 6,
        uint64: 7,
        int64: 8,
        uint128: 9,
        int128: 10,
        float16: half::f16::from_f32(1.0),
        float32: 1.0,
        float64: 1.0,
    };
    assert_eq!(packet, expected_packet);
    let data2 = Packet::Numbers(packet).write(PacketType::Classic);
    assert_eq!(data, data2);
}

#[test]
fn test_variable() {
    let mut data = vec![
        0, 0, 0, 0, // len
        1, 2, 4, 0, // id
        3, 0, 0, 0, // len
        1, 2, 3, // vec
        0, // padding
        4, 5, 6, 7, 8, 9, 10, 11, 12, 13, // fixed_vec
        0, 0, // padding
        4, 0, 0, 0, // len
        0x41, 0x00, 0x42, 0x00, 0x43, 0x00, 0x00, 0x00, // str
        0x41, 0x00, 0x42, 0x00, 0x43, 0x00, 0x00, 0x00, 0x00, 0x00, // fixed_str
        4, 0, 0, 0, // len
        0x41, 0x42, 0x43, 0x00, // astr
        0x41, 0x42, 0x43, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // fixed_astr
        1, 0,  // len
        14, // var_1
        2, 0, 0, 0, // len
        15, 16, // var_2
        0, 0, 0, // padding
    ];
    let len = data.len() as u32;
    data[..4].copy_from_slice(&len.to_le_bytes());
    let packet = Packet::read(&data, PacketType::Classic)
        .expect("Failed to read the packet")
        .pop()
        .expect("Failed to extract the packet");
    let Packet::Variables(packet) = packet else {
        panic!("Got incorrect packet")
    };
    let expected_packet = Variables {
        vec: vec![1, 2, 3].into(),
        fixed_vec: vec![4, 5, 6, 7, 8, 9, 10, 11, 12, 13].into(),
        str: String::from("ABC"),
        fixed_str: String::from("ABC").into(),
        astr: String::from("ABC").into(),
        fixed_astr: String::from("ABC").into(),
        var_1: vec![14].into(),
        var_2: vec![15, 16].into(),
    };
    assert_eq!(packet, expected_packet);
    let data2 = Packet::Variables(packet).write(PacketType::Classic);
    assert_eq!(data, data2);
}

#[test]
fn test_misc() {
    let mut data = vec![
        0, 0, 0, 0, // len
        1, 3, 0, 0, // id
        127, 0, 0, 1, // ip
        0, 0, 0, 0, // time,
        0x64, 0x88, 0x64, 0xE9, 0x95, 0x02, 0, 0, // pso2time
    ];
    let len = data.len() as u32;
    data[..4].copy_from_slice(&len.to_le_bytes());
    let packet = Packet::read(&data, PacketType::Classic)
        .expect("Failed to read the packet")
        .pop()
        .expect("Failed to extract the packet");
    let Packet::Misc(packet) = packet else {
        panic!("Got incorrect packet")
    };
    let expected_packet = Misc {
        ip: std::net::Ipv4Addr::LOCALHOST,
        time: std::time::Duration::from_secs(0),
        pso2_time: std::time::Duration::from_secs(0).into(),
    };
    assert_eq!(packet, expected_packet);
    let data2 = Packet::Misc(packet).write(PacketType::Classic);
    assert_eq!(data, data2);
}

#[test]
fn test_attrs() {
    let mut data = vec![
        0, 0, 0, 0, // len
        1, 4, 0, 0, // id
        0, 0, // seek
        1, //a
        0, 0, //seekafter
        5, 0, //const
        2, //b
        3, // c or d
        0, 0, 0, //padding
    ];
    let len = data.len() as u32;
    data[..4].copy_from_slice(&len.to_le_bytes());
    let packet = Packet::read(&data, PacketType::JP)
        .expect("Failed to read the packet")
        .pop()
        .expect("Failed to extract the packet");
    let Packet::Attributes(packet) = packet else {
        panic!("Got incorrect packet")
    };
    let expected_packet = Attributes {
        a: 1,
        b: 2,
        c: 3,
        d: 0,
    };
    assert_eq!(packet, expected_packet);
    let packet = Packet::read(&data, PacketType::NA)
        .expect("Failed to read the packet")
        .pop()
        .expect("Failed to extract the packet");
    let Packet::Attributes(packet) = packet else {
        panic!("Got incorrect packet")
    };
    let expected_packet = Attributes {
        a: 1,
        b: 2,
        c: 0,
        d: 3,
    };
    assert_eq!(packet, expected_packet);
    let data2 = Packet::Attributes(packet).write(PacketType::Classic);
    assert_eq!(data, data2);
}

#[test]
fn test_helpers() {
    let mut data = vec![
        0, 0, 0, 0, // len
        1, 5, 0, 0, // id
        6, // flags,
        5, 0, // bitflags,
        1, // enum,
    ];
    let len = data.len() as u32;
    data[..4].copy_from_slice(&len.to_le_bytes());
    let packet = Packet::read(&data, PacketType::Classic)
        .expect("Failed to read the packet")
        .pop()
        .expect("Failed to extract the packet");
    let Packet::Helpers(packet) = packet else {
        panic!("Got incorrect packet")
    };
    let expected_packet = Helpers {
        flags: HelperFlags { a: true, b: true },
        bitflags: HelperBitFlags::A | HelperBitFlags::C,
        e: Enum::B,
    };
    assert_eq!(packet, expected_packet);
    let data2 = Packet::Helpers(packet).write(PacketType::Classic);
    assert_eq!(data, data2);
}
