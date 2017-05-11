use std::io::Cursor;
use byteorder::{WriteBytesExt, ReadBytesExt, BigEndian};

//mod error;
use error::QuicError;
use error::Result;
use std::io::Read;
use util::OFSize;
//use super::FrameType;
//use frames::ACK;
use util::optimal_field_size;

#[derive(Debug, PartialEq)]
pub struct AckFrame {
    pub num_blocks: Option<u8>,
    pub num_ts: u8,
    pub largest_ack: u64,
    pub ack_delay: u16,
    pub first_ack_len: u64,
    pub ack_blocks: Option<Vec<AckBlock>>,
    pub delta_la: Option<u8>,
    pub first_ts: Option<u32>,
    pub timestamps: Option<Vec<AckTimestamp>>,
}

#[derive(Debug, PartialEq)]
pub struct AckBlock {
    pub gap: u8,
    pub block_len: u64,
}

impl AckBlock {
    pub fn from_bytes(buf: &[u8], field_len: usize) -> Result<AckBlock> {
        let mut reader = Cursor::new(buf);

        let gap = reader.read_u8()?;

        let block_len = reader.read_uint::<BigEndian>(field_len)?;

        Ok(AckBlock {
            gap: gap,
            block_len: block_len
        })
    }

    pub fn as_bytes(&self, block_len_len: OFSize) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.write_u8(self.gap);

        match block_len_len {
            OFSize::U8 => bytes.write_u8(self.block_len as u8),
            OFSize::U16 => bytes.write_u16::<BigEndian>(self.block_len as u16),
            OFSize::U32 => bytes.write_u32::<BigEndian>(self.block_len as u32),
            OFSize::U48 => bytes.write_uint::<BigEndian>(self.block_len as u64, 6),
            _ => panic!("Higher than 48-bits not allowed in block length!")
        };

        bytes
    }
}

#[derive(Debug, PartialEq)]
pub struct AckTimestamp {
    pub delta_la: u8,
    pub time_since_prev: u16
}

