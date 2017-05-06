use std::io::Cursor;
use byteorder::{WriteBytesExt, ReadBytesExt, BigEndian};
use error::Result;

#[derive(Debug, PartialEq)]
pub struct MaxStreamIdFrame {
    pub max_stream_id: u32,
}

impl MaxStreamIdFrame {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let first_byte = super::MAX_STREAM_ID.bits();

        bytes.write_u8(first_byte);

        bytes.write_u32::<BigEndian>(self.max_stream_id);

        bytes
    }

    pub fn from_bytes(buf: &Vec<u8>) -> Result<MaxStreamIdFrame> {
        let mut reader = Cursor::new(buf);

        let _ = reader.read_u8()?;

        let maximum_data = reader.read_u32::<BigEndian>()?;

        Ok(MaxStreamIdFrame {
            max_stream_id: maximum_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_max_stream_id_frame() {
        let frame = MaxStreamIdFrame {
            max_stream_id: 293521,
        };

        let frame_bytes = frame.as_bytes();
        let parsed_frame = MaxStreamIdFrame::from_bytes(&frame_bytes).unwrap();

        assert_eq!(frame, parsed_frame);
    }
}