use rsa::{pkcs8::DecodePrivateKey, BigUint, RsaPrivateKey};

#[cfg(feature = "ppac")]
use crate::ppac::{Direction, PPACWriter};
use crate::{
    encryption::Encryption,
    protocol::{Packet, PacketType, ProtocolRW},
};
use std::io::{Read, Write};

/// Represents a connection between a client and a server.
#[derive(Debug)]
pub struct Connection {
    stream: std::net::TcpStream,
    encryption: Encryption,
    read_buffer: Vec<u8>,
    read_packets: Vec<Packet>,
    write_buffer: Vec<u8>,
    packet_length: usize,
    in_keyfile: PrivateKey,
    packet_type: PacketType,
    #[cfg(feature = "ppac")]
    ppac: Option<PPACWriter<std::fs::File>>,
    #[cfg(feature = "ppac")]
    direction: Direction,
}

/// Possible RSA private key formats.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum PrivateKey {
    /// No private key provided.
    None,
    /// Path to the RSA key in the PEM-encoded PKCS#8 file.
    Path(std::path::PathBuf),
    /// RSA key in component form. All values are in little endian form.
    Params {
        /// RSA modulus 'n'.
        n: Vec<u8>,
        /// Public exponent 'e'.
        e: Vec<u8>,
        /// Private exponent 'd'.
        d: Vec<u8>,
        /// First prime 'p'.
        p: Vec<u8>,
        /// Second prime 'q'.
        q: Vec<u8>,
    },
    Key(RsaPrivateKey),
}

impl Connection {
    /// Creates a new connection.
    /// `in_keyfile` is the RSA key to decrypt client's encryption request.
    pub fn new(
        stream: std::net::TcpStream,
        packet_type: PacketType,
        in_keyfile: PrivateKey,
    ) -> Self {
        Self {
            stream,
            encryption: Encryption::None,
            read_buffer: Vec::new(),
            read_packets: Vec::new(),
            write_buffer: Vec::new(),
            packet_length: 0,
            in_keyfile,
            packet_type,
            #[cfg(feature = "ppac")]
            ppac: None,
            #[cfg(feature = "ppac")]
            direction: Direction::ToServer,
        }
    }
    /// Returns the ip address of the client.
    pub fn get_ip(&self) -> std::io::Result<std::net::Ipv4Addr> {
        let ip = self.stream.peer_addr()?.ip();
        let ip = match ip {
            std::net::IpAddr::V4(x) => x,
            std::net::IpAddr::V6(_) => std::net::Ipv4Addr::UNSPECIFIED,
        };
        Ok(ip)
    }
    /// Changes connection type.
    pub fn change_packet_type(&mut self, packet_type: PacketType) {
        #[cfg(feature = "ppac")]
        if let Some(writer) = &mut self.ppac {
            let _ = writer.change_packet_type(packet_type);
        }
        self.packet_type = packet_type;
    }
    /// Reads a packet from stream.
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

    /// Creates a packet storage file. `direction` is the direction of the `write` side of the
    /// connection.
    #[cfg(feature = "ppac")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ppac")))]
    pub fn create_ppac<P: AsRef<std::path::Path>>(
        &mut self,
        path: P,
        direction: Direction,
    ) -> std::io::Result<()> {
        self.ppac = Some(PPACWriter::new(
            std::fs::File::create(path)?,
            self.packet_type,
            true,
        )?);
        self.direction = direction;
        Ok(())
    }

    fn get_packet(&mut self) -> std::io::Result<Option<Packet>> {
        let mut output_data = vec![0u8; 0];
        if self.packet_length == 0 {
            self.get_length();
        }
        if self.read_buffer.len() >= self.packet_length && self.packet_length != 0 {
            output_data.extend(self.read_buffer.drain(..self.packet_length));
            self.packet_length = 0;
            let output_data = if self.encryption.is_rc4() {
                output_data
            } else {
                self.encryption.decrypt(&output_data)?
            };
            #[cfg(feature = "ppac")]
            if let Some(writer) = &mut self.ppac {
                let direction = match self.direction {
                    Direction::ToServer => Direction::ToClient,
                    Direction::ToClient => Direction::ToServer,
                };
                writer.write_data(crate::ppac::get_now(), direction, &output_data)?;
            }
            let mut packets = Packet::read(&output_data, self.packet_type)?;
            self.read_packets.append(&mut packets);
            let packet = self.get_one_packet();
            if let Packet::EncryptionRequest(data) = &packet {
                if !matches!(&self.in_keyfile, PrivateKey::None) {
                    self.encryption = Encryption::from_rsa_data(
                        &data.rsa_data,
                        matches!(self.packet_type, PacketType::NGS),
                        &self.in_keyfile,
                    )?;
                }
            }
            return Ok(Some(packet));
        }
        Ok(None)
    }

