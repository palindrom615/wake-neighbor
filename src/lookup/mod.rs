use pnet::util::MacAddr;
use std::net::IpAddr::V4;
use std::net::Ipv4Addr;
use std::net::ToSocketAddrs;

use crate::lookup::arp::get_mac_addr;

pub mod arp;

pub fn lookup_ipv4(ipv4_addr: Ipv4Addr) -> MacAddr {
    get_mac_addr(ipv4_addr)
}

pub fn lookup_hostname(hostname: String) -> MacAddr {
    match (hostname, 0)
        .to_socket_addrs()
        .expect("Host not resolvable")
        .next()
        .expect("Host not resolvable")
        .ip()
    {
        V4(addr) => lookup_ipv4(addr),
        _ => panic!("Not Implemented for IP v4"),
    }
}

#[cfg(test)]
mod test {
    use log::debug;
    use std::net::ToSocketAddrs;
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }
    #[test]
    fn test_resolve() {
        init();
        debug!("{:?}", ("google.com", 0).to_socket_addrs());
    }
}
