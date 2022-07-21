# p2p

The p2p package provides an abstraction around peer-to-peer communication.

## P2P Multiplex Connection
# MConnection
MConnection is a multiplex connection that supports multiple independent streams with distinct quality of service guarantees atop a single TCP connection. Each stream is known as a Channel and each Channel has a globally unique byte id. Each Channel also has a relative priority that determines the quality of service of the Channel compared to other Channels. The byte id and the relative priorities of each Channel are configured upon initialization of the connection.

The MConnection supports three packet types:

- Ping
- Pong
- Msg