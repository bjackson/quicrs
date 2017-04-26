pub mod stream_frame;
pub mod ack_frame;

use self::stream_frame::StreamFrame;
use self::ack_frame::AckFrame;

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
        const ACK = 0xa0,
        const STREAM = 0xc0,
    }
}

#[derive(Debug)]
pub enum QuicFrame {
    Stream(StreamFrame),
    Ack(AckFrame),
}