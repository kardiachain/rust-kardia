use types::channel;
/*
Each peer has one `MConnection` (multiplex connection) instance.

__multiplex__ *noun* a system or signal involving simultaneous transmission of
several messages along a single channel of communication.

Each `MConnection` handles message transmission on multiple abstract communication
`Channel`s.  Each channel has a globally unique byte id.
The byte id and the relative priorities of each `Channel` are configured upon
initialization of the connection.

There are two methods for sending messages:
	func (m MConnection) Send(chID byte, msgBytes []byte) bool {}
	func (m MConnection) TrySend(chID byte, msgBytes []byte}) bool {}

`Send(chID, msgBytes)` is a blocking call that waits until `msg` is
successfully queued for the channel with the given id byte `chID`, or until the
request times out.  The message `msg` is serialized using Protobuf.

`TrySend(chID, msgBytes)` is a nonblocking call that returns false if the
channel's queue is full.

Inbound message bytes are handled with an onReceive callback function.
*/

type ReceiveCbFunc = fn (byte, Vec<byte>);

type ErrorCbFunc = fn (T);

// initializing MConnection struct
pub struct MConnection {
    // BaseService

    // connection (net.Conn)

    // send channel in go

    // pong channel in go

    channels: Vec<Channel>,

    // config
    config: MConnConfig,
}

impl MConnection {
    pub fn set_logger() {}

    pub fn on_start() {}

    fn stop_services() {}

    pub fn flush_stop() {}

    fn on_stop() {}

    /// can this fn implemented into Display trait?
    fn string() {}

    fn flush() {}

    fn stop_for_error() {}

    pub fn send() {}

    pub fn try_send() {}

    pub fn can_send() {}

    fn send_routine() {}

    fn send_some_packet_msgs() -> bool {
        false
    }

    fn send_packet_msg() -> bool {
        false
    }

    fn recv_routine() {}

    fn stop_pong_timer() {}

    fn max_packet_msg_size() -> i32 {

    }


    
}

// MConnConfig is a MConnection configuration
pub struct MConnConfig {
    
}

// initializing Channel struct 
pub struct Channel {

}

