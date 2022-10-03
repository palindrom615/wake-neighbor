use pnet::util::MacAddr;
use std::env::args;
use std::net::Ipv4Addr;

use std::str::FromStr;
use wake_neighbor::lookup::{lookup_hostname, lookup_ipv4};
use wake_neighbor::send::MagicPacket;

fn main() {
    env_logger::init();

    fn lookup_hw_addr(arg: &str) -> MacAddr {
        if let Ok(mac_addr) = MacAddr::from_str(arg) {
            mac_addr
        } else if let Ok(ipv4_addr) = Ipv4Addr::from_str(arg) {
            lookup_ipv4(ipv4_addr)
        } else {
            lookup_hostname(String::from(arg))
        }
    }

    let arg1 = &args().nth(1).expect("no destination");

    MagicPacket::new(lookup_hw_addr(arg1)).send();
}
