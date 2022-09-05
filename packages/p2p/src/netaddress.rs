// most of TCP functions are in this file

use kai_utils::crypto::crypto;
use tokio::net::TcpStream;

use crate::key::{Id, ID_BYTE_LENGTH};
use crate::errors::{self, Error::*};
use std::net::{IpAddr, SocketAddr, ToSocketAddrs, Ipv4Addr};
use std::error::Error;
use std::time;

// EmptyNetAddress defines the string representation of an empty NetAddress
const EMPTY_NET_ADDRESS: &str = "<nil-NetAddress>";

// NetAddress defines information about a peer on the network
// including its ID, IP address, and port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetAddress {
    pub id: Option<Id>,
    pub ip: IpAddr,
    pub port: u16,
}

impl NetAddress {

    // ToProto converts a NetAddress to Protobuf.
    pub fn to_proto(&self) {
        todo!()
    }

    // Equals reports whether na and other are the same addresses,
    // including their ID, IP, and Port.
    pub fn equals(&self) -> bool {
        todo!()
    }

    // Same returns true is na has the same non-empty ID or DialString as other.
    pub fn same(&self) -> bool {
        todo!()
    }

    // String representation: <ID>@<IP>:<PORT>
    pub fn string(&self) -> &str {
        todo!()
    }

    pub fn dial_string(&self) -> String {
        if self.ip.is_ipv6() {
            return "[".to_owned() + &self.ip.to_string() + &"]:".to_owned() + &self.port.to_string()
        }
        self.ip.to_string() + ":" + &self.port.to_string()
    }

    // Dial calls net.Dial on the address.
    pub async fn dial(&self) -> Result<TcpStream, Box<dyn Error>> {
        let stream = TcpStream::connect(self.dial_string()).await?;
        Ok(stream)
    }

    // DialTimeout calls TCPStream.connect_timeout() on the address.
    pub fn dial_timeout(&self, timeout: time::Duration) -> Result<TcpStream, Box<dyn Error>> {
        
        todo!()
    }

    // Routable returns true if the address is routable.
    pub fn routable(&self) -> bool {
        todo!()
    }

    // For IPv4 these are either a 0 or all bits set address. For IPv6 a zero
    // address or one that matches the RFC3849 documentation address format.
    pub fn valid(&self) {
        todo!()
    }

    // HasID returns true if the address has an ID.
    // NOTE: It does not check whether the ID is valid or not.
    pub fn has_id(&self) -> bool {
        self.id != Some("".to_string())
    }

    // Local returns true if it is a local address.
    pub fn local(&self) -> bool {
        todo!()
    }

    pub fn reachabitity_to(&self) -> i32 {
        todo!()
    }

    pub fn rfc1918() -> bool {
        todo!()
    }

}

// IDAddressString returns id@hostPort. It strips the leading
// protocol from protocolHostPort if it exists.
pub fn ip_address_string(id: Id, protocol_host_port: &str) -> String {
    let host_port = remove_protocol_if_defined(protocol_host_port);
    let id_addr = id + "@" + host_port;
    return id_addr
}

// NewNetAddress returns a new NetAddress using the provided TCP
// address. When testing, other net.Addr (except TCP) will result in
// using 0.0.0.0:0. When normal run, other net.Addr (except TCP) will
// panic. Panics if ID is invalid.
pub fn new_net_address(id : Id, addr : &str) -> NetAddress {
    // parse into SocketAddr
    let socket_addr : SocketAddr = addr.parse().expect("cannot parse into SocketAddr");

    let ip = socket_addr.ip();
    let port = socket_addr.port();
    let mut na = new_net_address_ip_port(ip, port);
    na.id = Some(id);

    na

}

// NewNetAddressString returns a new NetAddress using the provided address in
// the form of "ID@IP:Port".
// Also resolves the host if host is not an IP.
// Errors are of type ErrNetAddressXxx where Xxx is in (NoID, Invalid, Lookup)
pub fn new_net_address_string(addr : &str) -> Result<NetAddress, Box<dyn Error>> {
    let mut addr_without_protocol = remove_protocol_if_defined(addr);
    let spl: Vec<&str> = addr_without_protocol.split("@").collect();
    if spl.len() != 2 {
        return Err(Box::new(ErrNetAddressNoId{addr: addr.to_string()}))
    }

    // get ID 
    match validate_id(spl[0].to_string()) {
        Ok(()) => {},
        Err(e) => {
            return Err(e)
        }
    }

    let id: Id = spl[0].to_string();
    addr_without_protocol = spl[1];

    // get host and port from addr_without_protocol
    let (host, port) = crypto::split_host_port(addr_without_protocol)?;

    if host.len() == 0 {
        return Err(Box::new(ErrNetAddressInvalid { addr: addr_without_protocol.to_string(), err: "host is empty".to_string() }))
    }

    let mut na = new_net_address_ip_port(host.parse::<IpAddr>()?, 1);
    na.id = Some(id);

    Ok(na)
}

