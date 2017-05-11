use std::io::Cursor;
use byteorder::{WriteBytesExt, ReadBytesExt, BigEndian};
use error::Result;

#[derive(Debug, PartialEq)]
pub struct MaxDataFrame {
    pub max_data: u64,
}

impl MaxDataFrame {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(13);

        let first_byte = super::MAX_DATA.bits();

        bytes.write_u8(first_byte);

        bytes.write_u64::<BigEndian>(self.max_data);

        bytes
    }
    
    pub fn from_bytes(buf: &[u8]) -> Result<MaxDataFrame> {
        let mut reader = Cursor::new(buf);

        let _ = reader.read_u8()?;

        let max_data = reader.read_u64::<BigEndian>()?;

        Ok(MaxDataFrame {
            max_data: max_data,
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
    fn serialize() {
        let frame = MaxDataFrame {
            max_data: 293521,
        };
        
        let frame_bytes = frame.as_bytes();
        let parsed_frame = MaxDataFrame::from_bytes(&frame_bytes).unwrap();

        assert_eq!(frame, parsed_frame);
    }
}