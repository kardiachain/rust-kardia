This PR introduces MConnection transport that multiplexes messages from individual streams onto a single TcpStream.

# Implemented approach
For each stream, a pair of ReadVirtualStream/WriteVirtualStream is created. flume channels are being used to send / receive PacketMsg to / from the secret connection. MConnection spawn a main loop, which reads and writes to the secret connection.

# Previous alterations
For each stream, it creates a VirtualStream which holds a copy of the SecretConnection. Note each copy holds a reference to the same underlying TCP stream, so they all will read the same data. VirtualStream checks if message.channel_id equals its stream_id. If so, it passes the message up to the caller.

The downside of the "copy approach" is each copy now has a separate Nonce. Therefore, it's possible for the remote peer to receive multiple messages with the same nonce (imagine we have 3 reactors: 1,1,1, 2,2, 3,3,3). But I think it should be fine as long as the remote peer is aware of this fact.

The other approach I've tried is sharing a single reference to SecretConnection between multiple VirtualStreams. But then I've spend too much time dealing with lifetimes and it ended up being too complex. Let me know if I should try again.