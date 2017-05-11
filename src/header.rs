
use std::io::Cursor;

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};

use error::Result;
use error::QuicError;

use packet::ShortPacketType;
use packet::PacketType;

use packet::ONE_BYTE;
use packet::TWO_BYTES;
use packet::FOUR_BYTES;

#[derive(Debug, PartialEq)]
pub enum PacketNumber {
    OneByte(u8),
    TwoBytes(u16),
    FourBytes(u32),
}

#[derive(Debug, PartialEq)]
pub struct ShortHeader {
    pub key_phase_bit: bool,
    pub conn_id_bit: bool,
    pub connection_id: Option<u64>,
    pub packet_number: u64,
    pub packet_type: ShortPacketType
}

#[derive(Debug, PartialEq)]
pub struct LongHeader {
    pub packet_type: PacketType,
    pub connection_id: u64,
    pub packet_number: u32,
    pub version: u32,
}

#[derive(Debug)]
pub enum QuicHeader {
    Short(ShortHeader),
    Long(LongHeader),
}

impl ShortHeader {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let mut first_octet = 0 as u8;

        if self.conn_id_bit {
            first_octet |= 0x40;
        }

        if self.key_phase_bit {
            first_octet |= 0x20;
        }

        first_octet |= self.packet_type.bits() & 0x1f;

        bytes.write_u8(first_octet);

        if self.conn_id_bit {
            bytes.write_u64::<BigEndian>(self.connection_id.expect("Packet ID not present but conn_id_bit set") as u64);
        }


        match self.packet_type {
            ONE_BYTE => bytes.write_u8(self.packet_number as u8),
            TWO_BYTES => bytes.write_u16::<BigEndian>(self.packet_number as u16),
            FOUR_BYTES | _ => bytes.write_u32::<BigEndian>(self.packet_number as u32),
        };

        bytes
    }

    pub fn from_bytes(buf: &[u8]) -> Result<ShortHeader> {
        let mut reader = Cursor::new(buf);

        let first_octet = reader.read_u8()?;

        let packet_type = match ShortPacketType::from_bits(first_octet & 0x1f) {
            Some(pt) => pt,
            None => return Err(QuicError::ParseError)
        };

        let conn_id_bit = first_octet & 0x40 > 0;
        let key_phase_bit = first_octet & 0x20 > 0;

        let connection_id = if conn_id_bit {
            Some(reader.read_u64::<BigEndian>()?)
        } else {
            None
        };

        let packet_number = match packet_type {
            ONE_BYTE => reader.read_u8()? as u64,
            TWO_BYTES => reader.read_u16::<BigEndian>()? as u64,
            FOUR_BYTES => reader.read_u32::<BigEndian>()? as u64,
            _ => return Err(QuicError::ParseError)
        };

//        let packet_number = reader.read_uint::<BigEndian>(packet_num_len)?;

        Ok(ShortHeader {
            key_phase_bit: key_phase_bit,
            conn_id_bit: conn_id_bit,
            connection_id: connection_id,
            packet_number: packet_number,
            packet_type: packet_type,
        })
    }
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

    pub fn from_bytes(buf: &[u8]) -> Result<LongHeader> {
        let mut reader = Cursor::new(buf);

        let first_octet = reader.read_u8()?;

        let packet_type = match PacketType::from_bits(first_octet & 0x7f) {
            Some(pt) => pt,
            None => return Err(QuicError::ParseError)
        };

        let connection_id = reader.read_u64::<BigEndian>()?;

        let packet_number = reader.read_u32::<BigEndian>()?;

        let version = reader.read_u32::<BigEndian>()?;

        if version == 0 {
            return Err(QuicError::ParseError);
        }

        Ok(LongHeader {
            packet_type: packet_type,
            connection_id: connection_id,
            packet_number: packet_number,
            version: version,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use packet::*;

    #[test]
    fn serialize_long_header() {
        let long_header = LongHeader {
            packet_type: VERSION_NEGOTIATION,
            connection_id: 2522352u64,
            packet_number: 25u32,
            version: 0x1,
        };

        let long_header_bytes = long_header.as_bytes();
        let long_header_parsed = LongHeader::from_bytes(&long_header_bytes).unwrap();

        assert_eq!(long_header, long_header_parsed);
    }

    #[test]
    fn serialize_short_header() {
        let header = ShortHeader {
            key_phase_bit: true,
            conn_id_bit: true,
            connection_id: Some(23u64),
            packet_number: 245u64,
            packet_type: TWO_BYTES,
        };

        let header_bytes = header.as_bytes();
        let header_parsed = ShortHeader::from_bytes(&header_bytes).unwrap();

        assert_eq!(header, header_parsed);
    }
}
