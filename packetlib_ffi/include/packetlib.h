#ifndef psopacketlib_ffi_h
#define psopacketlib_ffi_h

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

#define PLIB_API_VERSION 5

#define PLIB_PROTOCOL_VERSION 4

typedef enum PLIB_Direction {
  ToServer,
  ToClient,
} PLIB_Direction;

typedef enum PLIB_OutputType {
  /**
   * Output only parsed packet.
   */
  OutputPacket,
  /**
   * Output only raw packet.
   */
  OutputRaw,
  /**
   * Output both packets.
   */
  OutputBoth,
} PLIB_OutputType;

/**
 * Packet types.
 */
typedef enum PLIB_PacketType {
  NGS,
  Classic,
  NA,
  JP,
  Vita,
  Raw,
} PLIB_PacketType;

typedef enum PLIB_ReaderResult {
  Ok,
  RawOnly,
  ReaderEOF,
  PPACError,
} PLIB_ReaderResult;

/**
 * Serialized packet format
 */
typedef enum PLIB_SerializedFormat {
  JSON,
  MessagePack,
  MessagePackNamed,
} PLIB_SerializedFormat;

typedef enum PLIB_SocketResult {
  Ready,
  Blocked,
  NoSocket,
  SocketError,
} PLIB_SocketResult;

typedef struct PLIB_Connection PLIB_Connection;

typedef struct PLIB_PPACReader PLIB_PPACReader;

typedef struct PLIB_Packet PLIB_Packet;

typedef struct PLIB_PacketWorker PLIB_PacketWorker;

typedef struct PLIB_SocketFactory PLIB_SocketFactory;

/**
 * Fat pointer to data.
 */
typedef struct PLIB_DataBuffer {
  const uint8_t *ptr;
  size_t size;
} PLIB_DataBuffer;

typedef struct PLIB_PacketData {
  /**
   * When was the packet stored (in secs).
   */
  uint64_t time;
  /**
   * Where the packet was heading.
   */
  enum PLIB_Direction direction;
  /**
   * Which client version produced this packet.
   */
  enum PLIB_PacketType protocol_type;
  /**
   * Parsed packet (if requested)
   */
  struct PLIB_Packet *data;
  /**
   * Raw packet (if requested)
   */
  const uint8_t *raw_ptr;
  size_t raw_size;
} PLIB_PacketData;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

uint32_t get_api_version(void);

uint32_t get_protocol_version(void);

/**
 * Returns whether the library is built with connection support.
 */
bool have_connection(void);

/**
 * Returns whether the library is built with PPAC support.
 */
bool have_ppac(void);

/**
 * Creates a new packet worker.
 */
struct PLIB_PacketWorker *new_worker(enum PLIB_PacketType packet_type,
                                     enum PLIB_SerializedFormat serde_format);

/**
 * Destroys a packet worker.
 */
void free_worker(struct PLIB_PacketWorker *_worker);

/**
 * Destroys a packet.
 */
void free_packet(struct PLIB_Packet *_packet);

/**
 * Clones the packet.
 */
struct PLIB_Packet *clone_packet(const struct PLIB_Packet *packet);

/**
 * Checks if the packet is empty.
 */
bool packet_is_empty(const struct PLIB_Packet *packet);

/**
 * Sets a new packet type.
 */
void set_packet_type(struct PLIB_PacketWorker *worker, enum PLIB_PacketType packet_type);

/**
 * Sets a new serde format.
 */
void set_serde_format(struct PLIB_PacketWorker *worker, enum PLIB_SerializedFormat format);

/**
 * Checks if the specified serde format is supported.
 */
bool serde_supported(enum PLIB_SerializedFormat serde_format);

/**
 * Parses raw packet data and returns a [`Packet`] type or a null pointer if an error occured.
 *
 * # Safety
 * `data_ptr' must point to valid packet data up to `size` bytes.
 */
struct PLIB_Packet *raw_to_packet(struct PLIB_PacketWorker *worker,
                                  const uint8_t *data_ptr,
                                  size_t size);

/**
 * Parses serialized packet data and returns a [`Packet`] type or a null pointer if an error
 * occurred.
 *
 * # Safety
 * `data_ptr' must point to valid serialied data up to `size` bytes.
 */
