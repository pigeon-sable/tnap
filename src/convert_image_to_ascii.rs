use anyhow::{anyhow, Result};
use artem::{config::ConfigBuilder, convert};
use crossterm::terminal::size;
use image::io::Reader as ImageReader;
use std::num::NonZeroU32;
use std::path::Path;

pub fn convert_image_to_ascii(image_path: &Path) -> Result<String> {
    // Open the image file
    let img = ImageReader::open(image_path)
        .map_err(|e| anyhow!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| anyhow!("Failed to decode image: {}", e))?;

    // Read image data from memory
    // let img = image::load_from_memory(image_data)
    //     .map_err(|e| anyhow!("Failed to load image from memory: {}", e))?;

    // Conversion Config
    let target_size = NonZeroU32::new(ascii_size()?).expect("Width must be non-zero.");
    // println!("target_size: {}", target_size);

    let config = ConfigBuilder::new()
        .center_x(true)
        .center_y(true)
        .scale(0.380025f32) // magic number!
        .target_size(target_size)
        .build();

    // Convert image to ASCII
    let ascii_art = convert(img, &config);
    // println!("Converted image to ASCII art!");

    Ok(ascii_art)
}

fn ascii_size() -> Result<u32> {
    let (columns, rows) = size()?;
    let size = (std::cmp::min(columns, rows) * 2) as u32;
    // println!("size: {}", size);

    Ok(size)
}
