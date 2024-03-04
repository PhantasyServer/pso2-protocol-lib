#include "../include/packetlib.h"
#include <algorithm>
#include <iostream>
#include <vector>

namespace packetlib {
class Packet {
private:
  ::PLIB_Packet *packet;

public:
  Packet(::PLIB_Packet *ptr) { packet = ptr; }
  ~Packet() { ::free_packet(packet); }
  Packet(const Packet &) = delete;
  Packet &operator=(Packet other) {
    std::swap(packet, other.packet);
    return *this;
  }
  ::PLIB_Packet *get_ptr() { return packet; }
};
class PacketFactory {
private:
  PLIB_PacketWorker *worker;

public:
  PacketFactory(PLIB_PacketType pt, PLIB_SerializedFormat sf) {
    worker = ::new_worker(pt, sf);
  }
  ~PacketFactory() { ::free_worker(worker); }
  PacketFactory(const PacketFactory &) = delete;
  PacketFactory &operator=(PacketFactory other) {
    std::swap(worker, other.worker);
    return *this;
  }
  Packet raw_to_packet(char *ptr, size_t size) {
    ::PLIB_Packet *packet = ::raw_to_packet(worker, (uint8_t *)ptr, size);
    const char *err = (const char *)::get_pw_error(worker);
    if (err) {
      std::string err_str = std::string((const char *)err);
      throw err_str;
    }
    return Packet(packet);
  }
  Packet ser_to_packet(char *ptr, size_t size) {
    ::PLIB_Packet *packet = ::ser_to_packet(worker, (uint8_t *)ptr, size);
    const char *err = (const char *)::get_pw_error(worker);
    if (err) {
      std::string err_str = std::string((const char *)err);
      throw err_str;
    }
    return Packet(packet);
  }
  std::string packet_to_ser(Packet &packet) {
    PLIB_DataBuffer buf = ::packet_to_ser(worker, packet.get_ptr());
    const char *err = (const char *)::get_pw_error(worker);
    if (err) {
      std::string err_str = std::string((const char *)err);
      throw err_str;
    }
    if (buf.ptr && buf.size) {
      std::string result = std::string((const char *)buf.ptr);
      return result;
    }
    return "";
  }
  std::vector<uint8_t> packet_to_raw(Packet &packet) {
    PLIB_DataBuffer buf = ::packet_to_raw(worker, packet.get_ptr());
    const char *err = (const char *)::get_pw_error(worker);
    if (err) {
      std::string err_str = std::string((const char *)err);
      throw err_str;
    }
    if (buf.ptr && buf.size) {
      std::vector data = std::vector<uint8_t>(buf.size);
      std::copy_n(buf.ptr, buf.size, data.begin());
      return data;
    }
    return std::vector<uint8_t>();
  }
};

class Connection {
private:
  ::PLIB_Connection *conn;

public:
  Connection(uint64_t fd, enum ::PLIB_PacketType packet_type,
             const char *in_key, const char *out_key) {
    conn = ::new_connection(fd, packet_type, (const int8_t *)in_key,
                            (const int8_t *)out_key);
  }
  Connection(::PLIB_Connection *other) { conn = other; }
  ~Connection() { ::free_connection(conn); }
  Connection(const Connection &) = delete;
  Connection &operator=(Connection other) {
    std::swap(conn, other.conn);
    return *this;
  }
  uint32_t get_ip() { return ::get_conn_ip(conn); }
  void write_packet(Packet &packet) {
    PLIB_SocketResult sr = conn_write_packet(conn, packet.get_ptr());
    if (sr == SocketError) {
      const char *err = (const char *)::get_conn_error(conn);
      if (err) {
        std::string err_str = std::string((const char *)err);
        throw err_str;
      }
    }
  }
  Packet read_packet() {
    PLIB_SocketResult sr = ::conn_read_packet(conn);
    if (sr == SocketError) {
      const char *err = (const char *)::get_conn_error(conn);
      if (err) {
        std::string err_str = std::string((const char *)err);
        throw err_str;
      }
    } else if (sr == Ready) {
      ::PLIB_Packet *packet = ::conn_get_data(conn);
      return Packet(packet);
    }
    return Packet(0);
  }
};

class SocketFactory {
private:
  ::PLIB_SocketFactory *sf;

public:
  SocketFactory() { sf = ::new_factory(); }
  ~SocketFactory() { ::free_factory(sf); }
  SocketFactory(const SocketFactory &) = delete;
  SocketFactory &operator=(SocketFactory other) {
    std::swap(sf, other.sf);
    return *this;
  }
  void create_listener(const char *str) {
    ::create_listener(sf, (const int8_t *)str);
    const char *err = (const char *)::get_sf_error(sf);
    if (err) {
      std::string err_str = std::string((const char *)err);
      throw err_str;
    }
  }
  void set_listener(const char *str) {
    ::create_stream(sf, (const int8_t *)str);
    const char *err = (const char *)::get_sf_error(sf);
    if (err) {
      std::string err_str = std::string((const char *)err);
      throw err_str;
    }
  }
  uint64_t get_listener() { return ::listener_into_fd(sf); }
  void set_listener(uint64_t fd) { ::listener_from_fd(sf, fd); }
  void close_fd(uint64_t fd) { ::close_fd(fd); }
  void set_nonblocking(bool s) { ::listener_nonblocking(sf, s); }
  uint64_t clone_fd(uint64_t fd) {
    uint64_t cloned_fd = ::clone_fd(sf, fd);
    const char *err = (const char *)::get_sf_error(sf);
    if (err) {
      std::string err_str = std::string((const char *)err);
      throw err_str;
    }
    return cloned_fd;
  }
  Connection accept_connection(enum ::PLIB_PacketType packet_type,
                               const char *in_key, const char *out_key) {
    ::PLIB_SocketResult sr = Blocked;
    while (sr != Ready) {
      sr = accept_listener(sf);
      if (sr == Blocked)
        continue;
      else if (sr == SocketError) {
        const char *err = (const char *)::get_sf_error(sf);
        if (err) {
          std::string err_str = std::string((const char *)err);
          throw err_str;
        }
      }
    }
    return Connection(::stream_into_fd(sf), packet_type, in_key, out_key);
  }

