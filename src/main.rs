use pixelflut::client::Client;
use pixelflut::Color;
use pixelflut::Image;
use pixelflut::Pixel;

use std::env;
use std::fs::File;
use std::net::TcpStream;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::{thread, time};

use image::gif::Decoder;
use image::AnimationDecoder;
use rand::seq::SliceRandom;

fn main() -> std::io::Result<()> {
    let image = env::args().nth(1).unwrap_or_else(|| "cat.gif".to_string());
    let image = File::open(image)?;
    let image = Decoder::new(&image).expect("Image not recognized");
    let frames = image.into_frames();
    let frames = frames.collect_frames().unwrap();
    let mut encoded_frames: Vec<Image> = Vec::with_capacity(frames.len());
    for frame in frames {
        let mut pixels: Vec<_> = frame
            .buffer()
            .enumerate_pixels()
            .filter_map(|(x, y, pixel)| {
                let color = Color::from(pixel);
                if color.a == Some(0) {
                    return None;
                }
                let pixel = Pixel { x, y, color };
                Some(pixel.to_string())
            })
            .collect();
        pixels.shuffle(&mut rand::thread_rng());
        let mut data_stream = pixels.join("\n");
        data_stream.push_str(" \n");

        encoded_frames.push(data_stream);
    }
    println!("pregenerated the whole gif");
    let encoded_frames = Arc::new(encoded_frames);
    let mut threads: Vec<_> = Vec::with_capacity(10);
    for i in 0..10 {
        let encoded_frames = encoded_frames.clone();
        let (producer, consumer) = channel();
        let thread = thread::Builder::new()
            .name(format! {"Client {}", i})
            .spawn(move || -> std::io::Result<()> {
                let mut client = Client::new(TcpStream::connect("localhost:1337")?);
                println!("{} started working", thread::current().name().unwrap());
                let mut index = 0;
                loop {
                    match consumer.try_recv() {
                        Ok(Signal::Index(int)) => {index = int;},
                        Ok(Signal::Quit) => {return Ok(());}
                        Err(_) => {}
                    }
                    let pixels = &encoded_frames[index];
                    client.write_raw(&pixels)?;
                   
                }
            })?;
        threads.push((thread, producer));
    }
    enum Signal {
        Index(usize),
        Quit
    }
    for _ in 0..10 {
        for frame in 0..encoded_frames.len() {
            //thread::sleep(time::Duration::from_millis(200));
            println!("Setting new frame");

            for (_, producer) in &threads {
                producer.send(Signal::Index(frame)).unwrap();
            }
        }
    }

    for (thread, producer) in threads {
        producer.send(Signal::Quit).unwrap();
        thread.join().expect("Somehow couldn't join the threads")?;
    }
    Ok(())
}
