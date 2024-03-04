package main

// #cgo LDFLAGS: -lpacketlib_ffi -L ../target/release/
// #include <stdlib.h>
// #include "../include/packetlib.h"
import "C"
import "unsafe"
import "errors"
import "fmt"
import "net"

type Packet struct {
	packet *C.PLIB_Packet
}

func (p Packet) Close() {
	C.free_packet(p.packet)
	p.packet = nil
}

//------------------------
// Packet example
//------------------------

type PacketWorker struct {
	worker *C.PLIB_PacketWorker
}

func new_packetworker() PacketWorker {
	return PacketWorker{
		worker: C.new_worker(C.Classic, C.JSON),
	}
}

func (pw PacketWorker) Close() {
	C.free_worker(pw.worker)
	pw.worker = nil
}

func (pw *PacketWorker) raw_to_packet(b []byte) (Packet, error) {
	cb := C.CBytes(b)
	defer C.free(unsafe.Pointer(cb))
	ptr := C.raw_to_packet(pw.worker, (*C.uchar)(cb), (C.ulong)(len(b)))
	err := C.get_pw_error(pw.worker)
	if err != nil {
		err_str := C.GoString((*C.char)(unsafe.Pointer(err)))
		return Packet{}, errors.New(err_str)
	}
	return Packet{
		packet: ptr,
	}, nil
}

func (pw *PacketWorker) ser_to_packet(s string) (Packet, error) {
	cs := C.CString(s)
	defer C.free(unsafe.Pointer(cs))
	ptr := C.ser_to_packet(pw.worker, (*C.uchar)(unsafe.Pointer(cs)), (C.ulong)(len(s)+1))
	err := C.get_pw_error(pw.worker)
	if err != nil {
		err_str := C.GoString((*C.char)(unsafe.Pointer(err)))
		return Packet{}, errors.New(err_str)
	}
	return Packet{
		packet: ptr,
	}, nil
}

func (pw *PacketWorker) packet_to_raw(p Packet) []byte {
	buf := C.packet_to_raw(pw.worker, p.packet)
	if buf.size != 0 && buf.ptr != nil {
		return C.GoBytes(unsafe.Pointer(buf.ptr), (C.int)(buf.size))
	}
	return []byte{}
}

func (pw *PacketWorker) packet_to_ser(p Packet) (string, error) {
	buf := C.packet_to_ser(pw.worker, p.packet)
	err := C.get_pw_error(pw.worker)
	if err != nil {
		err_str := C.GoString((*C.char)(unsafe.Pointer(err)))
		return "", errors.New(err_str)
	}
	if buf.size != 0 && buf.ptr != nil {
		return C.GoString((*C.char)(unsafe.Pointer(buf.ptr))), nil

	}
	return "", nil

}

func packet_test() {
	worker := new_packetworker()
	defer worker.Close()

	// example of parsing packets
	packet, _ := worker.raw_to_packet([]byte{8, 0, 0, 0, 3, 4, 0, 0})
	defer packet.Close()
	str, _ := worker.packet_to_ser(packet)
	fmt.Println(str)

	// example of creating packets
	packet2, _ := worker.ser_to_packet("{\"ClientPing\":{}}")
	defer packet2.Close()
	fmt.Printf("% x\n", worker.packet_to_raw(packet2))

	// example of an error
	_, err := worker.ser_to_packet("{\"Invalid\":{}}")
	if err != nil {
		fmt.Println(err)
	}
}

//------------------------
// Socket example
//------------------------

type SocketFactory struct {
	sf *C.PLIB_SocketFactory
}

type Connection struct {
	conn *C.PLIB_Connection
}

func create_sf() SocketFactory {
	return SocketFactory{
		sf: C.new_factory(),
	}
}

func (sf SocketFactory) Close() {
	C.free_factory(sf.sf)
	sf.sf = nil
}

func (conn Connection) Close() {
	C.free_connection(conn.conn)
	conn.conn = nil
}

func (sf *SocketFactory) create_listener(addr string) error {
	cs := C.CString(addr)
	defer C.free(unsafe.Pointer(cs))
	C.create_listener(sf.sf, (*C.schar)(unsafe.Pointer(cs)))
	err := C.get_sf_error(sf.sf)
	if err != nil {
		err_str := C.GoString((*C.char)(unsafe.Pointer(err)))
		return errors.New(err_str)
	}
	return nil
}

func (sf *SocketFactory) accept_connection() (Connection, error) {
	// just for show
	C.listener_nonblocking(sf.sf, true)

	sr := C.accept_listener(sf.sf)
	for {
		if sr == C.Ready {
			break
		}
		sr = C.accept_listener(sf.sf)
		if sr == C.Blocked {
			continue
		} else if sr == C.SocketError {
			err := C.get_sf_error(sf.sf)
			err_str := C.GoString((*C.char)(unsafe.Pointer(err)))
			return Connection{}, errors.New(err_str)

		}
	}
	return Connection{
		conn: C.get_connection(sf.sf, C.Classic, nil, nil),
	}, nil
}

func connection_from_net(go_conn net.Conn) (Connection, error) {
	defer go_conn.Close()
	sf := create_sf()
	defer sf.Close()
	file, err := go_conn.(*net.TCPConn).File()
	if err != nil {
		return Connection{}, err
	}
	defer file.Close()
	fd := file.Fd()
	fd_clone := C.clone_fd(sf.sf, (C.long)(fd))
	err_ptr := C.get_sf_error(sf.sf)
	if fd_clone == -1 && err_ptr != nil {
		err_str := C.GoString((*C.char)(unsafe.Pointer(err_ptr)))
		return Connection{}, errors.New(err_str)
	}
	file.Close()
	conn := C.new_connection(fd_clone, C.NGS, nil, nil)
	return Connection{conn: conn}, nil
}

