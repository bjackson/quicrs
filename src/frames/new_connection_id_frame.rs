use std::io::Cursor;
use byteorder::{WriteBytesExt, ReadBytesExt, BigEndian};
use error::Result;

#[derive(Debug, PartialEq)]
pub struct NewConnectionIdFrame {
    pub sequence: u16,
    pub connection_id: u64,
    pub packet_number_gap: u32,
}

impl NewConnectionIdFrame {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let first_byte = super::NEW_CONNECTION_ID.bits();

        bytes.write_u8(first_byte);

        bytes.write_u16::<BigEndian>(self.sequence);

        bytes.write_u64::<BigEndian>(self.connection_id);

        bytes.write_u32::<BigEndian>(self.packet_number_gap);

        bytes
    }

    pub fn from_bytes(buf: &Vec<u8>) -> Result<NewConnectionIdFrame> {
        let mut reader = Cursor::new(buf);

        let _ = reader.read_u8()?;

        let sequence = reader.read_u16::<BigEndian>()?;

        let connection_id = reader.read_u64::<BigEndian>()?;

        let packet_number_gap = reader.read_u32::<BigEndian>()?;

        Ok(NewConnectionIdFrame {
            sequence: sequence,
            connection_id: connection_id,
            packet_number_gap: packet_number_gap,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let frame = NewConnectionIdFrame {
            sequence: 23235,
            connection_id: 544455,
            packet_number_gap: 5432,
        };

        let frame_bytes = frame.as_bytes();
        let parsed_frame = NewConnectionIdFrame::from_bytes(&frame_bytes).unwrap();

        assert_eq!(frame, parsed_frame);
    }
}