struct PLIB_Packet *ser_to_packet(struct PLIB_PacketWorker *worker,
                                  const uint8_t *data_ptr,
                                  size_t size);

/**
 * Parses [`Packet`] and returns raw packet data.
 *
 * # Safety
 * The returned pointer is only valid until the next data-returning function call.
 * If the returned array is empty, the pointer might be non-null but still invalid. This is not
 * considered an error.
 */
struct PLIB_DataBuffer packet_to_raw(struct PLIB_PacketWorker *worker,
                                     const struct PLIB_Packet *packet);

/**
 * Parses [`Packet`] and returns serialized packet data or a null pointer if an error occured.
 *
 * # Safety
 * The returned pointer is only valid until the next data-returning function call.
 * If the returned array is empty, the pointer might be non-null but still invalid. This is not
 * considered an error.
 */
struct PLIB_DataBuffer packet_to_ser(struct PLIB_PacketWorker *worker,
                                     const struct PLIB_Packet *packet);

/**
 * Parses packet data and returns a fat pointer to the serialized packet or a null pointer if
 * an error occurred.
 *
 * # Safety
 * `data_ptr' must point to valid packet data up to `size` bytes.
 *
 * The returned pointer is only valid until the next data-returning function call.
 * If the returned array is empty, the pointer might be non-null but still invalid. This is not
 * considered an error.
 */
struct PLIB_DataBuffer parse_packet(struct PLIB_PacketWorker *worker,
                                    const uint8_t *data_ptr,
                                    size_t size);

/**
 * Deserializes packet and returns a fat pointer to the packet data or a null pointer if an error
 * occured.
 *
 * # Safety
 * `data_ptr' must point to valid packet data up to `size` bytes.
 *
 * The returned pointer is only valid until the next data-returning function call.
 * If the returned array is empty, the pointer might be non-null but still invalid. This is not
 * considered an error.
 */
struct PLIB_DataBuffer create_packet(struct PLIB_PacketWorker *worker,
                                     const uint8_t *data_ptr,
                                     size_t size);

/**
 * Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
 * occurred.
 *
 * # Safety
 * The returned pointer is only valid until the next failable function call.
 */
const uint8_t *get_pw_error(const struct PLIB_PacketWorker *worker);

/**
 * Creates a new socket factory.
 */
struct PLIB_SocketFactory *new_factory(void);

/**
 * Destroys a socket factory.
 */
void free_factory(struct PLIB_SocketFactory *_factory);

/**
 * Creates a new listener on the specified address.
 */
bool create_listener(struct PLIB_SocketFactory *factory, const int8_t *addr);

/**
 * Sets the blocking mode of the listener.
 */
void listener_nonblocking(const struct PLIB_SocketFactory *factory, bool nonblocking);

/**
 * Accepts a new incoming connection from installed listener. To collect the resulting connection
 * call `get_connection` or `stream_into_fd".
 */
enum PLIB_SocketResult accept_listener(struct PLIB_SocketFactory *factory);

/**
 * Creates a new stream to the specified address. To collect the resulting stream
 * call `get_connection` or `stream_into_fd".
 */
bool create_stream(struct PLIB_SocketFactory *factory, const int8_t *addr);

/**
 * Sets the blocking mode of the stream.
 */
void stream_nonblocking(struct PLIB_SocketFactory *factory, bool nonblocking);

/**
 * Returns the IP address of the stream.
 */
uint32_t get_stream_ip(const struct PLIB_SocketFactory *factory);

/**
 * Creates a new connection from incoming connection.
 *
 * # Safety
 * 'in_key' must either be null or it must point to a UTF-8-encoded, zero-terminated
 * path to a PKCS#8 file containing a private key for decryption.
 * 'out_key' must either be null or it must point to a UTF-8-encoded, zero-terminated
 * path to a PKCS#8 file containing a public key for encryption.
 */
struct PLIB_Connection *get_connection(struct PLIB_SocketFactory *factory,
                                       enum PLIB_PacketType packet_type,
                                       const int8_t *in_key,
                                       const int8_t *out_key);

/**
 * Returns an incoming connection descriptor. Caller is responsible for closing the returned descriptor.
 * If no stream was opened, returns -1.
 */
int64_t stream_into_fd(struct PLIB_SocketFactory *factory);

/**
 * Clones the descriptor. Returns the cloned descriptor or -1 if an error occurred.
 */
