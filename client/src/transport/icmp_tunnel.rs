use pnet::packet::icmp::{IcmpTypes, MutableIcmpPacket};
use pnet::transport::{transport_channel, TransportChannelType::Layer4};

pub fn send_icmp(payload: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let (mut _tx, mut _rx) = transport_channel(4096, Layer4(IcmpTypes::EchoReply));

    // Формируем пакет
    // Отправляем payload в теле ICMP
    // Сервер расшифровывает
    Ok(())
}