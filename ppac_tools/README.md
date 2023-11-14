# PPAC Tools
A collection of various tools for PPAC archives.

## `ppak_reader`
A simple PPAC reader/dumper. Writes known packets to .txt file and dumps unknown ones (w/o the packet header).

Usage:
```
cargo run -- {archive.pak}
```

## `reppac`
A simple PPAC repacker.

Usage:
```
cargo run -- {archive file or folder} {packing flag: true|false}
```

## `packets.hexpat`
An ImHex pattern file for PPAC archives. Currently only for versions <=3.
