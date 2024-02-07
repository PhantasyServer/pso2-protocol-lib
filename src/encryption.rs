#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::PrivateKey;
#[cfg(feature = "proxy")]
use crate::PublicKey;
#[cfg(any(feature = "base_enc", feature = "ngs_enc"))]
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
#[cfg(any(feature = "base_enc", feature = "ngs_enc"))]
use byteorder::{BigEndian, LittleEndian, WriteBytesExt};
#[cfg(feature = "vita_enc")]
use rc4::{consts::U16, Rc4};
#[cfg(any(feature = "base_enc", feature = "ngs_enc", feature = "vita_enc"))]
use rsa::Pkcs1v15Encrypt;
#[cfg(any(feature = "base_enc", feature = "ngs_enc"))]
use sha2::Sha256;
use std::{
    fmt::Debug,
    io::{Error, ErrorKind},
};

#[derive(Debug, Default)]
pub enum Encryption {
    #[default]
    None,
    #[cfg(feature = "base_enc")]
    Aes(Aes),
    #[cfg(feature = "ngs_enc")]
    AesNgs(AesNgs),
    #[cfg(feature = "vita_enc")]
    Rc4((Rc4Dec, Rc4Enc)),
}

#[cfg(feature = "tokio")]
#[derive(Debug, Default)]
pub enum Encryptor {
    #[default]
    None,
    #[cfg(feature = "base_enc")]
    Aes(Aes),
    #[cfg(feature = "ngs_enc")]
    AesNgs(AesNgs),
    #[cfg(feature = "vita_enc")]
    Rc4(Rc4Enc),
}

#[cfg(feature = "tokio")]
#[derive(Debug, Default)]
pub enum Decryptor {
    #[default]
    None,
    #[cfg(feature = "base_enc")]
    Aes(Aes),
    #[cfg(feature = "ngs_enc")]
    AesNgs(AesNgs),
    #[cfg(feature = "vita_enc")]
    Rc4(Rc4Dec),
}

