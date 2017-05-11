use std::io::Cursor;
use byteorder::{WriteBytesExt, ReadBytesExt, BigEndian};
use error::Result;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MaxStreamDataFrame {
    pub stream_id: u32,
    pub max_stream_data: u64,
}

impl MaxStreamDataFrame {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(13);

        let first_byte = super::MAX_STREAM_DATA.bits();

        bytes.write_u8(first_byte);

        bytes.write_u32::<BigEndian>(self.stream_id);

        bytes.write_u64::<BigEndian>(self.max_stream_data);

        bytes
    }

    pub fn from_bytes(buf: &[u8]) -> Result<MaxStreamDataFrame> {
        let mut reader = Cursor::new(buf);

        let _ = reader.read_u8()?;

        let stream_id = reader.read_u32::<BigEndian>()?;

        let max_stream_data = reader.read_u64::<BigEndian>()?;

        Ok(MaxStreamDataFrame {
            stream_id: stream_id,
            max_stream_data: max_stream_data,
        })
    }

    pub fn frame_len() -> Result<usize> {
        Ok(13)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_max_stream_data_frame() {
        let frame = MaxStreamDataFrame {
            stream_id: 23523,
            max_stream_data: 293521,
        };

        let frame_bytes = frame.as_bytes();
        let parsed_frame = MaxStreamDataFrame::from_bytes(&frame_bytes).unwrap();

        assert_eq!(frame, parsed_frame);
    }
}