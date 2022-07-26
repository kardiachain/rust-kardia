use conn::MConnection;
// peer implements Peer.
//
// Before using a peer, you will need to perform a handshake on connection.
// go-kardia/p2p/peer.go#L100
struct Peer {
    // raw peerConn and the multiplex connection
    peer_conn: PeerConn,
    mconn: &MConnection,

    // peer's node info and the channel it knows about
	// channels = nodeInfo.Channels
	// cached to avoid copying nodeInfo in hasChannel
    
    channels: Vec<byte>,
}

// peerConn contains the raw connection and its config.
struct PeerConn {
}