use std::net::TcpListener;
fn main() {
    let _listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in _listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
    }

}