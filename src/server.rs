use f_messages::message_types::FM;
use rand::{thread_rng, Rng};
use std::{error::Error, io::prelude::*, net::TcpListener, thread};

// use std::env;

fn main() -> Result<(), Box<dyn Error>> {
    // MAKE TCP LISTENER
    let listener = TcpListener::bind("localhost:7878").unwrap();
    // MAKE A LOOP
    for stream_r in listener.incoming() {
        let mut stream = stream_r?;

        println!("Connection established! {}", stream.peer_addr()?);
        thread::spawn(move || {
            handle_connection(&mut stream).unwrap_or_else(|error| {
                eprintln!("{:?}", error);
            })
        });
    }

    Ok(())
}

fn handle_connection(stream: &mut std::net::TcpStream) -> std::io::Result<()> {
    // MAKE A BUFFER
    loop {
        let mut buffer: FM = [0; 128];
        // let _n = stream.read(&mut buffer)?;
        thread_rng().fill(&mut buffer[..]);
        thread::sleep(std::time::Duration::from_millis(1000));
        stream.write_all(&buffer[..])?;
    }
    // READ THE STREAM
}
