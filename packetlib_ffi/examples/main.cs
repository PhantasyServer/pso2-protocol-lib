using System.Net.Sockets;
using System.Runtime.InteropServices;
using System.Text;

internal class Program
{
    private static void Main()
    {
        packet_demo();
        socket_demo();
        ppac_demo();
    }

    static void packet_demo()
    {
        using PacketWorker pw = new PacketWorker(packetlib.PacketType.Classic, packetlib.SerializedFormat.JSON);
        //example of parsing packets
        {
            byte[] data = { 8, 0, 0, 0, 3, 4, 0, 0 };
            using Packet packet = pw.raw_to_packet(data);
            Console.WriteLine(pw.packet_to_ser(packet));
        }
        //example of creating packets
        {
            string str = "{\"LoadLevel\":{}}";
            using Packet packet = pw.ser_to_packet(str);
            byte[] buf = pw.packet_to_raw(packet);
            foreach (byte b in buf)
                Console.Write("{0:X} ", b);
            Console.WriteLine();
        }
        //example of an error
        {
            string str = "{\"Invalid\":{}}";
            try
            {
                pw.ser_to_packet(str);
            }
            catch (Exception ex)
            {
                Console.WriteLine(ex.ToString());
            }
        }   
    }
    static void socket_demo()
    {
        using PacketWorker pw = new PacketWorker(packetlib.PacketType.NGS, packetlib.SerializedFormat.JSON);

        using SocketFactory sf = new SocketFactory();
        sf.create_listener("0.0.0.0:13370");
        using Connection in_conn = sf.accept_connection();
        string str = "{\"LoadLevel\":{}}";
        using Packet out_packet = pw.ser_to_packet(str);
        in_conn.write_packet(out_packet);

        TcpClient tcp = new TcpClient("40.91.76.146", 12199);
        using Connection conn = new Connection(tcp);
        using Packet packet = conn.read_packet();
        Console.WriteLine(pw.packet_to_ser(packet));
    }
    static void ppac_demo()
    {
        using PacketWorker pw = new PacketWorker(packetlib.PacketType.Classic, packetlib.SerializedFormat.JSON);
        using PPACReader pr = new PPACReader("test.pak");
        while (true)
        {
            using PPACData pd = pr.read_packet();
            if (pd.is_eof)
                return;
            Console.WriteLine("---------");
            Console.WriteLine("Time: {0:d}", pd.time);
            Console.WriteLine("Direction: {0:d}", pd.direction);
            Console.WriteLine("Protocol Type: {0:d}", pd.protocol);
            if (pd.packet != null)
            {
                Console.WriteLine("Packet: {0}", pw.packet_to_ser(pd.packet));
            } else if (pd.raw != null && pd.raw.Length != 0)
            {
                Console.WriteLine("RAW");
            }
        }
    }
}

class Packet : IDisposable
{
    private unsafe packetlib.Packet* packet;
    public unsafe Packet(packetlib.Packet* ptr)
    {
        packet = ptr;
    }

    ~Packet()
    {
        Dispose();
    }

    public void Dispose()
    {
        unsafe
        {
            packetlib.NativeMethods.free_packet(packet);
            packet = null;
        }
    }

    public unsafe packetlib.Packet* get_ptr()
    {
        return packet;
    }
}

