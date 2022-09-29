use pnet::util::MacAddr;
use std::net::Ipv4Addr;

pub fn lookup_ipv4(ipv4_addr: Ipv4Addr) -> MacAddr {
    panic!("IPv4 Not Implemented, {}", ipv4_addr)
}

pub fn lookup_hostname(hostname: String) -> MacAddr {
    panic!("Hostname Not Implemented, {}", hostname)
}
