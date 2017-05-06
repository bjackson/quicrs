use std::io::Cursor;
use byteorder::{WriteBytesExt, ReadBytesExt, BigEndian};
use error::Result;

#[derive(Debug, PartialEq)]
pub struct StreamBlockedFrame {
    pub stream_id: u32,
}

impl StreamBlockedFrame {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let first_byte = super::STREAM_BLOCKED.bits();

        bytes.write_u8(first_byte);

        bytes.write_u32::<BigEndian>(self.stream_id);

        bytes
    }

    pub fn from_bytes(buf: &Vec<u8>) -> Result<StreamBlockedFrame> {
        let mut reader = Cursor::new(buf);

        let _ = reader.read_u8()?;

        let maximum_data = reader.read_u32::<BigEndian>()?;

        Ok(StreamBlockedFrame {
            stream_id: maximum_data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_max_stream_id_frame() {
        let frame = StreamBlockedFrame {
            stream_id: 293521,
        };

        let frame_bytes = frame.as_bytes();
        let parsed_frame = StreamBlockedFrame::from_bytes(&frame_bytes).unwrap();

        assert_eq!(frame, parsed_frame);
    }
}