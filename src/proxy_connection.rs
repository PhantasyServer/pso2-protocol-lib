use rsa::{pkcs8::DecodePublicKey, BigUint, RsaPublicKey};

#[cfg(feature = "ppac")]
use crate::ppac::{Direction, PPACWriter};
use crate::{
    connection::PrivateKey,
    encryption::{reencrypt, Encryption},
    protocol::{login::EncryptionRequestPacket, PacketType, ProtocolRW, ProxyPacket},
};
use std::io::{Read, Write};

/// Represents a proxy connection between a client and a server.
#[derive(Debug)]
pub struct ProxyConnection {
    stream: std::net::TcpStream,
    encryption: Encryption,
    read_buffer: Vec<u8>,
    read_packets: Vec<ProxyPacket>,
    write_buffer: Vec<u8>,
    packet_length: usize,
    in_keyfile: PrivateKey,
    out_keyfile: PublicKey,
    packet_type: PacketType,
    #[cfg(feature = "ppac")]
    ppac: Option<PPACWriter<std::fs::File>>,
    #[cfg(feature = "ppac")]
    direction: Direction,
}

/// Possible RSA public key formats.
#[derive(Debug, Clone)]
pub enum PublicKey {
    /// No public key provided.
    None,
    /// Path to the RSA key in the PEM-encoded PKCS#8 file.
    Path(std::path::PathBuf),
    /// RSA key in component form. All values are in little endian form.
    Params {
        /// RSA modulus 'n'.
        n: Vec<u8>,
        /// Public exponent 'e'.
        e: Vec<u8>,
    },
    Key(RsaPublicKey),
}

