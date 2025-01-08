#include "../include/packetlib.h"
#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

//------------------------
// Packet example
//------------------------

void packet_demo() {
  PLIB_PacketWorker *worker = new_worker(Classic, JSON);

  // example of parsing packets
  uint8_t data[] = {8, 0, 0, 0, 3, 4, 0, 0};
  PLIB_Packet *packet = raw_to_packet(worker, data, 8);
  const unsigned char *err = get_pw_error(worker);
  if (err != NULL)
    printf("%s\n", err);
  PLIB_DataBuffer buf = packet_to_ser(worker, packet);
  err = get_pw_error(worker);
  if (err != NULL)
    printf("%s\n", err);
  if (buf.ptr != NULL && buf.size != 0)
    printf("%s\n", buf.ptr);
  free_packet(packet);
  free_data(buf);

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
  free_data(buf);

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

//------------------------
// Socket example
//------------------------

void socket_demo() {
  PLIB_PacketWorker *worker = new_worker(Classic, JSON);
  PLIB_SocketFactory *sf = new_factory();

  // create a new listener
  if (!create_listener(sf, (const signed char *)"0.0.0.0:13370")) {
    const unsigned char *err = get_sf_error(sf);
    if (err != NULL)
      printf("%s\n", err);
    free_factory(sf);
    free_worker(worker);
    return;
  }

  // set listener to nonblocking mode
  listener_nonblocking(sf, true);

  // copy handle
  int64_t handle = listener_into_fd(sf);
  int64_t handle2;
  if ((handle2 = clone_fd(sf, handle)) == -1) {
    const unsigned char *err = get_sf_error(sf);
    if (err != NULL)
      printf("%s\n", err);
    free_factory(sf);
    free_worker(worker);
    return;
  }
  listener_from_fd(sf, handle2);
  close_fd(handle);

  // wait for connection
  enum PLIB_SocketResult sr = Blocked;
  while (sr != Ready) {
    sr = accept_listener(sf);
    if (sr == Blocked)
      continue;
    else if (sr == SocketError) {
      const unsigned char *err = get_sf_error(sf);
      if (err != NULL)
        printf("%s\n", err);
      free_factory(sf);
      free_worker(worker);
      return;
    }
  }

  // get received connection
  PLIB_Connection *conn = get_connection(sf, Classic, NULL, NULL);

  int ip = get_conn_ip(conn);
  printf("Ip: ");
  for (int i = 0; i < 4; i++)
    printf("%hhu ", ((char *)&ip)[3 - i]);
  printf("\n");

  // write data to client
  char str[] = "{\"LoadLevel\":{}}";
  PLIB_Packet *packet =
      ser_to_packet(worker, (const uint8_t *)str, strlen(str) + 1);
  {
    const unsigned char *err = get_pw_error(worker);
    if (err != NULL)
      printf("%s\n", err);
  }
  sr = conn_write_packet(conn, packet);
  free_packet(packet);
  if (sr == SocketError) {
    const unsigned char *err = get_conn_error(conn);
    if (err != NULL)
      printf("%s\n", err);
    free_connection(conn);
    free_factory(sf);
    free_worker(worker);
    return;
  }
  free_connection(conn);

  // connect to sega server
  if (!create_stream(sf, (const signed char *)"40.91.76.146:12199")) {
    const unsigned char *err = get_sf_error(sf);
    if (err != NULL)
      printf("%s\n", err);
    free_factory(sf);
    free_worker(worker);
    return;
  }
  conn = get_connection(sf, NGS, NULL, NULL);

  ip = get_conn_ip(conn);
  printf("Ip: ");
  for (int i = 0; i < 4; i++)
    printf("%hhu ", ((char *)&ip)[3 - i]);
  printf("\n");

  // read packet from server
  sr = conn_read_packet(conn);
  if (sr == Ready) {
    PLIB_Packet *packet = conn_get_data(conn);
    PLIB_DataBuffer data = packet_to_ser(worker, packet);
    printf("%s\n", data.ptr);
    free_packet(packet);
    free_data(data);
  } else if (sr == SocketError) {
    const unsigned char *err = get_conn_error(conn);
    if (err != NULL)
      printf("%s\n", err);
    free_connection(conn);
    free_factory(sf);
    free_worker(worker);
    return;
  }
  free_connection(conn);

  free_factory(sf);
  free_worker(worker);
}

//------------------------
// PPAC reader example
//------------------------

void ppac_demo() {
  PLIB_PacketWorker *worker = new_worker(Classic, JSON);
  PLIB_PPACReader *pr = new_reader((const signed char *)"test.pak");
  const unsigned char *err = get_reader_error(pr);
  if (err) {
    printf("%s\n", err);
    free_reader(pr);
    return;
  }
  set_out_type(pr, OutputBoth);
  PLIB_ReaderResult rr = Ok;
  while (rr == Ok || rr == RawOnly) {
    rr = read_packet(pr);
    err = get_reader_error(pr);
    if (err) {
      printf("%s\n", err);
      break;
    }
    if (rr == ReaderEOF)
      break;
    PLIB_PacketData pd = get_reader_data(pr);
    printf("----------\n");
    printf("Time: %lu\n", pd.time);
    printf("Direction: %u\n", pd.direction);
    printf("Protocol Type: %u\n", pd.protocol_type);
    if (pd.data) {
      PLIB_DataBuffer data = packet_to_ser(worker, pd.data);
      printf("Packet: %s\n", data.ptr);
      free_packet(pd.data);
      free_data(data);
    } else if (pd.raw_ptr && pd.raw_size) {
      printf("RAW\n");
    }
  }

  free_reader(pr);
  free_worker(worker);
}

int main(int argc, char **argv) {
  if (get_library_version() != PLIB_LIBRARY_VERSION)
    return -1;
  packet_demo();
  socket_demo();
  ppac_demo();
  return 0;
}
