

from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from "packetlib.h":

  const uint32_t PLIB_API_VERSION # = 5

  const uint32_t PLIB_PROTOCOL_VERSION # = 4

  cdef enum PLIB_Direction:
    ToServer,
    ToClient,

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

  cdef enum PLIB_ReaderResult:
    Ok,
    RawOnly,
    ReaderEOF,
    PPACError,

  # Serialized packet format
  cdef enum PLIB_SerializedFormat:
    JSON,
    MessagePack,
    MessagePackNamed,

  cdef enum PLIB_SocketResult:
    Ready,
    Blocked,
    NoSocket,
    SocketError,

  cdef struct PLIB_Connection:
    pass

  cdef struct PLIB_PPACReader:
    pass

  cdef struct PLIB_Packet:
    pass

  cdef struct PLIB_PacketWorker:
    pass

  cdef struct PLIB_SocketFactory:
    pass

  # Fat pointer to data.
  cdef struct PLIB_DataBuffer:
    const uint8_t *ptr;
    size_t size;

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

  uint32_t get_api_version();

  uint32_t get_protocol_version();

  # Returns whether the library is built with connection support.
  bool have_connection();

  # Returns whether the library is built with PPAC support.
  bool have_ppac();

  # Creates a new packet worker.
  PLIB_PacketWorker *new_worker(PLIB_PacketType packet_type, PLIB_SerializedFormat serde_format);

  # Destroys a packet worker.
  void free_worker(PLIB_PacketWorker *_worker);

  # Destroys a packet.
  void free_packet(PLIB_Packet *_packet);

  # Clones the packet.
  PLIB_Packet *clone_packet(const PLIB_Packet *packet);

  # Checks if the packet is empty.
  bool packet_is_empty(const PLIB_Packet *packet);

  # Sets a new packet type.
  void set_packet_type(PLIB_PacketWorker *worker, PLIB_PacketType packet_type);

  # Sets a new serde format.
  void set_serde_format(PLIB_PacketWorker *worker, PLIB_SerializedFormat format);

  # Checks if the specified serde format is supported.
  bool serde_supported(PLIB_SerializedFormat serde_format);

  # Parses raw packet data and returns a [`Packet`] type or a null pointer if an error occured.
  #
  # # Safety
  # `data_ptr' must point to valid packet data up to `size` bytes.
  PLIB_Packet *raw_to_packet(PLIB_PacketWorker *worker, const uint8_t *data_ptr, size_t size);

  # Parses serialized packet data and returns a [`Packet`] type or a null pointer if an error
  # occurred.
  #
  # # Safety
  # `data_ptr' must point to valid serialied data up to `size` bytes.
  PLIB_Packet *ser_to_packet(PLIB_PacketWorker *worker, const uint8_t *data_ptr, size_t size);

  # Parses [`Packet`] and returns raw packet data.
  #
  # # Safety
  # The returned pointer is only valid until the next data-returning function call.
  # If the returned array is empty, the pointer might be non-null but still invalid. This is not
  # considered an error.
  PLIB_DataBuffer packet_to_raw(PLIB_PacketWorker *worker, const PLIB_Packet *packet);

  # Parses [`Packet`] and returns serialized packet data or a null pointer if an error occured.
  #
  # # Safety
  # The returned pointer is only valid until the next data-returning function call.
  # If the returned array is empty, the pointer might be non-null but still invalid. This is not
  # considered an error.
  PLIB_DataBuffer packet_to_ser(PLIB_PacketWorker *worker, const PLIB_Packet *packet);

  # Parses packet data and returns a fat pointer to the serialized packet or a null pointer if
  # an error occurred.
  #
  # # Safety
  # `data_ptr' must point to valid packet data up to `size` bytes.
  #
  # The returned pointer is only valid until the next data-returning function call.
  # If the returned array is empty, the pointer might be non-null but still invalid. This is not
  # considered an error.
  PLIB_DataBuffer parse_packet(PLIB_PacketWorker *worker, const uint8_t *data_ptr, size_t size);

  # Deserializes packet and returns a fat pointer to the packet data or a null pointer if an error
  # occured.
  #
  # # Safety
  # `data_ptr' must point to valid packet data up to `size` bytes.
  #
  # The returned pointer is only valid until the next data-returning function call.
  # If the returned array is empty, the pointer might be non-null but still invalid. This is not
  # considered an error.
  PLIB_DataBuffer create_packet(PLIB_PacketWorker *worker, const uint8_t *data_ptr, size_t size);

  # Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
  # occurred.
  #
  # # Safety
  # The returned pointer is only valid until the next failable function call.
  const uint8_t *get_pw_error(const PLIB_PacketWorker *worker);

  # Creates a new socket factory.
  PLIB_SocketFactory *new_factory();

  # Destroys a socket factory.
  void free_factory(PLIB_SocketFactory *_factory);

  # Creates a new listener on the specified address.
  bool create_listener(PLIB_SocketFactory *factory, const int8_t *addr);

  # Sets the blocking mode of the listener.
  void listener_nonblocking(const PLIB_SocketFactory *factory, bool nonblocking);

  # Accepts a new incoming connection from installed listener. To collect the resulting connection
  # call `get_connection` or `stream_into_fd".
  PLIB_SocketResult accept_listener(PLIB_SocketFactory *factory);

  # Creates a new stream to the specified address. To collect the resulting stream
  # call `get_connection` or `stream_into_fd".
  bool create_stream(PLIB_SocketFactory *factory, const int8_t *addr);

  # Sets the blocking mode of the stream.
  void stream_nonblocking(PLIB_SocketFactory *factory, bool nonblocking);

  # Returns the IP address of the stream.
  uint32_t get_stream_ip(const PLIB_SocketFactory *factory);

  # Creates a new connection from incoming connection.
  #
  # # Safety
  # 'in_key' must either be null or it must point to a UTF-8-encoded, zero-terminated
  # path to a PKCS#8 file containing a private key for decryption.
  # 'out_key' must either be null or it must point to a UTF-8-encoded, zero-terminated
  # path to a PKCS#8 file containing a public key for encryption.
  PLIB_Connection *get_connection(PLIB_SocketFactory *factory,
                                  PLIB_PacketType packet_type,
                                  const int8_t *in_key,
                                  const int8_t *out_key);

  # Returns an incoming connection descriptor. Caller is responsible for closing the returned descriptor.
  # If no stream was opened, returns -1.
  int64_t stream_into_fd(PLIB_SocketFactory *factory);

  # Clones the descriptor. Returns the cloned descriptor or -1 if an error occurred.
  int64_t clone_fd(PLIB_SocketFactory *factory, int64_t fd);

  # Closes the file descriptor.
  void close_fd(int64_t fd);

  # Returns an owned socket descriptor. Caller is responsible for closing the returned descriptor.
  # If no listener was opened, returns -1.
  int64_t listener_into_fd(PLIB_SocketFactory *factory);

  # Installs the provided listener. This function takes ownership of the descriptor.
  #
  # # Safety
  # `fd` must be a valid descriptor.
  bool listener_from_fd(PLIB_SocketFactory *factory, int64_t fd);

  # Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
  # occurred.
  #
  # # Safety
  # The returned pointer is only valid until the next failable function call.
  const uint8_t *get_sf_error(const PLIB_SocketFactory *factory);

  # Creates a new connection from owned socket descriptor.
  #
  # # Safety
  # `fd` must be a valid descriptor.
  #
  # 'in_key' must either be null or it must point to a UTF-8-encoded, zero-terminated
  # path to a PKCS#8 file containing a private key for decryption.
  # 'out_key' must either be null or it must point to a UTF-8-encoded, zero-terminated
  # path to a PKCS#8 file containing a public key for encryption.
  PLIB_Connection *new_connection(int64_t fd,
                                  PLIB_PacketType packet_type,
                                  const int8_t *in_key,
                                  const int8_t *out_key);

  # Destroys a connection.
  void free_connection(PLIB_Connection *_conn);

  # Returns the IP address of the connection.
  uint32_t get_conn_ip(const PLIB_Connection *conn);

  # Changes the connection's packet type.
  void conn_set_packet_type(PLIB_Connection *conn, PLIB_PacketType packet_type);

  # Returns a [`Packet`] or a null pointer if no connection was provided.
  #
  # # Safety
  # The returned pointer is only valid until the next data-returning function call.
  # If the returned array is empty, the pointer might be non-null but still invalid. This is not
  # considered an error.
  PLIB_Packet *conn_get_data(PLIB_Connection *conn);

  # Reads a packet from the connection and stores it in the internal buffer. Call `conn_get_data`
  # to access it.
  PLIB_SocketResult conn_read_packet(PLIB_Connection *conn);

  # Writes a packet to the connection. If `ptr` is null, flushes the buffer.
  #
  # # Note
  # If this function returns [`SocketResult::Blocked`], then the data has been written to the
  # buffer.
  PLIB_SocketResult conn_write_packet(PLIB_Connection *conn, const PLIB_Packet *packet);

  # Returns the encryption key (for [`Packet::EncryptionResponse`]).
  PLIB_DataBuffer conn_get_key(PLIB_Connection *conn);

  # Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
  # occurred.
  #
  # # Safety
  # The returned pointer is only valid until the next failable function call.
  const uint8_t *get_conn_error(const PLIB_Connection *conn);

  # Creates a new PPAC reader. After creation don't forget to check for errors.
  PLIB_PPACReader *new_reader(const int8_t *path);

  # Destroys the reader.
  void free_reader(PLIB_PPACReader *_reader);

  # Sets the output type.
  void set_out_type(PLIB_PPACReader *reader, PLIB_OutputType out_type);

  # Reads the packet and returns if the function succeeded.
  PLIB_ReaderResult read_packet(PLIB_PPACReader *reader);

  # Returns a pointer to the packet data or a null pointer if no data exists.
  #
  # # Note
  # [`data`] field is only returned once and must be freed by the caller.
  #
  # # Safety
  # The returned pointer is only valid until the next data-returning function call.
  # If the returned array is empty, the pointer might be non-null but still invalid. This is not
  # considered an error.
  PLIB_PacketData get_reader_data(PLIB_PPACReader *reader);

  # Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
  # occurred.
  #
  # # Safety
  # The returned pointer is only valid until the next failable function call.
  const uint8_t *get_reader_error(const PLIB_PPACReader *reader);