impl ProxyConnection {
    /// Creates a new proxy connection.
    /// `in_keyfile` is the RSA key to decrypt client's encryption request.
    /// `out_keyfile` is the RSA key to reencrypt client's encryption request.
    pub fn new(
        stream: std::net::TcpStream,
        packet_type: PacketType,
        in_keyfile: PrivateKey,
        out_keyfile: PublicKey,
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
    /// Splits the connection into separate read and write components.
    #[cfg(feature = "tokio")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
    pub fn into_split(self) -> std::io::Result<(ProxyRead, ProxyWrite)> {
        let (read, write) = tokio::net::TcpStream::from_std(self.stream)?.into_split();
        #[cfg(feature = "ppac")]
        let ppac = self
            .ppac
            .map(|p| std::sync::Arc::new(tokio::sync::Mutex::new(p)));
        let (reader_send, writer_recv) = tokio::sync::mpsc::channel(1);
        let (writer_send, reader_recv) = tokio::sync::mpsc::channel(1);
        let (enc, dec) = self.encryption.into_split();
        let reader = ProxyRead {
            stream: read,
            write_channel: (reader_send, reader_recv),
            encryption: dec,
            read_buffer: self.read_buffer,
            read_packets: self.read_packets,
            packet_length: self.packet_length,
            in_keyfile: self.in_keyfile.clone(),
            packet_type: self.packet_type,
            #[cfg(feature = "ppac")]
            ppac: ppac.clone(),
            #[cfg(feature = "ppac")]
            direction: self.direction,
        };
        let writer = ProxyWrite {
            stream: write,
            read_channel: (writer_send, writer_recv),
            encryption: enc,
            write_buffer: self.write_buffer,
            in_keyfile: self.in_keyfile,
            out_keyfile: self.out_keyfile,
            packet_type: self.packet_type,
            #[cfg(feature = "ppac")]
            ppac,
            #[cfg(feature = "ppac")]
            direction: self.direction,
        };
        Ok((reader, writer))
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
    pub fn read_packet(&mut self) -> std::io::Result<ProxyPacket> {
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

    fn get_packet(&mut self) -> std::io::Result<Option<ProxyPacket>> {
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
            let mut packets = ProxyPacket::read(&output_data, self.packet_type)?;
            self.read_packets.append(&mut packets);
            let packet = self.get_one_packet();
            if let ProxyPacket::EncryptionRequest(data) = &packet {
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
    pub fn write_packet(&mut self, packet: &ProxyPacket) -> std::io::Result<usize> {
        if let ProxyPacket::EncryptionRequest(data) = packet {
            if !matches!(&self.out_keyfile, PublicKey::None)
                && !matches!(&self.in_keyfile, PrivateKey::None)
            {
                let mut new_packet = EncryptionRequestPacket::default();
                self.encryption = Encryption::from_rsa_data(
                    &data.rsa_data,
                    matches!(self.packet_type, PacketType::NGS),
                    &self.in_keyfile,
                )?;
                new_packet.rsa_data =
                    reencrypt(&data.rsa_data, &self.in_keyfile, &self.out_keyfile)?;
                self.write_buffer.extend_from_slice(&{
                    let packet = ProxyPacket::EncryptionRequest(new_packet).write(self.packet_type);
                    #[cfg(feature = "ppac")]
                    if let Some(writer) = &mut self.ppac {
                        writer.write_data(crate::ppac::get_now(), self.direction, &packet)?;
                    }
                    packet
                });
            }
        } else {
            self.write_buffer
                .extend_from_slice(&self.encryption.encrypt(&{
                    let packet = packet.write(self.packet_type);
                    #[cfg(feature = "ppac")]
                    if let Some(writer) = &mut self.ppac {
                        writer.write_data(crate::ppac::get_now(), self.direction, &packet)?;
                    }
                    packet
                })?);
        }
        if self.write_buffer.is_empty() {
            return Ok(0);
        }
        let wrote_bytes = check_disconnect(self.stream.write(&self.write_buffer))?;
        self.write_buffer.drain(..wrote_bytes).count();
        Ok(wrote_bytes)
    }

    /// Returns the encryption key (for [`ProxyPacket::EncryptionResponse`]).
    pub fn get_key(&mut self) -> Vec<u8> {
        self.encryption.get_key()
    }
    /// Writes all pending packets.
    pub fn flush(&mut self) -> std::io::Result<usize> {
        if self.write_buffer.is_empty() {
            return Ok(0);
        }
        self.write_packet(&ProxyPacket::None)
    }
    fn get_one_packet(&mut self) -> ProxyPacket {
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

impl PublicKey {
    pub fn into_key(&self) -> rsa::errors::Result<Option<RsaPublicKey>> {
        match self {
            Self::None => Ok(None),
            Self::Path(p) => Ok(Some(
                RsaPublicKey::read_public_key_pem_file(p)
                    .map_err(|e| rsa::Error::Pkcs8(rsa::pkcs8::Error::PublicKey(e)))?,
            )),
            Self::Params { n, e } => Ok(Some(RsaPublicKey::new(
                BigUint::from_bytes_le(n),
                BigUint::from_bytes_le(e),
            )?)),
            Self::Key(k) => Ok(Some(k.clone())),
        }
    }
}

#[cfg(feature = "tokio")]
use crate::encryption::{Decryptor, Encryptor};
#[cfg(all(feature = "tokio", feature = "ppac"))]
use std::sync::Arc;
#[cfg(all(feature = "tokio", feature = "ppac"))]
use tokio::sync::Mutex;
#[cfg(feature = "tokio")]
use tokio::{
    io::AsyncWriteExt,
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    sync::mpsc::{Receiver, Sender},
};

/// Represents a proxy reader between a client and a server.
#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
#[derive(Debug)]
pub struct ProxyRead {
    stream: OwnedReadHalf,
    write_channel: (Sender<Encryptor>, Receiver<Decryptor>),
    encryption: Decryptor,
    read_buffer: Vec<u8>,
    read_packets: Vec<ProxyPacket>,
    packet_length: usize,
    in_keyfile: PrivateKey,
    packet_type: PacketType,
    #[cfg(feature = "ppac")]
    ppac: Option<Arc<Mutex<PPACWriter<std::fs::File>>>>,
    #[cfg(feature = "ppac")]
    direction: Direction,
}

/// Represents a proxy writer between a client and a server.
#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
#[derive(Debug)]
pub struct ProxyWrite {
    stream: OwnedWriteHalf,
    read_channel: (Sender<Decryptor>, Receiver<Encryptor>),
    encryption: Encryptor,
    write_buffer: Vec<u8>,
    in_keyfile: PrivateKey,
    out_keyfile: PublicKey,
    packet_type: PacketType,
    #[cfg(feature = "ppac")]
    ppac: Option<Arc<Mutex<PPACWriter<std::fs::File>>>>,
    #[cfg(feature = "ppac")]
    direction: Direction,
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl ProxyRead {
    /// Returns the ip address of the client.
    pub fn get_ip(&self) -> std::io::Result<std::net::Ipv4Addr> {
        let ip = self.stream.peer_addr()?.ip();
        let ip = match ip {
            std::net::IpAddr::V4(x) => x,
            std::net::IpAddr::V6(_) => std::net::Ipv4Addr::UNSPECIFIED,
        };
        Ok(ip)
    }
    // /// Changes connection type.
    // pub fn change_packet_type(&mut self, packet_type: PacketType) {
    //     #[cfg(feature = "ppac")]
    //     if let Some(writer) = &mut self.ppac {
    //         let _ = writer.change_packet_type(packet_type);
    //     }
    //     self.packet_type = packet_type;
    // }
    /// Reads a packet from stream.
    pub async fn read_packet(&mut self) -> std::io::Result<ProxyPacket> {
        if !self.read_packets.is_empty() {
            return Ok(self.get_one_packet());
        }
        if !self.read_buffer.is_empty() {
            if let Some(packet) = self.get_packet().await? {
                return Ok(packet);
            }
        }
        loop {
            self.stream.readable().await?;
            // keep buf away from async task
            {
                let mut buf = [0u8; 4096];
                let read_bytes = match check_disconnect(self.stream.try_read(&mut buf)) {
                    Ok(n) => n,
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
                    Err(e) => return Err(e),
                };
                if matches!(self.encryption, Decryptor::None) {
                    if let Ok(dec) = self.write_channel.1.try_recv() {
                        self.encryption = dec
                    }
                }
                if self.encryption.is_rc4() {
                    let mut decrypted_stream = self.encryption.decrypt(&buf[..read_bytes])?;
                    self.read_buffer.append(&mut decrypted_stream);
                } else {
                    self.read_buffer.extend_from_slice(&buf[..read_bytes]);
                }
            }
            if let Some(packet) = self.get_packet().await? {
                return Ok(packet);
            }
        }
    }

    /// Inserts a packet storage file. `direction` is the direction of the `write` side of the
    /// connection.
    #[cfg(feature = "ppac")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ppac")))]
    pub fn set_ppac(
        &mut self,
        ppac: Arc<Mutex<PPACWriter<std::fs::File>>>,
        direction: Direction,
    ) -> std::io::Result<()> {
        self.ppac = Some(ppac);
        self.direction = direction;
        Ok(())
    }

    async fn get_packet(&mut self) -> std::io::Result<Option<ProxyPacket>> {
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
            if let Some(writer) = &self.ppac {
                let direction = match self.direction {
                    Direction::ToServer => Direction::ToClient,
                    Direction::ToClient => Direction::ToServer,
                };
                let mut lock = writer.lock().await;
                lock.write_data(crate::ppac::get_now(), direction, &output_data)?;
            }
            let mut packets = ProxyPacket::read(&output_data, self.packet_type)?;
            self.read_packets.append(&mut packets);
            let packet = self.get_one_packet();
            if let ProxyPacket::EncryptionRequest(data) = &packet {
                if !matches!(&self.in_keyfile, PrivateKey::None) {
                    let (enc, dec) = Encryption::from_rsa_data(
                        &data.rsa_data,
                        matches!(self.packet_type, PacketType::NGS),
                        &self.in_keyfile,
                    )?
                    .into_split();
                    let _ = self.write_channel.0.send(enc).await;
                    self.encryption = dec;
                }
            }
            return Ok(Some(packet));
        }
        Ok(None)
    }

    /// Returns the encryption key (for [`ProxyPacket::EncryptionResponse`]).
    pub fn get_key(&mut self) -> Vec<u8> {
        self.encryption.get_key()
    }
    fn get_one_packet(&mut self) -> ProxyPacket {
        self.read_packets.remove(0)
    }
    fn get_length(&mut self) {
        let data = &self.read_buffer;
        let mut len = 0;
        #[cfg(feature = "ngs_enc")]
        if matches!(self.encryption, Decryptor::AesNgs(_))
            && data.len() >= 0x48
            && data[0x40..0x44] == [1, 0, 255, 255]
        {
            len = u32::from_le_bytes(data[0x44..0x48].try_into().unwrap()) as usize;
        }
        #[cfg(feature = "base_enc")]
        if matches!(self.encryption, Decryptor::Aes(_))
            && data.len() >= 0x48
            && data[0x40..0x44] == [1, 0, 255, 255]
        {
            len = u32::from_le_bytes(data[0x44..0x48].try_into().unwrap()) as usize;
        }
        #[cfg(feature = "vita_enc")]
        if matches!(self.encryption, Decryptor::Rc4(_)) {
            len = u32::from_le_bytes(data[0x0..0x4].try_into().unwrap()) as usize;
        }
        if matches!(self.encryption, Decryptor::None) {
            len = u32::from_le_bytes(data[0x0..0x4].try_into().unwrap()) as usize;
        }
        self.packet_length = len
    }
}

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
impl ProxyWrite {
    /// Returns the ip address of the client.
    pub fn get_ip(&self) -> std::io::Result<std::net::Ipv4Addr> {
        let ip = self.stream.peer_addr()?.ip();
        let ip = match ip {
            std::net::IpAddr::V4(x) => x,
            std::net::IpAddr::V6(_) => std::net::Ipv4Addr::UNSPECIFIED,
        };
        Ok(ip)
    }
    // /// Changes connection type.
    // pub fn change_packet_type(&mut self, packet_type: PacketType) {
    //     #[cfg(feature = "ppac")]
    //     if let Some(writer) = &mut self.ppac {
    //         let _ = writer.change_packet_type(packet_type);
    //     }
    //     self.packet_type = packet_type;
    // }
    /// Inserts a packet storage file. `direction` is the direction of the `write` side of the
    /// connection.
    #[cfg(feature = "ppac")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ppac")))]
    pub fn set_ppac(
        &mut self,
        ppac: Arc<Mutex<PPACWriter<std::fs::File>>>,
        direction: Direction,
    ) -> std::io::Result<()> {
        self.ppac = Some(ppac);
        self.direction = direction;
        Ok(())
    }

    /// Sends a packet.
    pub async fn write_packet(&mut self, packet: &ProxyPacket) -> std::io::Result<()> {
        if let ProxyPacket::EncryptionRequest(data) = packet {
            if !matches!(&self.out_keyfile, PublicKey::None)
                && !matches!(&self.in_keyfile, PrivateKey::None)
            {
                let mut new_packet = EncryptionRequestPacket::default();
                let (enc, dec) = Encryption::from_rsa_data(
                    &data.rsa_data,
                    matches!(self.packet_type, PacketType::NGS),
                    &self.in_keyfile,
                )?
                .into_split();
                let _ = self.read_channel.0.send(dec).await;
                self.encryption = enc;
                new_packet.rsa_data =
                    reencrypt(&data.rsa_data, &self.in_keyfile, &self.out_keyfile)?;
                self.write_buffer.extend_from_slice(&{
                    let packet = ProxyPacket::EncryptionRequest(new_packet).write(self.packet_type);
                    #[cfg(feature = "ppac")]
                    if let Some(writer) = &self.ppac {
                        let mut lock = writer.lock().await;
                        lock.write_data(crate::ppac::get_now(), self.direction, &packet)?;
                    }
                    packet
                });
            }
        } else {
            if matches!(self.encryption, Encryptor::None) {
                if let Ok(enc) = self.read_channel.1.try_recv() {
                    self.encryption = enc
                }
            }
            self.write_buffer
                .extend_from_slice(&self.encryption.encrypt(&{
                    let packet = packet.write(self.packet_type);
                    #[cfg(feature = "ppac")]
                    if let Some(writer) = &self.ppac {
                        let mut lock = writer.lock().await;
                        lock.write_data(crate::ppac::get_now(), self.direction, &packet)?;
                    }
                    packet
                })?);
        }
        if self.write_buffer.is_empty() {
            return Ok(());
        }
        while !self.write_buffer.is_empty() {
            self.stream.writable().await?;
            let wrote_bytes = match check_disconnect(self.stream.try_write(&self.write_buffer)) {
                Ok(n) => n,
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => 0,
                Err(e) => return Err(e),
            };
            self.write_buffer.drain(..wrote_bytes).count();
        }
        Ok(())
    }

    /// Returns the encryption key (for [`ProxyPacket::EncryptionResponse`]).
    pub fn get_key(&mut self) -> Vec<u8> {
        self.encryption.get_key()
    }
    /// Writes all pending packets.
    pub async fn flush(&mut self) -> std::io::Result<()> {
        self.stream.flush().await?;
        if self.write_buffer.is_empty() {
            return Ok(());
        }
        self.write_packet(&ProxyPacket::None).await?;
        Ok(())
    }
}
