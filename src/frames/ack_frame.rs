use std::io::Cursor;
use byteorder::{ReadBytesExt, BigEndian};

//mod error;
use error::QuicError;
use error::Result;
use std::io::Read;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct AckBlock {
    pub gap: u8,
    pub block_len: u64,
}

impl AckBlock {
    pub fn from_bytes(buf: &Vec<u8>, field_len: usize) -> Result<AckBlock> {
        let mut reader = Cursor::new(buf);

        let gap = reader.read_u8()?;

        let block_len = reader.read_uint::<BigEndian>(field_len)?;

        Ok(AckBlock {
            gap: gap,
            block_len: block_len
        })
    }
}

#[derive(Debug)]
pub struct AckTimestamp {
    pub delta_la: u8,
    pub time_since_prev: u16
}

impl AckTimestamp {
    pub fn from_bytes(buf: &Vec<u8>) -> Result<AckTimestamp> {
        let mut reader = Cursor::new(buf);

        let delta_la = reader.read_u8()?;
        let time_since_prev = reader.read_u16::<BigEndian>()?;

        Ok(AckTimestamp {
            delta_la: delta_la,
            time_since_prev: time_since_prev,
        })
    }
}

impl AckFrame {
    pub fn from_bytes(buf: &Vec<u8>) -> Result<AckFrame> {
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

        let num_blocks;

        if n {
            num_blocks = Some(reader.read_u8()?);
        } else {
            num_blocks = None;
        }

        let num_ts = reader.read_u8()?;

        let largest_ack = reader.read_uint::<BigEndian>(la_len)?;

        let ack_delay = reader.read_u16::<BigEndian>()?;

        let first_ack_len = reader.read_uint::<BigEndian>(ack_len)?;

        let mut ack_blocks: Vec<AckBlock> = Vec::new();

        if let Some(n_blocks) = num_blocks {
            let ack_block_section_len = (1 + ack_len) * (n_blocks as usize);

            let mut reader_handle = reader.clone().take(ack_block_section_len as u64);

            let mut ack_block_slice = Vec::new();
            reader_handle.read(&mut ack_block_slice);


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
            reader_handle.read(&mut ts_block_slice);

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
}