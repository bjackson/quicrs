//use std::net::UdpSocket;
use tokio_core::net::UdpSocket;
use tokio_core::reactor::Core;
//use std::sync::{Arc, Mutex};
use error::Result;
use stream::QuicStream;

#[derive(Debug)]
pub struct QuicClient<'a> {
    pub socket: UdpSocket,
    pub current_packet_number: u32,
    pub address: String,
    pub streams: Vec<QuicStream<'a>>,
}


impl<'a> QuicClient<'a> {
    pub fn new(address: &str, port: u16) -> Result<QuicClient> {
        let core = Core::new().unwrap();
        let handle = core.handle();

        let address = format!("{}:{}", address, port).parse()?;
        let udp_socket = UdpSocket::bind(&"0.0.0.0:0".parse().unwrap(), &handle)?;

        let mut client = QuicClient {
            socket: udp_socket,
            current_packet_number: QuicClient::get_first_packet_number()?,
            address: address,
            streams: vec![]
        };

        let tls_stream = QuicStream::new(0, 2u64.pow(60))?;

        client.streams.push(tls_stream);


        Ok(client)
    }

    pub fn get_first_packet_number() -> Result<u32> {
        use rand::{OsRng};
        use rand::distributions::{IndependentSample, Range};

        let between = Range::new(0u32, 2u32.pow(31) - 1);
        let mut rng = OsRng::new()?;

        Ok(between.ind_sample(&mut rng))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn create_client() {
        let _ = QuicClient::new("127.0.0.1", 443);
    }
}