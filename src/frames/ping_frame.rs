use std::io::Cursor;
use byteorder::{WriteBytesExt, ReadBytesExt};
use error::Result;

#[derive(Debug, PartialEq)]
pub struct PingFrame {}

impl PingFrame {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let first_byte = super::PING.bits();

        bytes.write_u8(first_byte);

        bytes
    }

    pub fn from_bytes(buf: &[u8]) -> Result<PingFrame> {
        let mut reader = Cursor::new(buf);

        let _ = reader.read_u8()?;


        Ok(PingFrame {})
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
        let frame = PingFrame { };

        let frame_bytes = frame.as_bytes();
        let parsed_frame = PingFrame::from_bytes(&frame_bytes).unwrap();

        assert_eq!(frame, parsed_frame);
    }
}