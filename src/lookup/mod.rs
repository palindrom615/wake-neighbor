use log::info;
use pnet::datalink::interfaces;
use pnet::util::MacAddr;
use std::net::Ipv4Addr;

pub fn lookup_ipv4(ipv4_addr: Ipv4Addr) -> MacAddr {
    let interface = interfaces();
    info!("{:?}", interface);
    panic!("IPv4 Not Implemented, {}", ipv4_addr)
}

pub fn lookup_hostname(hostname: String) -> MacAddr {
    panic!("Hostname Not Implemented, {}", hostname)
}
