//use std::collections::VecDeque;

//use itertools::Itertools;

//use error::QuicError;
use error::Result;

use frames::stream_frame::StreamFrame;

#[derive(Debug, PartialEq)]
pub enum StreamState {
    Idle,
    Open,
    HalfClosedLocal,
    HalfClosedRemote,
    Closed
}

#[derive(Debug, PartialEq)]
pub struct Stream<'a> {
    pub id: u32,
    pub state: StreamState,
    pub max_data: u64,
    pub offset: u64,
    pub frame_queue: Vec<&'a StreamFrame>,
    pub next_offset: u64,
}

impl<'a> Stream<'a> {
    pub fn new(id: u32, max_data: u64) -> Result<Stream<'a>> {
        Ok(Stream {
            id: id,
            state: StreamState::Idle,
            max_data: max_data,
            offset: 0,
            frame_queue: Vec::with_capacity(100),
            next_offset: 0,
        })
    }

//    pub fn on_receive_frame(&mut self, frame: &'a StreamFrame) -> Option<Vec<u8>> {
//        if frame.offset == self.next_offset {
////            self.next_offset += frame.stream_data.len() as u64;
////            println!("self.next_offset = {:?}", self.next_offset);
//
//            self.frame_queue.push(frame);
//            self.frame_queue.sort_by_key(|frame| frame.offset);
//
//            let last_offset = self.frame_queue.iter()
//                .filter(|f| f.offset <= self.next_offset)
//                .max_by_key(|f| f.offset).unwrap().offset as u64;
//
//            let last_len = self.frame_queue.iter()
//                .filter(|f| f.offset <= self.next_offset)
//                .max_by_key(|f| f.offset).unwrap().stream_data.len() as u64;
//
//            self.next_offset += last_len + last_offset;
//            println!("self.next_offset = {:?}", self.next_offset);
//
//            let stream_bytes = self.frame_queue.iter()
//                .filter(|f| f.offset <= self.next_offset)
//                .map(|f| f.stream_data.clone())
//                .collect::<Vec<_>>()
//                .concat();
//
//
//
//            self.frame_queue = self.frame_queue.iter()
//                .filter(|f| f.offset > self.next_offset)
//                .map(|f| *f)
//                .collect();
//
//            return Some(stream_bytes)
//        } else {
//            self.frame_queue.push(frame);
//            self.frame_queue.sort_by_key(|frame| frame.offset);
////            println!("frame.offset = {:?}", frame.offset);
////            println!("pass self.next_offset = {:?}", self.next_offset);
////            println!("self.frame_queue = {:?}", self.frame_queue);
//            return None
//        }
//
//
//
//    }

    pub fn on_receive_frame(&mut self, frame: &'a StreamFrame) -> Option<Vec<u8>> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receive_frame() {
        let frame_1 = StreamFrame {
            f: false,
            data_length_present: true,
            data_length: Some(15),
            stream_id: 1,
            offset: 0,
            stream_data: vec![1u8; 15],
        };

        let frame_2 = StreamFrame {
            f: false,
            data_length_present: true,
            data_length: Some(25),
            stream_id: 1,
            offset: 15,
            stream_data: vec![2u8; 25],
        };

        let frame_3 = StreamFrame {
            f: false,
            data_length_present: true,
            data_length: Some(45),
            stream_id: 1,
            offset: 40,
            stream_data: vec![3u8; 45],
        };

        let frame_4 = StreamFrame {
            f: false,
            data_length_present: true,
            data_length: Some(20),
            stream_id: 1,
            offset: 85,
            stream_data: vec![4u8; 20],
        };
//
//        let frame_5 = StreamFrame {
//            f: false,
//            data_length_present: true,
//            data_length: Some(10),
//            stream_id: 1,
//            offset: 105,
//            stream_data: vec![4u8; 10],
//        };

        let mut stream = Stream::new(1, 250000).unwrap();

        let r_1 = stream.on_receive_frame(&frame_1);

        assert_eq!(frame_1.stream_data, r_1.unwrap());

        let r_3 = stream.on_receive_frame(&frame_3);

        assert_eq!(r_3, None);

        let r_2 = stream.on_receive_frame(&frame_2);

        assert_eq!(r_2.unwrap(), [frame_2.stream_data.clone(), frame_3.stream_data.clone()].concat());

        let r_4 = stream.on_receive_frame(&frame_4);

        assert_eq!(r_4.unwrap(), frame_4.stream_data);

    }
}