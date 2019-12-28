use pixelflut::client::Client;
use pixelflut::Color;
use pixelflut::Pixel;

use std::env;
use std::net::TcpStream;

use rand::seq::SliceRandom;

fn main() -> std::io::Result<()> {
    let image = env::args().nth(1)
        .unwrap_or_else(|| "Snooker_triangle.svg.png".to_string());

    let image = image::open(&image)
        .expect("Image not recognized")
        .to_rgba();

    let mut client = Client::new(TcpStream::connect("151.217.111.34:1234")?);
    let mut pixels: Vec<_> = image
        .enumerate_pixels()
        .filter_map(|(x, y, pixel)| {
            let image::Rgba([r,g,b,a]) = *pixel;

            if a == 0 {
                return None;
            }

            let pixel = Pixel {
                x: x + 100,
                y: y + 100,
                color: if a == 255 {
                    Color(r, g, b, None)
                } else {
                    Color(r, g, b, Some(a))
                },
            };

            Some(pixel.to_string())
        })
        .collect();

    pixels.shuffle(&mut rand::thread_rng());

    let pixels: Vec<_> = pixels
        .into_iter()
        .collect();

    let (x, y) = client.size()?;
    println!("SIZE {}, {}", x, y);
    client.write_raw("OFFSET 1200 600\n");

    let mut data_stream = pixels.join("\n");
    data_stream.push_str(" \n");

    loop {
        client.write_raw(&data_stream)?;
    }

    Ok(())
}
