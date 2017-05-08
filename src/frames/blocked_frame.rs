use std::io::Cursor;
use byteorder::{WriteBytesExt, ReadBytesExt};
use error::Result;

#[derive(Debug, PartialEq)]
pub struct BlockedFrame {}

impl BlockedFrame {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let first_byte = super::BLOCKED.bits();

        bytes.write_u8(first_byte);

        bytes
    }

    pub fn from_bytes(buf: &[u8]) -> Result<BlockedFrame> {
        let mut reader = Cursor::new(buf);

        let _ = reader.read_u8()?;


        Ok(BlockedFrame {})
    }

    pub fn frame_len() -> Result<usize> {
        Ok(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let frame = BlockedFrame { };

        let frame_bytes = frame.as_bytes();
        let parsed_frame = BlockedFrame::from_bytes(&frame_bytes).unwrap();

        assert_eq!(frame, parsed_frame);
    }
}