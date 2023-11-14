# PPAC file format
This is a packet log format. All packets are logged and saved in the ".pak" file. Here's the description of the format:

Header:

| Field   | Type      | Notes                                                                                         |
|---------|-----------|-----------------------------------------------------------------------------------------------|
| Header  | char[4]   | Always `PPAK`                                                                                 |
| Version | byte      | = 2..4                                                                                        |
| Client  | byte      | For version >=3 <br> 0 - Classic (generic) <br> 1 - NGS <br> 2 - NA <br> 3 - JP <br> 4 - Vita |
| Packed  | byte      | For version >=4 <br> 1 if the following data is zstd packed.                                  |
| Packets | Packet[_] | Format in the next table                                                                      |

Packet format: 

| Field     | Type    | Notes                                          |
|-----------|---------|------------------------------------------------|
| Timestamp | u128    | Nanosecond since Unix epoch                    |
| Direction | byte    | 0 - Client -> Server <br> 1 - Server -> Client |
| Data size | u64     | Length of the following data                   |
| Data      | byte[_] | Full decrypted packet                          |

