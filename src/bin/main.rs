extern crate quic;


fn main() {
    let sock = quic::QuicClient::new("127.0.0.1", 443);
    println!("{:?}", sock.socket)
}