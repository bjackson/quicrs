#![feature(plugin)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![plugin(bitfield)]
#[cfg(test)]

extern crate core;
extern crate byteorder;
#[macro_use]
extern crate bitflags;
extern crate rand;

use std::net::UdpSocket;


mod error;

use error::Result;

mod frames;
mod header;

pub mod packet;

pub struct QuicClient {
    pub socket: std::net::UdpSocket,
    pub current_packet_number: u32,
    pub address: String
}


impl QuicClient {
    pub fn new(address: &str, port: u16) -> Result<QuicClient> {
        let address = format!("{}:{}", address, port);
        let udp_socket = UdpSocket::bind("0.0.0.0:0")?;

        let client = QuicClient {
            socket: udp_socket,
            current_packet_number: QuicClient::get_first_packet_number(),
            address: address,
        };

//        let init_header = LongHeader {
//            packet_type: VERSION_NEGOTIATION,
//            connection_id: 1,
//            packet_number: client.current_packet_number,
//            version: 1,
//        };

        Ok(client)
    }

    pub fn get_first_packet_number() -> u32 {
        use rand::{OsRng};
        use rand::distributions::{IndependentSample, Range};

        let between = Range::new(0u32, 2u32.pow(31) - 1);
        let mut rng = OsRng::new().expect("Cannot get random number");

        between.ind_sample(&mut rng)
    }
}


mod tests {
    #[test]
    fn creates_new_client() {
        let _ = super::QuicClient::new("localhost", 443);
    }
}
