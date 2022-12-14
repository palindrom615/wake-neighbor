use log::debug;
use pnet::datalink::{interfaces, Channel, NetworkInterface};
use pnet::packet::arp::{ArpHardwareTypes, ArpOperations, ArpPacket, MutableArpPacket};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::{MutablePacket, Packet};
use pnet::util::MacAddr;
use std::net::{IpAddr, Ipv4Addr};
use std::time::{Duration, Instant};

pub fn get_mac_addr(target_ip: Ipv4Addr) -> MacAddr {
    let interfaces = interfaces();
    let interface = interfaces
        .iter()
        .max_by_key(|i| get_lpm(i, target_ip))
        .expect("No interface for destination IP address");
    debug!("{}", interface);

    let source_ip = interface
        .ips
        .iter()
        .find(|ip| ip.is_ipv4())
        .map(|ip| match ip.ip() {
            IpAddr::V4(ip) => ip,
            _ => unreachable!(),
        })
        .expect("No interface for destination IP address");
    debug!("{}", source_ip);

    let (mut sender, mut receiver) = match pnet::datalink::channel(interface, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unknown channel type"),
        Err(e) => panic!("Error happened {}", e),
    };

    let mut ethernet_buffer = [0u8; 42];
    let mut ethernet_packet = MutableEthernetPacket::new(&mut ethernet_buffer).unwrap();

    ethernet_packet.set_destination(MacAddr::broadcast());
    ethernet_packet.set_source(interface.mac.unwrap());
    ethernet_packet.set_ethertype(EtherTypes::Arp);

    let mut arp_buffer = [0u8; 28];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();

    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(ArpOperations::Request);
    arp_packet.set_sender_hw_addr(interface.mac.unwrap());
    arp_packet.set_sender_proto_addr(source_ip);
    arp_packet.set_target_hw_addr(MacAddr::zero());
    arp_packet.set_target_proto_addr(target_ip);

    ethernet_packet.set_payload(arp_packet.packet_mut());

    // Same default value with [linux arp](https://man7.org/linux/man-pages/man7/arp.7.html)
    const RETRANS_TIME_MS: u64 = 1000;
    const MCAST_SOLICIT: u8 = 3;

    for i in 0..MCAST_SOLICIT {
        sender
            .send_to(ethernet_packet.packet(), None)
            .expect("Send ARP failed")
            .expect("Send ARP failed");
        debug!("Sent ARP request");
        let start = Instant::now();
        while Instant::now() - start <= Duration::from_millis(RETRANS_TIME_MS) {
            let buf = receiver.next().unwrap();
            let pkt = EthernetPacket::new(buf).unwrap();
            if pkt.get_ethertype() != EtherTypes::Arp {
                continue;
            }
            pkt.payload();
            let arp = ArpPacket::new(pkt.payload()).unwrap();
            debug!("Received {:?}", arp);
            if arp.get_sender_proto_addr() == target_ip
                && arp.get_target_hw_addr() == interface.mac.unwrap()
            {
                debug!("Received reply");
                return arp.get_sender_hw_addr();
            }
        }
        debug!("ARP try {} timeout", i);
    }
    panic!("Host not reachable")
}

fn get_lpm(interface: &NetworkInterface, target_ip: Ipv4Addr) -> u8 {
    interface
        .ips
        .iter()
        .map(|ip_network| {
            if ip_network.contains(IpAddr::from(target_ip)) {
                ip_network.prefix()
            } else {
                0
            }
        })
        .max()
        .unwrap_or(0u8)
}

#[cfg(test)]
mod tests {
    use log::LevelFilter;
    use std::str::FromStr;

    use super::*;
    fn init() {
        let _ = env_logger::builder()
            .is_test(true)
            .filter_level(LevelFilter::Debug)
            .try_init();
    }
    #[test]
    fn it_works() {
        init();
        let ip_addr = "10.0.0.43";
        let ip = Ipv4Addr::from_str(ip_addr).expect("never reach here");
        debug!("IP {}, hw {}", ip_addr, get_mac_addr(ip));
    }
}
