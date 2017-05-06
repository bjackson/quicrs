use std::io::Cursor;
use byteorder::{WriteBytesExt, ReadBytesExt, BigEndian};
use error::Result;

#[derive(Debug, PartialEq)]
pub struct GoAwayFrame {
    pub largest_client_stream_id: u32,
    pub largest_server_stream_id: u32,
}

impl GoAwayFrame {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let first_byte = super::GOAWAY.bits();

        bytes.write_u8(first_byte);

        bytes.write_u32::<BigEndian>(self.largest_client_stream_id);
        bytes.write_u32::<BigEndian>(self.largest_server_stream_id);

        bytes
    }

    pub fn from_bytes(buf: &Vec<u8>) -> Result<GoAwayFrame> {
        let mut reader = Cursor::new(buf);

        let _ = reader.read_u8()?;

        let largest_client_stream_id = reader.read_u32::<BigEndian>()?;
        let largest_server_stream_id = reader.read_u32::<BigEndian>()?;

        Ok(GoAwayFrame {
            largest_client_stream_id: largest_client_stream_id,
            largest_server_stream_id: largest_server_stream_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let frame = GoAwayFrame {
            largest_client_stream_id: 634632634,
            largest_server_stream_id: 235962,
        };

        let frame_bytes = frame.as_bytes();
        let parsed_frame = GoAwayFrame::from_bytes(&frame_bytes).unwrap();

        assert_eq!(frame, parsed_frame);
    }
}