use std::io::Cursor;
use std::io::Read;
use byteorder::{WriteBytesExt, ReadBytesExt, BigEndian};
use error::Result;

#[derive(Debug, PartialEq)]
pub struct ConnectionCloseFrame {
    pub error_code: u32,
    pub reason_length: u16,
    pub reason_phrase: Option<String>,
}

impl ConnectionCloseFrame {
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let first_byte = super::CONNECTION_CLOSE.bits();

        bytes.write_u8(first_byte);

        bytes.write_u32::<BigEndian>(self.error_code);

        if let Some(ref phrase) = self.reason_phrase {
            bytes.write_u16::<BigEndian>(phrase.len() as u16);
        } else {
            bytes.write_u16::<BigEndian>(0);
        }

        if let Some(ref phrase) = self.reason_phrase {
            bytes.extend(phrase.as_bytes());
        }

        bytes
    }

    pub fn from_bytes(buf: &[u8]) -> Result<ConnectionCloseFrame> {
        let mut reader = Cursor::new(buf);

        let _ = reader.read_u8()?;

        let error_code = reader.read_u32::<BigEndian>()?;

        let reason_length = reader.read_u16::<BigEndian>()?;

        let reason_phrase;

        if reason_length > 0 {
            let mut phrase_reader = reader.clone().take(reason_length as u64);
            let mut phrase = Vec::new();

            phrase_reader.read_to_end(&mut phrase);

            reason_phrase = Some(String::from_utf8(phrase)?);
        } else {
            reason_phrase = None;
        }

        Ok(ConnectionCloseFrame {
            error_code: error_code,
            reason_length: reason_length,
            reason_phrase: reason_phrase,
        })
    }

    pub fn frame_len(buf: &[u8]) -> Result<usize> {
        let mut reader = Cursor::new(buf);

        let len = 1 + 4 + 2;

        let _ = reader.read_u8()?;
        let _ = reader.read_u32::<BigEndian>()?;
        let reason_len = reader.read_u16::<BigEndian>()? as usize;

        Ok(len + reason_len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let reason_string = "Bad connection!";

        let frame = ConnectionCloseFrame {
            error_code: 5423,
            reason_length: reason_string.len() as u16,
            reason_phrase: Some(reason_string.to_string()),
        };

        let frame_bytes = frame.as_bytes();
        let parsed_frame = ConnectionCloseFrame::from_bytes(&frame_bytes).unwrap();

        assert_eq!(frame, parsed_frame);
    }
}