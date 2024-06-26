// <auto-generated>
// This code is generated by csbindgen.
// DON'T CHANGE THIS DIRECTLY.
// </auto-generated>
#pragma warning disable CS8500
#pragma warning disable CS8981
using System;
using System.Runtime.InteropServices;


namespace packetlib
{
    internal static unsafe partial class NativeMethods
    {
        const string __DllName = "packetlib_ffi";



        /// <summary>Creates a new packet worker.</summary>
        [DllImport(__DllName, EntryPoint = "new_worker", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern PacketWorker* new_worker(PacketType packet_type, SerializedFormat serde_format);

        /// <summary>Destroys a packet worker.</summary>
        [DllImport(__DllName, EntryPoint = "free_worker", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void free_worker(PacketWorker* _worker);

        /// <summary>Destroys a packet.</summary>
        [DllImport(__DllName, EntryPoint = "free_packet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void free_packet(Packet* _packet);

        /// <summary>Clones the packet.</summary>
        [DllImport(__DllName, EntryPoint = "clone_packet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Packet* clone_packet(Packet* packet);

        /// <summary>Checks if the packet is empty.</summary>
        [DllImport(__DllName, EntryPoint = "packet_is_empty", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        public static extern bool packet_is_empty(Packet* packet);

        /// <summary>Sets a new packet type.</summary>
        [DllImport(__DllName, EntryPoint = "set_packet_type", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void set_packet_type(PacketWorker* worker, PacketType packet_type);

        /// <summary>Sets a new serde format.</summary>
        [DllImport(__DllName, EntryPoint = "set_serde_format", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void set_serde_format(PacketWorker* worker, SerializedFormat format);

        /// <summary>Checks if the specified serde format is supported.</summary>
        [DllImport(__DllName, EntryPoint = "serde_supported", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        public static extern bool serde_supported(SerializedFormat serde_format);

        /// <summary>Parses raw packet data and returns a [`Packet`] type or a null pointer if an error occured.  # Safety `data_ptr' must point to valid packet data up to `size` bytes.</summary>
        [DllImport(__DllName, EntryPoint = "raw_to_packet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Packet* raw_to_packet(PacketWorker* worker, byte* data_ptr, nuint size);

        /// <summary>Parses serialized packet data and returns a [`Packet`] type or a null pointer if an error occurred.  # Safety `data_ptr' must point to valid serialied data up to `size` bytes.</summary>
        [DllImport(__DllName, EntryPoint = "ser_to_packet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Packet* ser_to_packet(PacketWorker* worker, byte* data_ptr, nuint size);

        /// <summary>Parses [`Packet`] and returns raw packet data.  # Safety The returned pointer is only valid until the next data-returning function call. If the returned array is empty, the pointer might be non-null but still invalid. This is not considered an error.</summary>
        [DllImport(__DllName, EntryPoint = "packet_to_raw", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DataBuffer packet_to_raw(PacketWorker* worker, Packet* packet);

        /// <summary>Parses [`Packet`] and returns serialized packet data or a null pointer if an error occured.  # Safety The returned pointer is only valid until the next data-returning function call. If the returned array is empty, the pointer might be non-null but still invalid. This is not considered an error.</summary>
        [DllImport(__DllName, EntryPoint = "packet_to_ser", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DataBuffer packet_to_ser(PacketWorker* worker, Packet* packet);

        /// <summary>Parses packet data and returns a fat pointer to the serialized packet or a null pointer if an error occurred.  # Safety `data_ptr' must point to valid packet data up to `size` bytes.  The returned pointer is only valid until the next data-returning function call. If the returned array is empty, the pointer might be non-null but still invalid. This is not considered an error.</summary>
        [DllImport(__DllName, EntryPoint = "parse_packet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DataBuffer parse_packet(PacketWorker* worker, byte* data_ptr, nuint size);

        /// <summary>Deserializes packet and returns a fat pointer to the packet data or a null pointer if an error occured.  # Safety `data_ptr' must point to valid packet data up to `size` bytes.  The returned pointer is only valid until the next data-returning function call. If the returned array is empty, the pointer might be non-null but still invalid. This is not considered an error.</summary>
        [DllImport(__DllName, EntryPoint = "create_packet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DataBuffer create_packet(PacketWorker* worker, byte* data_ptr, nuint size);

        /// <summary>Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error occurred.  # Safety The returned pointer is only valid until the next failable function call.</summary>
        [DllImport(__DllName, EntryPoint = "get_pw_error", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern byte* get_pw_error(PacketWorker* worker);

        [DllImport(__DllName, EntryPoint = "get_api_version", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern uint get_api_version();

        [DllImport(__DllName, EntryPoint = "get_protocol_version", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern uint get_protocol_version();

        /// <summary>Returns whether the library is built with connection support.</summary>
        [DllImport(__DllName, EntryPoint = "have_connection", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        public static extern bool have_connection();

        /// <summary>Returns whether the library is built with PPAC support.</summary>
        [DllImport(__DllName, EntryPoint = "have_ppac", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        public static extern bool have_ppac();

        /// <summary>Creates a new socket factory.</summary>
        [DllImport(__DllName, EntryPoint = "new_factory", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern SocketFactory* new_factory();

        /// <summary>Destroys a socket factory.</summary>
        [DllImport(__DllName, EntryPoint = "free_factory", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void free_factory(SocketFactory* _factory);

        /// <summary>Creates a new listener on the specified address.</summary>
        [DllImport(__DllName, EntryPoint = "create_listener", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        public static extern bool create_listener(SocketFactory* factory, sbyte* addr);

        /// <summary>Sets the blocking mode of the listener.</summary>
        [DllImport(__DllName, EntryPoint = "listener_nonblocking", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void listener_nonblocking(SocketFactory* factory, [MarshalAs(UnmanagedType.U1)] bool nonblocking);

        /// <summary>Accepts a new incoming connection from installed listener. To collect the resulting connection call `get_connection` or `stream_into_fd\".</summary>
        [DllImport(__DllName, EntryPoint = "accept_listener", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern SocketResult accept_listener(SocketFactory* factory);

        /// <summary>Creates a new stream to the specified address. To collect the resulting stream call `get_connection` or `stream_into_fd\".</summary>
        [DllImport(__DllName, EntryPoint = "create_stream", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        public static extern bool create_stream(SocketFactory* factory, sbyte* addr);

        /// <summary>Sets the blocking mode of the stream.</summary>
        [DllImport(__DllName, EntryPoint = "stream_nonblocking", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void stream_nonblocking(SocketFactory* factory, [MarshalAs(UnmanagedType.U1)] bool nonblocking);

        /// <summary>Returns the IP address of the stream.</summary>
        [DllImport(__DllName, EntryPoint = "get_stream_ip", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern uint get_stream_ip(SocketFactory* factory);

        /// <summary>Creates a new connection from incoming connection.  # Safety 'in_key' must either be null or it must point to a UTF-8-encoded, zero-terminated path to a PKCS#8 file containing a private key for decryption. 'out_key' must either be null or it must point to a UTF-8-encoded, zero-terminated path to a PKCS#8 file containing a public key for encryption.</summary>
        [DllImport(__DllName, EntryPoint = "get_connection", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Connection* get_connection(SocketFactory* factory, PacketType packet_type, sbyte* in_key, sbyte* out_key);

        /// <summary>Returns an incoming connection descriptor. Caller is responsible for closing the returned descriptor. If no stream was opened, returns -1.</summary>
        [DllImport(__DllName, EntryPoint = "stream_into_fd", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern long stream_into_fd(SocketFactory* factory);

        /// <summary>Clones the descriptor. Returns the cloned descriptor or -1 if an error occurred.</summary>
        [DllImport(__DllName, EntryPoint = "clone_fd", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern long clone_fd(SocketFactory* factory, long fd);

        /// <summary>Closes the file descriptor.</summary>
        [DllImport(__DllName, EntryPoint = "close_fd", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void close_fd(long fd);

        /// <summary>Returns an owned socket descriptor. Caller is responsible for closing the returned descriptor. If no listener was opened, returns -1.</summary>
        [DllImport(__DllName, EntryPoint = "listener_into_fd", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern long listener_into_fd(SocketFactory* factory);

        /// <summary>Installs the provided listener. This function takes ownership of the descriptor.  # Safety `fd` must be a valid descriptor.</summary>
        [DllImport(__DllName, EntryPoint = "listener_from_fd", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        [return: MarshalAs(UnmanagedType.U1)]
        public static extern bool listener_from_fd(SocketFactory* factory, long fd);

        /// <summary>Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error occurred.  # Safety The returned pointer is only valid until the next failable function call.</summary>
        [DllImport(__DllName, EntryPoint = "get_sf_error", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern byte* get_sf_error(SocketFactory* factory);

        /// <summary>Creates a new connection from owned socket descriptor.  # Safety `fd` must be a valid descriptor.  'in_key' must either be null or it must point to a UTF-8-encoded, zero-terminated path to a PKCS#8 file containing a private key for decryption. 'out_key' must either be null or it must point to a UTF-8-encoded, zero-terminated path to a PKCS#8 file containing a public key for encryption.</summary>
        [DllImport(__DllName, EntryPoint = "new_connection", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Connection* new_connection(long fd, PacketType packet_type, sbyte* in_key, sbyte* out_key);

        /// <summary>Destroys a connection.</summary>
        [DllImport(__DllName, EntryPoint = "free_connection", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void free_connection(Connection* _conn);

        /// <summary>Returns the IP address of the connection.</summary>
        [DllImport(__DllName, EntryPoint = "get_conn_ip", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern uint get_conn_ip(Connection* conn);

        /// <summary>Changes the connection's packet type.</summary>
        [DllImport(__DllName, EntryPoint = "conn_set_packet_type", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void conn_set_packet_type(Connection* conn, PacketType packet_type);

        /// <summary>Returns a [`Packet`] or a null pointer if no connection was provided.  # Safety The returned pointer is only valid until the next data-returning function call. If the returned array is empty, the pointer might be non-null but still invalid. This is not considered an error.</summary>
        [DllImport(__DllName, EntryPoint = "conn_get_data", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern Packet* conn_get_data(Connection* conn);

        /// <summary>Reads a packet from the connection and stores it in the internal buffer. Call `conn_get_data` to access it.</summary>
        [DllImport(__DllName, EntryPoint = "conn_read_packet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern SocketResult conn_read_packet(Connection* conn);

        /// <summary>Writes a packet to the connection. If `ptr` is null, flushes the buffer.  # Note If this function returns [`SocketResult::Blocked`], then the data has been written to the buffer.</summary>
        [DllImport(__DllName, EntryPoint = "conn_write_packet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern SocketResult conn_write_packet(Connection* conn, Packet* packet);

        /// <summary>Returns the encryption key (for [`Packet::EncryptionResponse`]).</summary>
        [DllImport(__DllName, EntryPoint = "conn_get_key", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern DataBuffer conn_get_key(Connection* conn);

        /// <summary>Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error occurred.  # Safety The returned pointer is only valid until the next failable function call.</summary>
        [DllImport(__DllName, EntryPoint = "get_conn_error", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern byte* get_conn_error(Connection* conn);

        /// <summary>Creates a new PPAC reader. After creation don't forget to check for errors.</summary>
        [DllImport(__DllName, EntryPoint = "new_reader", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern PPACReader* new_reader(sbyte* path);

        /// <summary>Destroys the reader.</summary>
        [DllImport(__DllName, EntryPoint = "free_reader", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void free_reader(PPACReader* _reader);

        /// <summary>Sets the output type.</summary>
        [DllImport(__DllName, EntryPoint = "set_out_type", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern void set_out_type(PPACReader* reader, OutputType out_type);

        /// <summary>Reads the packet and returns if the function succeeded.</summary>
        [DllImport(__DllName, EntryPoint = "read_packet", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern ReaderResult read_packet(PPACReader* reader);

        /// <summary>Returns a pointer to the packet data or a null pointer if no data exists.  # Note [`data`] field is only returned once and must be freed by the caller.  # Safety The returned pointer is only valid until the next data-returning function call. If the returned array is empty, the pointer might be non-null but still invalid. This is not considered an error.</summary>
        [DllImport(__DllName, EntryPoint = "get_reader_data", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern PacketData get_reader_data(PPACReader* reader);

        /// <summary>Returns a pointer to a UTF-8-encoded zero-terminated error string or a null pointer if no error occurred.  # Safety The returned pointer is only valid until the next failable function call.</summary>
        [DllImport(__DllName, EntryPoint = "get_reader_error", CallingConvention = CallingConvention.Cdecl, ExactSpelling = true)]
        public static extern byte* get_reader_error(PPACReader* reader);


    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct DataBuffer
    {
        public byte* ptr;
        public nuint size;
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct PacketWorker
    {
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct Packet
    {
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct SocketFactory
    {
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct Connection
    {
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct PPACReader
    {
    }

    [StructLayout(LayoutKind.Sequential)]
    internal unsafe partial struct PacketData
    {
        public ulong time;
        public Direction direction;
        public PacketType protocol_type;
        public Packet* data;
        public byte* raw_ptr;
        public nuint raw_size;
    }


    internal enum PacketType : uint
    {
        NGS,
        Classic,
        NA,
        JP,
        Vita,
        Raw,
    }

    internal enum SerializedFormat : uint
    {
        JSON,
        MessagePack,
        MessagePackNamed,
    }

    internal enum SocketResult : uint
    {
        Ready,
        Blocked,
        NoSocket,
        SocketError,
    }

    internal enum ReaderResult : uint
    {
        Ok,
        RawOnly,
        ReaderEOF,
        PPACError,
    }

    internal enum Direction : uint
    {
        ToServer,
        ToClient,
    }

    internal enum OutputType : uint
    {
        OutputPacket,
        OutputRaw,
        OutputBoth,
    }


}
