use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{str, thread};

pub fn serve(addr: &str) -> Result<(), failure::Error> {
    let listener = TcpListener::bind(addr)?;
    loop {
        let (stream, _) = listener.accept()?;

        thread::spawn(move || {
            handler(stream).unwrap_or_else(|error| error!("{:?}", error));
        });
    }
}

fn handler(mut stream: TcpStream) -> Result<(), failure::Error> {
    debug!("Handling data from {}", stream.peer_addr()?);
    let mut buf = [0u8; 1024];
    loop {
        let nbytes = stream.read(&mut buf)?;
        if nbytes == 0 {
            debug!("Connection closed.");
            return Ok(());
        }
        print!("{}", str::from_utf8(&buf[..nbytes])?);
        stream.write_all(&buf[..nbytes])?;
    }
}
