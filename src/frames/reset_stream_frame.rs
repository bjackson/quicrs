use std::io::Cursor;
use byteorder::{WriteBytesExt, ReadBytesExt, BigEndian};
use error::Result;

#[derive(Debug, PartialEq)]
pub struct ResetStreamFrame {
    pub error_code: u32,
    pub stream_id: u32,
    pub final_offset: u64,
}

impl ResetStreamFrame {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let first_byte = super::RST_STREAM.bits();

        bytes.write_u8(first_byte);

        bytes.write_u32::<BigEndian>(self.error_code);
        bytes.write_u32::<BigEndian>(self.stream_id);
        bytes.write_u64::<BigEndian>(self.final_offset);

        bytes
    }

    pub fn from_bytes(buf: &[u8]) -> Result<ResetStreamFrame> {
        let mut reader = Cursor::new(buf);

        let _ = reader.read_u8()?;

        let error_code = reader.read_u32::<BigEndian>()?;
        let stream_id = reader.read_u32::<BigEndian>()?;
        let final_offset = reader.read_u64::<BigEndian>()?;

        Ok(ResetStreamFrame {
            error_code: error_code,
            stream_id: stream_id,
            final_offset: final_offset,
        })
    }

    pub fn frame_len() -> Result<usize> {
        Ok(11)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let frame = ResetStreamFrame {
            error_code: 4235,
            stream_id: 25232235,
            final_offset: 7563422532,
        };

        let frame_bytes = frame.as_bytes();
        let parsed_frame = ResetStreamFrame::from_bytes(&frame_bytes).unwrap();

        assert_eq!(frame, parsed_frame);
    }
}