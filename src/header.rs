


use byteorder::{WriteBytesExt, BigEndian};


use packet::ShortPacketType;
use packet::PacketType;

#[derive(Debug)]
pub enum PacketNumber {
    OneByte(u8),
    TwoBytes(u16),
    FourBytes(u32),
}

#[derive(Debug)]
pub struct ShortHeader {
    pub key_phase_bit: bool,
    pub conn_id_bit: bool,
    pub connection_id: Option<u64>,
    pub packet_number: PacketNumber,
    pub packet_type: ShortPacketType
}

#[derive(Debug)]
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

