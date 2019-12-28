use super::*;
use std::io::prelude::*;
use std::net::TcpStream;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(stream: TcpStream) -> Client {
        Client { stream }
    }
    pub fn read(&mut self, x: u32, y: u32) -> std::io::Result<Pixel> {
        self.stream.write(format!("PX {} {}\n", x, y).as_bytes())?;

        let result = self.read_line()?; //PX X Y RRGGBB
        let result = result
            .split_whitespace()
            .last()
            .expect("There was no string to split");
        let r = u8::from_str_radix(&result[0..2], 16).unwrap();
        let g = u8::from_str_radix(&result[2..4], 16).unwrap();
        let b = u8::from_str_radix(&result[4..], 16).unwrap();
        Ok(Pixel {
            x,
            y,
            color: Color(r, g, b, None),
        })
    }
    pub fn write(&mut self, p: Pixel) -> std::io::Result<()> {
        let message = p.to_string() + "\n";
        self.stream.write(&message.as_bytes())?;
        Ok(())
    }
    pub fn size(&mut self) -> std::io::Result<(u32, u32)> {
        self.stream.write(("SIZE\n").as_bytes())?;
        let line = self.read_line()?; //SIZE X Y
        let mut result = line.split_whitespace();
        result.next(); //Gets rid of SIZE
        let x = result
            .next()
            .expect("No X coordinate")
            .parse::<u32>()
            .unwrap();
        let y = result
            .next()
            .expect("No Y coordinate")
            .parse::<u32>()
            .unwrap();
        Ok((x, y))
    }
    
    fn read_line(&mut self) -> std::io::Result<String> {
        let mut result: Vec<u8> = vec![];
        while {
            let mut buf = vec![0; 1];
            self.stream.read(&mut buf[..])?;
            result.append(&mut buf);
            result.last().expect("The buffer was empty") != &b'\n'
        } {}
        Ok(String::from_utf8(result).expect("The pixelflut returned invalid UTF-8"))
    }
}
