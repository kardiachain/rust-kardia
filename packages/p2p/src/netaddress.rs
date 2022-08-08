// most of TCP functions are in this file
use crate::key::Id;
use std::net::IpAddr;

// EmptyNetAddress defines the string representation of an empty NetAddress
const EMPTY_NET_ADDRESS: &str = "<nil-NetAddress>";

// NetAddress defines information about a peer on the network
// including its ID, IP address, and port.
pub struct NetAddress {
    pub id: Id,
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
        self.id != "".to_string()
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
    todo!()
}

// NewNetAddress returns a new NetAddress using the provided TCP
// address. When testing, other net.Addr (except TCP) will result in
// using 0.0.0.0:0. When normal run, other net.Addr (except TCP) will
// panic. Panics if ID is invalid.
pub fn new_net_address(addr : &str) -> Option<&NetAddress> {
    todo!()
}

// NewNetAddressString returns a new NetAddress using the provided address in
// the form of "ID@IP:Port".
// Also resolves the host if host is not an IP.
// Errors are of type ErrNetAddressXxx where Xxx is in (NoID, Invalid, Lookup)
pub fn new_net_address_string(addr : &str) -> Option<&NetAddress> {
    todo!()
}

pub fn new_net_address_strings(addrs : Vec<&str>) -> Option<&NetAddress> {
    todo!()
}

// NewNetAddressIPPort returns a new NetAddress using the provided IP
// and port number.
pub fn new_net_address_ip_port(ip_addr: &IpAddr, port: u16) -> Option<NetAddress>{
    todo!()
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