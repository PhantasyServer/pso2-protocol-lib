cimport packetlib
from enum import Enum
import json
import time
import socket
import struct


class PacketType(Enum):
    NGS = packetlib.NGS
    Classic = packetlib.Classic
    NA = packetlib.NA
    JP = packetlib.JP
    Vita = packetlib.Vita
    Raw = packetlib.Raw


class OutputType(Enum):
    Packet = packetlib.OutputPacket
    Raw = packetlib.OutputRaw
    Both = packetlib.OutputBoth


cdef unicode to_unicode(const char* s):
    return s.decode("UTF-8", "strict")


cdef class Packet:
    cdef packetlib.PLIB_Packet* p

    def __dealloc__(self):
        packetlib.free_packet(self.p)
        self.p = NULL

    cpdef Packet clone(self):
        packet = Packet()
        packet.p = packetlib.clone_packet(self.p)
        return packet


cdef class PacketFactory:
    cdef packetlib.PLIB_PacketWorker* pw

    def __cinit__(self,
                  pt = PacketType.Classic):
        if type(pt) is not PacketType:
            raise TypeError("pt is not of type PacketType")
        self.pw = packetlib.new_worker(pt.value, packetlib.JSON)

    def __dealloc__(self):
        packetlib.free_worker(self.pw)
        self.pw = NULL

    def __throw_err(self):
        err = <const char*>packetlib.get_pw_error(self.pw)
        if err is not NULL:
            raise RuntimeError(to_unicode(err))

    cpdef Packet raw_to_packet(self, bytes data):
        ptr = <const uint8_t*>data
        size = len(data)
        packet_ptr = packetlib.raw_to_packet(self.pw, ptr, size)
        self.__throw_err()
        packet = Packet()
        packet.p = packet_ptr
        return packet

    cpdef Packet val_to_packet(self, data):
        string = json.dumps(data).encode("UTF-8")
        size = len(string) + 1
        packet_ptr = packetlib.ser_to_packet(self.pw, string, size)
        self.__throw_err()
        packet = Packet()
        packet.p = packet_ptr
        return packet

    def packet_to_val(self, Packet packet):
        data = packetlib.packet_to_ser(self.pw, packet.p)
        self.__throw_err()
        string = to_unicode(<const char*>data.ptr)
        packetlib.free_data(data)
        return json.loads(string)

    def packet_to_raw(self, Packet packet) -> bytes:
        data = packetlib.packet_to_raw(self.pw, packet.p)
        self.__throw_err()
        py_data = data.ptr[:data.size]
        packetlib.free_data(data)
        return py_data


cdef class Connection:
    cdef packetlib.PLIB_Connection* c

    def __dealloc__(self):
        packetlib.free_connection(self.c)
        self.c = NULL

    def __throw_err(self):
        err = <const char*>packetlib.get_conn_error(self.c)
        if err is not NULL:
            raise RuntimeError(to_unicode(err))

    cpdef str ip(self):
        ip_int = packetlib.get_conn_ip(self.c)
        return socket.inet_ntoa(struct.pack("!I", ip_int))

    cpdef write_packet(self, Packet packet):
        packetlib.conn_write_packet(self.c, packet.p)
        self.__throw_err()

    cpdef Packet read_packet(self):
        packetlib.conn_read_packet(self.c)
        self.__throw_err()
        ptr = packetlib.conn_get_data(self.c)
        packet = Packet()
        packet.p = ptr
        return packet


cdef class SocketFactory:
    cdef packetlib.PLIB_SocketFactory* sf

    def __cinit__(self):
        self.sf = packetlib.new_factory()

    def __dealloc__(self):
        packetlib.free_factory(self.sf)
        self.sf = NULL

    def __throw_err(self):
        err = <const char*>packetlib.get_sf_error(self.sf)
        if err is not NULL:
            raise RuntimeError(to_unicode(err))

    cpdef create_listener(self, str addr):
        string = addr.encode("UTF-8")
        packetlib.create_listener(self.sf, string)
        self.__throw_err()

    cpdef Connection accept_connection(self,
                                       pt = PacketType.Classic):
        if type(pt) is not PacketType:
            raise TypeError("pt is not of type PacketType")

        # just for show
        packetlib.listener_nonblocking(self.sf, True)

        while True:
            sr = packetlib.accept_listener(self.sf)
            if sr == packetlib.Blocked:
                time.sleep(0.001)
                continue
            elif sr == packetlib.SocketError:
                self.__throw_err()
            else:
                conn = Connection()
                conn.c = packetlib.get_connection(self.sf,
                                                  pt.value,
                                                  NULL,
                                                  NULL)
                return conn

    cpdef Connection from_socket(self, soc,
                                 pt = PacketType.Classic):
        if type(pt) is not PacketType:
            raise TypeError("pt is not of type PacketType")

        fd = soc.fileno()
        fd_clone = packetlib.clone_fd(self.sf, fd)
        self.__throw_err()
        conn = Connection()
        conn.c = packetlib.new_connection(fd_clone,
                                          pt.value,
                                          NULL,
                                          NULL)
        return conn


class PacketData:
    time = 0
    direction = 0
    protocol = 0
    packet = None
    raw = None

cdef class PPACReader:
    cdef packetlib.PLIB_PPACReader* r

    def __cinit__(self, str path):
        string = path.encode("UTF-8")
        self.r = packetlib.new_reader(string)
        self.__throw_err()

    def __dealloc__(self):
        packetlib.free_reader(self.r)
        self.r = NULL

    def __throw_err(self):
        err = <const char*>packetlib.get_reader_error(self.r)
        if err is not NULL:
            raise RuntimeError(to_unicode(err))

    cpdef set_out_type(self, out_type):
        if type(out_type) is not OutputType:
            raise TypeError("out_type is not of type OutputType")

        packetlib.set_out_type(self.r, out_type.value)

    cpdef read_packet(self):
        result = packetlib.read_packet(self.r)
        self.__throw_err()
        data = PacketData()
        if result == packetlib.Ok or result == packetlib.RawOnly:
            pd = packetlib.get_reader_data(self.r)
            data.time = pd.time
            data.direction = pd.direction
            data.protocol = pd.protocol_type
            if pd.data is not NULL:
                packet = Packet()
                packet.p = pd.data
                data.packet = packet
            if pd.raw_ptr is not NULL and pd.raw_size != 0:
                pd.raw_ptr[:pd.raw_size]
            return data
        else:
            return None
