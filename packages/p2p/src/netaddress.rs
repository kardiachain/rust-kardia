// most of TCP functions are in this file

use crate::key::{Id, ID_BYTE_LENGTH};
use crate::errors::{self, Error::*};
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::error::Error;

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

    pub fn dial_string(&self) -> &str {
        todo!()
    }

    // Dial calls net.Dial on the address.
    pub fn dial(&self) {
        todo!()
    }

    // DialTimeout calls TCPStream.connect_timeout() on the address.
    pub fn dial_timeout(&self) {
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

    // validate id
    // match validate_id(&id) {
    //     Some(Box::new(dyn Error)) => {
    //         panic!("Invalid ID");
    //     },
    //     _ => {}
    // }

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
pub fn new_net_address_string(addr : &str) -> Result<&NetAddress, Box<dyn Error>> {
    let mut addr_without_protocol = remove_protocol_if_defined(addr);
    let spl: Vec<&str> = addr_without_protocol.split("@").collect();
    if spl.len() != 2 {
        return Err(Box::new(ErrNetAddressNoId{addr: addr.to_string()}))
    }

    // get ID 
    validate_id(spl[0].to_string());
    // Ok(())
    todo!()

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
    // if id_bytes.is_err() {
    //     return Err(Box::new(id_bytes.err().unwrap()))
    // }

    // len id_bytes == ID_BYTE_LENGTH
    if id_bytes.len() as i32 != ID_BYTE_LENGTH {
        // return Err("invalid hex length - got {id_bytes.unwrap().len():?}, expected {ID_BYTE_LENGTH:?}".to_string())
        return Err(Box::new(MsgError { err: "invalid hex length - got {id_bytes.unwrap().len():?}, expected {ID_BYTE_LENGTH:?}".to_string() }))
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{remove_protocol_if_defined, validate_id};
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
}
