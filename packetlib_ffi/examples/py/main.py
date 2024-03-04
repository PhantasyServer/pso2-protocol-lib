import packetlib
import socket
from datetime import datetime


def packet_demo():
    pw = packetlib.PacketFactory()

    # example of parsing packets
    packet = pw.raw_to_packet(bytes([8, 0, 0, 0, 3, 4, 0, 0]))
    print(pw.packet_to_val(packet))

    # example of creating packets
    data = {"LoadLevel": {}}
    packet = pw.val_to_packet(data)
    print(pw.packet_to_raw(packet))

    # example of an error
    data = {"Invalid": {}}
    try:
        packet = pw.val_to_packet(data)
    except RuntimeError as e:
        print(e)


def socket_demo():
    pw = packetlib.PacketFactory()
    sf = packetlib.SocketFactory()

    # create a new listener
    sf.create_listener("0.0.0.0:13370")

    # wait for connection
    conn = sf.accept_connection(packetlib.PacketType.NGS)
    print(f"Ip: {conn.ip()}")

    # write data to client
    data = {"LoadLevel": {}}
    conn.write_packet(pw.val_to_packet(data))

    # connect to sega server
    s = socket.socket()
    s.connect(("40.91.76.146", 12199))
    conn = sf.from_socket(s, packetlib.PacketType.NGS)
    print(f"Ip: {conn.ip()}")

    # read packet from server
    data = pw.packet_to_val(conn.read_packet())
    print(data)


def ppac_demo():
    pw = packetlib.PacketFactory()

    # open a packet archive
    reader = packetlib.PPACReader("test.pak")

    # set output type
    reader.set_out_type(packetlib.OutputType.Both)

    while True:
        data = reader.read_packet()
        if data is None:
            break
        print("----------")
        print(f"Time: {datetime.fromtimestamp(data.time)}")
        print(f"Direction: {data.direction}")
        print(f"Protocol Type: {data.protocol}")
        if data.packet is not None:
            print(f"Packet: {pw.packet_to_val(data.packet)}")
        elif data.raw is not None:
            print(f"RAW: {data.raw}")


packet_demo()
socket_demo()
ppac_demo()
