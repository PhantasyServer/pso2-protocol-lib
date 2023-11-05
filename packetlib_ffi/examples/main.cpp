#include "../packetlib.h"
#include <algorithm>
#include <codecvt>
#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <iostream>
#include <locale>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

namespace packetlib {
class PacketFactory {
private:
  PacketWorker *worker;

public:
  PacketFactory(PacketType pt, SerializedFormat sf) {
    worker = ::new_worker(pt, sf);
  }
  ~PacketFactory() { ::free_worker(worker); }
  std::string parse_packet(char *ptr, size_t size) {
    DataBuffer buf = ::parse_packet(worker, (uint8_t *)ptr, size);
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
  std::vector<uint8_t> create_packet(char *ptr, size_t size) {
    DataBuffer buf = ::create_packet(worker, (uint8_t *)ptr, size);
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

} // namespace packetlib

int main(int argc, char **argv) {
  if (get_api_version() != API_VERSION)
    return -1;
  packetlib::PacketFactory pf = packetlib::PacketFactory(Classic, JSON);

  // example of parsing packets
  char data[] = {8, 0, 0, 0, 3, 4, 0, 0};
  std::string json_data = pf.parse_packet(data, sizeof(data));
  std::cout << json_data << std::endl;

  // example of creating packets
  std::string str = "{\"LoadLevel\":{}}";
  std::vector<uint8_t> buf = pf.create_packet(str.data(), str.length() + 1);
  for (auto &e : buf)
    std::cout << std::hex << +e << " ";
  std::cout << std::dec << std::endl;

  // example of an error
  str = "{\"Invalid\":{}}";
  try {
    pf.create_packet(str.data(), str.length() + 1);
  } catch (std::string e) {
    std::cerr << e << std::endl;
  }

  return 0;
}
