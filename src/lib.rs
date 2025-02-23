use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmp::{checksum, IcmpPacket, IcmpTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;
use pnet::transport::{
    icmp_packet_iter, transport_channel, TransportChannelType, TransportProtocol,
};
use std::net::IpAddr;
use std::str::FromStr;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Probe {
    pub ips: Option<IpAddr>,
    pub rtt: Option<u128>,
    pub timestamp: Option<u64>,
    pub geo_data: Option<String>,
    pub active: bool,
}
impl Probe {
    fn new() -> Probe {
        Probe {
            ips: None,
            rtt: None,
            timestamp: None,
            geo_data: None,
            active: false,
        }
    }
}

pub struct HopData {
    probes: Vec<Probe>,
}

impl Default for HopData {
    fn default() -> Self {
        Self::new()
    }
}

impl HopData {
    pub fn new() -> HopData {
        HopData { probes: Vec::new() }
    }

    pub fn add(&mut self, val: Probe) {
        self.probes.push(val);
    }

    pub fn list_all_probes(&self) {
        for probe in &self.probes {
            println!("{:?}", probe);
        }
    }

    pub fn echo_request_ipv4(
        &mut self,
        destination: &str,
        ttl: u8,
        buffersize: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let destination = IpAddr::from_str(destination)?;
        let protocol =
            TransportChannelType::Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp));

        // Create channel for sending/receiving packets and set ttl
        let (mut tx, mut rx) = transport_channel(1024, protocol)?;
        tx.set_ttl(ttl)?;

        // Create buffer for our ICMP Echo packet
        //let mut vec: Vec<u8> = vec![0; 64];
        let mut vec: Vec<u8> = vec![0; buffersize];
        let mut echo_packet = MutableEchoRequestPacket::new(&mut vec[..]).unwrap();

        // Set the ICMP Echo Request parameters
        echo_packet.set_icmp_type(IcmpTypes::EchoRequest);
        echo_packet.set_sequence_number(1);
        echo_packet.set_identifier(1);

        //checksum calculate
        let icmp_packet = IcmpPacket::new(echo_packet.packet()).unwrap();
        echo_packet.set_checksum(checksum(&icmp_packet));

        let check = Instant::now();
        tx.send_to(echo_packet, destination)?;

        //recieving response
        let mut iter = icmp_packet_iter(&mut rx);

        if let Ok(Some((packet, address))) = iter.next_with_timeout(Duration::from_secs(1)) {
            let mut probe = Probe::new();
            probe.ips = Some(address);
            probe.rtt = Some(check.elapsed().as_micros());
            probe.timestamp = None;
            probe.geo_data = None;

            println!("Recv response from: {}", address);
            println!("ICMP type : {:?}", packet.get_icmp_type());

            self.add(probe);
            match packet.get_icmp_type() {
                IcmpTypes::EchoReply => println!("Got reply"),
                IcmpTypes::TimeExceeded => println!("time exceeded for intermediate router"),
                _ => println!("unexpected icmp type {:?}", packet.get_icmp_type()),
            }
        } else {
            println!("No response recieved within timeout");
        }

        Ok(())
    }
}
