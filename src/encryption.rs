#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::connection::{PrivateKey, PublicKey};
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

/// Error type returned by encryption methods.
#[derive(Debug, thiserror::Error)]
pub enum EncryptionError {
    /// Error has occured in [`rsa`] functions.
    #[error("RSA error occured: {0}")]
    RSAError(#[from] rsa::errors::Error),
    /// No private key was provided for decryption.
    #[error("no private key provided")]
    NoPrivateKey,
    /// No public key was provided for encryption.
    #[error("no public key provided")]
    NoPublicKey,
    /// AES encryption padding failed.
    #[error("AES encryption padding failed")]
    PadError,
    /// AES decryption unpadding failed.
    #[error("AES decryption unpadding failed")]
    UnpadError,
    /// Error occured during ZSTD operations.
    #[error("error occured while performing ZSTD operations: {error}")]
    ZSTDError {
        #[source]
        error: std::io::Error,
    },
}

pub trait Encryptor {
    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, EncryptionError>;
}
pub trait Decryptor {
    fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, EncryptionError>;
    fn is_rc4(&self) -> bool {
        false
    }
    fn get_len_type(&self) -> LengthType;
}

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

pub enum LengthType {
    Default,
    Aes,
}

#[cfg(feature = "split_connection")]
#[derive(Debug, Default)]
pub enum EncryptorType {
    #[default]
    None,
    #[cfg(feature = "base_enc")]
    Aes(Aes),
    #[cfg(feature = "ngs_enc")]
    AesNgs(AesNgs),
    #[cfg(feature = "vita_enc")]
    Rc4(Rc4Enc),
}

#[cfg(feature = "split_connection")]
#[derive(Debug, Default)]
pub enum DecryptorType {
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
    pub fn decrypt_rsa_data(packet: &[u8], key: &PrivateKey) -> Result<Vec<u8>, EncryptionError> {
        #[cfg(any(feature = "base_enc", feature = "ngs_enc", feature = "vita_enc"))]
        let private_key = match key.into_key()? {
            Some(x) => x,
            None => {
                return Err(EncryptionError::NoPrivateKey);
            }
        };
        #[cfg(any(feature = "base_enc", feature = "ngs_enc", feature = "vita_enc"))]
        let dec_data = private_key.decrypt(Pkcs1v15Encrypt, packet)?;
        #[cfg(not(any(feature = "base_enc", feature = "ngs_enc", feature = "vita_enc")))]
        let dec_data = packet.to_vec();
        Ok(dec_data)
    }
    pub fn from_dec_data(data: &[u8], is_ngs: bool) -> Result<Self, EncryptionError> {
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
            aes.decrypt_padded_mut::<Pkcs7>(&mut key_d)
                .map_err(|_| EncryptionError::UnpadError)?;
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
    #[cfg(feature = "split_connection")]
    pub fn into_split(self) -> (EncryptorType, DecryptorType) {
        match self {
            Encryption::None => (EncryptorType::None, DecryptorType::None),
            #[cfg(feature = "base_enc")]
            Encryption::Aes(x) => (EncryptorType::Aes(x.clone()), DecryptorType::Aes(x)),
            #[cfg(feature = "ngs_enc")]
            Encryption::AesNgs(x) => (EncryptorType::AesNgs(x.clone()), DecryptorType::AesNgs(x)),
            #[cfg(feature = "vita_enc")]
            Encryption::Rc4((dec, enc)) => (EncryptorType::Rc4(enc), DecryptorType::Rc4(dec)),
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
}

impl Encryptor for Encryption {
    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
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
}
impl Decryptor for Encryption {
    fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
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
    fn is_rc4(&self) -> bool {
        match self {
            #[cfg(feature = "base_enc")]
            Self::Aes(x) => x.is_rc4(),
            #[cfg(feature = "ngs_enc")]
            Self::AesNgs(x) => x.is_rc4(),
            #[cfg(feature = "vita_enc")]
            Self::Rc4((x, _)) => x.is_rc4(),
            Self::None => false,
        }
    }
    fn get_len_type(&self) -> LengthType {
        match self {
            #[cfg(feature = "base_enc")]
            Self::Aes(_) => LengthType::Aes,
            #[cfg(feature = "ngs_enc")]
            Self::AesNgs(_) => LengthType::Aes,
            #[cfg(feature = "vita_enc")]
            Self::Rc4(_) => LengthType::Default,
            Self::None => LengthType::Default,
        }
    }
}

#[cfg(feature = "split_connection")]
impl EncryptorType {
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
}
#[cfg(feature = "split_connection")]
impl Encryptor for EncryptorType {
    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
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

#[cfg(feature = "split_connection")]
impl DecryptorType {
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
}

#[cfg(feature = "split_connection")]
impl Decryptor for DecryptorType {
    fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
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
    fn is_rc4(&self) -> bool {
        match self {
            #[cfg(feature = "base_enc")]
            Self::Aes(x) => x.is_rc4(),
            #[cfg(feature = "ngs_enc")]
            Self::AesNgs(x) => x.is_rc4(),
            #[cfg(feature = "vita_enc")]
            Self::Rc4(x) => x.is_rc4(),
            Self::None => false,
        }
    }
    fn get_len_type(&self) -> LengthType {
        match self {
            #[cfg(feature = "base_enc")]
            Self::Aes(_) => LengthType::Aes,
            #[cfg(feature = "ngs_enc")]
            Self::AesNgs(_) => LengthType::Aes,
            #[cfg(feature = "vita_enc")]
            Self::Rc4(_) => LengthType::Default,
            Self::None => LengthType::Default,
        }
    }
}

#[cfg(feature = "base_enc")]
#[derive(Debug, Clone)]
pub struct Aes {
    key: [u8; 0x20],
    secret: Vec<u8>,
}
#[cfg(feature = "base_enc")]
impl Decryptor for Aes {
    fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        let mut iv = [0u8; 0x10];
        iv.copy_from_slice(&data[0x48..0x58]);
        let aes = cbc::Decryptor::<aes::Aes256>::new(&self.key.into(), &iv.into());
        let mut data_copy = data[0x58..].to_vec();
        let plain_data = aes
            .decrypt_padded_mut::<Pkcs7>(&mut data_copy[..])
            .map_err(|x| EncryptionError::UnpadError)?;
        Ok(plain_data.to_vec())
    }
    fn get_len_type(&self) -> LengthType {
        LengthType::Aes
    }
}
#[cfg(feature = "base_enc")]
impl Encryptor for Aes {
    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        use hmac::Hmac;
        use hmac::Mac;
        use rand::RngCore;

        let mut iv = [0u8; 0x10];
        rand::thread_rng().fill_bytes(&mut iv);

        let mut out_data = vec![0u8; 0x40];
        out_data
            .write_u32::<BigEndian>(0x01_00_FF_FF)
            .expect("writing to Vec should not fail");

        let mut in_data = data.to_vec();
        let len = in_data.len();
        in_data.resize(len + 16, 0);

        let crypt_data = cbc::Encryptor::<aes::Aes256>::new(&self.key.into(), &iv.into())
            .encrypt_padded_mut::<Pkcs7>(&mut in_data, len)
            .map_err(|_| EncryptionError::PadError)?;

        out_data
            .write_u32::<LittleEndian>((crypt_data.len() + 0x58) as u32)
            .expect("writing to Vec should not fail");
        out_data.extend_from_slice(&iv);
        out_data.extend_from_slice(crypt_data);

        // hash calculation
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
impl Decryptor for AesNgs {
    fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        let mut next_iv = [0u8; 0x10];
        next_iv.copy_from_slice(&data[data.len() - 0x10..]);
        let aes = cbc::Decryptor::<aes::Aes256>::new(&self.key.into(), &self.iv_in.into());
        let mut data_copy = data[0x48..].to_vec();
        let plain_data = aes
            .decrypt_padded_mut::<Pkcs7>(&mut data_copy[..])
            .map_err(|x| EncryptionError::UnpadError)?;
        let mut ready_data = vec![];
        if plain_data[1..=3] == [0xb5, 0x2f, 0xfd] {
            let mut unpacked_data = zstd::stream::decode_all(plain_data)
                .map_err(|e| EncryptionError::ZSTDError { error: e })?;
            ready_data.append(&mut unpacked_data);
        } else {
            ready_data.append(&mut plain_data.to_vec());
        }
        self.iv_in = next_iv;
        Ok(ready_data)
    }
    fn get_len_type(&self) -> LengthType {
        LengthType::Aes
    }
}
#[cfg(feature = "ngs_enc")]
impl Encryptor for AesNgs {
    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
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

        let mut out_data = vec![0u8; 0x40];
        out_data
            .write_u32::<BigEndian>(0x01_00_FF_FF)
            .expect("writing to Vec should not fail");

        let len = tmp_data.len();
        tmp_data.resize(len + 16, 0);

        let crypt_data = cbc::Encryptor::<aes::Aes256>::new(&self.key.into(), &self.iv_out.into())
            .encrypt_padded_mut::<Pkcs7>(&mut tmp_data, len)
            .map_err(|_| EncryptionError::PadError)?;

        out_data
            .write_u32::<LittleEndian>((crypt_data.len() + 0x48) as u32)
            .expect("writing to Vec should not fail");
        out_data.extend_from_slice(crypt_data);

        // hash calculation
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
impl Encryptor for Rc4Enc {
    fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
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
impl Decryptor for Rc4Dec {
    fn decrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        use rc4::StreamCipher;
        let mut data = data.to_vec();
        self.decryptor.apply_keystream(&mut data);
        Ok(data)
    }
    fn is_rc4(&self) -> bool {
        true
    }
    fn get_len_type(&self) -> LengthType {
        LengthType::Default
    }
}
#[cfg(feature = "vita_enc")]
impl Debug for Rc4Dec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Rc4_Dec")
    }
}

#[cfg(any(feature = "base_enc", feature = "ngs_enc", feature = "vita_enc"))]
pub fn encrypt(packet: &[u8], out_key: &PublicKey) -> Result<Vec<u8>, EncryptionError> {
    let out_key = match out_key.into_key()? {
        Some(x) => x,
        None => {
            return Err(EncryptionError::NoPublicKey);
        }
    };
    let enc_data = out_key.encrypt(&mut rand::thread_rng(), Pkcs1v15Encrypt, packet)?;
    Ok(enc_data)
}

#[cfg(not(any(feature = "base_enc", feature = "ngs_enc", feature = "vita_enc")))]
pub fn encrypt(packet: &[u8], _: &PublicKey) -> Result<Vec<u8>, EncryptionError> {
    Ok(packet.to_vec())
}
