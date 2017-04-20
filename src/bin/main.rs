extern crate quic;


fn main() {
    let sock = quic::QuicClient::new("google.com", 443);
    println!("{:?}", sock.socket)
}