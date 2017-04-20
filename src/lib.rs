#[cfg(test)]
#[allow(dead_code)]


use std::net::UdpSocket;
// use std::ops::BitAnd;

#[allow(dead_code)]
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum PublicFlags {
    PublicFlagVersion = 0x01,
    PublicFlagReset = 0x02,
    NoncePresent = 0x04,
    IdPresent = 0x08,
    PktNumLen4 = 0x30,
    PktNumLen2 = 0x20,
    PktNumLen1 = 0x10,
    Multipath = 0x40,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct QuicPacketHeader<'a> {
    public_flags: u8,
    connection_id: u64,
    quic_version: &'a[u8],
    packet_number: u64,
}

pub struct QuicClient {
    socket: std::net::UdpSocket,
    current_packet_number: u64,
    address: String
}

impl QuicClient {
    pub fn new(address: &str, port: u16) -> QuicClient {
        let address = format!("{}:{}", address, port);
        let udp_socket = std::net::UdpSocket::bind("127.0.0.1:0").expect("couldn't bind to address");

        println!("EXECUTED");

        let mut client = QuicClient {
            socket: udp_socket,
            current_packet_number: 0,
            address: String::from("localhost:4000")
        };

        let chlo_packet_header = QuicPacketHeader {
            public_flags: (PublicFlags::PublicFlagVersion as u8 & PublicFlags::IdPresent as u8),
            connection_id: 0,
            quic_version: "Q025".as_bytes(),
            packet_number: 0
        };

        client.socket.send_to(&[0; 10], &client.address);

        println!("EXECUTED");



        client
    }



    pub fn get<'a>(&self, url: &str) -> Vec<u8> {
        vec![0x01, 0x02]
    }
}


mod tests {
    use super::*;

    #[test]
    fn creates_new_socket() {
        let _ = QuicClient::new("localhost", 443);
    }

    #[test]
    fn get_url() {
        let client = QuicClient::new("0.0.0.0", 0);
        assert_eq!(client.get("hello"), vec![0x01, 0x02]);
    }
}
