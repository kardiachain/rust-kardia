// type BaseProtocol interface {
// 	// AddPeer is called by the protocol manager when a new peer is added.
// 	AddPeer(p p2p.Peer)

// 	// RemovePeer is called by the switch when the peer is stopped (due to error
// 	// or other reason).
// 	RemovePeer(p *p2p.Peer, reason interface{})

// 	// Broadcast message to all other peers.
// 	Broadcast(msg interface{}, msgType uint64)
// }