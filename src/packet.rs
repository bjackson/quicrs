

use error::QuicError;
use error::Result;


use std::io::Cursor;
use std::io::Read;

use byteorder::{WriteBytesExt, ReadBytesExt, BigEndian};


use header::QuicHeader;
use header::ShortHeader;
use header::LongHeader;

use frames::QuicFrame;

bitflags! {
    pub flags ShortPacketType: u8 {
        const ONE_BYTE = 0x01,
        const TWO_BYTES = 0x02,
        const FOUR_BYTES = 0x03,
    }
}

bitflags! {
    pub flags PacketType: u8 {
        const VERSION_NEGOTIATION = 0x01,
        const CLIENT_CLEARTEXT = 0x02,
        const NON_FINAL_CLEARTEXT = 0x03,
        const FINAL_SERVER_CLEAR_TEXT = 0x04,
        const RTT0_ENCRYPTED = 0x05,
        const RTT1_ENCRYPTED_PHASE0 = 0x06,
        const RTT1_ENCRYPTED_PHASE1 = 0x07,
        const PUBLIC_RESET = 0x08,
    }
}

#[derive(Debug)]
pub struct PublicResetPayload {

}

#[derive(Debug)]
pub struct VersionNegotiationPayload {
    pub versions: Vec<u32>,
}

impl VersionNegotiationPayload {
    pub fn from_bytes(buf: &[u8]) -> Result<VersionNegotiationPayload> {
        let mut reader = Cursor::new(buf);

        let mut versions = Vec::new();

        loop {
            match reader.read_u32::<BigEndian>() {
                Ok(version) => versions.push(version),
                Err(_) => break
            }
        }

        Ok(VersionNegotiationPayload {
            versions: versions,
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        self.versions.iter().map(|&version| bytes.write_u32::<BigEndian>(version));

        bytes
    }
}

#[derive(Debug)]
pub enum QuicPayload {
    Frames(Vec<QuicFrame>),
    PublicReset(PublicResetPayload),
    VersionNegotiation(VersionNegotiationPayload),
}

impl QuicPayload {
    pub fn as_bytes(&self) -> Vec<u8> {
        match *self {
            QuicPayload::Frames(ref frames) => {
                frames.iter().map(|ref frame| {
                    frame.as_bytes()
                })
                    .collect::<Vec<_>>()
                    .concat()
            },
            QuicPayload::PublicReset(_) => vec![],
            QuicPayload::VersionNegotiation(ref version_payload) => version_payload.as_bytes(),
        }
    }
}

#[derive(Debug)]
pub struct QuicPacket {
    pub header: QuicHeader,
    pub payload: QuicPayload,
}

impl QuicPacket {
    pub fn from_bytes(buf: &Vec<u8>) -> Result<QuicPacket> {
        let mut reader = Cursor::new(buf);
        let first_byte = reader.read_uint::<BigEndian>(1)? as u8;

        if first_byte & 0x80 != 0 { // Long Header
            let mut header_bytes = [0u8; 17];

            reader.read_exact(&mut header_bytes);

            let header = LongHeader::from_bytes(&header_bytes)?;

            let mut payload_bytes = Vec::new();
            let _ = reader.read_to_end(&mut payload_bytes);


            let payload = match PacketType::from_bits(first_byte) {
                Some(CLIENT_CLEARTEXT) | Some(NON_FINAL_CLEARTEXT) | Some(FINAL_SERVER_CLEAR_TEXT) =>
                    QuicPayload::Frames(QuicPacket::parse_decrypted_payload(payload_bytes.as_slice())?),
                Some(VERSION_NEGOTIATION) =>
                    QuicPayload::VersionNegotiation(VersionNegotiationPayload::from_bytes(&payload_bytes)?),
                Some(_) => return Err(QuicError::ParseError),
                None => return Err(QuicError::ParseError),
            };

            return Ok(QuicPacket {
                header: QuicHeader::Long(header),
                payload: payload
            })
        } else { // ShortHeader
            let packet_type = match ShortPacketType::from_bits(first_byte & 0x1f) {
                Some(pt) => pt,
                None => return Err(QuicError::ParseError)
            };

            let mut header_len = 1;

            let conn_id_bit = first_byte & 0x40 > 0;

            if conn_id_bit {
                header_len += 8;
            }

            match packet_type {
                ONE_BYTE => header_len += 1,
                TWO_BYTES => header_len += 2,
                FOUR_BYTES => header_len += 4,
                _ => return Err(QuicError::ParseError)
            };
            
            let mut header_bytes = vec![0u8; header_len];

            reader.read_exact(&mut header_bytes);

            let header = ShortHeader::from_bytes(header_bytes.as_slice())?;

            // TODO: Decrypt frames and return the payloads.
            return Ok(QuicPacket {
                header: QuicHeader::Short(header),
                payload: QuicPayload::Frames(vec![])
            })
        }
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        let header_bytes = match self.header {
            QuicHeader::Short(ref header) => header.as_bytes(),
            QuicHeader::Long(ref header) => header.as_bytes(),
        };

        let payload_bytes = self.payload.as_bytes();

        let packet_bytes = [header_bytes, payload_bytes].concat();

        if packet_bytes.len() > 1232 {
            return Err(QuicError::PacketTooLarge);
        }

        Ok(packet_bytes)
    }

    pub fn parse_decrypted_payload(buf: &[u8]) -> Result<Vec<QuicFrame>> {
        let mut frames: Vec<QuicFrame> = Vec::new();

        let mut position = 0;
        loop {
            let frame_len = QuicFrame::frame_length(&buf[position..])?;
            let frame_end = position + frame_len;

            let frame = QuicFrame::from_bytes(&buf[position..frame_end])?;

            position += frame_len;

            frames.push(frame);

            if position == buf.len() {
                break;
            }
        }

        Ok(frames)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frames;

    #[test]
    fn parse_decrypted_payload_1() {
        let reason = "Terrible connection!";
        let frames = vec![
            QuicFrame::ConnectionClose(frames::connection_close_frame::ConnectionCloseFrame {
                error_code: 29,
                reason_length: reason.len() as u16,
                reason_phrase: Some(reason.to_string()),
            }),
            QuicFrame::MaxStreamData(frames::max_stream_data_frame::MaxStreamDataFrame {
                stream_id: 20099 as u32,
                max_stream_data: 290 as u64,
            }),
        ];
        
        let mut bytes = Vec::new();

        for frame in &frames {
            let frame_bytes = frame.as_bytes();
            bytes.extend(frame_bytes);
        }

        let parsed_frames = QuicPacket::parse_decrypted_payload(&bytes).unwrap();

        assert_eq!(&frames, &parsed_frames);
    }
}

