

from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from "packetlib.h":

  # Current library version.
  const uint32_t PLIB_LIBRARY_VERSION # = 4

  # Packet direction.
  cdef enum PLIB_Direction:
    # Packet was sent to the server.
    ToServer,
    # Packet was sent to the client.
    ToClient,

  # Packet output format.
  cdef enum PLIB_OutputType:
    # Output only parsed packet.
    OutputPacket,
    # Output only raw packet.
    OutputRaw,
    # Output both packets.
    OutputBoth,

  # Packet types.
  cdef enum PLIB_PacketType:
    NGS,
    Classic,
    NA,
    JP,
    Vita,
    Raw,

  # Result of reader operations.
  cdef enum PLIB_ReaderResult:
    # Read operation was successful.
    Ok,
    # Only raw packet was read.
    RawOnly,
    # Reader reached end of file.
    ReaderEOF,
    # Reader produced an error, call [`get_reader_error`] to get an error message.
    PPACError,

  # Serialized packet format.
  cdef enum PLIB_SerializedFormat:
    # Packets are serialized in JSON format.
    JSON,
    # Packets are serialized in MessagePack format (all fields are arrays).
    MessagePack,
    # Packets are serialized in MessagePack format (fields are named).
    MessagePackNamed,

  # Result of socket operations.
  cdef enum PLIB_SocketResult:
    # Socket/Data is ready.
    Ready,
    # Socket would block.
    Blocked,
    # No socket is actually open.
    NoSocket,
    # Socket operation produced an error, call [`get_sf_error`] or [`get_conn_error`] to get an
    # error message.
    SocketError,

  # Connection between a client and a server.
  cdef struct PLIB_Connection:
    pass

  # PPAC archive reader.
  cdef struct PLIB_PPACReader:
    pass

  # Wrapper type for [`pso2packetlib::protocol::Packet`].
  cdef struct PLIB_Packet:
    pass

  # Factory for [`Packet`]. Handles error messages, stores packets and current serialization
  # format.
  cdef struct PLIB_PacketWorker:
    pass

  # Wrapper for [`pso2packetlib::PrivateKey`]
  cdef struct PLIB_PrivateKey:
    pass

  # Wrapper for [`pso2packetlib::PublicKey`]
  cdef struct PLIB_PublicKey:
    pass

  # Factory for [`Connection`]. Handles error messages, listen sockets and temporarily stores
  # accepted connections.
  cdef struct PLIB_SocketFactory:
    pass

  # Fat pointer to data.
  cdef struct PLIB_DataBuffer:
    const uint8_t *ptr;
    size_t size;
    # INTERNAL: vector capacity
    size_t _cap;

  # Read packet data
  cdef struct PLIB_PacketData:
    # When was the packet stored (in secs).
    uint64_t time;
    # Where the packet was heading.
    PLIB_Direction direction;
    # Which client version produced this packet.
    PLIB_PacketType protocol_type;
    # Parsed packet (if requested)
    PLIB_Packet *data;
    # Raw packet (if requested)
    const uint8_t *raw_ptr;
    size_t raw_size;

  # Returns the compiled library version.
  uint32_t get_library_version();

  # Returns whether the library is built with connection support.
  bool have_connection();

  # Returns whether the library is built with PPAC support.
  bool have_ppac();

  # Creates a new packet worker.
  #
  # # Safety
  # - `packet_type` must be a valid variant of `PacketType`.
  # - `serde_format` must be a valid variant of [`SerializedFormat`].
  PLIB_PacketWorker *new_worker(PLIB_PacketType packet_type, PLIB_SerializedFormat serde_format);

  # Destroys a packet worker.
  #
  # # Safety
  # `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
  void free_worker(PLIB_PacketWorker *_worker);

  # Destroys a packet.
  #
  # # Safety
  # `packet` must either be NULL or it must point to a valid [`Packet`] structure.
  void free_packet(PLIB_Packet *_packet);

  # Destroys a data pointer and deallocates pointed at memory.
  #
  # # Safety
  # - `data` must be a valid [`DataBuffer`] structure with valid data pointer.
  void free_data(PLIB_DataBuffer data);

  # Clones the data pointer.
  #
  # # Safety
  # - `data` must be a valid [`DataBuffer`] structure with valid data pointer.
  PLIB_DataBuffer clone_data(PLIB_DataBuffer data);

  # Clones the packet.
  #
  # # Safety
  # `packet` must either be NULL or it must point to a valid [`Packet`] structure.
  PLIB_Packet *clone_packet(const PLIB_Packet *packet);

  # Checks if the packet is empty.
  #
  # # Safety
  # `packet` must either be NULL or it must point to a valid [`Packet`] structure.
  bool packet_is_empty(const PLIB_Packet *packet);

  # Sets a new packet type.
  #
  # # Safety
  # - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
  # - `packet_type` must be a valid variant of `PacketType`.
  void set_packet_type(PLIB_PacketWorker *worker, PLIB_PacketType packet_type);

  # Sets a new serde format.
  #
  # # Safety
  # - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
  # - `format` must be a valid variant of [`SerializedFormat`].
  void set_serde_format(PLIB_PacketWorker *worker, PLIB_SerializedFormat format);

  # Checks if the specified serde format is supported.
  #
  # # Safety
  # `format` must be a valid variant of [`SerializedFormat`].
  bool serde_supported(PLIB_SerializedFormat serde_format);

  # Parses raw packet data and returns a [`Packet`] type or a null pointer if an error occured.
  #
  # # Safety
  # - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
  # - `data_ptr' must point to valid packet data up to `size` bytes.
  PLIB_Packet *raw_to_packet(PLIB_PacketWorker *worker, const uint8_t *data_ptr, size_t size);

  # Parses serialized packet data and returns a [`Packet`] type or a null pointer if an error
  # occurred.
  #
  # # Safety
  # - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
  # - `data_ptr' must point to valid serialied data up to `size` bytes.
  PLIB_Packet *ser_to_packet(PLIB_PacketWorker *worker, const uint8_t *data_ptr, size_t size);

  # Parses [`Packet`] and returns raw packet data.
  #
  # # Safety
  # - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
  # - If the returned array is empty, the pointer might be non-null but still invalid. This is not
  #   considered an error.
  PLIB_DataBuffer packet_to_raw(PLIB_PacketWorker *worker, const PLIB_Packet *packet);

  # Parses [`Packet`] and returns serialized packet data or a null pointer if an error occured.
  #
  # # Safety
  # - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
  # - If the returned array is empty, the pointer might be non-null but still invalid. This is not
  #   considered an error.
  PLIB_DataBuffer packet_to_ser(PLIB_PacketWorker *worker, const PLIB_Packet *packet);

  # Parses packet data and returns a fat pointer to the serialized packet or a null pointer if
  # an error occurred.
  #
  # # Safety
  # - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
  # - `data_ptr' must point to valid packet data up to `size` bytes.
  # - If the returned array is empty, the pointer might be non-null but still invalid. This is not
  #   considered an error.
  PLIB_DataBuffer parse_packet(PLIB_PacketWorker *worker, const uint8_t *data_ptr, size_t size);

  # Deserializes packet and returns a fat pointer to the packet data or a null pointer if an error
  # occured.
  #
  # # Safety
  # - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
  # - `data_ptr' must point to valid packet data up to `size` bytes.
  # - If the returned array is empty, the pointer might be non-null but still invalid. This is not
  #   considered an error.
  PLIB_DataBuffer create_packet(PLIB_PacketWorker *worker, const uint8_t *data_ptr, size_t size);

  # Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
  # occurred.
  #
  # # Safety
  # - `worker` must either be NULL or it must point to a valid [`PacketWorker`] structure.
  # - The returned pointer is only valid until the next failable function call.
  const uint8_t *get_pw_error(const PLIB_PacketWorker *worker);

  # Creates a new socket factory.
  PLIB_SocketFactory *new_factory();

  # Destroys a socket factory.
  #
  # # Safety
  # `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
  void free_factory(PLIB_SocketFactory *_factory);

  # Creates a new listener on the specified address.
  #
  # # Safety
  # - `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
  # - `addr` must be a valid NULL terminated string in the form of "ip:port"
  bool create_listener(PLIB_SocketFactory *factory, const int8_t *addr);

  # Sets the blocking mode of the listener.
  #
  # # Safety
  # `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
  void listener_nonblocking(const PLIB_SocketFactory *factory, bool nonblocking);

  # Accepts a new incoming connection from installed listener. To collect the resulting connection
  # call [`get_connection`] or [`stream_into_fd`].
  #
  # # Safety
  # `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
  PLIB_SocketResult accept_listener(PLIB_SocketFactory *factory);

  # Creates a new stream to the specified address. To collect the resulting stream
  # call [`get_connection`] or [`stream_into_fd`].
  #
  # # Safety
  # - `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
  # - `addr` must be a valid NULL terminated string in the form of "ip:port"
  bool create_stream(PLIB_SocketFactory *factory, const int8_t *addr);

  # Sets the blocking mode of the stream.
  #
  # # Safety
  # `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
  void stream_nonblocking(PLIB_SocketFactory *factory, bool nonblocking);

  # Returns the IP address of the stream.
  #
  # # Safety
  # `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
  uint32_t get_stream_ip(const PLIB_SocketFactory *factory);

  # Creates a new connection from incoming connection.
  #
  # # Safety
  # - `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
  # - `packet_type` must be a valid variant of `PacketType`.
  # - 'in_key' must either be null or it must point to a valid [`PrivateKey`] strucure
  # - 'out_key' must either be null or it must point to a valid [`PublicKey`] structure.
  #
  # # Note
  # This function takes ownership of `in_key` and `out_key`.
  PLIB_Connection *get_connection(PLIB_SocketFactory *factory,
                                  PLIB_PacketType packet_type,
                                  PLIB_PrivateKey *in_key,
                                  PLIB_PublicKey *out_key);

  # Returns an incoming connection descriptor. Caller is responsible for closing the returned descriptor.
  # If no stream was opened, returns -1.
  #
  # # Safety
  # `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
  int64_t stream_into_fd(PLIB_SocketFactory *factory);

  # Clones the descriptor. Returns the cloned descriptor or -1 if an error occurred.
  #
  # # Safety
  # - `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
  # - `fd` must be a valid descriptor.
  int64_t clone_fd(PLIB_SocketFactory *factory, int64_t fd);

  # Closes the file descriptor.
  #
  # # Safety
  # `fd` must be a valid descriptor.
  void close_fd(int64_t fd);

  # Returns an owned socket descriptor. Caller is responsible for closing the returned descriptor.
  # If no listener was opened, returns -1.
  #
  # # Safety
  # `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
  int64_t listener_into_fd(PLIB_SocketFactory *factory);

  # Installs the provided listener. This function takes ownership of the descriptor.
  #
  # # Safety
  # - `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
  # - `fd` must be a valid descriptor.
  #
  # # Notes
  # This function takes ownership of `fd`.
  bool listener_from_fd(PLIB_SocketFactory *factory, int64_t fd);

  # Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
  # occurred.
  #
  # # Safety
  # - `factory` must either be NULL or it must point to a valid [`SocketFactory`] structure.
  # - The returned pointer is only valid until the next failable function call.
  const uint8_t *get_sf_error(const PLIB_SocketFactory *factory);

  # Creates a new connection from owned socket descriptor.
  #
  # # Safety
  # - `fd` must be a valid descriptor.
  # - `packet_type` must be a valid variant of `PacketType`.
  # - 'in_key' must either be null or it must point to a valid [`PrivateKey`] strucure
  # - 'out_key' must either be null or it must point to a valid [`PublicKey`] structure.
  #
  # # Note
  # This function takes ownership of `in_key`, `out_key` and `fd`.
  PLIB_Connection *new_connection(int64_t fd,
                                  PLIB_PacketType packet_type,
                                  PLIB_PrivateKey *in_key,
                                  PLIB_PublicKey *out_key);

  # Destroys a connection.
  #
  # # Safety
  # `conn` must either be NULL or it must point to a valid [`Connection`] structure.
  void free_connection(PLIB_Connection *_conn);

  # Returns the IP address of the connection.
  #
  # # Safety
  # `conn` must either be NULL or it must point to a valid [`Connection`] structure.
  uint32_t get_conn_ip(const PLIB_Connection *conn);

  # Changes the connection's packet type.
  #
  # # Safety
  # - `conn` must either be NULL or it must point to a valid [`Connection`] structure.
  # - `packet_type` must be a valid variant of `PacketType`.
  void conn_set_packet_type(PLIB_Connection *conn, PLIB_PacketType packet_type);

  # Returns a [`Packet`] or a null pointer if no connection was provided.
  #
  # # Safety
  # - `conn` must either be NULL or it must point to a valid [`Connection`] structure.
  # - The returned pointer is only valid until the next data-returning function call.
  PLIB_Packet *conn_get_data(PLIB_Connection *conn);

  # Reads a packet from the connection and stores it in the internal buffer. Call [`conn_get_data`]
  # to access it.
  #
  # # Safety
  # `conn` must either be NULL or it must point to a valid [`Connection`] structure.
  PLIB_SocketResult conn_read_packet(PLIB_Connection *conn);

  # Writes a packet to the connection. If `ptr` is null, flushes the buffer.
  #
  # # Safety
  # - `conn` must either be NULL or it must point to a valid [`Connection`] structure.
  # - `packet` must either be NULL or it must point to a valid [`Packet`] structure.
  #
  # # Note
  # If this function returns [`SocketResult::Blocked`], then the data has been written to the
  # buffer.
  PLIB_SocketResult conn_write_packet(PLIB_Connection *conn, const PLIB_Packet *packet);

  # Returns the encryption key (for [`Packet::EncryptionResponse`]).
  #
  # # Safety
  # `conn` must either be NULL or it must point to a valid [`Connection`] structure.
  PLIB_DataBuffer conn_get_key(PLIB_Connection *conn);

  # Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
  # occurred.
  #
  # # Safety
  # - `conn` must either be NULL or it must point to a valid [`Connection`] structure.
  # - The returned pointer is only valid until the next failable function call.
  const uint8_t *get_conn_error(const PLIB_Connection *conn);

  # Creates a new public key from PEM-encoded PKCS#8 file.
  #
  # # Safety
  # `path` must either be NULL or it must point to a valid NULL terminated string.
  PLIB_PublicKey *new_pub_key_file(const int8_t *path);

  # Creates a new private key from PEM-encoded PKCS#8 file.
  #
  # # Safety
  # `path` must either be NULL or it must point to a valid NULL terminated string.
  PLIB_PrivateKey *new_priv_key_file(const int8_t *path);

  # Creates a new public key from RSA parameters.
  #
  # # Arguments
  # - `n` - RSA modulus
  # - `e` - RSA public exponent
  #
  # # Safety
  # - `n` must either be NULL or it must point to a valid byte array up to `n_size` bytes.
  # - `e` must either be NULL or it must point to a valid byte array up to `e_size` bytes.
  PLIB_PublicKey *new_pub_key_params(const uint8_t *n,
                                     size_t n_size,
                                     const uint8_t *e,
                                     size_t e_size);

  # Creates a new private key from RSA parameters.
  #
  # # Arguments
  # - `n` - RSA modulus
  # - `e` - RSA public exponent
  # - `d` - RSA private exponent
  # - `p` - RSA first prime
  # - `q` - RSA second prime
  #
  # # Safety
  # - `n` must either be NULL or it must point to a valid byte array up to `n_size` bytes.
  # - `e` must either be NULL or it must point to a valid byte array up to `e_size` bytes.
  # - `d` must either be NULL or it must point to a valid byte array up to `d_size` bytes.
  # - `p` must either be NULL or it must point to a valid byte array up to `p_size` bytes.
  # - `q` must either be NULL or it must point to a valid byte array up to `q_size` bytes.
  PLIB_PrivateKey *new_priv_key_params(const uint8_t *n,
                                       size_t n_size,
                                       const uint8_t *e,
                                       size_t e_size,
                                       const uint8_t *d,
                                       size_t d_size,
                                       const uint8_t *p,
                                       size_t p_size,
                                       const uint8_t *q,
                                       size_t q_size);

  # Destroys a public key
  #
  # # Safety
  # `key` must either be NULL or it must point to a valid [`PublicKey`] structure.
  void free_pub_key(PLIB_PublicKey *_key);

  # Destroys a private key
  #
  # # Safety
  # `key` must either be NULL or it must point to a valid [`PrivateKey`] structure.
  void free_priv_key(PLIB_PrivateKey *_key);

  # Creates a new PPAC reader. After creation don't forget to check for errors.
  #
  # # Safety
  # `path` must be a valid NULL terminated string.
  PLIB_PPACReader *new_reader(const int8_t *path);

  # Destroys the reader.
  #
  # # Safety
  # `reader` must either be NULL or it must point to a valid [`PPACReader`] structure.
  void free_reader(PLIB_PPACReader *_reader);

  # Sets the output type.
  #
  # # Safety
  # - `reader` must either be NULL or it must point to a valid [`PPACReader`] structure.
  # - `out_type` must be a valid variant of [`OutputType`].
  void set_out_type(PLIB_PPACReader *reader, PLIB_OutputType out_type);

  # Reads the packet and returns if the function succeeded.
  #
  # # Safety
  # `reader` must either be NULL or it must point to a valid [`PPACReader`] structure.
  PLIB_ReaderResult read_packet(PLIB_PPACReader *reader);

  # Returns a pointer to the packet data or a null pointer if no data exists.
  #
  # # Note
  # [`data`] field is only returned once and must be freed by the caller.
  #
  # # Safety
  # - `reader` must either be NULL or it must point to a valid [`PPACReader`] structure.
  # - If the returned array is empty, the pointer might be non-null but still invalid. This is not
  #   considered an error.
  PLIB_PacketData get_reader_data(PLIB_PPACReader *reader);

  # Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
  # occurred.
  #
  # # Safety
  # - `reader` must either be NULL or it must point to a valid [`PPACReader`] structure.
  # - The returned pointer is only valid until the next failable function call.
  const uint8_t *get_reader_error(const PLIB_PPACReader *reader);
