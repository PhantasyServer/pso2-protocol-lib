use std::{
    env,
    error::Error,
    fs::{create_dir_all, read_dir, File},
    io::BufReader,
    path::{Path, PathBuf},
};

use pso2packetlib::{
    ppac::{PPACReader, PPACWriter},
    protocol::Packet,
};

fn main() {
    let mut args = env::args();
    args.next();
    let dir = args.next().expect("Enter directory");
    let to_enc: bool = args.next().and_then(|s| s.parse().ok()).unwrap_or(true);
    traverse_dir(&dir, &dir, to_enc).unwrap();
}

fn traverse_dir<T: AsRef<Path>, P: AsRef<Path>>(
    path: T,
    original_path: P,
    to_enc: bool,
) -> Result<(), Box<dyn Error>> {
    if path.as_ref().is_file() {
        to_repack(&path, &original_path.as_ref().parent().unwrap(), to_enc)?;
        return Ok(());
    }
    for entry in read_dir(path)? {
        let entry = entry?.path();
        if entry.is_dir() {
            traverse_dir(entry, original_path.as_ref(), to_enc)?;
        } else if entry.is_file() {
            to_repack(entry, &original_path, to_enc)?;
        }
    }
    Ok(())
}

fn to_repack<T: AsRef<Path>, P: AsRef<Path>>(
    path: T,
    original_path: P,
    to_enc: bool,
) -> Result<(), Box<dyn Error>> {
    let mut out_path = PathBuf::from("out");
    out_path.push(path.as_ref().strip_prefix(&original_path)?);
    let _ = create_dir_all(&out_path.parent().unwrap());
    let Ok(reader) = File::open(&path) else {
        return Ok(());
    };
    let reader = BufReader::new(reader);
    let Ok(mut reader) = PPACReader::open(reader) else {
        return Ok(());
    };
    println!("{out_path:?}");
    reader.set_out_type(pso2packetlib::ppac::OutputType::Raw);
    let mut writer = PPACWriter::new(File::create(&out_path)?, reader.get_protocol_type(), to_enc)?;
    while let Some(packet) = reader.read()? {
        let raw_packet = Packet::Raw(packet.data.unwrap());
        writer.write_packet(packet.time, packet.direction, &raw_packet)?;
    }
    Ok(())
}
