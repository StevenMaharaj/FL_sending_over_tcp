//
// tcp client

use std::io::{self, Read};
use std::net::TcpStream;

// use f_messages::message_types::FM;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("localhost:7878")?;
    println!("Connection established!");
    loop {
        let mut buf = [0; 128];
        // io::stdin().read_line(&mut input)?;
        stream.read_exact(&mut buf)?;
        println!("Recv: {:?}", buf);
        println!("Recv: {:?}", String::from_utf8_lossy(&buf));
    }
}
