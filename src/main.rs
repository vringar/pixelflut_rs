use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("localhost:1337")?;
    let message = format!("PX {} {}\n", 0, 0);
    stream.write(&message.as_bytes())?;
    println!("The message was sent.");
    let mut result : Vec<u8> = vec!(0;1);
    while  {
        let mut buf = vec!(0;1);
        stream.read(&mut buf[..])?;
        result.append(&mut buf);
        result.last().expect("Idk what I'm doing anyways") != &b'\n'
    } {}
    println!("{}",String::from_utf8(result).expect("This wasn't valid unicode"));
    Ok(())
}
struct PixelFlutClient {
    stream : TcpStream 
}


