use pnet::util::MacAddr;
use std::env;
use std::net::Ipv4Addr;

use std::str::FromStr;
use wake_neighbor::lookup::{lookup_hostname, lookup_ipv4};
use wake_neighbor::send::MagicPacket;

fn main() {
    env_logger::init();

    let arg1 = &env::args().nth(1).expect("no destination");

    let mac_addr = match parse_arg(arg1) {
        Destination::Mac(mac_addr) => mac_addr,
        Destination::Ipv4(ip_addr) => lookup_ipv4(ip_addr),
        Destination::HostName(hostname) => lookup_hostname(hostname),
    };
    MagicPacket::new(mac_addr).send();
}

fn parse_arg(arg1: &str) -> Destination {
    if let Ok(mac_addr) = MacAddr::from_str(arg1) {
        Destination::Mac(mac_addr)
    } else if let Ok(ipv4_addr) = Ipv4Addr::from_str(arg1) {
        Destination::Ipv4(ipv4_addr)
    } else {
        Destination::HostName(String::from(arg1))
    }
}

enum Destination {
    Mac(MacAddr),
    Ipv4(Ipv4Addr),
    HostName(String),
}
