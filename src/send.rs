use log::debug;

use pnet::util::MacAddr;
use std::net::UdpSocket;

pub struct MagicPacket {
    _bytes: [u8; 102],
}

impl MagicPacket {
    pub fn new(mac_addr: MacAddr) -> MagicPacket {
        let mut magic_packet = [0u8; 102];
        magic_packet[..6].copy_from_slice(b"\xff\xff\xff\xff\xff\xff");
        for i in 1..17 {
            let start_pos = i * 6;
            let end_pos = (i + 1) * 6;
            magic_packet[start_pos..end_pos].copy_from_slice(&mac_addr.octets());
        }
        MagicPacket {
            _bytes: magic_packet,
        }
    }

    pub fn send(&self) -> usize {
        let socket = UdpSocket::bind("0.0.0.0:0").expect("bind failed");
        debug!("bound socket {}", socket.local_addr().unwrap());

        socket.set_broadcast(true).expect("broadcast failed");

        let res = socket
            .send_to(&self._bytes, "255.255.255.255:9")
            .expect("send magic packet failed");
        debug!("send bytes length {}", res);
        res
    }
}
