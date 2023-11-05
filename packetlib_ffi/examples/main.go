package main

// #cgo LDFLAGS: -lpacketlib_ffi -L ../target/release/
// #include <stdlib.h>
// #include "../packetlib.h"
import "C"
import "unsafe"
import "errors"
import "fmt"

func parse_packet(worker *C.PacketWorker, b []byte) (string, error) {
	cb := C.CBytes(b)
	defer C.free(unsafe.Pointer(cb))
	buf := C.parse_packet(worker, (*C.uchar)(cb), (C.ulong)(len(b)))
	err := C.get_pw_error(worker)
	if err != nil {
		err_str := C.GoString((*C.char)(unsafe.Pointer(err)))
		return "", errors.New(err_str)
	}
	if buf.size != 0 && buf.ptr != nil {
		return C.GoString((*C.char)(unsafe.Pointer(buf.ptr))), nil

	}
	return "", nil
}

func create_packet(worker *C.PacketWorker, s string) ([]byte, error) {
	cs := C.CString(s)
	defer C.free(unsafe.Pointer(cs))
	buf := C.create_packet(worker, (*C.uchar)(unsafe.Pointer(cs)), (C.ulong)(len(s)+1))
	err := C.get_pw_error(worker)
	if err != nil {
		err_str := C.GoString((*C.char)(unsafe.Pointer(err)))
		return []byte{}, errors.New(err_str)
	}
	if buf.size != 0 && buf.ptr != nil {
		return C.GoBytes(unsafe.Pointer(buf.ptr), (C.int)(buf.size)), nil

	}
	return []byte{}, nil
}

func main() {
	if C.API_VERSION != C.get_api_version() {
		return
	}
	worker := C.new_worker(C.Classic, C.JSON)
	defer C.free_worker(worker)

	// example of parsing packets
	str, err := parse_packet(worker, []byte{8, 0, 0, 0, 3, 4, 0, 0})
	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Println(str)
	}

	// example of creating packets
	buf, err := create_packet(worker, "{\"ClientPing\":{}}")
	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Printf("% x\n", buf)
	}

	// example of an error
	buf, err = create_packet(worker, "{\"Invalid\":{}}")
	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Printf("% x\n", buf)
	}
}
