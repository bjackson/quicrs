use std::io::Cursor;
use byteorder::{WriteBytesExt, ReadBytesExt, BigEndian};

//mod error;
use error::QuicError;
use error::Result;
use std::io::Read;

#[derive(Debug, PartialEq)]
pub struct StreamFrame {
    pub f: bool,
    pub data_length_present: bool,
    pub data_length: Option<u16>,
    pub stream_id: u32,
    pub offset: u64,
    pub stream_data: Vec<u8>,
}

impl StreamFrame {
    pub fn from_bytes(buf: &Vec<u8>) -> Result<StreamFrame> {
        let mut reader = Cursor::new(buf);
        let first_octet = reader.read_u8()?;

        let f = first_octet & 0x20 > 0;
        let data_length_present = first_octet & 0x10 > 0;

        let oo = (first_octet >> 2) & 0x03;
        let ss = first_octet & 0x03;

        let data_length = match data_length_present {
            true => Some(reader.read_u16::<BigEndian>()?),
            false => None,
        };


        let stream_id = reader.read_uint::<BigEndian>((ss + 1) as usize)? as u32;

        let offset = match oo {
            0 => 0,
            1 => reader.read_u16::<BigEndian>()? as u64,
            2 => reader.read_u32::<BigEndian>()? as u64,
            3 => reader.read_u64::<BigEndian>()?,
            _ => return Err(QuicError::ParseError)
        };

        let mut stream_data = Vec::new();

        match data_length {
            None => {
                reader.read_to_end(&mut stream_data)?;
            },
            Some(length) => {
                let mut reader_handle = reader.take(length as u64);
                reader_handle.read_to_end(&mut stream_data)?;
            }
        }


        Ok(StreamFrame {
            f: f,
            data_length_present: data_length_present,
            data_length: data_length,
            stream_id: stream_id,
            offset: offset,
            stream_data: stream_data,
        })
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut byte_vector = Vec::new();

        let mut type_byte = 0xc0;

        if self.f {
            type_byte |= 0x20;
        }

        // Signify that we are putting the data length in.
        type_byte |= 0x10;

        if self.offset != 0 {
            if self.offset <= u16::max_value() as u64 {
                type_byte |= 0x04;
            } else if self.offset <= u32::max_value() as u64 {
                type_byte |= 0x08;
            } else if self.offset <= u64::max_value() {
                type_byte |= 0x0c;
            }
        }

        // Skipping 3 byte stream_id's for now.
        if self.stream_id <= (u8::max_value() as u32) {
            type_byte = type_byte | 0x00;
            byte_vector.write_u8(type_byte);
            byte_vector.write_u16::<BigEndian>(self.data_length.unwrap());
            byte_vector.write_u8(self.stream_id as u8);
        } else if self.stream_id <= (u16::max_value() as u32) {
            type_byte = type_byte | 0x01;
            byte_vector.write_u8(type_byte);
            byte_vector.write_u16::<BigEndian>(self.data_length.unwrap());
            byte_vector.write_u16::<BigEndian>(self.stream_id as u16);
        } else {
            type_byte = type_byte | 0x03;
            byte_vector.write_u8(type_byte);
            byte_vector.write_u16::<BigEndian>(self.data_length.unwrap());
            byte_vector.write_u32::<BigEndian>(self.stream_id as u32);
        }


        if self.offset != 0 {
            if self.offset <= u16::max_value() as u64 {
                byte_vector.write_u16::<BigEndian>(self.offset as u16);
            } else if self.offset <= u32::max_value() as u64 {
                byte_vector.write_u32::<BigEndian>(self.offset as u32);
            } else if self.offset <= u64::max_value() {
                byte_vector.write_u64::<BigEndian>(self.offset);
            }
        }

        byte_vector.extend(&self.stream_data);



        Ok(byte_vector)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_stream_frame() {
        let frame = StreamFrame {
            f: false,
            data_length_present: true,
            data_length: Some(50),
            stream_id: 259,
            offset: 340,
            stream_data: vec![10u8; 50],
        };

        let bytes = frame.as_bytes().unwrap();
        let parsed_frame = StreamFrame::from_bytes(&bytes).unwrap();

        assert_eq!(parsed_frame, frame);
    }
}