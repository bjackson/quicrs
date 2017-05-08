pub mod stream_frame;
pub mod ack_frame;
pub mod max_data_frame;
pub mod max_stream_data_frame;
pub mod max_stream_id_frame;
pub mod blocked_frame;
pub mod stream_blocked_frame;
pub mod stream_id_needed_frame;
pub mod padding_frame;
pub mod ping_frame;
pub mod new_connection_id_frame;
pub mod connection_close_frame;
pub mod goaway_frame;
pub mod reset_stream_frame;

use self::stream_frame::StreamFrame;
use self::ack_frame::AckFrame;
use self::max_data_frame::MaxDataFrame;
use self::max_stream_data_frame::MaxStreamDataFrame;
use self::max_stream_id_frame::MaxStreamIdFrame;
use self::blocked_frame::BlockedFrame;
use self::stream_blocked_frame::StreamBlockedFrame;
use self::stream_id_needed_frame::StreamIdNeededFrame;
use self::padding_frame::PaddingFrame;
use self::ping_frame::PingFrame;
use self::new_connection_id_frame::NewConnectionIdFrame;
use self::connection_close_frame::ConnectionCloseFrame;
use self::goaway_frame::GoAwayFrame;
use self::reset_stream_frame::ResetStreamFrame;

use super::error::Result;
use super::error::QuicError;

use std::io::Cursor;
use byteorder::{ReadBytesExt};


bitflags! {
    pub flags FrameType: u8 {
        const PADDING = 0x00,
        const RST_STREAM = 0x01,
        const CONNECTION_CLOSE = 0x02,
        const GOAWAY = 0x03,
        const MAX_DATA = 0x04,
        const MAX_STREAM_DATA = 0x05,
        const MAX_STREAM_ID = 0x06,
        const PING = 0x07,
        const BLOCKED = 0x08,
        const STREAM_BLOCKED = 0x09,
        const STREAM_ID_NEEDED = 0x0a,
        const ACK = 0xa0,
        const NEW_CONNECTION_ID = 0x0b,
        const STREAM = 0xc0,
    }
}

//pub trait QuicFrame {
//    fn as_bytes(&self) -> Vec<u8>;
//    fn from_bytes<T>(buf: &Vec<u8>) -> Result<T>;
//}

#[derive(Debug, PartialEq)]
pub enum QuicFrame {
    Stream(StreamFrame),
    Ack(AckFrame),
    MaxData(MaxDataFrame),
    MaxStreamData(MaxStreamDataFrame),
    MaxStreamId(MaxStreamIdFrame),
    Blocked(BlockedFrame),
    StreamBlocked(StreamBlockedFrame),
    StreamIdNeeded(StreamIdNeededFrame),
    Padding(PaddingFrame),
    Ping(PingFrame),
    NewConnectionId(NewConnectionIdFrame),
    ConnectionClose(ConnectionCloseFrame),
    GoAway(GoAwayFrame),
    ResetStream(ResetStreamFrame),
}

impl QuicFrame {
    pub fn as_bytes(&self) -> Vec<u8> {
        match &self {
            &&QuicFrame::Stream(ref f) => f.as_bytes(),
            &&QuicFrame::Ack(ref f) => f.as_bytes(),
            &&QuicFrame::MaxData(ref f) => f.as_bytes(),
            &&QuicFrame::MaxStreamData(ref f) => f.as_bytes(),
            &&QuicFrame::MaxStreamId(ref f) => f.as_bytes(),
            &&QuicFrame::Blocked(ref f) => f.as_bytes(),
            &&QuicFrame::StreamBlocked(ref f) => f.as_bytes(),
            &&QuicFrame::StreamIdNeeded(ref f) => f.as_bytes(),
            &&QuicFrame::Padding(ref f) => f.as_bytes(),
            &&QuicFrame::Ping(ref f) => f.as_bytes(),
            &&QuicFrame::NewConnectionId(ref f) => f.as_bytes(),
            &&QuicFrame::ConnectionClose(ref f) => f.as_bytes(),
            &&QuicFrame::GoAway(ref f) => f.as_bytes(),
            &&QuicFrame::ResetStream(ref f) => f.as_bytes(),
        }
    }

