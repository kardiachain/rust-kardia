use std::{net::IpAddr, collections::HashMap};

use tokio::net::TcpStream;

// ConnSet is a lookup table for connections and all their ips.
pub trait ConnSetTrait {
    fn has(&self, conn: TcpStream) -> bool;
    fn has_ip(&self, ip: IpAddr) -> bool;
    fn set(&self, conn: TcpStream, ips: Vec<IpAddr>);
    fn remove(&self, conn: TcpStream);
    fn remove_addr(&self, addr: String);
}

pub struct ConnSetItem{
    conn: TcpStream,
    ips: [IpAddr],
}

pub struct ConnSet {
    // ksync.RWMutex

    conns: HashMap<String, &'static ConnSetItem>
}

impl ConnSetTrait for ConnSet {
    fn has(&self, conn: TcpStream) -> bool {
        todo!()
    }

    fn has_ip(&self, ip: IpAddr) -> bool {
        todo!()
    }

    fn set(&self, conn: TcpStream, ips: Vec<IpAddr>) {
        todo!()
    }

    fn remove(&self, conn: TcpStream) {
        todo!()
    }

    fn remove_addr(&self, addr: String) {
        todo!()
    }
}