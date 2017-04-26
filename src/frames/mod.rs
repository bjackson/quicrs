pub mod stream_frame;
pub mod ack_frame;

use self::stream_frame::StreamFrame;
use self::ack_frame::AckFrame;


#[derive(Debug)]
pub enum QuicFrame {
    Stream(StreamFrame),
    Ack(AckFrame),
}