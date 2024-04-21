use crate::encryption::{Decryptor, Encryptor, LengthType};
use std::io::{Read, Write};
use super::ConnectionError;

pub trait ConnReadAsync {
    fn readable_conn(&self) -> impl std::future::Future<Output = std::io::Result<()>> + Send;
    fn try_read_conn(&mut self, buf: &mut [u8]) -> std::io::Result<usize>;
}
pub trait ConnWriteAsync {
    fn writable_conn(&self) -> impl std::future::Future<Output = std::io::Result<()>> + Send;
    fn try_write_conn(&mut self, buf: &[u8]) -> std::io::Result<usize>;
}

impl ConnReadAsync for std::net::TcpStream {
    async fn readable_conn(&self) -> std::io::Result<()> {
        unimplemented!()
    }

    fn try_read_conn(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.read(buf)
    }
}
impl ConnWriteAsync for std::net::TcpStream {
    async fn writable_conn(&self) -> std::io::Result<()> {
        unimplemented!()
    }
    fn try_write_conn(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write(buf)
    }
}
#[cfg(feature = "tokio")]
impl ConnReadAsync for tokio::net::TcpStream {
    async fn readable_conn(&self) -> std::io::Result<()> {
        self.readable().await
    }
    fn try_read_conn(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.try_read(buf)
    }
}
#[cfg(feature = "tokio")]
impl ConnWriteAsync for tokio::net::TcpStream {
    async fn writable_conn(&self) -> std::io::Result<()> {
        self.writable().await
    }
    fn try_write_conn(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.try_write(buf)
    }
}
#[cfg(feature = "tokio")]
impl ConnReadAsync for tokio::net::tcp::OwnedReadHalf {
    async fn readable_conn(&self) -> std::io::Result<()> {
        self.readable().await
    }
    fn try_read_conn(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.try_read(buf)
    }
}
#[cfg(feature = "tokio")]
impl ConnWriteAsync for tokio::net::tcp::OwnedWriteHalf {
    async fn writable_conn(&self) -> std::io::Result<()> {
        self.writable().await
    }
    fn try_write_conn(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.try_write(buf)
    }
}

#[derive(Default, Debug)]
pub struct ConnectionReader {
    read_buffer: Vec<u8>,
    packet_length: usize,
}

#[derive(Default, Debug)]
pub struct ConnectionWriter {
    write_buffer: Vec<u8>,
}

impl ConnectionReader {
    /// Reads a packet data from stream.
    pub fn try_read_data(
        &mut self,
        stream: &mut (impl ConnReadAsync + Send),
        dec: &mut impl Decryptor,
    ) -> Result<Vec<u8>, ConnectionError> {
        if !self.read_buffer.is_empty() {
            if let Some(packet) = self.get_packet_data(dec)? {
                return Ok(packet);
            }
        }
        let mut buf = [0; 4096];
        loop {
            let read_bytes = stream.try_read_conn(&mut buf)?;
            if let Some(packet) = self.handle_data(dec, &buf[..read_bytes])? {
                return Ok(packet);
            }
        }
    }
    #[cfg(feature = "tokio")]
    pub async fn read_data_async(
        &mut self,
        stream: &mut (impl ConnReadAsync + Send),
        dec: &mut (impl Decryptor + Send),
    ) -> Result<Vec<u8>, ConnectionError> {
        if !self.read_buffer.is_empty() {
            if let Some(packet) = self.get_packet_data(dec)? {
                return Ok(packet);
            }
        }
        loop {
            stream.readable_conn().await?;
            let mut buf = [0; 4096];
            let read_bytes = match stream.try_read_conn(&mut buf) {
                Ok(0) => return Err(ConnectionError::Io(std::io::ErrorKind::ConnectionAborted.into())),
                Ok(n) => n,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
                Err(e) => return Err(e.into()),
            };
            if let Some(packet) = self.handle_data(dec, &buf[..read_bytes])? {
                return Ok(packet);
            }
        }
    }
    fn handle_data(
        &mut self,
        dec: &mut impl Decryptor,
        buf: &[u8],
    ) -> Result<Option<Vec<u8>>, ConnectionError> {
        if dec.is_rc4() {
            let mut decrypted_stream = dec.decrypt(buf)?;
            self.read_buffer.append(&mut decrypted_stream);
        } else {
            self.read_buffer.extend_from_slice(buf);
        }
        self.get_packet_data(dec)
    }
    fn get_packet_data(&mut self, dec: &mut impl Decryptor) -> Result<Option<Vec<u8>>, ConnectionError> {
        let mut output_data = vec![0u8; 0];
        if self.packet_length == 0 {
            self.get_length(dec);
        }
        if self.read_buffer.len() >= self.packet_length && self.packet_length != 0 {
            output_data.extend(self.read_buffer.drain(..self.packet_length));
            self.packet_length = 0;
            let output_data = if dec.is_rc4() {
                output_data
            } else {
                dec.decrypt(&output_data)?
            };
            return Ok(Some(output_data));
        }
        Ok(None)
    }
    fn get_length(&mut self, dec: &impl Decryptor) {
        let data = &self.read_buffer;
        let len_type = dec.get_len_type();
        let len = match (data.len(), len_type) {
            (0x48.., LengthType::Aes) => {
                u32::from_le_bytes(data[0x44..0x48].try_into().unwrap()) as usize
            }
            (0x4.., LengthType::Default) => {
                u32::from_le_bytes(data[0x0..0x4].try_into().unwrap()) as usize
            }
            _ => 0,
        };
        self.packet_length = len
    }
}

impl ConnectionWriter {
    pub fn prepare_data(&mut self, data: &[u8], enc: &mut impl Encryptor) -> Result<(), ConnectionError> {
        self.write_buffer.extend_from_slice(&enc.encrypt(data)?);
        Ok(())
    }
    pub fn flush(&mut self, stream: &mut (impl ConnWriteAsync + Send)) -> std::io::Result<()> {
        while !self.write_buffer.is_empty() {
            let wrote_bytes = stream.try_write_conn(&self.write_buffer)?;
            self.write_buffer.drain(..wrote_bytes).count();
        }
        Ok(())
    }
    #[cfg(feature = "tokio")]
    pub async fn flush_async(
        &mut self,
        stream: &mut (impl ConnWriteAsync + Send),
    ) -> std::io::Result<()> {
        while !self.write_buffer.is_empty() {
            stream.writable_conn().await?;
            let wrote_bytes = match stream.try_write_conn(&self.write_buffer) {
                Ok(n) => n,
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => 0,
                Err(e) => return Err(e),
            };
            self.write_buffer.drain(..wrote_bytes).count();
        }
        Ok(())
    }
}