func (conn *Connection) ip() [4]byte {
	ip := [4]byte{0, 0, 0, 0}
	ip_num := C.get_conn_ip(conn.conn)
	// some really scary stuff
	ip_bytes := (*[4]byte)(unsafe.Pointer(&ip_num))
	for i := 0; i < 4; i++ {
		ip[i] = ip_bytes[3-i]
	}

	return ip
}

func (conn *Connection) write_packet(packet Packet) error {
	C.conn_write_packet(conn.conn, packet.packet)
	err := C.get_conn_error(conn.conn)
	if err != nil {
		err_str := C.GoString((*C.char)(unsafe.Pointer(err)))
		return errors.New(err_str)
	}
	return nil
}
func (conn *Connection) read_packet() (Packet, error) {
	C.conn_read_packet(conn.conn)
	err := C.get_conn_error(conn.conn)
	if err != nil {
		err_str := C.GoString((*C.char)(unsafe.Pointer(err)))
		return Packet{}, errors.New(err_str)
	}
	ptr := C.conn_get_data(conn.conn)
	return Packet{
		packet: ptr,
	}, nil
}

func socket_test() {
	pf := new_packetworker()
	defer pf.Close()
	sf := create_sf()
	defer sf.Close()

	// create a new listener
	err := sf.create_listener("0.0.0.0:13370")
	if err != nil {
		fmt.Println(err)
		return
	}

	// wait for connection
	conn, err := sf.accept_connection()
	if err != nil {
		fmt.Println(err)
		return
	}
	defer conn.Close()
	fmt.Printf("Ip: %d\n", conn.ip())

	// write data to client
	packet, _ := pf.ser_to_packet("{\"LoadLevel\":{}}")
	defer packet.Close()
	err = conn.write_packet(packet)
	if err != nil {
		fmt.Println(err)
		return
	}

	// connect to sega server
	go_conn, err := net.Dial("tcp", "40.91.76.146:12199")
	if err != nil {
		fmt.Println(err)
		return
	}
	conn2, err := connection_from_net(go_conn)
	if err != nil {
		fmt.Println(err)
		return
	}
	defer conn2.Close()
	fmt.Printf("Ip: %d\n", conn2.ip())

	// read packet from server
	packet2, err := conn2.read_packet()
	if err != nil {
		fmt.Println(err)
		return
	}
	defer packet2.Close()
	str, _ := pf.packet_to_ser(packet2)
	fmt.Println(str)
}

//------------------------
// PPAC reader example
//------------------------

type PPACReader struct {
	reader *C.PLIB_PPACReader
}

func create_reader(s string) (PPACReader, error) {
	out_st := PPACReader{
		reader: nil,
	}
	cs := C.CString(s)
	defer C.free(unsafe.Pointer(cs))
	reader := C.new_reader((*C.schar)(unsafe.Pointer(cs)))
	err := C.get_reader_error(reader)
	if err != nil {
		err_str := C.GoString((*C.char)(unsafe.Pointer(err)))
		return out_st, errors.New(err_str)
	}
	out_st.reader = reader
	return out_st, nil
}

func (reader PPACReader) Close() {
	C.free_reader(reader.reader)
	reader.reader = nil
}

func (reader *PPACReader) set_out_type(out_type uint32) {
	C.set_out_type(reader.reader, out_type)
}

type PacketData struct {
	Time      uint64
	Direction uint32
	Protocol  uint32
	Packet    Packet
	Raw       []byte
	Eof       bool
}

func (reader *PPACReader) read_packet() (PacketData, error) {
	out_pd := PacketData{
		Eof: true,
	}
	result := C.read_packet(reader.reader)
	err := C.get_reader_error(reader.reader)
	if err != nil {
		err_str := C.GoString((*C.char)(unsafe.Pointer(err)))
		return out_pd, errors.New(err_str)
	}
	if result == C.Ok || result == C.RawOnly {
		pd := C.get_reader_data(reader.reader)
		out_pd.Time = (uint64)(pd.time)
		out_pd.Direction = pd.direction
		out_pd.Protocol = pd.protocol_type
		out_pd.Eof = false
		if pd.data != nil {
			out_pd.Packet = Packet{packet: pd.data}
		}
		if pd.raw_ptr != nil && pd.raw_size != 0 {
			out_pd.Raw = C.GoBytes(unsafe.Pointer(pd.raw_ptr), (C.int)(pd.raw_size))
		}
	}
	return out_pd, nil
}

func ppac_test() {
	pw := new_packetworker()
	defer pw.Close()
	reader, err := create_reader("test.pak")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer reader.Close()
	reader.set_out_type(C.OutputBoth)
	data, err := reader.read_packet()
	for {
		if data.Eof || err != nil {
			break
		}
		fmt.Printf("----------\n")
		fmt.Printf("Time: %d\n", data.Time)
		fmt.Printf("Direction: %d\n", data.Direction)
		fmt.Printf("Protocol Type: %d\n", data.Protocol)
		if data.Packet.packet != nil {
			defer data.Packet.Close()
			str, _ := pw.packet_to_ser(data.Packet)
			fmt.Printf("Packet: %s\n", str)
		} else if data.Raw != nil {
			fmt.Printf("RAW % x\n", data.Raw)
		}
		data, err = reader.read_packet()
	}
	if err != nil {
		fmt.Println(err)
	}
}

func main() {
	if C.PLIB_API_VERSION != C.get_api_version() {
		return
	}
	packet_test()
	socket_test()
	ppac_test()
}
