use std::env;
use log::{debug, error, info, warn};
use std::net::UdpSocket;
use std::str::FromStr;
use pnet::util::MacAddr;

fn main() {
    env_logger::init();

    let mac_addr = MacAddr::from_str(
        &env::args().nth(1).expect("no mac address")
    ).expect("no valid mac address");

    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
    debug!("bound socket {}", socket.local_addr().unwrap());

    socket.set_broadcast(true).expect("broadcast failed");

    let res = socket
        .send_to(&get_magic_packet(&mac_addr.octets()), "255.255.255.255:9")
        .expect("send magic packet failed");
    debug!("send bytes length {}", res)
}

fn get_magic_packet(mac_addr: &[u8; 6]) -> [u8; 102] {
    let mut magic_packet = [0u8; 102];
    magic_packet[..6].copy_from_slice(b"\xff\xff\xff\xff\xff\xff");
    for i in 1..17 {
        let start_pos = i * 6;
        let end_pos = (i + 1) * 6;
        magic_packet[start_pos..end_pos].copy_from_slice(mac_addr);
    }
    magic_packet
}
