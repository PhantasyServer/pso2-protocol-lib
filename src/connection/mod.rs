pub(crate) mod conn_impl;
#[cfg(feature = "proxy")]
pub mod proxy;
#[cfg(feature = "split_connection")]
use crate::encryption::{DecryptorType, EncryptorType};
#[cfg(feature = "ppac")]
use crate::ppac::{Direction, PPACWriter};
use crate::{
    encryption::Encryption,
    protocol::{Packet, PacketType, ProtocolRW},
};
use conn_impl::{ConnectionReader, ConnectionWriter};
use rsa::{pkcs8::DecodePrivateKey, BigUint, RsaPrivateKey};
#[cfg(all(feature = "split_connection", not(feature = "tokio")))]
use std::sync::mpsc::{Receiver, Sender};
#[cfg(all(feature = "split_connection", feature = "ppac"))]
use std::sync::{Arc, Mutex};
#[cfg(all(feature = "split_connection", feature = "tokio"))]
use tokio::{
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    sync::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender},
};

//TODO: this code currently only functions as a server.
//make it work as a client also.

/// Represents a connection between a client and a server.
#[derive(Debug)]
pub struct Connection {
    // this is probably not the best way to do this
    #[cfg(not(feature = "tokio"))]
    stream: std::net::TcpStream,
    #[cfg(feature = "tokio")]
    stream: tokio::net::TcpStream,
    encryption: Encryption,
    read: ConnectionReader,
    write: ConnectionWriter,
    read_packets: Vec<Packet>,
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
    ///
    /// # Note
    ///
    /// If the provided stream is not set to a nonblocking mode then any read/write operation will
    /// block.
    ///
    /// # Panics
    ///
    /// If `tokio` feature is enabled then this function will implicitly convert a stream to an
    /// async stream. This function panics if this conversion fails.
    pub fn new(
        stream: std::net::TcpStream,
        packet_type: PacketType,
        in_keyfile: PrivateKey,
    ) -> Self {
        #[cfg(feature = "tokio")]
        let stream = {
            stream
                .set_nonblocking(true)
                .expect("set_nonblocking failed");
            tokio::net::TcpStream::from_std(stream).expect("Failed to make async stream")
        };
        Self {
            stream,
            encryption: Encryption::None,
            read: ConnectionReader::default(),
            write: ConnectionWriter::default(),
            read_packets: Vec::new(),
            in_keyfile,
            packet_type,
            #[cfg(feature = "ppac")]
            ppac: None,
            #[cfg(feature = "ppac")]
            direction: Direction::ToServer,
        }
    }

