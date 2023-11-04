#include "../packetlib.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>

int main(int argc, char **argv) {
  if (get_api_version() != API_VERSION)
    return -1;
  PacketWorker *worker = new_worker(Classic, JSON);

  // example of parsing packets
  uint8_t data[] = {8, 0, 0, 0, 3, 4, 0, 0};
  DataBuffer buf = parse_packet(worker, data, 8);
  const unsigned char *err = get_error(worker);
  if (buf.ptr != NULL && buf.size != 0)
    printf("%s\n", buf.ptr);
  if (err != NULL) {
    printf("%s\n", err);
  }

  // example of creating packets
  char str[] = "{\"LoadLevel\":{}}";
  buf = create_packet(worker, (uint8_t *)str, strlen(str) + 1);
  err = get_error(worker);
  if (buf.ptr != NULL)
    for (int i = 0; i < buf.size; i++)
      printf("%x ", buf.ptr[i]);
  printf("\n");
  if (err != NULL) {
    printf("%s\n", err);
  }

  // example of an error
  char str2[] = "{\"Invalid\":{}}";
  buf = create_packet(worker, (uint8_t *)str2, strlen(str2) + 1);
  err = get_error(worker);
  if (buf.ptr != NULL)
    for (int i = 0; i < buf.size; i++)
      printf("%x ", buf.ptr[i]);
  printf("\n");
  if (err != NULL) {
    printf("%s\n", err);
  }

  free_worker(worker);
  return 0;
}
