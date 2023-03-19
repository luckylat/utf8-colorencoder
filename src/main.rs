use std::{io, env, fs};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::VecDeque;

use bmp::{Image, Pixel};

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Arguments Invalid: utf8tocl <Text File Path>");
        return Ok(())
    }

    let input = File::open(&args[1])?;
    let reader = BufReader::new(input); 
    let mut queue: VecDeque<u8> = VecDeque::new();
    let mut output: Vec<Color> = Vec::new();
    for line in reader.lines() {
        let line = format!("{}\n", &line?);
        for byte in line.as_bytes() {
            queue.push_back(*byte);
            if queue.len() == 3 {
                let red = queue.pop_front().unwrap();
                let green = queue.pop_front().unwrap();
                let blue = queue.pop_front().unwrap();
                let next_color = Color {red, green, blue};
                output.push(next_color);
            }
        }
    }
    if queue.len() != 0 {
        let red = queue.pop_front().unwrap_or(0);
        let green = queue.pop_front().unwrap_or(0);
        let blue = queue.pop_front().unwrap_or(0);
        let next_color = Color {red, green, blue};
        output.push(next_color);
    }

    let mut imgs: Vec<Image> = Vec::new();

    let height: usize = 100;
    let width: usize = 100;
    for (i, color) in output.iter().enumerate() {
        if i % (height * width) == 0 {
            imgs.push(Image::new(height as u32, width as u32));
        }
        let idx = i / (height * width);
        let img_height = (i % (height * width)) / height;
        let img_width = (i % (height * width)) % width;
        imgs[idx].set_pixel(img_width as u32, img_height as u32, Pixel::new(color.red, color.green, color.blue));
    }

    fs::create_dir_all("out")?;

    for (i, img) in imgs.iter().enumerate() {
        img.save(format!("out/image_{}.bmp", i))?;
    }
    Ok(())
}
