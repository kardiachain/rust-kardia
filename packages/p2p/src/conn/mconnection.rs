use std::time::{Duration, self};

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
pub const DEFAULT_MAX_PACKET_MSG_PAY_LOAD_SIZE: i32 = 1024;

pub const NUM_BATCH_PACKET_MSGS : i32 = 10;
pub const MIN_READ_BUFFER_SIZE : i32 = 1024;
pub const MIN_WRITE_BUFFER_SIZE : i32 = 65536;
pub const UPDATE_STATS : Duration = Duration::from_secs(5);

// some of these defaults are written in the user config
// flushThrottle, sendRate, recvRate
pub const DEFAULT_FLUSH_THROTTLE : Duration = Duration::from_millis(100);


pub const DEFAULT_SEND_QUEUE_CAPACITY : i32 = 1;
pub const DEFAULT_RECV_BUFFER_CAPACITY : i32 = 4096;
pub const DEFAULT_SEND_RATE : i64 = 512000; // 500KB/s
pub const DEFAULT_RECV_RATE : i64 = 512000; // 500KB/s
pub const DEFAULT_RECV_MESSAGE_CAPACITY : i32 = 22020096; // 21MB
pub const DEFAULT_SEND_TIMEOUT : Duration = Duration::from_secs(10);
pub const DEFAULT_PING_INTERVAL : Duration = Duration::from_secs(60);
pub const DEFAULT_PONG_TIMEOUT : Duration = Duration::from_secs(45);



type ReceiveCbFunc = fn(u8, Vec<u8>);

type ErrorCbFunc = fn();

// initializing MConnection struct
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct MConnection {
    // BaseService

    // connection (net.Conn)

    // send channel in go

    // pong channel in go

    channels: Vec<Channel>,

    errored: u32,
    on_receive: ReceiveCbFunc,
    on_error: ErrorCbFunc,
    // config
    config: MConnConfig,

    created : time::Instant,

    _max_packet_msg_size : i32,
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

    fn max_packet_msg_size(&self) -> i32 {
        self._max_packet_msg_size
    } 

    fn status(&self)  {}
}

// MConnConfig is a MConnection configuration
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct MConnConfig {
    pub send_rate: i64,
    pub recv_rate: i64,

    // Maximum payload size
    pub max_packet_msg_payload_size : i32,

    // Interval to flush writes (throttled)
    pub flush_throttle : Duration,

    // Interval to send pings
    pub ping_interval : Duration,

    // Maximum wait time for pongs
    pub pong_timeout : Duration,

    
}

pub fn default_kai_conn_config() -> MConnConfig {
    MConnConfig { 
        send_rate: DEFAULT_SEND_RATE,
        recv_rate: DEFAULT_RECV_RATE,
        max_packet_msg_payload_size : DEFAULT_MAX_PACKET_MSG_PAY_LOAD_SIZE,
        flush_throttle : DEFAULT_FLUSH_THROTTLE,
        ping_interval : DEFAULT_PING_INTERVAL,
        pong_timeout : DEFAULT_PONG_TIMEOUT,
    }
}

pub fn new_mconn() {
   
}

pub fn new_mconn_with_config() {
    
}

pub struct ConnectionStatus {
    duration : Duration,
    send_monitor : String, // ??
    recv_monitor : String,
    channels : Vec<ChannelStatus>,
}

pub struct ChannelStatus {
    id : u8,
    send_queue_capacity : i32,
    send_queue_size : i32,
    priority : i32,
    recently_sent : i64,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct ChannelDescriptor {
    id : u8,
    priority : i32,

    send_queue_capacity : i32,
    recv_message_capacity : i32,

    recv_buffer_capacity : i32,
}

impl ChannelDescriptor {
    pub fn fill_defaults(&mut self) -> &Self {
        if self.send_queue_capacity == 0 {
            self.send_queue_capacity = DEFAULT_SEND_QUEUE_CAPACITY;
        }

        if self.recv_buffer_capacity == 0 {
            self.recv_buffer_capacity = DEFAULT_RECV_BUFFER_CAPACITY;
        }

        if self.recv_message_capacity == 0 {
            self.recv_message_capacity = DEFAULT_RECV_MESSAGE_CAPACITY;
        }
        self
    }
    pub fn set_logger(&mut self) -> Self {
        *self
    }

    fn send_bytes(bytes : Vec<u8>) -> bool {
        false
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Channel {
    conn : &'static MConnection,
    desc : ChannelDescriptor,
    send_queue : Option<String>, // ??
    send_queue_size : Option<i32>,
    recving : Option<Vec<u8>>,
    sending : Option<Vec<u8>>,
    recently_sent : Option<i64>,

    max_packet_msg_payload_size : i32,
}

fn new_channel(conn : &'static MConnection, desc : &mut ChannelDescriptor) -> Channel {
    let desc = *desc.fill_defaults();
    if desc.priority <= 0 {
        panic!("Channel default priority must be a positive integer");
    }

    Channel { conn: conn, 
        desc: desc, 
        send_queue: None, 
        send_queue_size: None, 
        recving: None, 
        sending: None, 
        recently_sent: None, 
        max_packet_msg_payload_size: conn.config.max_packet_msg_payload_size }
}

impl Channel {
    pub fn set_logger(&self) {}

    // Queues message to send to this channel.
    // Goroutine-safe
    // Times out (and returns false) after defaultSendTimeout
    fn send_bytes(&self, bytes : Vec<u8>) -> bool {
        false
    }

    // Queues message to send to this channel.
    // Nonblocking, returns true if successful.
    // Goroutine-safe
    fn try_send_bytes(&self, bytes : Vec<u8>) -> bool {
        false
    }

    fn load_send_queue_size(&self) -> Option<i32> {
        self.send_queue_size
    }

    fn can_send(&self) -> bool {
        false
    }

    // Returns true if any PacketMsgs are pending to be sent.
    // Call before calling nextPacketMsg()
    // Goroutine-safe
    fn is_send_pending(&self) -> bool {
        false
    }

    // Creates a new PacketMsg to send.
    // Not goroutine-safe
    fn next_packet_msg() {}

    // Writes next PacketMsg to w and updates c.recentlySent.
    // Not goroutine-safe
    fn write_packet_msg_to() {}

    // Handles incoming PacketMsgs. It returns a message bytes if message is
    // complete. NOTE message bytes may change on next call to recvPacketMsg.
    // Not goroutine-safe
    fn rec_packet_msg() {}

    // Call this periodically to update stats for throttling purposes.
    // Not goroutine-safe
    fn update_stats() {}

}

//----------------------------------------
// Packet

// mustWrapPacket takes a packet kind (oneof) and wraps it in a kp2p.Packet message.
fn must_wrap_packet() {}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_default_kai_conn_config() {
        let default_kconn = MConnConfig { 
            send_rate: DEFAULT_SEND_RATE,
            recv_rate: DEFAULT_RECV_RATE,
            max_packet_msg_payload_size : DEFAULT_MAX_PACKET_MSG_PAY_LOAD_SIZE,
            flush_throttle : DEFAULT_FLUSH_THROTTLE,
            ping_interval : DEFAULT_PING_INTERVAL,
            pong_timeout : DEFAULT_PONG_TIMEOUT,
        };

        assert_eq!(default_kai_conn_config(), default_kconn)
    }
}