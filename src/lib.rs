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

// Private modules
mod frames;
mod header;

// Public modules
pub mod error;
pub mod packet;
pub mod client;


mod tests {
    #[test]
    fn creates_new_client() {
        let _ = super::client::QuicClient::new("localhost", 443);
    }
}