    /// Sends a packet and returns bytes written.
    pub fn write_packet(&mut self, packet: &Packet) -> std::io::Result<usize> {
        self.write_buffer
            .extend_from_slice(&self.encryption.encrypt(&{
                let packet = packet.write(self.packet_type);
                #[cfg(feature = "ppac")]
                if let Some(writer) = &mut self.ppac {
                    writer.write_data(crate::ppac::get_now(), self.direction, &packet)?;
                }
                packet
            })?);
        if self.write_buffer.is_empty() {
            return Ok(0);
        }
        let wrote_bytes = check_disconnect(self.stream.write(&self.write_buffer))?;
        self.write_buffer.drain(..wrote_bytes).count();
        Ok(wrote_bytes)
    }
    /// Returns the encryption key (for [`Packet::EncryptionResponse`]).
    pub fn get_key(&mut self) -> Vec<u8> {
        self.encryption.get_key()
    }
    /// Writes all pending packets.
    pub fn flush(&mut self) -> std::io::Result<usize> {
        if self.write_buffer.is_empty() {
            return Ok(0);
        }
        self.write_packet(&Packet::None)
    }
    fn get_one_packet(&mut self) -> Packet {
        self.read_packets.remove(0)
    }
    fn get_length(&mut self) {
        let data = &self.read_buffer;
        let mut len = 0;
        #[cfg(feature = "ngs_enc")]
        if matches!(self.encryption, Encryption::AesNgs(_))
            && data.len() >= 0x48
            && data[0x40..0x44] == [1, 0, 255, 255]
        {
            len = u32::from_le_bytes(data[0x44..0x48].try_into().unwrap()) as usize;
        }
        #[cfg(feature = "base_enc")]
        if matches!(self.encryption, Encryption::Aes(_))
            && data.len() >= 0x48
            && data[0x40..0x44] == [1, 0, 255, 255]
        {
            len = u32::from_le_bytes(data[0x44..0x48].try_into().unwrap()) as usize;
        }
        #[cfg(feature = "vita_enc")]
        if matches!(self.encryption, Encryption::Rc4(_)) {
            len = u32::from_le_bytes(data[0x0..0x4].try_into().unwrap()) as usize;
        }
        if matches!(self.encryption, Encryption::None) {
            len = u32::from_le_bytes(data[0x0..0x4].try_into().unwrap()) as usize;
        }
        self.packet_length = len
    }
}

fn check_disconnect(to_check: std::io::Result<usize>) -> std::io::Result<usize> {
    match to_check {
        Ok(0) => Err(std::io::ErrorKind::ConnectionAborted.into()),
        Ok(data) => Ok(data),
        Err(err) if err.kind() == std::io::ErrorKind::ConnectionReset => {
            Err(std::io::ErrorKind::ConnectionAborted.into())
        }
        Err(err) if err.kind() == std::io::ErrorKind::Interrupted => {
            Err(std::io::ErrorKind::WouldBlock.into())
        }
        Err(err) => Err(err),
    }
}

impl PrivateKey {
    pub fn into_key(&self) -> rsa::errors::Result<Option<RsaPrivateKey>> {
        match self {
            Self::None => Ok(None),
            Self::Path(p) => Ok(Some(RsaPrivateKey::read_pkcs8_pem_file(p)?)),
            Self::Params { n, e, d, p, q } => Ok(Some(RsaPrivateKey::from_components(
                BigUint::from_bytes_le(n),
                BigUint::from_bytes_le(e),
                BigUint::from_bytes_le(d),
                vec![BigUint::from_bytes_le(p), BigUint::from_bytes_le(q)],
            )?)),
            Self::Key(k) => Ok(Some(k.clone())),
        }
    }
}
