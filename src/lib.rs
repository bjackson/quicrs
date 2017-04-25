#![feature(plugin)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![plugin(bitfield)]
#[cfg(test)]

extern crate core;
extern crate byteorder;
#[macro_use]
extern crate bitflags;
extern crate rand;

use std::io;
use std::fmt;

use std::net::UdpSocket;
use std::io::Cursor;
use std::io::Read;
use std::error::Error;

use byteorder::{WriteBytesExt, ReadBytesExt, BigEndian};




bitflags! {
    flags PacketType: u8 {
        const VERSION_NEGOTIATION = 0x01,
        const CLIENT_CLEARTEXT = 0x02,
        const NON_FINAL_CLEARTEXT = 0x03,
        const FINAL_SERVER_CLEAR_TEXT = 0x04,
        const RTT0_ENCRYPTED = 0x05,
        const RTT1_ENCRYPTED_PHASE0 = 0x06,
        const RTT1_ENCRYPTED_PHASE1 = 0x07,
        const PUBLIC_RESET = 0x08,
    }
}

bitflags! {
    flags ShortPacketType: u8 {
        const ONE_BYTE = 0x01,
        const TWO_BYTES = 0x02,
        const FOUR_BYTES = 0x03,
    }
}



#[derive(Debug)]
enum QuicError {
    Io(io::Error),
    ParseError,
}

impl Error for QuicError {
    fn description(&self) -> &str {
        match *self {
            QuicError::Io(ref err) => err.description(),
            QuicError::ParseError => "Error parsing packet",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            QuicError::Io(ref err) => Some(err),
            QuicError::ParseError => None,
        }
    }
}

impl From<io::Error> for QuicError {
    fn from(err: io::Error) -> QuicError {
        QuicError::Io(err)
    }
}

impl fmt::Display for QuicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            QuicError::Io(ref err) => err.fmt(f),
            QuicError::ParseError => write!(f, "Error parsing packet"),
        }
    }
}

#[derive(Debug)]
pub enum PacketNumber {
    OneByte(u8),
    TwoBytes(u16),
    FourBytes(u32),
}

#[derive(Debug)]
pub struct ShortHeader {
    key_phase_bit: bool,
    conn_id_bit: bool,
    connection_id: Option<u64>,
    packet_number: PacketNumber,
    packet_type: ShortPacketType
}

impl ShortHeader {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut first_octet = 0 as u8;

        if self.conn_id_bit {
            first_octet = first_octet | 0x40;
        }

        if self.key_phase_bit {
            first_octet = first_octet | 0x20;
        }

        first_octet = first_octet | self.packet_type.bits();

        bytes.write_u8(first_octet);

        if self.conn_id_bit {
            bytes.write_u32::<BigEndian>(self.connection_id.expect("Packet ID not present but conn_id_bit set") as u32);
        }

        match self.packet_number {
            PacketNumber::OneByte(num) => bytes.write_u8(num),
            PacketNumber::TwoBytes(num) => bytes.write_u16::<BigEndian>(num),
            PacketNumber::FourBytes(num) => bytes.write_u32::<BigEndian>(num)
        };

        bytes
    }
}

#[derive(Debug)]
pub struct LongHeader {
    packet_type: PacketType,
    connection_id: u64,
    packet_number: u32,
    version: u32,
}

impl LongHeader {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let first_octet = 0x80 | self.packet_type.bits();

        bytes.write_u8(first_octet);

        bytes.write_u64::<BigEndian>(self.connection_id);

        bytes.write_u32::<BigEndian>(self.packet_number as u32);

        bytes.write_u32::<BigEndian>(self.version as u32);

        bytes
    }
}

#[derive(Debug)]
enum QuicHeader {
    Short(ShortHeader),
    Long(LongHeader),
}



#[derive(Debug)]
pub struct QuicPacket {
    header: QuicHeader,
    payload: Vec<u8>,
}

enum ErrorType {
    BadPacket
}

