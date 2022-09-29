use log::{debug, error, info, warn};
use std::net::UdpSocket;

const MAC_ADDR: &[u8; 6] = b"\xa8\xa1\x59\x5c\xd7\xff";

fn main() {
    env_logger::init();
    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
    debug!("bound socket {}", socket.local_addr().unwrap());

    socket.set_broadcast(true).expect("broadcast failed");

    let res = socket
        .send_to(&get_magic_packet(MAC_ADDR), "255.255.255.255:9")
        .expect("send magic packet failed");
    debug!("send bytes of {}", res)
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
