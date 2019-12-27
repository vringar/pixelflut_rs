use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("localhost:1337")?;
    for i in 0..100 {
        let pixel = Pixel{x:i, y: i, color: Color(99,99,99, None)};
        let message = pixel.to_string() + "\n";
        stream.write(&message.as_bytes())?;
    }
    println!("The message was sent.");
    Ok(())
}
struct PixelFlutClient {
    stream: TcpStream,
}
impl PixelFlutClient {
    fn read(&mut self, x: u32, y: u32) -> std::io::Result<Pixel> {
        let mut result: Vec<u8> = vec![0; 1];
        while {
            let mut buf = vec![0; 1];
            self.stream.read(&mut buf[..])?;
            result.append(&mut buf);
            result.last().expect("Idk what I'm doing anyways") != &b'\n'
        } {}
        todo!();
    }
    fn write(&mut self, p: Pixel) -> std::io::Result<()> {
        todo!();
    }
    fn size(&mut self) -> std::io::Result<(u32, u32)> {
        todo!();
    }
}
#[derive(Default)]
struct Color(u8, u8, u8, Option<u8>);
impl ToString for Color {
    fn to_string(&self) -> String {
        let Color(r, g, b, a) = self;
        if let Some(a) = a {
            format!("{:02}{:02}{:02}{:02}", r, g, b, a)
        } else {
            format!("{:02}{:02}{:02}", r, g, b)
        }
    }
}
#[derive(Default)]
struct Pixel {
    x: u32,
    y: u32,
    color: Color,
}

impl ToString for Pixel {
    fn to_string(&self) -> String {
        format!("PX {} {} {} ", self.x, self.y, self.color.to_string())
    }
}
