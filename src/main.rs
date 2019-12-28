use pixelflut::client::Client;
use pixelflut::Color;
use pixelflut::Pixel;
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let img = image::open("Snooker_triangle.svg.png").unwrap().to_rgb();
    let mut client = Client::new(TcpStream::connect("localhost:1337")?);
    for (x, y, pixel) in img.enumerate_pixels() {
        let image::Rgb([r,g,b]) = *pixel;
        let pixel = Pixel {
            x: x,
            y: y,
            color: Color(r, g, b, None),
        };
        client.write(pixel)?;
    }
    let result = client.read(99, 99)?;
    println!("{}", result.to_string());
    let (x, y) = client.size()?;
    println!("SIZE {}, {}", x, y);
    Ok(())
}
