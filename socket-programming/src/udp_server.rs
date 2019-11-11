use std::net::UdpSocket;
use std::str;

pub fn serve(addr: &str) -> Result<(), failure::Error> {
    let server_socket = UdpSocket::bind(addr)?;
    loop {
        let mut buf = [0u8; 1024];
        let (size, src) = server_socket.recv_from(&mut buf)?;
        debug!("Handling data from {}", src);
        print!("{}", str::from_utf8(&buf[..size])?);
        server_socket.send_to(&buf, src)?;
    }
}
