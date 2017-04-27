

use error::QuicError;
use error::Result;


use std::io::Cursor;
use std::io::Read;

use byteorder::{ReadBytesExt, BigEndian};


use header::QuicHeader;
use header::ShortHeader;
use header::LongHeader;



bitflags! {
    pub flags ShortPacketType: u8 {
        const ONE_BYTE = 0x01,
        const TWO_BYTES = 0x02,
        const FOUR_BYTES = 0x03,
    }
}

bitflags! {
    pub flags PacketType: u8 {
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




#[derive(Debug)]
pub struct QuicPacket {
    pub header: QuicHeader,
    pub payload: Vec<u8>,
}

impl QuicPacket {
    pub fn from_bytes(buf: &Vec<u8>) -> Result<QuicPacket> {
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
            let _ = reader.read(&mut payload);

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


            let packet_number = match packet_number_size {
                1 => reader.read_uint::<BigEndian>(1)? as u64,
                2 => reader.read_uint::<BigEndian>(2)? as u64,
                4 => reader.read_uint::<BigEndian>(4)? as u64,
                _ => return Err(QuicError::ParseError),
            };

            let mut payload = Vec::new();
            let _ = reader.read(&mut payload);

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