int64_t clone_fd(struct PLIB_SocketFactory *factory, int64_t fd);

/**
 * Closes the file descriptor.
 */
void close_fd(int64_t fd);

/**
 * Returns an owned socket descriptor. Caller is responsible for closing the returned descriptor.
 * If no listener was opened, returns -1.
 */
int64_t listener_into_fd(struct PLIB_SocketFactory *factory);

/**
 * Installs the provided listener. This function takes ownership of the descriptor.
 *
 * # Safety
 * `fd` must be a valid descriptor.
 */
bool listener_from_fd(struct PLIB_SocketFactory *factory, int64_t fd);

/**
 * Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
 * occurred.
 *
 * # Safety
 * The returned pointer is only valid until the next failable function call.
 */
const uint8_t *get_sf_error(const struct PLIB_SocketFactory *factory);

/**
 * Creates a new connection from owned socket descriptor.
 *
 * # Safety
 * `fd` must be a valid descriptor.
 *
 * 'in_key' must either be null or it must point to a UTF-8-encoded, zero-terminated
 * path to a PKCS#8 file containing a private key for decryption.
 * 'out_key' must either be null or it must point to a UTF-8-encoded, zero-terminated
 * path to a PKCS#8 file containing a public key for encryption.
 */
struct PLIB_Connection *new_connection(int64_t fd,
                                       enum PLIB_PacketType packet_type,
                                       const int8_t *in_key,
                                       const int8_t *out_key);

/**
 * Destroys a connection.
 */
void free_connection(struct PLIB_Connection *_conn);

/**
 * Returns the IP address of the connection.
 */
uint32_t get_conn_ip(const struct PLIB_Connection *conn);

/**
 * Changes the connection's packet type.
 */
void conn_set_packet_type(struct PLIB_Connection *conn, enum PLIB_PacketType packet_type);

/**
 * Returns a [`Packet`] or a null pointer if no connection was provided.
 *
 * # Safety
 * The returned pointer is only valid until the next data-returning function call.
 * If the returned array is empty, the pointer might be non-null but still invalid. This is not
 * considered an error.
 */
struct PLIB_Packet *conn_get_data(struct PLIB_Connection *conn);

/**
 * Reads a packet from the connection and stores it in the internal buffer. Call `conn_get_data`
 * to access it.
 */
enum PLIB_SocketResult conn_read_packet(struct PLIB_Connection *conn);

/**
 * Writes a packet to the connection. If `ptr` is null, flushes the buffer.
 *
 * # Note
 * If this function returns [`SocketResult::Blocked`], then the data has been written to the
 * buffer.
 */
enum PLIB_SocketResult conn_write_packet(struct PLIB_Connection *conn,
                                         const struct PLIB_Packet *packet);

/**
 * Returns the encryption key (for [`Packet::EncryptionResponse`]).
 */
struct PLIB_DataBuffer conn_get_key(struct PLIB_Connection *conn);

/**
 * Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
 * occurred.
 *
 * # Safety
 * The returned pointer is only valid until the next failable function call.
 */
const uint8_t *get_conn_error(const struct PLIB_Connection *conn);

/**
 * Creates a new PPAC reader. After creation don't forget to check for errors.
 */
struct PLIB_PPACReader *new_reader(const int8_t *path);

/**
 * Destroys the reader.
 */
void free_reader(struct PLIB_PPACReader *_reader);

/**
 * Sets the output type.
 */
void set_out_type(struct PLIB_PPACReader *reader, enum PLIB_OutputType out_type);

/**
 * Reads the packet and returns if the function succeeded.
 */
enum PLIB_ReaderResult read_packet(struct PLIB_PPACReader *reader);

/**
 * Returns a pointer to the packet data or a null pointer if no data exists.
 *
 * # Note
 * [`data`] field is only returned once and must be freed by the caller.
 *
 * # Safety
 * The returned pointer is only valid until the next data-returning function call.
 * If the returned array is empty, the pointer might be non-null but still invalid. This is not
 * considered an error.
 */
struct PLIB_PacketData get_reader_data(struct PLIB_PPACReader *reader);

/**
 * Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
 * occurred.
 *
 * # Safety
 * The returned pointer is only valid until the next failable function call.
 */
const uint8_t *get_reader_error(const struct PLIB_PPACReader *reader);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* psopacketlib_ffi_h */
