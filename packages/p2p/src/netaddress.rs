// most of TCP functions are in this file
use key::Id;
use std::net::IpAddr;

// EmptyNetAddress defines the string representation of an empty NetAddress
const EMPTYNETADDRESS: &str = "<nil-NetAddress>";

// NetAddress defines information about a peer on the network
// including its ID, IP address, and port.
pub struct NetAddress {
    pub id: Id,
    pub ip: IpAddr,
    pub port: u16,
}

// IDAddressString returns id@hostPort. It strips the leading
// protocol from protocolHostPort if it exists.
pub fn ip_address_string(id: Id, protocol_host_port: String) -> String {

}

// NewNetAddress returns a new NetAddress using the provided TCP
// address. When testing, other net.Addr (except TCP) will result in
// using 0.0.0.0:0. When normal run, other net.Addr (except TCP) will
// panic. Panics if ID is invalid.
pub fn new_net_address() -> &NetAddress {

}

// NewNetAddressIPPort returns a new NetAddress using the provided IP
// and port number.
pub fn new_net_address_ip_port(ip_addr: IpAddr, port: u16) -> &NetAddress {
    &NetAddress{ip: ip_addr, port: port}
}