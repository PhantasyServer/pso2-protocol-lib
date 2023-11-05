#include "../packetlib.h"
#include <stdint.h>
#include <stdio.h>
#include <string.h>

void packet_demo() {
  PacketWorker *worker = new_worker(Classic, JSON);

  // example of parsing packets
  uint8_t data[] = {8, 0, 0, 0, 3, 4, 0, 0};
  DataBuffer buf = parse_packet(worker, data, 8);
  const unsigned char *err = get_pw_error(worker);
  if (err != NULL)
    printf("%s\n", err);
  if (buf.ptr != NULL && buf.size != 0)
    printf("%s\n", buf.ptr);

  // example of creating packets
  char str[] = "{\"LoadLevel\":{}}";
  buf = create_packet(worker, (uint8_t *)str, strlen(str) + 1);
  err = get_pw_error(worker);
  if (err != NULL)
    printf("%s\n", err);
  if (buf.ptr != NULL)
    for (int i = 0; i < buf.size; i++)
      printf("%x ", buf.ptr[i]);
  printf("\n");

  // example of an error
  char str2[] = "{\"Invalid\":{}}";
  buf = create_packet(worker, (uint8_t *)str2, strlen(str2) + 1);
  err = get_pw_error(worker);
  if (err != NULL)
    printf("%s\n", err);
  if (buf.ptr != NULL)
    for (int i = 0; i < buf.size; i++)
      printf("%x ", buf.ptr[i]);
  printf("\n");

  free_worker(worker);
}

void socket_demo() {
  SocketFactory *sf = new_factory();

  // create a new listener
  if (!create_listener(sf, (const signed char *)"0.0.0.0:13370")) {
    const unsigned char *err = get_sf_error(sf);
    if (err != NULL)
      printf("%s\n", err);
    free_factory(sf);
    return;
  }

  // set listener to nonblocking mode
  listener_nonblocking(sf, true);

  // copy handle
  int64_t handle = listener_into_fd(sf);
  if (!listener_from_borrowed_fd(sf, handle)) {
    const unsigned char *err = get_sf_error(sf);
    if (err != NULL)
      printf("%s\n", err);
    free_factory(sf);
    return;
  }
  close_listener(handle);

  // wait for connection
  enum SocketResult sr = Blocked;
  while (sr != Ready) {
    sr = accept_listener(sf);
    if (sr == Blocked)
      continue;
    else if (sr == SocketError) {
      const unsigned char *err = get_sf_error(sf);
      if (err != NULL)
        printf("%s\n", err);
      free_factory(sf);
      return;
    }
  }

  // get received connection
  Connection *conn = get_connection(sf, Classic, NULL, NULL, JSON);

  int ip = get_conn_ip(conn);
  printf("Ip: ");
  for (int i = 0; i < 4; i++)
    printf("%hhu ", ((char *)&ip)[3 - i]);
  printf("\n");

  // write data to client
  char str[] = "{\"LoadLevel\":{}}";
  sr = conn_write_packet(conn, (const uint8_t *)str, strlen(str) + 1);
  if (sr == SocketError) {
    const unsigned char *err = get_sf_error(sf);
    if (err != NULL)
      printf("%s\n", err);
    free_connection(conn);
    free_factory(sf);
    return;
  }
  free_connection(conn);

  // connect to sega server
  if (!create_stream(sf, (const signed char *)"40.91.76.146:12199")) {
    const unsigned char *err = get_sf_error(sf);
    if (err != NULL)
      printf("%s\n", err);
    free_factory(sf);
    return;
  }
  conn = get_connection(sf, NGS, NULL, NULL, JSON);

  ip = get_conn_ip(conn);
  printf("Ip: ");
  for (int i = 0; i < 4; i++)
    printf("%hhu ", ((char *)&ip)[3 - i]);
  printf("\n");

  // read packet from server
  sr = conn_read_packet(conn);
  if (sr == Ready) {
    DataBuffer data = conn_get_data(conn);
    printf("%s\n", data.ptr);
  } else if (sr == SocketError) {
    const unsigned char *err = get_sf_error(sf);
    if (err != NULL)
      printf("%s\n", err);
    free_connection(conn);
    free_factory(sf);
    return;
  }
  free_connection(conn);

  free_factory(sf);
}

void ppac_demo() {
  PPACReader *pr = new_reader((const signed char *)"test.pak", JSON);
  const unsigned char *err = get_reader_error(pr);
  if (err) {
    printf("%s\n", err);
    free_reader(pr);
    return;
  }
  set_out_type(pr, OutputBoth);
  ReaderResult rr = Ok;
  while (rr == Ok) {
    rr = read_packet(pr);
    err = get_reader_error(pr);
    if (err) {
      printf("%s\n", err);
      break;
    }
    if (rr == ReaderEOF)
      break;
    PacketData pd = get_reader_data(pr);
    printf("----------\n");
    printf("Time: %lu\n", pd.time);
    if (pd.data_ptr && pd.data_size) {
      printf("Packet: %s\n", pd.data_ptr);
    } else if (pd.raw_ptr && pd.raw_size) {
      printf("RAW");
    }
  }

  free_reader(pr);
}

int main(int argc, char **argv) {
  if (get_api_version() != API_VERSION)
    return -1;
  packet_demo();
  socket_demo();
  ppac_demo();
  return 0;
}
