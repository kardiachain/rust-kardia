use ed25519_dalek as ed25519;
use flume::{self, Sender};
use std::net::{SocketAddr, TcpListener, TcpStream};

pub struct MConnection {
    public_key: ed25519::PublicKey,

    // stream clone for shutting down connection
    stream: TcpStream,
    local_addr: SocketAddr,
    peer_addr: SocketAddr,

    sender: Sender<()>,
}