  Connection new_connection(const char *ip, enum ::PLIB_PacketType packet_type,
                            const char *in_key, const char *out_key) {
    if (!::create_stream(sf, (const int8_t *)ip)) {
      const char *err = (const char *)::get_sf_error(sf);
      if (err) {
        std::string err_str = std::string((const char *)err);
        throw err_str;
      }
    }
    return Connection(::stream_into_fd(sf), packet_type, in_key, out_key);
  }
};

typedef struct {
  uint64_t time;
  int direction;
  int protocol;
  bool is_eof;
  Packet packet;
  std::vector<uint8_t> raw;
} PPACData;

class PPACReader {
private:
  ::PLIB_PPACReader *pr;

public:
  PPACReader(const char *path) {
    ::PLIB_PPACReader *reader = ::new_reader((const int8_t *)path);
    const char *err = (const char *)::get_reader_error(reader);
    if (err) {
      std::string err_str = std::string((const char *)err);
      free_reader(reader);
      throw err_str;
    }
    ::set_out_type(reader, ::OutputBoth);
    pr = reader;
  }
  ~PPACReader() { ::free_reader(pr); }
  PPACReader(const PPACReader &) = delete;
  PPACReader &operator=(PPACReader other) {
    std::swap(pr, other.pr);
    return *this;
  }
  PPACData read_packet() {
    PLIB_ReaderResult rr = ::read_packet(pr);
    switch (rr) {

    case Ok:
    case RawOnly: {
      ::PLIB_PacketData pd = ::get_reader_data(pr);
      std::vector data = std::vector<uint8_t>(pd.raw_size);
      std::copy_n(pd.raw_ptr, pd.raw_size, data.begin());
      return {.time = pd.time,
              .direction = pd.direction,
              .protocol = pd.protocol_type,
              .is_eof = false,
              .packet = Packet(pd.data),
              .raw = data};
    }
    case ReaderEOF:
      return {.is_eof = true, .packet = Packet(0)};
    case PPACError: {
      const char *err = (const char *)::get_reader_error(pr);
      std::string err_str = std::string((const char *)err);
      throw err_str;
    }
    }
    return {.is_eof = true, .packet = Packet(0)};
  }
};

} // namespace packetlib