impl Encryption {
    pub fn from_rsa_data(packet: &[u8], is_ngs: bool, key: &PrivateKey) -> std::io::Result<Self> {
        #[cfg(any(feature = "base_enc", feature = "ngs_enc", feature = "vita_enc"))]
        let private_key = match key.into_key() {
            Ok(Some(x)) => x,
            Ok(None) => {
                return Err(Error::new(ErrorKind::Other, "No key provided".to_string()));
            }
            Err(x) => {
                return Err(Error::new(ErrorKind::Other, format!("{x}")));
            }
        };
        #[cfg(any(feature = "base_enc", feature = "ngs_enc", feature = "vita_enc"))]
        let dec_data = match private_key.decrypt(Pkcs1v15Encrypt, packet) {
            Ok(x) => x,
            Err(x) => {
                return Err(Error::new(ErrorKind::Other, format!("{x}")));
            }
        };
        #[cfg(not(any(feature = "base_enc", feature = "ngs_enc", feature = "vita_enc")))]
        let dec_data = packet;
        Self::from_dec_data(&dec_data, is_ngs)
    }
    fn from_dec_data(data: &[u8], is_ngs: bool) -> Result<Self, Error> {
        #[cfg(any(feature = "base_enc", feature = "ngs_enc"))]
        if data.len() > 0x30 {
            let mut iv: [u8; 0x10] = [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D,
                0x0E, 0x0F,
            ];
            let mut key = [0u8; 32];
            let mut key_d = [0u8; 0x30];
            key_d.copy_from_slice(&data[0x0..0x30]);
            key.copy_from_slice(&data[0x30..0x50]);
            let aes = cbc::Decryptor::<aes::Aes256>::new(&key.into(), &iv.into());
            match aes.decrypt_padded_mut::<Pkcs7>(&mut key_d) {
                Ok(_) => {}
                Err(x) => {
                    return Err(Error::new(ErrorKind::Other, format!("{x}")));
                }
            }
            iv.copy_from_slice(&key_d[0x00..0x10]);
            if is_ngs {
                #[cfg(feature = "ngs_enc")]
                return Ok(Self::AesNgs(AesNgs {
                    iv_in: iv,
                    iv_out: iv,
                    key,
                    secret: key_d.to_vec(),
                }));
            } else {
                #[cfg(feature = "base_enc")]
                return Ok(Self::Aes(Aes {
                    key,
                    secret: key_d.to_vec(),
                }));
            }
        }
        #[cfg(feature = "vita_enc")]
        if data.len() <= 0x30 {
            use rc4::{KeyInit, StreamCipher};
            let mut rc4_key = [0u8; 0x10];
            let mut secret = [0u8; 0x10];
            rc4_key.clone_from_slice(&data[0x10..0x20]);
            secret.clone_from_slice(&data[0x00..0x10]);
            let mut tmp_dec = Rc4::new(&rc4_key.into());
            tmp_dec.apply_keystream(&mut secret);
            return Ok(Self::Rc4((
                Rc4Dec {
                    decryptor: Box::new(Rc4::new(&rc4_key.into())),
                    secret,
                },
                Rc4Enc {
                    encryptor: Box::new(Rc4::new(&rc4_key.into())),
                    secret,
                },
            )));
        }
        Ok(Self::None)
    }
    #[cfg(feature = "tokio")]
    pub fn into_split(self) -> (Encryptor, Decryptor) {
        match self {
            Encryption::None => (Encryptor::None, Decryptor::None),
            #[cfg(feature = "base_enc")]
            Encryption::Aes(x) => (Encryptor::Aes(x.clone()), Decryptor::Aes(x)),
            #[cfg(feature = "ngs_enc")]
            Encryption::AesNgs(x) => (Encryptor::AesNgs(x.clone()), Decryptor::AesNgs(x)),
            #[cfg(feature = "vita_enc")]
            Encryption::Rc4((dec, enc)) => (Encryptor::Rc4(enc), Decryptor::Rc4(dec)),
        }
    }
    pub fn get_key(&self) -> Vec<u8> {
        match self {
            Self::None => Vec::new(),
            #[cfg(feature = "base_enc")]
            Self::Aes(x) => x.secret.clone(),
            #[cfg(feature = "ngs_enc")]
            Self::AesNgs(x) => x.secret.clone(),
            #[cfg(feature = "vita_enc")]
            Self::Rc4((x, _)) => x.secret.to_vec(),
        }
    }
    pub fn decrypt(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        if data.is_empty() {
            return Ok(vec![]);
        }
        match self {
            #[cfg(feature = "base_enc")]
            Self::Aes(x) => x.decrypt(data),
            #[cfg(feature = "ngs_enc")]
            Self::AesNgs(x) => x.decrypt(data),
            #[cfg(feature = "vita_enc")]
            Self::Rc4((x, _)) => x.decrypt(data),
            Self::None => Ok(data.to_vec()),
        }
    }
    pub fn encrypt(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        if data.is_empty() {
            return Ok(vec![]);
        }
        match self {
            #[cfg(feature = "base_enc")]
            Self::Aes(x) => x.encrypt(data),
            #[cfg(feature = "ngs_enc")]
            Self::AesNgs(x) => x.encrypt(data),
            #[cfg(feature = "vita_enc")]
            Self::Rc4((_, x)) => x.encrypt(data),
            Self::None => Ok(data.to_vec()),
        }
    }
    pub fn is_rc4(&self) -> bool {
        #[cfg(feature = "vita_enc")]
        return matches!(self, Self::Rc4(_));
        #[cfg(not(feature = "vita_enc"))]
        false
    }
}

#[cfg(feature = "tokio")]
impl Encryptor {
    pub fn get_key(&self) -> Vec<u8> {
        match self {
            Self::None => Vec::new(),
            #[cfg(feature = "base_enc")]
            Self::Aes(x) => x.secret.clone(),
            #[cfg(feature = "ngs_enc")]
            Self::AesNgs(x) => x.secret.clone(),
            #[cfg(feature = "vita_enc")]
            Self::Rc4(x) => x.secret.to_vec(),
        }
    }
    pub fn encrypt(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        if data.is_empty() {
            return Ok(vec![]);
        }
        match self {
            #[cfg(feature = "base_enc")]
            Self::Aes(x) => x.encrypt(data),
            #[cfg(feature = "ngs_enc")]
            Self::AesNgs(x) => x.encrypt(data),
            #[cfg(feature = "vita_enc")]
            Self::Rc4(x) => x.encrypt(data),
            Self::None => Ok(data.to_vec()),
        }
    }
}

