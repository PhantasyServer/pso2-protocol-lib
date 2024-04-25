# PSO2 Packet Library

[<img alt="GitHub" src="https://img.shields.io/badge/github-PhantasyServer%2Fpso2--protocol--lib-blue?style=for-the-badge&logo=github">](https://github.com/PhantasyServer/pso2-protocol-lib)
[<img alt="crates.io" src="https://img.shields.io/crates/v/pso2packetlib?style=for-the-badge&logo=rust">](https://crates.io/crates/pso2packetlib)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/pso2packetlib?style=for-the-badge&logo=docs.rs">](https://docs.rs/pso2packetlib)

Library that aims to provide easy(er) interaction with the PSO2 (Phantasy Star Online 2) protocol.

## Targeted versions

 * Pre-NGS (Global: 6.1253.3, JP: 6.1201.0, Vita: 6.1001.2)
 * NGS

## Compatability with other languages

This library has an [FFI](packetlib_ffi). It allows for working with packets using JSON or MessagePack.

## Notes

 - Because a lot of information about the protocol is unknown, expect breaking changes (such as adding/deleting/renaming of struct's fields).
 - Any contributions are welcome (e.g. code style suggestions, comments, additional packet information, etc.)

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or conditions.
