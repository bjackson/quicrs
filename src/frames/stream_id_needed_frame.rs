use std::io::Cursor;
use byteorder::{WriteBytesExt, ReadBytesExt};
use error::Result;


#[derive(Debug, PartialEq)]
pub struct StreamIdNeededFrame {}

impl StreamIdNeededFrame {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(17);

        let first_byte = super::STREAM_ID_NEEDED.bits();

        bytes.write_u8(first_byte);

        bytes
    }

    pub fn from_bytes(buf: &[u8]) -> Result<StreamIdNeededFrame> {
        let mut reader = Cursor::new(buf);

        let _ = reader.read_u8()?;


        Ok(StreamIdNeededFrame {})
    }

    pub fn frame_len() -> Result<usize> {
        Ok(17)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let frame = StreamIdNeededFrame { };

        let frame_bytes = frame.as_bytes();
        let parsed_frame = StreamIdNeededFrame::from_bytes(&frame_bytes).unwrap();

        assert_eq!(frame, parsed_frame);
    }
}