#[cfg(feature = "tokio")]
impl Decryptor {
    pub fn get_key(&self) -> Vec<u8> {
        match self {
            Self::None => Vec::new(),
            #[cfg(feature = "base_enc")]
            Self::Aes(x) => x.secret.clone(),
            #[cfg(feature = "ngs_enc")]
            Self::AesNgs(x) => x.secret.clone(),
            #[cfg(feature = "vita_enc")]
            Self::Rc4(x) => x.secret.to_vec(),
        }
    }
    pub fn decrypt(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        if data.is_empty() {
            return Ok(vec![]);
        }
        match self {
            #[cfg(feature = "base_enc")]
            Self::Aes(x) => x.decrypt(data),
            #[cfg(feature = "ngs_enc")]
            Self::AesNgs(x) => x.decrypt(data),
            #[cfg(feature = "vita_enc")]
            Self::Rc4(x) => x.decrypt(data),
            Self::None => Ok(data.to_vec()),
        }
    }
    pub fn is_rc4(&self) -> bool {
        #[cfg(feature = "vita_enc")]
        return matches!(self, Self::Rc4(_));
        #[cfg(not(feature = "vita_enc"))]
        false
    }
}

#[cfg(feature = "base_enc")]
#[derive(Debug, Clone)]
pub struct Aes {
    key: [u8; 0x20],
    secret: Vec<u8>,
}
#[cfg(feature = "base_enc")]
impl Aes {
    fn decrypt(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut iv = [0u8; 0x10];
        iv.copy_from_slice(&data[0x48..0x58]);
        let aes = cbc::Decryptor::<aes::Aes256>::new(&self.key.into(), &iv.into());
        let mut data_copy = data[0x58..].to_vec();
        let plain_data = match aes.decrypt_padded_mut::<Pkcs7>(&mut data_copy[..]) {
            Ok(x) => x,
            Err(x) => {
                return Err(Error::new(ErrorKind::Other, format!("{x}")));
            }
        };
        Ok(plain_data.to_vec())
    }
    fn encrypt(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        use hmac::Hmac;
        use hmac::Mac;
        use rand::RngCore;

        let mut iv = [0u8; 0x10];
        rand::thread_rng().fill_bytes(&mut iv);
        let aes = cbc::Encryptor::<aes::Aes256>::new(&self.key.into(), &iv.into());
        let mut out_data = vec![0u8; 0x40];
        out_data.write_u32::<BigEndian>(0x01_00_FF_FF)?;
        let mut in_data = data.to_vec();
        let len = in_data.len();
        in_data.resize(len + 16, 0);
        let crypt_data = match aes.encrypt_padded_mut::<Pkcs7>(&mut in_data, len) {
            Ok(x) => x,
            Err(x) => {
                return Err(Error::new(ErrorKind::Other, format!("{x}")));
            }
        };
        out_data.write_u32::<LittleEndian>((crypt_data.len() + 0x58) as u32)?;
        out_data.extend_from_slice(&iv);
        out_data.extend_from_slice(crypt_data);
        let mut sha = Hmac::<Sha256>::new_from_slice(b"passwordxxxxxxxx").unwrap();
        sha.update(&out_data[0x44..]);
        out_data[0x20..0x40].copy_from_slice(&sha.finalize().into_bytes().to_vec()[..]);
        let mut sha = Hmac::<Sha256>::new_from_slice(b"passwordxxxxxxxx").unwrap();
        sha.update(&out_data[..0x58]);
        out_data[..0x20].copy_from_slice(&sha.finalize().into_bytes().to_vec()[..]);
        Ok(out_data)
    }
}

