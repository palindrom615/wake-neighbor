use log::debug;

use pnet::util::MacAddr;
use std::net::UdpSocket;

pub fn send_magic_packet(mac_addr: MacAddr) -> usize {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
    debug!("bound socket {}", socket.local_addr().unwrap());

    socket.set_broadcast(true).expect("broadcast failed");

    let res = socket
        .send_to(&make_magic_packet(mac_addr), "255.255.255.255:9")
        .expect("send magic packet failed");
    debug!("send bytes length {}", res);
    res
}

fn make_magic_packet(mac_addr: MacAddr) -> [u8; 102] {
    let mut magic_packet = [0u8; 102];
    magic_packet[..6].copy_from_slice(b"\xff\xff\xff\xff\xff\xff");
    for i in 1..17 {
        let start_pos = i * 6;
        let end_pos = (i + 1) * 6;
        magic_packet[start_pos..end_pos].copy_from_slice(&mac_addr.octets());
    }
    magic_packet
}
