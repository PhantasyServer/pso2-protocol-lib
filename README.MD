# PSO2 Packet Library

Library that aims to provide easy(er) interaction with the PSO2 protocol.

## Targeted versions

 * Pre-NGS (Global: 6.1253.3, JP: 6.1201.0, Vita: 6.1001.2)
 * NGS

## Compatability with other languages

This library has an [FFI](packetlib_ffi). It allows for working with packets using JSON or MessagePack.

## Notes

 - For more information about the protocol, see [this file](packet_docs/README.MD) (currently unmaintained, contributions are welcome).
 - Because a lot of information about the protocol is unknown, expect breaking changes (such as adding/deleting/renaming of struct's fields).
 - Any contributions are welcome (e.g. code style suggestions, comments, additional packet information, etc.)
