use anyhow::{anyhow, Result};
use artem::{config::ConfigBuilder, convert};
use core::num::NonZeroU32;
use image::io::Reader as ImageReader;
use std::path::Path;

pub fn convert_image_to_ascii(image_path: &Path, size: Option<u32>) -> Result<String> {
    // Open the image file
    let img = ImageReader::open(image_path)
        .map_err(|e| anyhow!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| anyhow!("Failed to decode image: {}", e))?;

    // // Conversion Config
    let config = if let Some(s) = size {
        let target_size = NonZeroU32::new(s).expect("Width must be non-zero.");
        println!("target_size: {}", target_size);
        ConfigBuilder::new()
            .center_x(true)
            .center_y(true)
            .scale(0.40f32)
            .target_size(target_size)
            .build()
    } else {
        ConfigBuilder::new().build()
    };

    // Convert image to ASCII
    let ascii_art = convert(img, &config);

    // println!("{}", ascii_art);

    Ok(ascii_art)
}
