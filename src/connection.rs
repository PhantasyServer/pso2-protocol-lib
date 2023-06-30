use std::io::{Read, Write};

use crate::{
    encryption::{reencrypt, Encryption},
    protocol::{login::EncryptionRequestPacket, Packet},
};

/// Represents a connection between a client and a server
#[derive(Debug)]
pub struct Connection {
    stream: std::net::TcpStream,
    encryption: Encryption,
    read_buffer: Vec<u8>,
    read_packets: Vec<Packet>,
    write_buffer: Vec<u8>,
    packet_length: usize,
    in_keyfile: Option<std::path::PathBuf>,
    out_keyfile: Option<std::path::PathBuf>,
    is_ngs: bool,
}
impl Connection {
    /// Create a new connection.
    /// `in_keyfile` is the path to the keyfile to decrypt client's key,
    /// `out_keyfile` is the path to the keyfile to encrypt client's key
    /// (only useful to proxies).
    pub fn new(
        stream: std::net::TcpStream,
        is_ngs: bool,
        in_keyfile: Option<std::path::PathBuf>,
        out_keyfile: Option<std::path::PathBuf>,
    ) -> Self {
        Self {
            stream,
            encryption: Encryption::None,
            read_buffer: Vec::new(),
            read_packets: Vec::new(),
            write_buffer: Vec::new(),
            packet_length: 0,
            in_keyfile,
            out_keyfile,
            is_ngs,
        }
    }
    /// Read a packet from stream.
    pub fn read_packet(&mut self) -> std::io::Result<Packet> {
        if !self.read_packets.is_empty() {
            return Ok(self.get_one_packet());
        }
        if !self.read_buffer.is_empty() {
            if let Some(packet) = self.get_packet()? {
                return Ok(packet);
            }
        }
        let mut buf = [0u8; 4096];
        loop {
            let read_bytes = check_disconnect(self.stream.read(&mut buf))?;
            if self.encryption.is_rc4() {
                let mut decrypted_stream = self.encryption.decrypt(&buf[..read_bytes])?;
                self.read_buffer.append(&mut decrypted_stream);
            } else {
                self.read_buffer.extend_from_slice(&buf[..read_bytes]);
            }
            if let Some(packet) = self.get_packet()? {
                return Ok(packet);
            }
        }
    }

    fn get_packet(&mut self) -> std::io::Result<Option<Packet>> {
        let mut output_data = vec![0u8; 0];
        if self.packet_length == 0 {
            self.packet_length = get_length(&self.read_buffer);
        }
        if self.read_buffer.len() >= self.packet_length {
            output_data.extend(self.read_buffer.drain(..self.packet_length));
            self.packet_length = 0;
            let output_data = if self.encryption.is_rc4() {
                output_data
            } else {
                self.encryption.decrypt(&output_data)?
            };
            let mut packets = Packet::read(&output_data, self.is_ngs)?;
            if packets.0 != output_data.len() {
                println!("Investigate this");
            }
            self.read_packets.append(&mut packets.1);
            let packet = self.get_one_packet();
            if let Packet::EncryptionRequest(data) = &packet {
                if let Some(keyfile) = &self.in_keyfile {
                    self.encryption =
                        Encryption::from_rsa_data(&data.rsa_data, self.is_ngs, keyfile)?;
                }
            }
            return Ok(Some(packet));
        }
        Ok(None)
    }

    /// Send a packet. Return bytes written.
    pub fn write_packet(&mut self, packet: Packet) -> std::io::Result<usize> {
        if let Packet::EncryptionRequest(data) = &packet {
            if let Some(out_keyfile) = &self.out_keyfile {
                if let Some(in_keyfile) = &self.in_keyfile {
                    let mut new_packet = EncryptionRequestPacket::default();
                    self.encryption =
                        Encryption::from_rsa_data(&data.rsa_data, self.is_ngs, in_keyfile)?;
                    new_packet.rsa_data = reencrypt(&data.rsa_data, in_keyfile, out_keyfile)?;
                    self.write_buffer.extend_from_slice(
                        &Packet::EncryptionRequest(new_packet).write(self.is_ngs),
                    );
                }
            }
        } else {
            self.write_buffer
                .extend_from_slice(&self.encryption.encrypt(&{ packet.write(self.is_ngs) })?[..]);
        }
        if self.write_buffer.is_empty() {
            return Ok(0);
        }
        let wrote_bytes = self.stream.write(&self.write_buffer)?;
        self.write_buffer.drain(..wrote_bytes);
        Ok(wrote_bytes)
    }
    /// Get the encryption key (for [`Packet::EncryptionResponse`])
    pub fn get_key(&mut self) -> Vec<u8> {
        self.encryption.get_key()
    }
    /// Write all pending packets.
    pub fn flush(&mut self) -> std::io::Result<usize> {
        if self.write_buffer.is_empty() {
            return Ok(0);
        }
        self.write_packet(Packet::None)
    }
    fn get_one_packet(&mut self) -> Packet {
        self.read_packets.remove(0)
    }
}

fn get_length(data: &Vec<u8>) -> usize {
    let mut len = u32::from_le_bytes(data[0x0..0x4].try_into().unwrap()) as usize;
    if data.len() >= 0x48 && data[0x40..0x44] == [1, 0, 255, 255] {
        len = u32::from_le_bytes(data[0x44..0x48].try_into().unwrap()) as usize;
    }
    if data.len() >= 0x58 && data[0x50..0x54] == [1, 0, 255, 255] {
        len = u32::from_le_bytes(data[0x54..0x58].try_into().unwrap()) as usize;
    }
    len
}

fn check_disconnect(to_check: std::io::Result<usize>) -> std::io::Result<usize> {
    match to_check {
        Ok(0) => Err(std::io::ErrorKind::ConnectionAborted.into()),
        Ok(data) => Ok(data),
        Err(err) if err.kind() == std::io::ErrorKind::ConnectionReset => {
            Err(std::io::ErrorKind::ConnectionAborted.into())
        }
        Err(err) => Err(err),
    }
}