pub fn new_net_address_strings(addrs : Vec<&str>) -> Option<&NetAddress> {
    todo!()
}

// NewNetAddressIPPort returns a new NetAddress using the provided IP
// and port number.
pub fn new_net_address_ip_port(ip_addr: IpAddr, port: u16) -> NetAddress {
    NetAddress { id: None, ip: ip_addr, port: port }
}

// NetAddressFromProto converts a Protobuf NetAddress into a native struct.
pub fn net_address_from_proto() -> Option<NetAddress> {
    todo!()
}

// NetAddressesFromProto converts a slice of Protobuf NetAddresses into a native slice.
pub fn net_addresses_from_proto() -> Option<NetAddress> {
    todo!()
}

// NetAddressesToProto converts a slice of NetAddresses into a Protobuf slice.
pub fn net_addresses_to_proto(nas :Vec<&NetAddress>) {
    todo!()
}

fn ip_net(ip : &str, ones : i32, bits : i32) {
    todo!()
}

fn remove_protocol_if_defined(addr : &str) -> &str {
    if addr.contains("://") {
        let (_protocol, host_port) = addr.split_once("://").expect("cannot remove protocol");
        return host_port
    }
    addr
}

fn validate_id(id : Id) -> Result<(), Box<dyn Error>> {
    if id.len() == 0 {
        // return Err("no ID".to_string())
        return Err(Box::new(MsgError { err: "No Id".to_string() }))
    }

    // hex decode string Id
    let id_bytes = hex::decode(&id)?;

    // len id_bytes == ID_BYTE_LENGTH
    if id_bytes.len() as i32 != ID_BYTE_LENGTH {
        // return Err("invalid hex length - got {id_bytes.unwrap().len():?}, expected {ID_BYTE_LENGTH:?}".to_string())
        return Err(Box::new(MsgError { err: "invalid hex length - got {id_bytes.unwrap().len():?}, expected {ID_BYTE_LENGTH:?}".to_string() }))
    }
    Ok(())
}

// IDAddressString returns id@hostPort. It strips the leading
// protocol from protocolHostPort if it exists.
pub fn id_address_string(id: Id, protocol_host_port: &str) -> String {
    let host_port = remove_protocol_if_defined(protocol_host_port);
    let v = vec![id.as_str(), "@", host_port];

    v.concat()
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, Ipv6Addr};

    use super::{remove_protocol_if_defined, validate_id, id_address_string, NetAddress};
    use crate::errors::Error::{self, MsgError};

    #[test]
    fn test_remove_protocol_if_define() {
        assert_eq!(remove_protocol_if_defined("tcp://0.0.0.0:3000"), "0.0.0.0:3000")
    }

    #[test]
    fn test_validate_id_with_no_id() {
        let e = validate_id("".to_string()).unwrap_err();
        assert!(e.is::<Error>())
        // assert!(matches!(e, Err(Box::new(MsgError{"No Id".to_string()}))))    
    }

    #[test]
    fn test_validate_id_with_wrong_id_byte_length() {
        // assert_eq!(validate_id("MockPeer".to_string()), Err("invalid hex length - got {id_bytes.unwrap().len():?}, expected 20".to_string()))
    }

    #[test]
    fn test_id_address_string_ok() {
        assert_eq!(id_address_string("123".to_string(), "tcp://0.0.0.0:3000"), "123@0.0.0.0:3000")
    }

    #[test] 
    fn test_dial_string_v4_ok() {
        let v4 = NetAddress{id: None, ip: std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port: 8080};

        assert_eq!(v4.dial_string(), "127.0.0.1:8080".to_string())
    }

    #[test]
    fn test_dial_string_v6_ok() {
        let v6 = NetAddress{id:None, ip: std::net::IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0xffff, 0xc00a, 0x2ff)), port: 8080};
        assert_eq!(v6.dial_string(), "[::ffff:192.10.2.255]:8080" )
    }
}
