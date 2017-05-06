use std::io::Cursor;
use byteorder::{WriteBytesExt, ReadBytesExt};
use error::Result;

#[derive(Debug, PartialEq)]
pub struct PaddingFrame {}

impl PaddingFrame {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let first_byte = super::PADDING.bits();

        bytes.write_u8(first_byte);

        bytes
    }

    pub fn from_bytes(buf: &Vec<u8>) -> Result<PaddingFrame> {
        let mut reader = Cursor::new(buf);

        let _ = reader.read_u8()?;


        Ok(PaddingFrame {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let frame = PaddingFrame { };

        let frame_bytes = frame.as_bytes();
        let parsed_frame = PaddingFrame::from_bytes(&frame_bytes).unwrap();

        assert_eq!(frame, parsed_frame);
    }
}