//------------------------
// Packet example
//------------------------

void packet_demo() {
  packetlib::PacketFactory pf = packetlib::PacketFactory(Classic, JSON);

  // example of parsing packets
  char data[] = {8, 0, 0, 0, 3, 4, 0, 0};
  packetlib::Packet packet = pf.raw_to_packet(data, sizeof(data));
  std::string json_data = pf.packet_to_ser(packet);
  std::cout << json_data << std::endl;

  // example of creating packets
  std::string str = "{\"LoadLevel\":{}}";
  packet = pf.ser_to_packet(str.data(), str.length() + 1);
  std::vector<uint8_t> buf = pf.packet_to_raw(packet);
  for (auto &e : buf)
    std::cout << std::hex << +e << " ";
  std::cout << std::dec << std::endl;

  // example of an error
  str = "{\"Invalid\":{}}";
  try {
    pf.ser_to_packet(str.data(), str.length() + 1);
  } catch (std::string e) {
    std::cerr << e << std::endl;
  }
}

//------------------------
// Socket example
//------------------------

void socket_demo() {
  packetlib::PacketFactory pf = packetlib::PacketFactory(Classic, JSON);
  packetlib::SocketFactory sf = packetlib::SocketFactory();

  // create a new listener
  sf.create_listener("0.0.0.0:13370");

  // set listener to nonblocking mode
  sf.set_nonblocking(true);

  // copy handle
  uint64_t fd = sf.get_listener();
  uint64_t fd_clone = sf.clone_fd(fd);
  sf.set_listener(fd_clone);
  sf.close_fd(fd);

  // get received connection
  packetlib::Connection conn = sf.accept_connection(Classic, NULL, NULL);

  int ip = conn.get_ip();
  printf("Ip: ");
  for (int i = 0; i < 4; i++)
    printf("%hhu ", ((char *)&ip)[3 - i]);
  printf("\n");

  // write data to client
  std::string str = "{\"LoadLevel\":{}}";
  packetlib::Packet packet = pf.ser_to_packet(str.data(), str.length() + 1);
  conn.write_packet(packet);

  // connect to sega server
  conn = sf.new_connection("40.91.76.146:12199", NGS, NULL, NULL);

  ip = conn.get_ip();
  printf("Ip: ");
  for (int i = 0; i < 4; i++)
    printf("%hhu ", ((char *)&ip)[3 - i]);
  printf("\n");

  // read packet from server
  packet = conn.read_packet();
  std::string data = pf.packet_to_ser(packet);
  printf("%s\n", data.data());
}

//------------------------
// PPAC reader example
//------------------------

void ppac_demo() {
  packetlib::PacketFactory pf = packetlib::PacketFactory(Classic, JSON);
  packetlib::PPACReader reader = packetlib::PPACReader("test.pak");
  while (1) {
    packetlib::PPACData data = reader.read_packet();
    if (data.is_eof)
      break;
    printf("----------\n");
    printf("Time: %lu\n", data.time);
    printf("Direction: %u\n", data.direction);
    printf("Protocol Type: %u\n", data.protocol);
    if (data.packet.get_ptr()) {
      std::string data_out = pf.packet_to_ser(data.packet);
      printf("Packet: %s\n", data_out.data());
    } else {
      printf("RAW\n");
    }
  }
}

int main(int argc, char **argv) {
  if (get_api_version() != PLIB_API_VERSION)
    return -1;
  packet_demo();
  socket_demo();
  ppac_demo();
  return 0;
}
