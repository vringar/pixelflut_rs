use std::io::prelude::*;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let stream = TcpStream::connect("localhost:1337")?;
    let mut client = PixelFlutClient { stream };
    for i in 100..200 {
        let pixel = Pixel {
            x: i,
            y: i,
            color: Color(99, 99, 99, None),
        };
        client.write(pixel)?;
    }
    let result = client.read(99, 99)?;
    println!("{}", result.to_string());
    let (x,y) = client.size()?;
    println!("SIZE {}, {}", x,y);
    Ok(())
}
struct PixelFlutClient {
    stream: TcpStream,
}

impl PixelFlutClient {
    pub fn read(&mut self, x: u32, y: u32) -> std::io::Result<Pixel> {
        self.stream.write(format!("PX {} {}\n", x, y).as_bytes())?;

        let result = self.read_line()?;
        let result  = result.split_whitespace()
            .last()
            .expect("There was no string to split");
        let r = result[0..2].parse::<u8>().unwrap();
        let g = result[2..4].parse::<u8>().unwrap();
        let b = result[4..].parse::<u8>().unwrap();
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
        let x = result.next().map(|string| string.parse::<u32>()).unwrap().unwrap();
        let y = result.next().map(|string| string.parse::<u32>()).unwrap().unwrap();
        Ok((x,y))
    }
    fn read_line(&mut self) -> std::io::Result<String> {
        let mut result: Vec<u8> = vec![0; 1];
        while {
            let mut buf = vec![0; 1];
            self.stream.read(&mut buf[..])?;
            result.append(&mut buf);
            result.last().expect("The buffer was empty") != &b'\n'
        } {}
        Ok(String::from_utf8(result).expect("The pixelflut returned invalid UTF-8"))
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
