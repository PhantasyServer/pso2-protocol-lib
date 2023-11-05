using System.Runtime.InteropServices;
using System.Text;

internal class Program
{
    private static void Main(string[] args)
    {
        PacketWorker pw = new PacketWorker(packetlib.PacketType.Classic, packetlib.SerializedFormat.JSON);
        
        //example of parsing packets
        byte[] data = { 8, 0, 0, 0, 3, 4, 0, 0 };
        Console.WriteLine(pw.parse_packet(data));

        //example of creating packets
        string str = "{\"LoadLevel\":{}}";
        byte[] buf = pw.create_packet(str);
        foreach (byte b in buf)
            Console.Write("{0:X} ", b);
        Console.WriteLine();

        //example of an error
        str = "{\"Invalid\":{}}";
        try
        {
            buf = pw.create_packet(str);
        } catch (Exception ex)
        {
            Console.WriteLine(ex.ToString());
        }
    }
}

class PacketWorker
{
    private unsafe packetlib.PacketWorker *worker;
    public PacketWorker(packetlib.PacketType packetType, packetlib.SerializedFormat serdeFormat)
    {
        unsafe
        {
            worker = packetlib.NativeMethods.new_worker(packetType, serdeFormat);
        }
    }
    public string parse_packet(byte[] b)
    {
        unsafe
        {
            fixed (byte* ptr = b)
            {
                var buf = packetlib.NativeMethods.parse_packet(worker, ptr, (nuint) b.Length);
                var err = packetlib.NativeMethods.get_pw_error(worker);
                if (err != null)
                {
                    int i = 0;
                    for (i = 0; err[i] != 0; i++) { }
                    byte[] bytes = new byte[i];
                    Marshal.Copy((IntPtr)err, bytes,0, i);
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
            
        }
        return "";
    }
    public byte[] create_packet(string s)
    {
        unsafe
        {
            byte[] str_data = Encoding.UTF8.GetBytes(s);
            byte[] str_data2 = new byte[str_data.Length + 1];
            str_data.CopyTo(str_data2, 0);
            fixed (byte* ptr = str_data2)
            {
                var buf = packetlib.NativeMethods.create_packet(worker, ptr, (nuint)str_data2.Length);
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
                    return bytes;
                }
            }
        }
        return new byte[0];
    }

    ~PacketWorker()
    {
        unsafe
        {
            packetlib.NativeMethods.free_worker(worker);
        }
    }
}