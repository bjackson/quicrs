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

#[derive(Debug)]
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
}