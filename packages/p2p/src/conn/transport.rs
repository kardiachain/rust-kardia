use std::net::{SocketAddr, ToSocketAddrs};

use async_trait::async_trait;
use eyre::Result;
use futures_core::Stream;

/// Trait which describes the core concept of a connection between two peers established by
/// `[Transport]`.
#[async_trait]
pub trait Connection: Send {
    /// Errors emitted by the connection.
    type Error;
    /// Read end of a bidirectional stream. Carries a finite stream of framed messages. Decoding is
    /// left to the caller and should correspond to the type of stream.
    type StreamRead: Stream<Item = Result<Vec<u8>>> + Send + Sync;
    /// Send end of a stream.
    type StreamSend: StreamSend;

    /// Returns the list of advertised addresses known for this connection.
    fn advertised_addrs(&self) -> Vec<SocketAddr>;
    /// Tears down the connection  and releases all attached resources.
    ///
    /// # Errors
    ///
    /// * If release of attached resources failed.
    async fn close(&self) -> Result<()>;
    /// Returns the local address for the connection.
    fn local_addr(&self) -> SocketAddr;
    /// Opens a new bi-bidirectional stream for the given [`StreamId`].
    ///
    /// # Errors
    ///
    /// * If the stream type is not supported.
    /// * If the peer is gone.
    /// * If resources necessary for the stream creation aren't available/accessible.
    async fn open_bidirectional(
        &self,
        stream_id: StreamId,
    ) -> Result<(Self::StreamRead, Self::StreamSend), Self::Error>;
    /// Public key of the remote peer.
    // fn public_key(&self) -> PublicKey;
    /// Local address(es) to the endpoint listens on.
    fn remote_addr(&self) -> SocketAddr;
}

pub trait Transport {
    type Connection: Connection;
    // type Endpoint: Endpoint<Connection = <Self as Transport>::Connection>;
    // type Incoming: Stream<Item = Result<<Self as Transport>::Connection>> + Send + Sync;

    // async fn bind(self, bind_info: BindInfo) -> Result<(Self::Endpoint, Self::Incoming)>;
}

/// Trait that describes the send end of a stream.
#[async_trait]
pub trait StreamSend: Send + Sync {
    /// Sends the message to the peer over the open stream. `msg` should be a valid and properly
    /// encoded byte array according to the supported messages of the stream.
    ///
    /// # Errors
    ///
    /// * If the underlying I/O operations fail.
    /// * If the stream is closed.
    /// * If the peer is gone
    async fn send<B: AsRef<[u8]>>(msg: B) -> Result<()>;
}

/// Known list of typed streams.
#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub enum StreamId {
    /// Stream to exchange message concerning Peer Exchange.
    Pex,
}