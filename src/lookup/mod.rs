use pnet::util::MacAddr;
use std::net::Ipv4Addr;

use crate::lookup::arp::get_mac_addr;

pub mod arp;

pub fn lookup_ipv4(ipv4_addr: Ipv4Addr) -> MacAddr {
    get_mac_addr(ipv4_addr)
}

pub fn lookup_hostname(hostname: String) -> MacAddr {
    panic!("Hostname Not Implemented, {}", hostname)
}