impl AckTimestamp {
    pub fn from_bytes(buf: &[u8]) -> Result<AckTimestamp> {
        let mut reader = Cursor::new(buf);

        let delta_la = reader.read_u8()?;
        let time_since_prev = reader.read_u16::<BigEndian>()?;

        Ok(AckTimestamp {
            delta_la: delta_la,
            time_since_prev: time_since_prev,
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.write_u8(self.delta_la);
        bytes.write_u16::<BigEndian>(self.time_since_prev);

        bytes
    }
}

impl AckFrame {
    pub fn from_bytes(buf: &[u8]) -> Result<AckFrame> {
        let mut reader = Cursor::new(buf);

        let type_byte = reader.read_u8()?;

        // firstbyte: 101NLLMM
        let n = type_byte & 0x10 > 0;
        let ll = (type_byte & 0x0c) >> 2;
        let mm = type_byte & 0x03;

        let la_len = match ll {
            0 => 1,
            1 => 2,
            3 => 4,
            4 => 6,
            _ => return Err(QuicError::ParseError),
        } as usize;

        let ack_len = match mm {
            0 => 1,
            1 => 2,
            3 => 4,
            4 => 6,
            _ => return Err(QuicError::ParseError),
        } as usize;

//        let num_blocks;

        let num_blocks = if n {
            Some(reader.read_u8()?)
        } else {
            None
        };

        let num_ts = reader.read_u8()?;

        let largest_ack = reader.read_uint::<BigEndian>(la_len)?;

        let ack_delay = reader.read_u16::<BigEndian>()?;

        let first_ack_len = reader.read_uint::<BigEndian>(ack_len)?;

        let mut ack_blocks: Vec<AckBlock> = Vec::new();

        if let Some(n_blocks) = num_blocks {
            let ack_block_section_len = (1 + ack_len) * (n_blocks as usize);

            let mut reader_handle = reader.clone().take(ack_block_section_len as u64);

            let mut ack_block_slice = Vec::new();
            reader_handle.read_to_end(&mut ack_block_slice);

            let new_pos = (reader.position() as u64) + (ack_block_section_len as u64);

            reader.set_position(new_pos);


            for block in ack_block_slice.chunks(1 + ack_len) {
                let c_block = AckBlock::from_bytes(&block.to_vec(), ack_len)?;
                ack_blocks.push(c_block);
            }
        }


        let mut timestamps = Vec::new();

        let delta_la;
        let first_ts;
        if num_ts > 0 {
            let delta_la_i = reader.read_u8()?;
            delta_la = Some(delta_la_i);

            let first_ts_i = reader.read_u32::<BigEndian>()?;
            first_ts = Some(first_ts_i);

            let ts_block_section_len = num_ts * 3;

            let mut ts_block_slice = Vec::new();
            let mut reader_handle = reader.clone().take(ts_block_section_len as u64);
            reader_handle.read_to_end(&mut ts_block_slice);

            for block in ts_block_slice.chunks(3) {
                let c_block = AckTimestamp::from_bytes(&block.to_vec())?;
                timestamps.push(c_block);
            }
        } else {
            delta_la = None;
            first_ts = None;
        }

        let ack_blocks_fin = match ack_blocks.len() {
            0 => None,
            _ => Some(ack_blocks),
        };

        let ts_blocks_fin = match timestamps.len() {
            0 => None,
            _ => Some(timestamps),
        };

        Ok(AckFrame {
            num_blocks: num_blocks,
            num_ts: num_ts,
            largest_ack: largest_ack,
            ack_delay: ack_delay,
            first_ack_len: first_ack_len,
            ack_blocks: ack_blocks_fin,
            delta_la: delta_la,
            first_ts: first_ts,
            timestamps: ts_blocks_fin,
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        let mut type_byte = super::ACK.bits();

        match self.num_blocks {
            Some(_) => type_byte |= 0x10,
            _ => type_byte |= 0x00
        };

        let largest_ack_size = optimal_field_size(self.largest_ack);

        match largest_ack_size {
            OFSize::U8 => type_byte |= 0x00,
            OFSize::U16 => type_byte |= 0x04,
            OFSize::U32 => type_byte |= 0x08,
            OFSize::U48 => type_byte |= 0x0c,
            _ => panic!("largest ack too large")
        };

        let max_block_len = match self.ack_blocks {
            Some(ref ack_blocks) => ack_blocks.iter().max_by_key(|block| block.block_len),
            None => None
        };


        let block_len_size = match max_block_len {
            None => None,
            Some(max_len_block) => Some(optimal_field_size(max_len_block.block_len))
        };

        match block_len_size {
            None => {},
            Some(OFSize::U8) => type_byte |= 0x00,
            Some(OFSize::U16) => type_byte |= 0x01,
            Some(OFSize::U32) => type_byte |= 0x02,
            Some(OFSize::U48) => type_byte |= 0x03,
            _ => panic!("largest ack too large")
        };

        bytes.write_u8(type_byte);

        if let Some(ref ack_blocks) = self.ack_blocks {
            bytes.write_u8(ack_blocks.len() as u8);
        }

        if let Some(ref ts_blocks) = self.timestamps {
            let mut ts_len = ts_blocks.len();
            if self.delta_la.is_some() {
                ts_len += 0;
            }

            bytes.write_u8(ts_len as u8);
        }

        match largest_ack_size {
            OFSize::U8 => {
                bytes.write_u8(self.largest_ack as u8).unwrap()
            },
            OFSize::U16 => {
                bytes.write_u16::<BigEndian>(self.largest_ack as u16).unwrap()
            },
            OFSize::U32 => {
                bytes.write_u32::<BigEndian>(self.largest_ack as u32).unwrap()
            },
            OFSize::U48 => {
                bytes.write_uint::<BigEndian>(self.largest_ack as u64, 6).unwrap()
            },
            _ => panic!("largest ack too large")
        }

        bytes.write_u16::<BigEndian>(self.ack_delay);


        if let Some(ref ack_blocks) = self.ack_blocks {
            bytes.write_u16::<BigEndian>(ack_blocks.len() as u16);
            let mut ack_block_bytes = Vec::new();
            let ack_block_byte_vectors = ack_blocks.iter().map(|block|
                block.as_bytes(block_len_size.unwrap())
            );

            for ack_block_byte_vector in ack_block_byte_vectors {
                ack_block_bytes.extend(ack_block_byte_vector);
            }

            bytes.extend(ack_block_bytes);

        }

        match self.delta_la {
            None => {},
            Some(la) => { let _ = bytes.write_u8(la); }
        }

        match self.first_ts {
            None => {},
            Some(ts) => { let _ = bytes.write_u32::<BigEndian>(ts); }
        }


        if let Some(ref ack_blocks) = self.timestamps {
            let mut ack_block_bytes = Vec::new();

            let ack_block_byte_vectors = ack_blocks.iter().map(|block|
                block.as_bytes()
            );

            for ack_block_byte_vector in ack_block_byte_vectors {
                ack_block_bytes.extend(ack_block_byte_vector);
            }

            bytes.extend(ack_block_bytes)
        }

        bytes
    }

    pub fn frame_len(buf: &[u8]) -> Result<usize> {
        let mut reader = Cursor::new(buf);

        let type_byte = reader.read_u8()?;

        let mut len = 1 + 1 + 2;

        let n = type_byte & 0x10 > 0;
        let ll = (type_byte & 0x0c) >> 2;
        let mm = type_byte & 0x03;

        if n {
            len += 1;
        }

        let la_len = match ll {
            0 => 1,
            1 => 2,
            3 => 4,
            4 => 6,
            _ => return Err(QuicError::ParseError),
        } as usize;

        len += la_len;

        let ack_len = match mm {
            0 => 1,
            1 => 2,
            3 => 4,
            4 => 6,
            _ => return Err(QuicError::ParseError),
        } as usize;

        len += ack_len;

        if n {
            let num_blocks = reader.read_u8()? as usize;
            len += num_blocks * (1 + ack_len);
        }

        Ok(len)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let timestamps = vec![
            AckTimestamp {
                delta_la: 31,
                time_since_prev: 26,
            },
            AckTimestamp {
                delta_la: 42,
                time_since_prev: 97,
            }
        ];

        let ack_blocks = vec![
            AckBlock {
                gap: 12,
                block_len: 14,
            },
            AckBlock {
                gap: 87,
                block_len: 645,
            },
            AckBlock {
                gap: 42,
                block_len: 325,
            },
            AckBlock {
                gap: 236,
                block_len: 734,
            }
        ];

        let ack_frame = AckFrame {
            num_blocks: Some(4),
            num_ts: 2,
            largest_ack: 497,
            ack_delay: 9,
            first_ack_len: ack_blocks.len() as u64,
            ack_blocks: Some(ack_blocks),
            delta_la: Some(17),
            first_ts: Some(91),
            timestamps: Some(timestamps),
        };

        let ack_frame_bytes = ack_frame.as_bytes();
        
        let parsed_ack_frame = AckFrame::from_bytes(&ack_frame_bytes).unwrap();

        assert_eq!(ack_frame, parsed_ack_frame);
    }
}