#[cfg(feature = "ngs_enc")]
#[derive(Debug, Clone)]
pub struct AesNgs {
    iv_in: [u8; 0x10],
    iv_out: [u8; 0x10],
    key: [u8; 0x20],
    secret: Vec<u8>,
}
#[cfg(feature = "ngs_enc")]
impl AesNgs {
    fn decrypt(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        let mut next_iv = [0u8; 0x10];
        next_iv.copy_from_slice(&data[data.len() - 0x10..]);
        let aes = cbc::Decryptor::<aes::Aes256>::new(&self.key.into(), &self.iv_in.into());
        let mut data_copy = data[0x48..].to_vec();
        let plain_data = match aes.decrypt_padded_mut::<Pkcs7>(&mut data_copy[..]) {
            Ok(x) => x,
            Err(e) => {
                return Err(Error::new(ErrorKind::Other, format!("{e}")));
            }
        };
        let mut ready_data = vec![];
        if plain_data[1..=3] == [0xb5, 0x2f, 0xfd] {
            match zstd::stream::decode_all(plain_data) {
                Ok(ref mut unpacked_data) => ready_data.append(unpacked_data),
                Err(e) => return Err(Error::new(ErrorKind::Other, format!("{e}"))),
            };
        } else {
            ready_data.append(&mut plain_data.to_vec());
        }
        self.iv_in = next_iv;
        Ok(ready_data)
    }
    fn encrypt(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        use sha2::Digest;
        let mut tmp_data = vec![];
        // if enc {
        //     match zstd_enc(data) {
        //         Ok(ref mut packed_data) => tmp_data.append(packed_data),
        //         Err(e) => return Err(Error::new(ErrorKind::Other, format!("{e}"))),
        //     }
        // } else {
        tmp_data.extend_from_slice(data);
        // }

        let aes = cbc::Encryptor::<aes::Aes256>::new(&self.key.into(), &self.iv_out.into());
        let mut out_data = vec![0u8; 0x40];
        out_data.write_u32::<BigEndian>(0x01_00_FF_FF)?;
        let len = tmp_data.len();
        tmp_data.resize(len + 16, 0);
        let crypt_data = match aes.encrypt_padded_mut::<Pkcs7>(&mut tmp_data, len) {
            Ok(x) => x,
            Err(x) => {
                return Err(Error::new(ErrorKind::Other, format!("{x}")));
            }
        };
        out_data.write_u32::<LittleEndian>((crypt_data.len() + 0x48) as u32)?;
        out_data.extend_from_slice(crypt_data);

        let mut sha_hasher = Sha256::new();
        sha_hasher.update(&out_data[0x44..]);
        out_data[0x20..0x40].copy_from_slice(&sha_hasher.finalize());
        let mut sha_hasher = Sha256::new();
        sha_hasher.update(&out_data[..0x48]);
        out_data[..0x20].copy_from_slice(&sha_hasher.finalize());
        self.iv_out
            .copy_from_slice(&out_data[out_data.len() - 0x10..]);
        Ok(out_data)
    }
}

#[cfg(feature = "vita_enc")]
pub struct Rc4Enc {
    encryptor: Box<Rc4<U16>>,
    secret: [u8; 0x10],
}
#[cfg(feature = "vita_enc")]
impl Rc4Enc {
    fn encrypt(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        use rc4::StreamCipher;
        let mut data = data.to_vec();
        self.encryptor.apply_keystream(&mut data);
        Ok(data)
    }
}
#[cfg(feature = "vita_enc")]
impl Debug for Rc4Enc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rc4_Enc")
    }
}

#[cfg(feature = "vita_enc")]
pub struct Rc4Dec {
    decryptor: Box<Rc4<U16>>,
    secret: [u8; 0x10],
}
#[cfg(feature = "vita_enc")]
impl Rc4Dec {
    fn decrypt(&mut self, data: &[u8]) -> std::io::Result<Vec<u8>> {
        use rc4::StreamCipher;
        let mut data = data.to_vec();
        self.decryptor.apply_keystream(&mut data);
        Ok(data)
    }
}
#[cfg(feature = "vita_enc")]
impl Debug for Rc4Dec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rc4_Dec")
    }
}

#[cfg(all(
    any(feature = "base_enc", feature = "ngs_enc", feature = "vita_enc"),
    feature = "proxy"
))]
pub fn reencrypt(
    packet: &[u8],
    in_key: &PrivateKey,
    out_key: &PublicKey,
) -> std::io::Result<Vec<u8>> {
    let private_key = match in_key.into_key() {
        Ok(Some(x)) => x,
        Ok(None) => {
            return Err(Error::new(ErrorKind::Other, "No key provided".to_string()));
        }
        Err(x) => {
            return Err(Error::new(ErrorKind::Other, format!("{x}")));
        }
    };
    let dec_data = match private_key.decrypt(Pkcs1v15Encrypt, packet) {
        Ok(x) => x,
        Err(x) => {
            return Err(Error::new(ErrorKind::Other, format!("{x}")));
        }
    };
    let out_key = match out_key.into_key() {
        Ok(Some(x)) => x,
        Ok(None) => {
            return Err(Error::new(ErrorKind::Other, "No key provided".to_string()));
        }
        Err(x) => {
            return Err(Error::new(ErrorKind::Other, format!("{x}")));
        }
    };
    let enc_data = match out_key.encrypt(&mut rand::thread_rng(), Pkcs1v15Encrypt, &dec_data) {
        Ok(x) => x,
        Err(x) => {
            return Err(Error::new(ErrorKind::Other, format!("{x}")));
        }
    };
    Ok(enc_data)
}

#[cfg(all(
    not(any(feature = "base_enc", feature = "ngs_enc", feature = "vita_enc")),
    feature = "proxy"
))]
pub fn reencrypt(packet: &[u8], _: &PrivateKey, _: &PublicKey) -> std::io::Result<Vec<u8>> {
    Ok(packet.to_vec())
}
