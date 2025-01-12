use pnet::packet::icmp::echo_request::{EchoRequestPacket, MutableEchoRequestPacket};
use pnet::packet::icmp::{checksum, IcmpPacket, IcmpTypes, MutableIcmpPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;
use pnet::transport::{
    icmp_packet_iter, transport_channel, TransportChannelType, TransportProtocol,
    TransportReceiver, TransportSender,
};
use std::net::IpAddr;
use std::str::FromStr;
use std::time::Duration;

fn send_echo(packet: &mut MutableEchoRequestPacket, seq: u16) {
    //set icmp options
    packet.set_icmp_type(IcmpTypes::EchoRequest);
    packet.set_sequence_number(1);
    packet.set_identifier(1);

    let icmp_packet = IcmpPacket::new(packet.packet()).unwrap();
    packet.set_checksum(checksum(&icmp_packet));
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let destination = IpAddr::from_str("8.8.8.8")?;

    let protocol =
        TransportChannelType::Layer4(TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp));
    // Create channel for sending/receiving packets
    let (mut tx, mut rx) = transport_channel(1024, protocol)?;
    //setting TTL to something
    tx.set_ttl(1)?;

    // Create buffer for our ICMP packet
    let mut vec: Vec<u8> = vec![0; 64]; // Buffer for packet
    let mut echo_packet = MutableEchoRequestPacket::new(&mut vec[..]).unwrap();

    // Set the ICMP Echo Request parameters
    echo_packet.set_icmp_type(IcmpTypes::EchoRequest);
    echo_packet.set_sequence_number(1);
    echo_packet.set_identifier(1);

    //checksum calculate
    let icmp_packet = IcmpPacket::new(echo_packet.packet()).unwrap();
    echo_packet.set_checksum(checksum(&icmp_packet));
    tx.send_to(echo_packet, destination)?;

    //recieving response
    let mut iter = icmp_packet_iter(&mut rx);

    if let Ok(Some((packet, address))) = iter.next_with_timeout(Duration::from_secs(1)) {
        println!("Recv response from: {}", address);
        println!("ICMP type : {:?}", packet.get_icmp_type());

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
