

from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  const uint32_t API_VERSION # = 5

  const uint32_t PROTOCOL_VERSION # = 4

  cdef enum Direction:
    ToServer,
    ToClient,

  cdef enum OutputType:
    # Output only parsed packet.
    OutputPacket,
    # Output only raw packet.
    OutputRaw,
    # Output both packets.
    OutputBoth,

  # Packet types.
  cdef enum PacketType:
    NGS,
    Classic,
    NA,
    JP,
    Vita,
    Raw,

  cdef enum ReaderResult:
    Ok,
    RawOnly,
    ReaderEOF,
    PPACError,

  # Serialized packet format
  cdef enum SerializedFormat:
    JSON,
    MessagePack,
    MessagePackNamed,

  cdef enum SocketResult:
    Ready,
    Blocked,
    NoSocket,
    SocketError,

  cdef struct Connection:
    pass

  cdef struct PPACReader:
    pass

  cdef struct Packet:
    pass

  cdef struct PacketWorker:
    pass

  cdef struct SocketFactory:
    pass

  # Fat pointer to data.
  cdef struct DataBuffer:
    const uint8_t *ptr;
    size_t size;

  cdef struct PacketData:
    # When was the packet stored (in secs).
    uint64_t time;
    # Where the packet was heading.
    Direction direction;
    # Which client version produced this packet.
    PacketType protocol_type;
    # Parsed packet (if requested)
    Packet *data;
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
  PacketWorker *new_worker(PacketType packet_type, SerializedFormat serde_format);

  # Destroys a packet worker.
  void free_worker(PacketWorker *_worker);

  # Destroys a packet.
  void free_packet(Packet *_packet);

  # Clones the packet.
  Packet *clone_packet(const Packet *packet);

  # Checks if the packet is empty.
  bool packet_is_empty(const Packet *packet);

  # Sets a new packet type.
  void set_packet_type(PacketWorker *worker, PacketType packet_type);

  # Sets a new serde format.
  void set_serde_format(PacketWorker *worker, SerializedFormat format);

  # Checks if the specified serde format is supported.
  bool serde_supported(SerializedFormat serde_format);

  # Parses raw packet data and returns a [`Packet`] type or a null pointer if an error occured.
  #
  # # Safety
  # `data_ptr' must point to valid packet data up to `size` bytes.
  Packet *raw_to_packet(PacketWorker *worker, const uint8_t *data_ptr, size_t size);

  # Parses serialized packet data and returns a [`Packet`] type or a null pointer if an error
  # occurred.
  #
  # # Safety
  # `data_ptr' must point to valid serialied data up to `size` bytes.
  Packet *ser_to_packet(PacketWorker *worker, const uint8_t *data_ptr, size_t size);

  # Parses [`Packet`] and returns raw packet data.
  #
  # # Safety
  # The returned pointer is only valid until the next data-returning function call.
  # If the returned array is empty, the pointer might be non-null but still invalid. This is not
  # considered an error.
  DataBuffer packet_to_raw(PacketWorker *worker, const Packet *packet);

  # Parses [`Packet`] and returns serialized packet data or a null pointer if an error occured.
  #
  # # Safety
  # The returned pointer is only valid until the next data-returning function call.
  # If the returned array is empty, the pointer might be non-null but still invalid. This is not
  # considered an error.
  DataBuffer packet_to_ser(PacketWorker *worker, const Packet *packet);

  # Parses packet data and returns a fat pointer to the serialized packet or a null pointer if
  # an error occurred.
  #
  # # Safety
  # `data_ptr' must point to valid packet data up to `size` bytes.
  #
  # The returned pointer is only valid until the next data-returning function call.
  # If the returned array is empty, the pointer might be non-null but still invalid. This is not
  # considered an error.
  DataBuffer parse_packet(PacketWorker *worker, const uint8_t *data_ptr, size_t size);

  # Deserializes packet and returns a fat pointer to the packet data or a null pointer if an error
  # occured.
  #
  # # Safety
  # `data_ptr' must point to valid packet data up to `size` bytes.
  #
  # The returned pointer is only valid until the next data-returning function call.
  # If the returned array is empty, the pointer might be non-null but still invalid. This is not
  # considered an error.
  DataBuffer create_packet(PacketWorker *worker, const uint8_t *data_ptr, size_t size);

  # Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
  # occurred.
  #
  # # Safety
  # The returned pointer is only valid until the next failable function call.
  const uint8_t *get_pw_error(const PacketWorker *worker);

  # Creates a new socket factory.
  SocketFactory *new_factory();

  # Destroys a socket factory.
  void free_factory(SocketFactory *_factory);

  # Creates a new listener on the specified address.
  bool create_listener(SocketFactory *factory, const int8_t *addr);

  # Sets the blocking mode of the listener.
  void listener_nonblocking(const SocketFactory *factory, bool nonblocking);

  # Accepts a new incoming connection from installed listener. To collect the resulting connection
  # call `get_connection` or `stream_into_fd".
  SocketResult accept_listener(SocketFactory *factory);

  # Creates a new stream to the specified address. To collect the resulting stream
  # call `get_connection` or `stream_into_fd".
  bool create_stream(SocketFactory *factory, const int8_t *addr);

  # Sets the blocking mode of the stream.
  void stream_nonblocking(SocketFactory *factory, bool nonblocking);

  # Returns the IP address of the stream.
  uint32_t get_stream_ip(const SocketFactory *factory);

  # Creates a new connection from incoming connection.
  #
  # # Safety
  # 'in_key' must either be null or it must point to a UTF-8-encoded, zero-terminated
  # path to a PKCS#8 file.
  Connection *get_connection(SocketFactory *factory, PacketType packet_type, const int8_t *in_key);

  # Returns an incoming connection descriptor. Caller is responsible for closing the returned descriptor.
  # If no stream was opened, returns -1.
  int64_t stream_into_fd(SocketFactory *factory);

  # Clones the descriptor. Returns the cloned descriptor or -1 if an error occurred.
  int64_t clone_fd(SocketFactory *factory, int64_t fd);

  # Closes the file descriptor.
  void close_fd(int64_t fd);

  # Returns an owned socket descriptor. Caller is responsible for closing the returned descriptor.
  # If no listener was opened, returns -1.
  int64_t listener_into_fd(SocketFactory *factory);

  # Installs the provided listener. This function takes ownership of the descriptor.
  #
  # # Safety
  # `fd` must be a valid descriptor.
  bool listener_from_fd(SocketFactory *factory, int64_t fd);

  # Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
  # occurred.
  #
  # # Safety
  # The returned pointer is only valid until the next failable function call.
  const uint8_t *get_sf_error(const SocketFactory *factory);

  # Creates a new connection from owned socket descriptor.
  #
  # # Safety
  # `fd` must be a valid descriptor.
  #
  # 'in_key' must either be null or it must point to a UTF-8-encoded, zero-terminated
  # path to a PKCS#8 file.
  Connection *new_connection(int64_t fd, PacketType packet_type, const int8_t *in_key);

  # Destroys a connection.
  void free_connection(Connection *_conn);

  # Returns the IP address of the connection.
  uint32_t get_conn_ip(const Connection *conn);

  # Changes the connection's packet type.
  void conn_set_packet_type(Connection *conn, PacketType packet_type);

  # Returns a [`Packet`] or a null pointer if no connection was provided.
  #
  # # Safety
  # The returned pointer is only valid until the next data-returning function call.
  # If the returned array is empty, the pointer might be non-null but still invalid. This is not
  # considered an error.
  Packet *conn_get_data(Connection *conn);

  # Reads a packet from the connection and stores it in the internal buffer. Call `conn_get_data`
  # to access it.
  SocketResult conn_read_packet(Connection *conn);

  # Writes a packet to the connection. If `ptr` is null, flushes the buffer.
  #
  # # Note
  # If this function returns [`SocketResult::Blocked`], then the data has been written to the
  # buffer.
  SocketResult conn_write_packet(Connection *conn, const Packet *packet);

  # Returns the encryption key (for [`Packet::EncryptionResponse`]).
  DataBuffer conn_get_key(Connection *conn);

  # Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
  # occurred.
  #
  # # Safety
  # The returned pointer is only valid until the next failable function call.
  const uint8_t *get_conn_error(const Connection *conn);

  # Creates a new PPAC reader. After creation don't forget to check for errors.
  PPACReader *new_reader(const int8_t *path);

  # Destroys the reader.
  void free_reader(PPACReader *_reader);

  # Sets the output type.
  void set_out_type(PPACReader *reader, OutputType out_type);

  # Reads the packet and returns if the function succeeded.
  ReaderResult read_packet(PPACReader *reader);

  # Returns a pointer to the packet data or a null pointer if no data exists.
  #
  # # Note
  # [`data`] field is only returned once and must be freed by the caller.
  #
  # # Safety
  # The returned pointer is only valid until the next data-returning function call.
  # If the returned array is empty, the pointer might be non-null but still invalid. This is not
  # considered an error.
  PacketData get_reader_data(PPACReader *reader);

  # Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error
  # occurred.
  #
  # # Safety
  # The returned pointer is only valid until the next failable function call.
  const uint8_t *get_reader_error(const PPACReader *reader);
