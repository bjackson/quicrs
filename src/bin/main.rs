extern crate quic;
extern crate byteorder;


//use quic::header::ShortHeader;
//use quic::packet::TWO_BYTES;

fn main() {
    let sock = quic::client::QuicClient::new("127.0.0.1", 443).expect("Unable to create quic client");
    println!("{:?}", sock.socket);

    let byte_vector = vec![0, 0, 0, 0];
    let packet = match quic::packet::QuicPacket::from_bytes(&byte_vector) {
        Ok(packet) => packet,
        Err(e) => return println!("{:?}", e)
    };

    println!("{:?}", byte_vector);

    println!("{:?}", packet.payload)
}