use anyhow::{anyhow, Result};
use artem::{config::ConfigBuilder, convert};
use image::io::Reader as ImageReader;
use std::num::NonZeroU32;
use std::path::Path;

pub fn convert_image_to_ascii(image_path: &str, width: Option<u32>) -> Result<()> {
    // Open the image file
    let img = ImageReader::open(Path::new(image_path))
        .map_err(|e| anyhow!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| anyhow!("Failed to decode image: {}", e))?;

    // // Conversion Config
    let mut config_builder = ConfigBuilder::new();
    if let Some(w) = width {
        if let Some(target_width) = NonZeroU32::new(w) {
            config_builder.target_size(target_width);
        }
    }
    let config = config_builder.build();

    // Convert image to ASCII
    let ascii_art = convert(img, &config);

    println!("{}", ascii_art);

    Ok(())
}
