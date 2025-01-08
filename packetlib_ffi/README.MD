# PSO2 Packet Library FFI

A basic FFI for communicating with the packet library.

## Usage

 1) Include `packetlib.h`.
 2) Check the library version (e.g. in C: `if (get_library_version() != PLIB_LIBRARY_VERSION) return -1`).
 3) Create a new packet worker with `new_worker`.
 4) Parse packets with `parse_packet` or create packets with `create_packet`. Don't forget to check for errors with `get_error`!
 5) Destroy the worker with `free_worker`

## Examples

You can find examples for various languages in the [examples](examples) folder. New language contributions are always welcome.

## Something is missing?

If you wish for some additional functionality, please create an issue, and we'll see what we can do.
