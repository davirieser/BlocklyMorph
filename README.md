
## Netcode branch

This branch holds the code that the blocks use for communicating with each other.

#### Inter-block Protocol specification

The Protocol used between the blocks is a message-based peer-to-peer communication protocol using sequence numbers and retransmits. 
See [TCP](https://en.wikipedia.org/wiki/Transmission_Control_Protocol)
The sequence numbers are seperate for each pair of blocks that try and communicate with each other.
The messages are serialized using a modified version of the Redis Serialization Protocol.

##### Serialization 

See [Redis Docs](https://redis.io/docs/latest/develop/reference/protocol-spec/#resp-protocol-description).

The three major differences of this protocol to the redis serialization protocol is that:
- There are no simple strings, simple errors and pushes.
- Many of the type determining characters are changed.
- Some more types were introduced so that [serde](docs.rs/serde/latest/serde) can be used.

Here is a list of the types of values and the character used to determine them:

| Type                  | Character | Remarks                                    |
|-----------------------|-----------|--------------------------------------------|
| Unit                  | _         |                                            |
| None                  | -         |                                            |
| Bool                  | !         | 't' and 'f' are used as true and false     |
| Number                | +         | Can be both signed and unsigned numbers    |
| Floating Point Number | .         |                                            |
| Character             | ^         |                                            |
| String                | $         |                                            |
| Array                 | *         |                                            |
| Map                   | @         |                                            |
| Bytes                 | #         |                                            |