impl QuicPacket {
    fn from_bytes(buf: Vec<u8>) -> Result<QuicPacket, QuicError> {
        let mut reader = Cursor::new(buf);
        let first_byte = reader.read_uint::<BigEndian>(1)? as u8;

        if first_byte & 0x80 != 0 { // Long Header
            let packet_type = match PacketType::from_bits(first_byte & 0x7f) {
                Some(pt) => pt,
                None => return Err(QuicError::ParseError)
            };

            let connection_id = reader.read_u64::<BigEndian>()?;
            let packet_number = reader.read_u32::<BigEndian>()?;
            let version = reader.read_u32::<BigEndian>()?;

            if version == 0 {
                return Err(QuicError::ParseError)
            }

            let mut payload = Vec::new();
            let _ = reader.read_to_end(&mut payload);

            return Ok(QuicPacket {
                header: QuicHeader::Long(LongHeader {
                    packet_type: packet_type,
                    connection_id: connection_id,
                    packet_number: packet_number,
                    version: version,
                }),
                payload: payload
            })
        } else { // ShortHeader
            let conn_id_flag = first_byte & 0x40 != 0;
            let key_phase_bit = first_byte & 0x20 != 0;
            let packet_type = ShortPacketType::from_bits(first_byte & 0x1f).expect("Invalid packet type");

            let mut connection_id: Option<u64> = None;

            if conn_id_flag {
                connection_id = Some(reader.read_u64::<BigEndian>()?);
            }



            let packet_number_size = match packet_type {
                ONE_BYTE => Some(1u8),
                TWO_BYTES => Some(2u8),
                FOUR_BYTES => Some(4u8),
                _ => return Err(QuicError::ParseError)
            }.unwrap();

            let packet_number: PacketNumber;

            packet_number = match packet_number_size {
                1 => PacketNumber::OneByte(reader.read_uint::<BigEndian>(1).expect("Packet number is empty") as u8),
                2 => PacketNumber::TwoBytes(reader.read_uint::<BigEndian>(2).expect("Packet number is empty") as u16),
                4 => PacketNumber::FourBytes(reader.read_uint::<BigEndian>(4).expect("Packet number is empty") as u32),
                _ => return Err(QuicError::ParseError),
            };

            let mut payload = Vec::new();
            let _ = reader.read_to_end(&mut payload);

            return Ok(QuicPacket {
                header: QuicHeader::Short(ShortHeader {
                    key_phase_bit: key_phase_bit,
                    connection_id: connection_id,
                    packet_number: packet_number,
                    conn_id_bit: conn_id_flag,
                    packet_type: packet_type
                }),
                payload: payload
            })
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let header_bytes = match self.header {
            QuicHeader::Short(ref header) => header.as_bytes(),
            QuicHeader::Long(ref header) => header.as_bytes(),
        };

        [header_bytes, self.payload.clone()].concat()
    }
}

pub struct QuicClient {
    pub socket: std::net::UdpSocket,
    pub current_packet_number: u32,
    pub address: String
}


impl QuicClient {
    pub fn new(address: &str, port: u16) -> QuicClient {
        let address = format!("{}:{}", address, port);
        let udp_socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to address");

        let client = QuicClient {
            socket: udp_socket,
            current_packet_number: QuicClient::get_first_packet_number(),
            address: address,
        };

//        let init_header = LongHeader {
//            packet_type: VERSION_NEGOTIATION,
//            connection_id: 1,
//            packet_number: client.current_packet_number,
//            version: 1,
//        };

        client
    }

    pub fn get_first_packet_number() -> u32 {
        use rand::{OsRng};
        use rand::distributions::{IndependentSample, Range};

        let between = Range::new(0u32, 2u32.pow(31) - 1);
        let mut rng = OsRng::new().expect("Cannot get random number");

        between.ind_sample(&mut rng)
    }


    pub fn get<'a>(&self, url: &str) -> Vec<u8> {
        String::from(url).into_bytes()
    }
}


mod tests {
    #[test]
    fn creates_new_socket() {
        let _ = super::QuicClient::new("localhost", 443);
    }

    #[test]
    fn get_url() {
        let client = super::QuicClient::new("google.com", 443);
        assert_eq!(client.get("hello"), String::from("hello").into_bytes());
    }
}
