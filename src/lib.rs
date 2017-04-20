#[cfg(test)]
#[allow(dead_code)]
extern crate core;
extern crate byteorder;

use std::net::UdpSocket;


use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

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
#[repr(packed)]
pub struct QuicPacketHeader {
    public_flags: u8,
    connection_id: u64,
    quic_version: String,
    diversication_nonce: u32,
    packet_number: u64,
}

impl QuicPacketHeader {
    fn as_bytes(&self) -> Vec<u8> {
        use std::mem::transmute;

        let mut contents = Vec::new();

        contents.write_u8(self.public_flags);

//        unsafe {
//            let conn64: [u8; 8] = transmute(self.connection_id.to_be());
//            for byte in conn64.iter() {
//                contents.push(*byte);
//            }
//        };

        contents.write_u64::<LittleEndian>(self.connection_id);

        contents.push(self.quic_version.as_bytes()[0]);
        contents.push(self.quic_version.as_bytes()[1]);
        contents.push(self.quic_version.as_bytes()[2]);
        contents.push(self.quic_version.as_bytes()[3]);

        contents.write_u32::<LittleEndian>(self.diversication_nonce);

        contents.write_uint::<LittleEndian>(self.packet_number, 6);

        contents
    }
}

pub struct QuicClient {
    pub socket: std::net::UdpSocket,
    current_packet_number: u64,
    address: String
}


impl QuicClient {
    pub fn new(address: &str, port: u16) -> QuicClient {
        let address = format!("{}:{}", address, port);
        let udp_socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");

        let mut client = QuicClient {
            socket: udp_socket,
            current_packet_number: 0,
            address: address
        };

        let chlo_packet_header = QuicPacketHeader {
            public_flags: (PublicFlags::PublicFlagVersion as u8 | PublicFlags::IdPresent as u8 | PublicFlags::NoncePresent as u8 | PublicFlags::PktNumLen4 as u8),
            connection_id: 8,
            quic_version: "Q035".to_string(),
            packet_number: client.current_packet_number + 1,
            diversication_nonce: 0x10
        };

        println!("{:x}", &chlo_packet_header.public_flags);
        println!("{:?}", &chlo_packet_header.quic_version);

        let _ = client.socket.send_to(&chlo_packet_header.as_bytes(), &client.address);

        client.current_packet_number += 1;


        client
    }



    pub fn get<'a>(&self, url: &str) -> Vec<u8> {
        String::from(url).into_bytes()
    }
}


mod tests {
    #[test]
    fn creates_new_socket() {
        let _ = QuicClient::new("localhost", 443);
    }

    #[test]
    fn get_url() {
        let client = QuicClient::new("google.com", 443);
        assert_eq!(client.get("hello"), vec![0x01, 0x02]);
    }
}