    /// Creates a new connection.
    /// `in_keyfile` is the RSA key to decrypt client's encryption request.
    #[cfg(feature = "tokio")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
    pub fn new_async(
        stream: tokio::net::TcpStream,
        packet_type: PacketType,
        in_keyfile: PrivateKey,
    ) -> Self {
        Self {
            stream,
            encryption: Encryption::None,
            read: ConnectionReader::default(),
            write: ConnectionWriter::default(),
            read_packets: Vec::new(),
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

    /// Splits the connection into separate read and write components.
    #[cfg(feature = "split_connection")]
    #[cfg_attr(docsrs, doc(cfg(feature = "split_connection")))]
    pub fn into_split(self) -> std::io::Result<(ConnectionRead, ConnectionWrite)> {
        #[cfg(feature = "tokio")]
        let (read, write) = self.stream.into_split();
        #[cfg(not(feature = "tokio"))]
        let (read, write) = (self.stream.try_clone()?, self.stream);
        #[cfg(feature = "ppac")]
        let ppac = self
            .ppac
            .map(|p| std::sync::Arc::new(std::sync::Mutex::new(p)));
        #[cfg(feature = "tokio")]
        let ((reader_send, writer_recv), (writer_send, reader_recv)) = (
            tokio::sync::mpsc::unbounded_channel(),
            tokio::sync::mpsc::unbounded_channel(),
        );
        #[cfg(not(feature = "tokio"))]
        let ((reader_send, writer_recv), (writer_send, reader_recv)) =
            (std::sync::mpsc::channel(), std::sync::mpsc::channel());
        #[cfg(feature = "tokio")]
        let ((readpt_send, writept_recv), (writept_send, readpt_recv)) = (
            tokio::sync::mpsc::unbounded_channel(),
            tokio::sync::mpsc::unbounded_channel(),
        );
        #[cfg(not(feature = "tokio"))]
        let ((readpt_send, writept_recv), (writept_send, readpt_recv)) =
            (std::sync::mpsc::channel(), std::sync::mpsc::channel());
        let (enc, dec) = self.encryption.into_split();
        let reader = ConnectionRead {
            stream: read,
            enc_channel: (reader_send, reader_recv),
            packettype_channel: (readpt_send, readpt_recv),
            encryption: dec,
            read: self.read,
            read_packets: self.read_packets,
            in_keyfile: self.in_keyfile.clone(),
            packet_type: self.packet_type,
            #[cfg(feature = "ppac")]
            ppac: ppac.clone(),
            #[cfg(feature = "ppac")]
            direction: self.direction,
        };
        let writer = ConnectionWrite {
            stream: write,
            enc_channel: (writer_send, writer_recv),
            packettype_channel: (writept_send, writept_recv),
            write: self.write,
            encryption: enc,
            packet_type: self.packet_type,
            #[cfg(feature = "ppac")]
            ppac,
            #[cfg(feature = "ppac")]
            direction: self.direction,
        };
        Ok((reader, writer))
    }

    /// Reads a packet from the stream.
    ///
    /// # Note
    ///
    /// If `tokio` feature is enabled this function becomes nonblocking
    pub fn read_packet(&mut self) -> std::io::Result<Packet> {
        if !self.read_packets.is_empty() {
            return Ok(self.read_packets.remove(0));
        }
        let data = self
            .read
            .try_read_data(&mut self.stream, &mut self.encryption)?;
        #[cfg(feature = "ppac")]
        if let Some(writer) = &mut self.ppac {
            let direction = match self.direction {
                Direction::ToServer => Direction::ToClient,
                Direction::ToClient => Direction::ToServer,
            };
            writer.write_data(crate::ppac::get_now(), direction, &data)?;
        }
        self.parse_packet(&data)
    }

    /// Reads a packet from the stream.
    #[cfg(feature = "tokio")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
    pub async fn read_packet_async(&mut self) -> std::io::Result<Packet> {
        if !self.read_packets.is_empty() {
            return Ok(self.read_packets.remove(0));
        }
        let data = self
            .read
            .read_data_async(&mut self.stream, &mut self.encryption)
            .await?;
        #[cfg(feature = "ppac")]
        if let Some(writer) = &mut self.ppac {
            let direction = match self.direction {
                Direction::ToServer => Direction::ToClient,
                Direction::ToClient => Direction::ToServer,
            };
            writer.write_data(crate::ppac::get_now(), direction, &data)?;
        }
        self.parse_packet(&data)
    }
    fn parse_packet(&mut self, data: &[u8]) -> std::io::Result<Packet> {
        let mut packets = Packet::read(data, self.packet_type)?;
        let mut packet = packets.remove(0);
        self.read_packets.append(&mut packets);
        if let Packet::EncryptionRequest(data) = &mut packet {
            if !matches!(&self.in_keyfile, PrivateKey::None) {
                let dec_data = Encryption::decrypt_rsa_data(&data.rsa_data, &self.in_keyfile)?;
                self.encryption = Encryption::from_dec_data(
                    &dec_data,
                    matches!(self.packet_type, PacketType::NGS),
                )?;
                data.rsa_data = dec_data;
            }
        }
        Ok(packet)
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

    /// Sends a packet.
    ///
    /// # Note
    ///
    /// If `tokio` feature is enabled this function becomes nonblocking
    pub fn write_packet(&mut self, packet: &Packet) -> std::io::Result<()> {
        self.prepare_data(packet)?;
        self.write.flush(&mut self.stream)?;
        Ok(())
    }

    /// Sends a packet.
    #[cfg(feature = "tokio")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
    pub async fn write_packet_async(&mut self, packet: &Packet) -> std::io::Result<()> {
        self.prepare_data(packet)?;
        self.write.flush_async(&mut self.stream).await?;
        Ok(())
    }

    fn prepare_data(&mut self, packet: &Packet) -> std::io::Result<()> {
        let packet = packet.write(self.packet_type);
        #[cfg(feature = "ppac")]
        if let Some(writer) = &mut self.ppac {
            writer.write_data(crate::ppac::get_now(), self.direction, &packet)?;
        }
        self.write.prepare_data(&packet, &mut self.encryption)?;
        Ok(())
    }

    /// Returns the encryption key (for [`Packet::EncryptionResponse`]).
    pub fn get_key(&mut self) -> Vec<u8> {
        self.encryption.get_key()
    }
    /// Writes all pending packets.
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.write.flush(&mut self.stream)
    }
    /// Writes all pending packets.
    #[cfg(feature = "tokio")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
    pub async fn flush_async(&mut self) -> std::io::Result<()> {
        self.write.flush_async(&mut self.stream).await
    }
}

/// Represents a reader portion of the connection between a client and a server.
#[cfg(feature = "split_connection")]
#[cfg_attr(docsrs, doc(cfg(feature = "split_connection")))]
#[derive(Debug)]
pub struct ConnectionRead {
    #[cfg(not(feature = "tokio"))]
    stream: std::net::TcpStream,
    #[cfg(feature = "tokio")]
    stream: OwnedReadHalf,
    enc_channel: (Sender<EncryptorType>, Receiver<DecryptorType>),
    packettype_channel: (Sender<PacketType>, Receiver<PacketType>),
    read: ConnectionReader,
    encryption: DecryptorType,
    read_packets: Vec<Packet>,
    in_keyfile: PrivateKey,
    packet_type: PacketType,
    #[cfg(feature = "ppac")]
    ppac: Option<Arc<Mutex<PPACWriter<std::fs::File>>>>,
    #[cfg(feature = "ppac")]
    direction: Direction,
}

/// Represents a writer portion of the connection between a client and a server.
#[cfg(feature = "split_connection")]
#[cfg_attr(docsrs, doc(cfg(feature = "split_connection")))]
#[derive(Debug)]
pub struct ConnectionWrite {
    #[cfg(not(feature = "tokio"))]
    stream: std::net::TcpStream,
    #[cfg(feature = "tokio")]
    stream: OwnedWriteHalf,
    enc_channel: (Sender<DecryptorType>, Receiver<EncryptorType>),
    packettype_channel: (Sender<PacketType>, Receiver<PacketType>),
    write: ConnectionWriter,
    encryption: EncryptorType,
    packet_type: PacketType,
    #[cfg(feature = "ppac")]
    ppac: Option<Arc<Mutex<PPACWriter<std::fs::File>>>>,
    #[cfg(feature = "ppac")]
    direction: Direction,
}

#[cfg(feature = "split_connection")]
#[cfg_attr(docsrs, doc(cfg(feature = "split_connection")))]
impl ConnectionRead {
    /// Returns the ip address of the client.
    pub fn get_ip(&self) -> std::io::Result<std::net::Ipv4Addr> {
        let ip = self.stream.peer_addr()?.ip();
        let ip = match ip {
            std::net::IpAddr::V4(x) => x,
            std::net::IpAddr::V6(_) => std::net::Ipv4Addr::UNSPECIFIED,
        };
        Ok(ip)
    }

    /// Changes connection type. Automatically changes the other side.
    pub fn change_packet_type(&mut self, packet_type: PacketType) {
        #[cfg(feature = "ppac")]
        if let Some(writer) = &mut self.ppac {
            let mut lock = writer.lock().unwrap();
            let _ = lock.change_packet_type(packet_type);
        }
        self.packet_type = packet_type;
        let _ = self.packettype_channel.0.send(packet_type);
    }

    /// Same as [`std::net::TcpStream::set_nonblocking`]. Does nothing if `tokio` feature is
    /// enabled.
    pub fn set_nonblocking(&self, _nonblocking: bool) -> std::io::Result<()> {
        #[cfg(not(feature = "tokio"))]
        self.stream.set_nonblocking(_nonblocking)?;
        Ok(())
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

    /// Reads a packet from stream.
    ///
    /// # Note
    ///
    /// If the encryption was not yet setup (i.e [`Packet::EncryptionResponse`] was not
    /// sent) and the stream is in a blocking mode then this function might not setup
    /// encryption correctly  
    pub fn read_packet(&mut self) -> std::io::Result<Packet> {
        if !self.read_packets.is_empty() {
            return Ok(self.get_one_packet());
        }
        if let Ok(enc) = self.enc_channel.1.try_recv() {
            self.encryption = enc
        }
        let data = self
            .read
            .try_read_data(&mut self.stream, &mut self.encryption)?;
        if let Ok(packet_type) = self.packettype_channel.1.try_recv() {
            self.packet_type = packet_type
        }
        #[cfg(feature = "ppac")]
        if let Some(writer) = &self.ppac {
            let direction = match self.direction {
                Direction::ToServer => Direction::ToClient,
                Direction::ToClient => Direction::ToServer,
            };
            let mut lock = writer.lock().unwrap();
            lock.write_data(crate::ppac::get_now(), direction, &data)?;
        }
        self.parse_packet(&data)
    }
    /// Reads a packet from stream.
    #[cfg(feature = "tokio")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
    pub async fn read_packet_async(&mut self) -> std::io::Result<Packet> {
        if !self.read_packets.is_empty() {
            return Ok(self.get_one_packet());
        }
        let data = loop {
            tokio::select! {
                result = self
                    .read
                    .read_data_async(&mut self.stream, &mut self.encryption) =>
                {
                    let data = result?;
                    break data;
                }

                Some(enc) = self.enc_channel.1.recv() => {
                    self.encryption = enc
                }

                Some(packet_type) = self.packettype_channel.1.recv() => {
                    self.packet_type = packet_type
                }
            }
        };
        #[cfg(feature = "ppac")]
        if let Some(writer) = &self.ppac {
            let direction = match self.direction {
                Direction::ToServer => Direction::ToClient,
                Direction::ToClient => Direction::ToServer,
            };
            let mut lock = writer.lock().unwrap();
            lock.write_data(crate::ppac::get_now(), direction, &data)?;
        }
        self.parse_packet(&data)
    }
    fn parse_packet(&mut self, data: &[u8]) -> std::io::Result<Packet> {
        let mut packets = Packet::read(data, self.packet_type)?;
        let mut packet = packets.remove(0);
        self.read_packets.append(&mut packets);
        if let Packet::EncryptionRequest(data) = &mut packet {
            if !matches!(&self.in_keyfile, PrivateKey::None) {
                let dec_data = Encryption::decrypt_rsa_data(&data.rsa_data, &self.in_keyfile)?;
                let (enc, dec) = Encryption::from_dec_data(
                    &dec_data,
                    matches!(self.packet_type, PacketType::NGS),
                )?
                .into_split();
                data.rsa_data = dec_data;
                let _ = self.enc_channel.0.send(enc);
                self.encryption = dec;
            }
        }
        Ok(packet)
    }

    /// Returns the encryption key (for [`Packet::EncryptionResponse`]).
    pub fn get_key(&mut self) -> Vec<u8> {
        if matches!(self.encryption, DecryptorType::None) {
            if let Ok(enc) = self.enc_channel.1.try_recv() {
                self.encryption = enc
            }
        }
        self.encryption.get_key()
    }
    fn get_one_packet(&mut self) -> Packet {
        self.read_packets.remove(0)
    }
}

#[cfg(feature = "split_connection")]
#[cfg_attr(docsrs, doc(cfg(feature = "split_connection")))]
impl ConnectionWrite {
    /// Returns the ip address of the client.
    pub fn get_ip(&self) -> std::io::Result<std::net::Ipv4Addr> {
        let ip = self.stream.peer_addr()?.ip();
        let ip = match ip {
            std::net::IpAddr::V4(x) => x,
            std::net::IpAddr::V6(_) => std::net::Ipv4Addr::UNSPECIFIED,
        };
        Ok(ip)
    }

    /// Changes connection type. Automatically changes the other side.
    pub fn change_packet_type(&mut self, packet_type: PacketType) {
        #[cfg(feature = "ppac")]
        if let Some(writer) = &mut self.ppac {
            let mut lock = writer.lock().unwrap();
            let _ = lock.change_packet_type(packet_type);
        }
        self.packet_type = packet_type;
        let _ = self.packettype_channel.0.send(packet_type);
    }

    /// Same as [`std::net::TcpStream::set_nonblocking`]. Does nothing if `tokio` feature is
    /// enabled.
    pub fn set_nonblocking(&self, _nonblocking: bool) -> std::io::Result<()> {
        #[cfg(not(feature = "tokio"))]
        self.stream.set_nonblocking(_nonblocking)?;
        Ok(())
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

    /// Sends a packet.
    ///
    /// # Note
    ///
    /// If `tokio` feature is enabled this function becomes nonblocking
    pub fn write_packet(&mut self, packet: &Packet) -> std::io::Result<()> {
        self.prepare_data(packet)?;
        self.write.flush(&mut self.stream)?;
        Ok(())
    }
    /// Sends a packet.
    #[cfg(feature = "tokio")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
    pub async fn write_packet_async(&mut self, packet: &Packet) -> std::io::Result<()> {
        self.prepare_data(packet)?;
        self.write.flush_async(&mut self.stream).await?;
        Ok(())
    }
    fn prepare_data(&mut self, packet: &Packet) -> std::io::Result<()> {
        if matches!(self.encryption, EncryptorType::None) {
            if let Ok(enc) = self.enc_channel.1.try_recv() {
                self.encryption = enc
            }
        }
        if let Ok(packet_type) = self.packettype_channel.1.try_recv() {
            self.packet_type = packet_type
        }
        let packet = packet.write(self.packet_type);
        #[cfg(feature = "ppac")]
        if let Some(writer) = &self.ppac {
            let mut lock = writer.lock().unwrap();
            lock.write_data(crate::ppac::get_now(), self.direction, &packet)?;
        }
        self.write.prepare_data(&packet, &mut self.encryption)?;

        Ok(())
    }

    /// Returns the encryption key (for [`Packet::EncryptionResponse`]).
    pub fn get_key(&mut self) -> Vec<u8> {
        if matches!(self.encryption, EncryptorType::None) {
            if let Ok(enc) = self.enc_channel.1.try_recv() {
                self.encryption = enc
            }
        }
        self.encryption.get_key()
    }
    /// Writes all pending packets.
    pub fn flush(&mut self) -> std::io::Result<()> {
        self.write.flush(&mut self.stream)
    }
    /// Writes all pending packets.
    #[cfg(feature = "tokio")]
    #[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
    pub async fn flush_async(&mut self) -> std::io::Result<()> {
        self.write.flush_async(&mut self.stream).await
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
