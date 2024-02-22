use anyhow::{anyhow, Result};
use artem::{config::ConfigBuilder, convert};
use crossterm::terminal::size;
use image::io::Reader as ImageReader;
use std::num::NonZeroU32;
use std::path::Path;

pub fn convert_image_to_ascii(image_path: &str) -> Result<()> {
    // Open the image file
    let img = ImageReader::open(Path::new(image_path))
        .map_err(|e| anyhow!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| anyhow!("Failed to decode image: {}", e))?;

    // Conversion Config
    let (columns, rows) = size()?;
    // println!("columns: {}, rows: {}", columns, rows);
    let size = (std::cmp::min(columns, rows) * 2) as u32;
    // println!("size: {}", size);
    let config = if let Some(s) = Some(size) {
        let target_size = NonZeroU32::new(s).expect("Width must be non-zero.");
        // println!("target_size: {}", target_size);
        ConfigBuilder::new()
            .center_x(true)
            .center_y(true)
            .scale(0.380025f32)
            .target_size(target_size)
            .build()
    } else {
        ConfigBuilder::new().build()
    };

    // Convert image to ASCII
    let ascii_art = convert(img, &config);
    println!("{}", ascii_art);

    Ok(())
}
