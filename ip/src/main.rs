use pnet::{
    datalink::{self, Channel, ChannelType, Config},
    packet::{ip::IpNextHeaderProtocol, ipv4::MutableIpv4Packet, Packet},
    util,
};
use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    const INTERNET_PROTOCOL: u16 = 0x800;
    const TESTING: u8 = 0xFE;
    let args: Vec<String> = env::args().collect();
    let if_name = &args[1];
    let destination = args[2].parse()?;

    let iface = datalink::interfaces()
        .into_iter()
        .find(|i| i.name == *if_name)
        .ok_or("failed to get interface")?;

    let config = Default::default();
    let (mut sender, _receiver) = match datalink::channel(
        &iface,
        Config {
            channel_type: ChannelType::Layer3(INTERNET_PROTOCOL),
            ..config
        },
    ) {
        Ok(Channel::Ethernet(sender, receiver)) => (sender, receiver),
        _ => return Err("failed to open channel".into()),
    };

    let mut ip_buf = [0; 1500];
    let mut packet = MutableIpv4Packet::new(&mut ip_buf).ok_or("failed to create a packet")?;

    packet.set_version(4);
    packet.set_total_length(1500);
    packet.set_identification(0xabcd);
    packet.set_ttl(64);
    packet.set_next_level_protocol(IpNextHeaderProtocol::new(TESTING));
    packet.set_source("127.0.0.1".parse()?);
    packet.set_destination(destination);
    packet.set_checksum(util::checksum(&packet.packet(), 5));
    sender
        .send_to(packet.packet(), None)
        .ok_or("failed to send")??;
    Ok(())
}