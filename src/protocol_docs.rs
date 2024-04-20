//! Documentation about the PSO2 protocol.

/// Information about NGS encryption.
///
/// ## Encryption handshake
/// ### Client
///
/// Packet encryption starts when the client sends an [`crate::protocol::Packet::EncryptionRequest`] packet.
/// Within this packet the data contains an RSA encrypted data blob with PKCS#1 v1.5 padding.
///
/// Decrypting this blob yields a block of data with the following structure:
///
/// | Offset | Size |                  Description                  |
/// |--------|------|-----------------------------------------------|
/// | 0x00   | 0x30 | AES256-CBC encrypted data with PKCS#7 padding |
/// | 0x30   | 0x20 | AES key for the encryption                    |
///
/// Decrypting the data with the provided key and an IV of
/// `[0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF]`
/// yields the encryption secret value.
///
/// ### Server
///
/// After parsing the above packet, server sets up the AES256-CBC encryption for all future
/// packets.
///
/// Encryption parameters:
/// - key - received from the client
/// - receive IV, send IV - initially
/// `[0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF]`,
/// after each packet it is updated to match the last 16 bytes of the packet (note: if the packet
/// is received then the receive IV is updated, if the packet is sent then the send IV is updated).
///
/// After that the server responds with the [`crate::protocol::Packet::EncryptionResponse`] packet
/// which contains the encryption secret AND the client's padding.
///
/// ## Encrypted packet structure
///
/// Every encrypted packet has the following structure:
///
/// | Offset |   Size   |                    Description                    |
/// |--------|----------|---------------------------------------------------|
/// | 0x00   | 0x20     | SHA256 of the header (i.e. from 0x00 to 0x48)     |
/// | 0x20   | 0x20     | SHA256 of the data (i.e. from 0x44 to the end)    |
/// | 0x40   | 0x4      | Unknown, always `[0x01, 0x00, 0xFF, 0xFF]`        |
/// | 0x44   | 0x4      | Packet length (hashes + unknown + length + data)  |
/// | 0x48   | Variable | AES256-CBC encrypted data with PKCS#7 padding     |
///
/// ## Data compression
///
/// NGS allows the packet data to be compressed. If after decryption the packet starts with `[0x28,
/// 0xB5, 0x2F, 0xFD]`, then this packet is compressed using ZSTD compression. When recompressing
/// set the pledged source size to the full packet size.
///
/// ## Calculating hashes
///
/// 1) Prepare the encrypted packet with zeroed hashes.
/// 2) Calculate the SHA256 of the encrypted packet from 0x44 to the end.
/// 3) Place the resulting hash at 0x20.
/// 4) Calculate the SHA256 of the encrypted packet from the start to 0x48.
/// 5) Place the resulting hash at 0x00.
pub enum NGSEncryption {}

/// Information about NA encryption.
///
/// ## Encryption handshake
/// ### Client
///
/// Packet encryption starts when the client sends an [`crate::protocol::Packet::EncryptionRequest`] packet.
/// Within this packet the data contains an RSA encrypted data blob with PKCS#1 v1.5 padding.
///
/// Decrypting this blob yields a block of data with the following structure:
///
/// | Offset | Size |                  Description                  |
/// |--------|------|-----------------------------------------------|
/// | 0x00   | 0x30 | AES256-CBC encrypted data with PKCS#7 padding |
/// | 0x30   | 0x20 | AES key for the encrypted data                |
///
/// Decrypting the data with the provided key and an IV of
/// `[0x0, 0x1, 0x2, 0x3, 0x4, 0x5, 0x6, 0x7, 0x8, 0x9, 0xA, 0xB, 0xC, 0xD, 0xE, 0xF]`
/// yields the encryption secret value.
///
/// ### Server
///
/// After parsing the above packet, server sets up the AES256-CBC encryption for all future
/// packets.
///
/// Encryption parameters:
/// - key - received from the client
/// - IV - stored in the encrypted packet.
///
/// After that the server responds with the [`crate::protocol::Packet::EncryptionResponse`] packet
/// which contains the encryption secret AND the client's padding.
///
/// ## Encrypted packet structure
///
/// Every encrypted packet has the following structure:
///
/// | Offset |   Size   |                    Description                         |
/// |--------|----------|--------------------------------------------------------|
/// | 0x00   | 0x20     | HMAC-SHA256 of the header (i.e. from 0x00 to 0x58)     |
/// | 0x20   | 0x20     | HMAC-SHA256 of the data (i.e. from 0x44 to the end)    |
/// | 0x40   | 0x4      | Unknown, always `[0x01, 0x00, 0xFF, 0xFF]`             |
/// | 0x44   | 0x4      | Packet length (hashes + unknown + length + data + IV)  |
/// | 0x48   | 0x10     | Packet AES IV                                          |
/// | 0x58   | Variable | AES256-CBC encrypted data with PKCS#7 padding          |
///
/// ## Calculating hashes
///
/// 1) Prepare the encrypted packet with zeroed hashes.
/// 2) Calculate the HMAC-SHA256 of the encrypted packet from 0x44 to the end with the key
///    `passwordxxxxxxxx`.
/// 3) Place the resulting hash at 0x20.
/// 4) Calculate the HMAC-SHA256 of the encrypted packet from the start to 0x58 with the key
///    `passwordxxxxxxxx`.
/// 5) Place the resulting hash at 0x00.
pub enum NAEncryption {}