    pub fn from_bytes(buf: &[u8]) -> Result<QuicFrame> {
//        use std::ops::Index;

        if buf.len() == 0 {
            return Err(QuicError::ParseError);
        }

        let frame_type = buf[0];

        if (frame_type & 0xf0) == ACK.bits() {
            let frame = AckFrame::from_bytes(&buf)?;

            return Ok(QuicFrame::Ack(frame));
        }

        if (frame_type & 0xf0) == STREAM.bits() {
            let frame = StreamFrame::from_bytes(&buf)?;

            return Ok(QuicFrame::Stream(frame));
        }

        if frame_type == RST_STREAM.bits() {
            let frame = ResetStreamFrame::from_bytes(&buf)?;

            return Ok(QuicFrame::ResetStream(frame));
        }

        if frame_type == CONNECTION_CLOSE.bits() {
            let frame = ConnectionCloseFrame::from_bytes(&buf)?;

            return Ok(QuicFrame::ConnectionClose(frame));
        }

        if frame_type == GOAWAY.bits() {
            let frame = GoAwayFrame::from_bytes(&buf)?;

            return Ok(QuicFrame::GoAway(frame));
        }

        if frame_type == MAX_DATA.bits() {
            let frame = MaxDataFrame::from_bytes(&buf)?;

            return Ok(QuicFrame::MaxData(frame));
        }

        if frame_type == MAX_STREAM_DATA.bits() {
            let frame = MaxStreamDataFrame::from_bytes(&buf)?;

            return Ok(QuicFrame::MaxStreamData(frame));
        }

        if frame_type == MAX_STREAM_ID.bits() {
            let frame = MaxStreamIdFrame::from_bytes(&buf)?;

            return Ok(QuicFrame::MaxStreamId(frame));
        }

        if frame_type == PING.bits()  {
            let frame = PingFrame::from_bytes(&buf)?;

            return Ok(QuicFrame::Ping(frame));
        }

        if frame_type == BLOCKED.bits() {
            let frame = BlockedFrame::from_bytes(&buf)?;

            return Ok(QuicFrame::Blocked(frame));
        }

        if frame_type == STREAM_BLOCKED.bits() {
            let frame = StreamBlockedFrame::from_bytes(&buf)?;

            return Ok(QuicFrame::StreamBlocked(frame));
        }

        if frame_type == STREAM_ID_NEEDED.bits() {
            let frame = StreamIdNeededFrame::from_bytes(&buf)?;

            return Ok(QuicFrame::StreamIdNeeded(frame));
        }

        if frame_type == NEW_CONNECTION_ID.bits() {
            let frame = NewConnectionIdFrame::from_bytes(&buf)?;

            return Ok(QuicFrame::NewConnectionId(frame));
        }

        Ok(QuicFrame::Padding(PaddingFrame{}))
    }

    pub fn frame_length(buf: &[u8]) -> Result<usize> {
        let mut reader = Cursor::new(buf);

        let type_byte = reader.read_u8()?;

        return match FrameType::from_bits(type_byte) {
            Some(PADDING) => Ok(PaddingFrame::frame_len()?),
            Some(RST_STREAM) => Ok(ResetStreamFrame::frame_len()?),
            Some(CONNECTION_CLOSE) => Ok(ConnectionCloseFrame::frame_len(&buf)?),
            Some(GOAWAY) => Ok(GoAwayFrame::frame_len()?),
            Some(MAX_DATA) => Ok(MaxDataFrame::frame_len()?),
            Some(MAX_STREAM_DATA) => Ok(MaxStreamDataFrame::frame_len()?),
            Some(MAX_STREAM_ID) => Ok(MaxStreamIdFrame::frame_len()?),
            Some(PING) => Ok(PingFrame::frame_len()?),
            Some(BLOCKED) => Ok(BlockedFrame::frame_len()?),
            Some(STREAM_BLOCKED) => Ok(StreamBlockedFrame::frame_len()?),
            Some(STREAM_ID_NEEDED) => Ok(StreamIdNeededFrame::frame_len()?),
            Some(NEW_CONNECTION_ID) => Ok(NewConnectionIdFrame::frame_len()?),
            Some(ACK) => Ok(AckFrame::frame_len(&buf)?),
            Some(STREAM) => Ok(StreamFrame::frame_len(&buf)?),
            Some(_) => return Err(QuicError::ParseError),
            None => return Err(QuicError::ParseError)
        }
    }
}