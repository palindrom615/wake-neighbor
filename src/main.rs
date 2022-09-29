use std::env;
use log::{debug, error, info, warn};
use std::net::UdpSocket;
use std::str::FromStr;
use pnet::util::MacAddr;
use wake_neighbor::send::send_magic_packet;

fn main() {
    env_logger::init();

    let mac_addr = MacAddr::from_str(
        &env::args().nth(1).expect("no mac address")
    ).expect("no valid mac address");
    send_magic_packet(&mac_addr.octets());
}