class PacketWorker : IDisposable
{
    private unsafe packetlib.PacketWorker *worker;
    public PacketWorker(packetlib.PacketType packetType, packetlib.SerializedFormat serdeFormat)
    {
        unsafe
        {
            worker = packetlib.NativeMethods.new_worker(packetType, serdeFormat);
        }
    }
    ~PacketWorker()
    {
        Dispose();
    }
    public void Dispose()
    {
        unsafe
        {
            packetlib.NativeMethods.free_worker(worker);
            worker = null;
        }
    }
    public Packet raw_to_packet(byte[] b) {
        unsafe
        {
            fixed (byte* ptr = b)
            {
                var packet = packetlib.NativeMethods.raw_to_packet(worker, ptr, (nuint)b.Length);
                var err = packetlib.NativeMethods.get_pw_error(worker);
                if (err != null)
                {
                    int i = 0;
                    for (i = 0; err[i] != 0; i++) { }
                    byte[] bytes = new byte[i];
                    Marshal.Copy((IntPtr)err, bytes, 0, i);
                    string err_str = Encoding.UTF8.GetString(bytes);
                    throw new Exception(err_str);
                }
                return new Packet(packet);
            }
        }
    }
    public Packet ser_to_packet(string s)
    {
        unsafe
        {
            byte[] str_data = Encoding.UTF8.GetBytes(s);
            byte[] str_data2 = new byte[str_data.Length + 1];
            str_data.CopyTo(str_data2, 0);
            fixed (byte* ptr = str_data2)
            {
                var packet = packetlib.NativeMethods.ser_to_packet(worker, ptr, (nuint)str_data2.Length);
                var err = packetlib.NativeMethods.get_pw_error(worker);
                if (err != null)
                {
                    int i = 0;
                    for (i = 0; err[i] != 0; i++) { }
                    byte[] bytes = new byte[i];
                    Marshal.Copy((IntPtr)err, bytes, 0, i);
                    string err_str = Encoding.UTF8.GetString(bytes);
                    throw new Exception(err_str);
                }
                return new Packet(packet);
            }
        }
    }
    public string packet_to_ser(Packet packet)
    {
        unsafe
        {
            var buf = packetlib.NativeMethods.packet_to_ser(worker, packet.get_ptr());
            var err = packetlib.NativeMethods.get_pw_error(worker);
            if (err != null)
            {
                int i = 0;
                for (i = 0; err[i] != 0; i++) { }
                byte[] bytes = new byte[i];
                Marshal.Copy((IntPtr)err, bytes, 0, i);
                string err_str = Encoding.UTF8.GetString(bytes);
                throw new Exception(err_str);
            }
            if (buf.ptr != null && buf.size != 0)
            {
                byte[] bytes = new byte[buf.size];
                Marshal.Copy((IntPtr)buf.ptr, bytes, 0, (int)buf.size);
                string err_str = Encoding.UTF8.GetString(bytes);
                return err_str;
            }
        }
        return "";
    }
    public byte[] packet_to_raw(Packet packet)
    {
        unsafe
        {
            var buf = packetlib.NativeMethods.packet_to_raw(worker, packet.get_ptr());
            if (buf.ptr != null && buf.size != 0)
            {
                byte[] bytes = new byte[buf.size];
                Marshal.Copy((IntPtr)buf.ptr, bytes, 0, (int)buf.size);
                return bytes;
            }

            return new byte[0];
        }
    }
}

class SocketFactory : IDisposable
{
    private unsafe packetlib.SocketFactory* sf;
    public SocketFactory()
    {
        unsafe
        {
            sf = packetlib.NativeMethods.new_factory();
        }
    }
    ~SocketFactory()
    {
        Dispose();
    }
    public void Dispose()
    {
        unsafe
        {
            packetlib.NativeMethods.free_factory(sf);
            sf = null;
        }
    }
    public void create_listener(string addr)
    {
        unsafe
        {
            byte[] str_data = Encoding.UTF8.GetBytes(addr);
            byte[] str_data2 = new byte[str_data.Length + 1];
            str_data.CopyTo(str_data2, 0);
            fixed (byte* ptr = str_data2)
            {
                var packet = packetlib.NativeMethods.create_listener(sf, (sbyte*)ptr);
                var err = packetlib.NativeMethods.get_sf_error(sf);
                if (err != null)
                {
                    int i = 0;
                    for (i = 0; err[i] != 0; i++) { }
                    byte[] bytes = new byte[i];
                    Marshal.Copy((IntPtr)err, bytes, 0, i);
                    string err_str = Encoding.UTF8.GetString(bytes);
                    throw new Exception(err_str);
                }
            }
        }
    }
    public Connection accept_connection()
    {
        unsafe
        {
            packetlib.SocketResult sr = packetlib.SocketResult.Blocked;
            while (sr != packetlib.SocketResult.Ready)
            {
                sr = packetlib.NativeMethods.accept_listener(sf);
                if (sr == packetlib.SocketResult.Blocked)
                    continue;
                else if (sr == packetlib.SocketResult.SocketError)
                {
                    var err = packetlib.NativeMethods.get_sf_error(sf);
                    int i = 0;
                    for (i = 0; err[i] != 0; i++) { }
                    byte[] bytes = new byte[i];
                    Marshal.Copy((IntPtr)err, bytes, 0, i);
                    string err_str = Encoding.UTF8.GetString(bytes);
                    throw new Exception(err_str);
                }
            }
            return new Connection(packetlib.NativeMethods.stream_into_fd(sf));
        }
        
    }
}