/// Information about classic (Vita + JP) encryption.
///
/// ## Encryption handshake
/// ### Client
///
/// Packet encryption starts when the client sends an [`crate::protocol::Packet::EncryptionRequest`] packet.
/// Within this packet the data contains an RSA encrypted data blob with PKCS#1 v1.5 padding.
///
/// Decrypting this blob yields a block of data with the following structure:
///
/// | Offset | Size |        Description         |
/// |--------|------|----------------------------|
/// | 0x00   | 0x10 | RC4 encrypted data         |
/// | 0x10   | 0x10 | RC4 key for the encryption |
///
/// Decrypting the data with the provided key yields the encryption secret value.
///
/// ### Server
///
/// After parsing the above packet, server sets up the RC4 encryption for all future packets.
///
/// Encryption parameters:
/// - key - received from the client
///
/// After that the server responds with the [`crate::protocol::Packet::EncryptionResponse`] packet
/// which contains the encryption secret.
///
/// ## Encrypted packet structure
///
/// Because the RC4 encryption is a stream encryption the packet has the same structure as
/// regular packets, but with the keystream applied on top (including length int).
pub enum ClassicEncryption {}

/// Information about packet structure.
///
/// ## Packet structure
///
/// Every packet in the protocol has the following structure:
///
/// | Offset |   Size   |               Description                |
/// |--------|----------|------------------------------------------|
/// | 0x0    | 0x4      | Packet length (data + header + size int) |
/// | 0x4    | 0x4      | Packet header                            |
/// | 0x8    | Variable | Packet data                              |
///
/// ## Packet header structure [`crate::protocol::PacketHeader`]
///
/// Pre-NGS:
///
/// | Offset | Size |               Description               |
/// |--------|------|-----------------------------------------|
/// | 0x0    | 0x1  | Packet category (ID)                    |
/// | 0x1    | 0x1  | Packet ID in the category (SubID)       |
/// | 0x2    | 0x1  | Packet flags [`crate::protocol::Flags`] |
/// | 0x3    | 0x1  | Padding (?)                             |
///
/// NGS:
///
/// | Offset | Size |               Description               |
/// |--------|------|-----------------------------------------|
/// | 0x0    | 0x1  | Packet flags [`crate::protocol::Flags`] |
/// | 0x1    | 0x1  | Packet category (ID)                    |
/// | 0x2    | 0x2  | Packet ID in the category (SubID)       |
///
/// ## Variable length fields
///
/// Most variable length values are prefixed with their length using encoded [`u32`].
///
/// Values needed for encoding/decoding (varies from packet to packet):
/// - xor - ranges typically from 0x0000 to 0xFFFF
/// - sub - ranges typically from 0x000 to 0x100
///
/// Encoding formula:
/// ```
/// # let xor = 0x100;
/// # let sub = 0x20;
/// # let input = 1;
/// let output = (input + sub) ^ xor;
/// # assert_eq!(output, 0x121);
/// ```
///
/// Decoding formula:
/// ```
/// # let xor = 0x100;
/// # let sub = 0x20;
/// # let input = 0x121;
/// let output = (input ^ xor) - sub;
/// # assert_eq!(output, 1);
/// ```
pub enum PacketStructure {}
