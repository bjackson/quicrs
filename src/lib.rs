#![feature(plugin)]
#![allow(dead_code)]
#![plugin(bitfield)]
#[cfg(test)]

extern crate core;
extern crate byteorder;
#[macro_use]
extern crate bitflags;


use std::net::UdpSocket;
use std::io::Cursor;
use std::io::Read;

use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian, BigEndian};


bitfield!{ShortPacketType,
    header_type: 1,
    conn_id_flag: 1,
    key_phase: 1,
    packet_type: 5,
}


bitflags! {
    flags PacketType: u8 {
        const VersionNegotiation = 0x01,
        const ClientCleartext = 0x02,
        const NonFinalCleartext = 0x03,
        const FinalServerClearText = 0x04,
        const RTT0Encrypted = 0x05,
        const RTT1EncryptedPhase0 = 0x06,
        const RTT1EncryptedPhase1 = 0x07,
        const PublicReset = 0x08,
    }
}

//#[repr(u8)]
//#[derive(Debug, PartialEq)]
//pub enum PacketType {
//    VersionNegotiation = 0x01,
//    ClientCleartext = 0x02,
//    NonFinalCleartext = 0x03,
//    FinalServerClearText = 0x04,
//    RTT0Encrypted = 0x05,
//    RTT1EncryptedPhase0 = 0x06,
//    RTT1EncryptedPhase1 = 0x07,
//    PublicReset = 0x08,
//}

#[derive(Debug)]
pub enum PacketNumber {
    OneByte(u8),
    TwoBytes(u16),
    FourBytes(u32),
}

#[derive(Debug)]
pub struct ShortHeader {
    key_phase_bit: bool,
    connection_id: Option<u64>,
    packet_number: PacketNumber,
}

#[derive(Debug)]
pub struct LongHeader {
    packet_type: PacketType,
    connection_id: u64,
    packet_number: u32,
    version: u32,
}

enum QuicHeader {
    Short(ShortHeader),
    Long(LongHeader),
}

pub struct QuicPacket {
    header: QuicHeader,
    payload: Vec<u8>,
}

enum ErrorType {
    BadPacket
}

impl QuicPacket {
    fn from_bytes(buf: Vec<u8>) -> Result<QuicPacket, String> {
        let mut reader = Cursor::new(buf);
        let first_byte = reader.read_uint::<BigEndian>(1).expect("Packet is empty") as u8;

        if first_byte & 0x80 != 0 { // Long Header
            let packet_type = first_byte & 0x7f;
            let connection_id = reader.read_u64::<BigEndian>().expect("Connection ID not present");
            let packet_number = reader.read_u32::<BigEndian>().expect("Packet number not present");
            let version = reader.read_u32::<BigEndian>().expect("Version not present");

            let mut payload = Vec::new();
            let _ = reader.read_to_end(&mut payload);

            return Ok(QuicPacket {
                header: QuicHeader::Long(LongHeader {
                    packet_type: PacketType::from_bits(packet_type).expect("Invalid Packet Type"),
                    connection_id: connection_id,
                    packet_number: packet_number,
                    version: version,
                }),
                payload: vec![0, 0]
            })
        } else { // ShortHeader
            let conn_id_flag = first_byte & 0x40 != 0;
            let key_phase_bit = first_byte & 0x20 != 0;
            let packet_type = first_byte & 0x1f;

            let mut connection_id: Option<u64> = None;

            if conn_id_flag {
                connection_id = Some(reader.read_u64::<BigEndian>().expect("Connection ID not present"));
            }

            return Ok(QuicPacket {
                header: QuicHeader::Short(ShortHeader {
                    key_phase_bit: key_phase_bit,
                    connection_id: connection_id,
                    packet_number: PacketNumber::OneByte(0)
                }),
                payload: vec![0, 0]
            })
        }

        return Err("unimplemented".to_string());
    }
}

//impl QuicPacketHeader {
//    fn as_bytes(&self) -> Vec<u8> {
//        use std::mem::transmute;
//
//        let mut contents = Vec::new();
//
//        contents.write_u8(self.public_flags);
//
////        unsafe {
////            let conn64: [u8; 8] = transmute(self.connection_id.to_be());
////            for byte in conn64.iter() {
////                contents.push(*byte);
////            }
////        };
//
//        contents.write_u64::<LittleEndian>(self.connection_id);
//
//        contents.write_u8(self.quic_version.as_bytes()[0]);
//        contents.write_u8(self.quic_version.as_bytes()[1]);
//        contents.write_u8(self.quic_version.as_bytes()[2]);
//        contents.write_u8(self.quic_version.as_bytes()[3]);
//
//        contents.write_uint::<LittleEndian>(self.packet_number, 8);
//
//        contents
//    }
//}

pub struct QuicClient {
    pub socket: std::net::UdpSocket,
    current_packet_number: u32,
    address: String
}


impl QuicClient {
    pub fn new(address: &str, port: u16) -> QuicClient {
        let address = format!("{}:{}", address, port);
        let udp_socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");

        let mut client = QuicClient {
            socket: udp_socket,
            current_packet_number: 0,
            address: address,
        };

        let init_header = LongHeader {
            packet_type: VersionNegotiation,
            connection_id: 1,
            packet_number: 1,
            version: 1,
        };


        let payload = String::from("The FitnessGram Pacer Test is a multistage aerobic capacity test that progressively gets more difficult as it continues. The 20 meter pacer test will begin in 30 seconds. Line up at the start. The running speed starts slowly but gets faster each minute after you hear this signal bodeboop. A sing lap should be completed every time you hear this sound. ding Remember to run in a straight line and run as long as possible. The second time you fail to complete a lap before the sound, your test is over. The test will begin on the word start. On your mark. Get ready!… Start.");

//        let packet_data = [&chlo_packet_header.as_bytes(), payload.as_bytes()].concat();

//        let _ = client.socket.send_to(packet_data.as_slice(), &client.address);

//        client.current_packet_number += 1;


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