class Connection : IDisposable
{
    private unsafe packetlib.Connection* conn;
    private TcpClient? socket;
    public Connection(TcpClient client)
    {
        IntPtr handle = client.GetStream().Socket.Handle;
        socket = client;
        unsafe
        {
            conn = packetlib.NativeMethods.new_connection((long)handle, packetlib.PacketType.NGS, null, null);
        }
    }
    public Connection(long fd)
    {
        unsafe
        {
            conn = packetlib.NativeMethods.new_connection(fd, packetlib.PacketType.NGS, null, null);
        }
    }
    ~Connection()
    {
        Dispose();
    }
    public void Dispose()
    {
        unsafe
        {
            packetlib.NativeMethods.free_connection(conn);
            conn = null;
        }
        if (socket != null)
        {
            socket.Dispose();
            socket = null;
        }
    }
    public Packet read_packet()
    {
        unsafe
        {
            packetlib.SocketResult sr = packetlib.NativeMethods.conn_read_packet(conn);
            if (sr == packetlib.SocketResult.SocketError)
            {
                var err = packetlib.NativeMethods.get_conn_error(conn);
                int i = 0;
                for (i = 0; err[i] != 0; i++) { }
                byte[] bytes = new byte[i];
                Marshal.Copy((IntPtr)err, bytes, 0, i);
                string err_str = Encoding.UTF8.GetString(bytes);
                throw new Exception(err_str);
            }
            return new Packet(packetlib.NativeMethods.conn_get_data(conn));
        }
    }
    public void write_packet(Packet packet)
    {
        unsafe
        {
            packetlib.SocketResult sr = packetlib.NativeMethods.conn_write_packet(conn, packet.get_ptr());
            if (sr == packetlib.SocketResult.SocketError)
            {
                var err = packetlib.NativeMethods.get_conn_error(conn);
                int i = 0;
                for (i = 0; err[i] != 0; i++) { }
                byte[] bytes = new byte[i];
                Marshal.Copy((IntPtr)err, bytes, 0, i);
                string err_str = Encoding.UTF8.GetString(bytes);
                throw new Exception(err_str);
            }
        }
    }
}
struct PPACData : IDisposable
{
    public ulong time;
    public packetlib.Direction direction;
    public packetlib.PacketType protocol;
    public bool is_eof;
    public Packet? packet;
    public byte[] raw;

    public void Dispose()
    {
        if (packet != null) {
            packet.Dispose();
            packet = null;
        }
    }
}
class PPACReader : IDisposable
{
    private unsafe packetlib.PPACReader* pr;
    public PPACReader(string path)
    {
        unsafe
        {
            byte[] str_data = Encoding.UTF8.GetBytes(path);
            byte[] str_data2 = new byte[str_data.Length + 1];
            str_data.CopyTo(str_data2, 0);
            fixed (byte* ptr = str_data2)
            {
                packetlib.PPACReader* pr_tmp = packetlib.NativeMethods.new_reader((sbyte*)ptr);
                var err = packetlib.NativeMethods.get_reader_error(pr_tmp);
                if (err != null)
                {
                    packetlib.NativeMethods.free_reader(pr_tmp);
                    int i = 0;
                    for (i = 0; err[i] != 0; i++) { }
                    byte[] bytes = new byte[i];
                    Marshal.Copy((IntPtr)err, bytes, 0, i);
                    string err_str = Encoding.UTF8.GetString(bytes);
                    throw new Exception(err_str);
                }
                packetlib.NativeMethods.set_out_type(pr_tmp, packetlib.OutputType.OutputBoth);
                pr = pr_tmp;
            }
        }
    }
    ~PPACReader()
    {
        Dispose();
    }
    public void Dispose()
    {
        unsafe
        {
            packetlib.NativeMethods.free_reader(pr);
            pr = null;
        }
    }
    public PPACData read_packet()
    {
        unsafe
        {
            switch (packetlib.NativeMethods.read_packet(pr))
            {
                case packetlib.ReaderResult.Ok:
                case packetlib.ReaderResult.RawOnly:
                    packetlib.PacketData pd = packetlib.NativeMethods.get_reader_data(pr);
                    PPACData data = new();
                    data.time = pd.time;
                    data.direction = pd.direction;
                    data.protocol = pd.protocol_type;
                    data.is_eof = false;
                    if (pd.data!= null)
                    {
                        data.packet = new Packet(pd.data);
                    }
                    if (pd.raw_ptr != null && pd.raw_size != 0)
                    {
                        byte[] raw_bytes = new byte[pd.raw_size];
                        Marshal.Copy((IntPtr)pd.raw_ptr, raw_bytes, 0, (int)pd.raw_size);
                        data.raw = raw_bytes;
                    }
                    return data;
                case packetlib.ReaderResult.ReaderEOF:
                    PPACData data_eof = new();
                    data_eof.is_eof = true;
                    return data_eof;
                case packetlib.ReaderResult.PPACError:
                default:
                    var err = packetlib.NativeMethods.get_reader_error(pr);
                    int i = 0;
                    for (i = 0; err[i] != 0; i++) { }
                    byte[] bytes = new byte[i];
                    Marshal.Copy((IntPtr)err, bytes, 0, i);
                    string err_str = Encoding.UTF8.GetString(bytes);
                    throw new Exception(err_str);
            }
        }
    }
}