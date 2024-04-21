use chrono::TimeZone;
use chrono::Utc;
use pso2packetlib::{
    ppac::{Direction, OutputType, PPACReader, PacketData},
    protocol::Packet,
};
use std::{env, fs::File, io::Write};

fn main() {
    let mut args = env::args();
    args.next();
    let filename = args.next().unwrap();

    let mut text = filename.clone();
    let out_dir = filename.replace(".", "");
    let _ = std::fs::create_dir(&out_dir);
    let mut objects = out_dir.clone();
    objects.push_str("/objects");
    let _ = std::fs::create_dir(&objects);
    let mut npcs = out_dir.clone();
    npcs.push_str("/npcs");
    let _ = std::fs::create_dir(&npcs);
    let mut dmg_text = text.clone();
    dmg_text.push_str("dmg.txt");
    text.push_str(".txt");
    let mut ppac = PPACReader::open(File::open(&filename).unwrap()).unwrap();
    ppac.set_out_type(OutputType::Both);
    let mut out_file = File::create(&text).unwrap();
    while let Ok(Some(PacketData {
        time,
        direction,
        packet,
        data,
        parse_error,
        ..
    })) = ppac.read()
    {
        let packet = match packet {
            Some(x) => x,
            None => pso2packetlib::protocol::Packet::Raw(data.unwrap()),
        };
        let time = time.as_nanos();
        let dir = match direction {
            Direction::ToServer => "(C -> S)",
            Direction::ToClient => "(S -> C)",
        };
        let timestamp = Utc
            .timestamp_opt((time / 1000000000) as i64, (time % 1000000000) as u32)
            .unwrap();
        match packet {
            Packet::None => break,
            Packet::Unknown((header, data)) => {
                writeln!(
                    &mut out_file,
                    "{dir} {} {{ id: {:X}, subid: {:X}, flags: {:?} }}",
                    timestamp.format("%H-%M-%S"),
                    header.id,
                    header.subid,
                    header.flag
                )
                .unwrap();
                let out_name = format!("{out_dir}/{}_{:X}_{:X}", time, header.id, header.subid);
                File::create(out_name).unwrap().write_all(&data).unwrap();
            }
            Packet::Raw(data) => {
                let header = u32::from_be_bytes(data[4..8].try_into().unwrap());
                writeln!(
                    &mut out_file,
                    "{dir} {} RAW {{ header: {:X} }}: {}",
                    timestamp.format("%H-%M-%S"),
                    header,
                    parse_error.unwrap(),
                )
                .unwrap();
                let out_name = format!("{out_dir}/{}_{:X}", time, header);
                File::create(out_name).unwrap().write_all(&data).unwrap();
            }
            Packet::LoadItemAttributes(p) => {
                writeln!(
                    &mut out_file,
                    "{dir} {} LoadItemAttributes(id: {})",
                    timestamp.format("%H-%M-%S"),
                    p.id
                )
                .unwrap();
                let out_name = format!("{out_dir}/item_attr_{}.bin", time);
                File::create(out_name).unwrap().write_all(&p.data).unwrap();
            }
            Packet::ClientPing(_) => {}
            Packet::ClientPong(_) => {}
            x => {
                writeln!(
                    &mut out_file,
                    "{dir} {} {x:?}",
                    timestamp.format("%H-%M-%S")
                )
                .unwrap();
            }
        }
    }
}
