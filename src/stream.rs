//use std::rc::Rc;
//use std::cell::RefCell;

use error::Result;
use error::QuicError;
use frames::stream_frame::StreamFrame;
use futures::Poll;
use futures::Async;
use futures::AsyncSink;

use futures::Stream;
use futures::Sink;
use futures::StartSend;

#[derive(Debug, PartialEq)]
pub enum StreamState {
    Idle,
    Open,
    HalfClosedLocal,
    HalfClosedRemote,
    Closed
}

#[derive(Debug, PartialEq)]
pub struct QuicStream {
    pub id: u32,
    pub state: StreamState,
    pub max_data: u64,
    pub offset: u64,
    pub send_offset: u64,
    pub frame_queue: Vec<StreamFrame>,
    pub next_offset: u64,
    prepared_stream: Vec<u8>,
    frames_to_send: Vec<StreamFrame>
}

impl QuicStream {
    pub fn new(id: u32, max_data: u64) -> Result<QuicStream> {
        Ok(QuicStream {
            id: id,
            state: StreamState::Idle,
            max_data: max_data,
            offset: 0,
            frame_queue: Vec::with_capacity(128),
            next_offset: 0,
            prepared_stream: Vec::with_capacity(1024),
            send_offset: 0,
            frames_to_send: Vec::with_capacity(1024),
        })
    }

    pub fn on_receive_frame(&mut self, frame: &StreamFrame) -> Option<Vec<u8>> {
        self.frame_queue.push(frame.clone());
        self.frame_queue.sort_by_key(|f| f.offset);
        self.frame_queue.dedup_by_key(|f| f.offset);

        let mut next_offset = self.next_offset;

        let mut bytes: Vec<u8> = Vec::with_capacity(256);

        for f in &self.frame_queue {
            if f.offset == next_offset {
                bytes.extend(&f.stream_data);

                next_offset += f.stream_data.len() as u64;
            }
        }

        if !bytes.is_empty() {
            self.frame_queue = self.frame_queue.iter()
                .filter(|f| f.offset >= next_offset)
                .map(|f| f.clone())
                .collect();

            // Set stream state to half-closed (remote) if
            // we receive a packet with the fin flag.
            if frame.fin {
                self.state = StreamState::HalfClosedRemote;
            }

            self.next_offset = next_offset;
            self.prepared_stream.extend(bytes.clone());
            Some(bytes)
        } else {
            None
        }
    }
}

impl Stream for QuicStream {
    type Item = Vec<u8>;
    type Error = QuicError;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if self.prepared_stream.is_empty() {
            Ok(Async::NotReady)
        } else {
            let returned_bytes = self.prepared_stream.clone();
            self.prepared_stream.clear();

            Ok(Async::Ready(Some(returned_bytes)))
        }
    }
}

impl Sink for QuicStream {
    type SinkItem = Vec<u8>;
    type SinkError = QuicError;

    fn start_send(&mut self,
                  item: Self::SinkItem) -> StartSend<Self::SinkItem, Self::SinkError> {
        let frame = StreamFrame {
            fin: false,
            data_length_present: true,
            data_length: Some(item.len() as u16),
            stream_id: self.id,
            offset: self.send_offset,
            stream_data: item.clone(),
        };

        self.frames_to_send.push(frame);

        Ok(AsyncSink::Ready)
    }

    fn poll_complete(&mut self) -> Poll<(), Self::SinkError> {
        Ok(Async::Ready(()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receive_frame() {
        let frame_1 = StreamFrame {
            fin: false,
            data_length_present: true,
            data_length: Some(15),
            stream_id: 1,
            offset: 0,
            stream_data: vec![1u8; 15],
        };

        let frame_2 = StreamFrame {
            fin: false,
            data_length_present: true,
            data_length: Some(25),
            stream_id: 1,
            offset: 15,
            stream_data: vec![2u8; 25],
        };

        let frame_3 = StreamFrame {
            fin: false,
            data_length_present: true,
            data_length: Some(45),
            stream_id: 1,
            offset: 40,
            stream_data: vec![3u8; 45],
        };

        let frame_4 = StreamFrame {
            fin: false,
            data_length_present: true,
            data_length: Some(20),
            stream_id: 1,
            offset: 85,
            stream_data: vec![4u8; 20],
        };

        let frame_5 = StreamFrame {
            fin: false,
            data_length_present: true,
            data_length: Some(10),
            stream_id: 1,
            offset: 105,
            stream_data: vec![5u8; 10],
        };

        let frame_6 = StreamFrame {
            fin: false,
            data_length_present: true,
            data_length: Some(15),
            stream_id: 1,
            offset: 115,
            stream_data: vec![6u8; 15],
        };

        let frame_7 = StreamFrame {
            fin: false,
            data_length_present: true,
            data_length: Some(12),
            stream_id: 1,
            offset: 130,
            stream_data: vec![7u8; 12],
        };

        let mut stream = QuicStream::new(1, 250000).unwrap();

//        stream.for_each(|bytes| {
//            println!("bytes = {:?}", bytes);
//            Ok(())
//        });

        let r_1 = stream.on_receive_frame(&frame_1);

        assert_eq!(frame_1.stream_data, r_1.unwrap());

        let r_3 = stream.on_receive_frame(&frame_3);

        assert_eq!(r_3, None);

        let r_2 = stream.on_receive_frame(&frame_2);

        assert_eq!(r_2.unwrap(), [frame_2.stream_data.clone(), frame_3.stream_data.clone()].concat());

        let r_4 = stream.on_receive_frame(&frame_4);

        assert_eq!(r_4.unwrap(), frame_4.stream_data);

        let r_5 = stream.on_receive_frame(&frame_5);

        assert_eq!(r_5.unwrap(), frame_5.stream_data);

        let r_7 = stream.on_receive_frame(&frame_7);

        assert_eq!(r_7, None);

        let r_6 = stream.on_receive_frame(&frame_6);

        assert_eq!(r_6.unwrap(), [frame_6.stream_data.clone(), frame_7.stream_data.clone()].concat());

        println!("stream = {:?}", stream);

    }
}