use anyhow::{anyhow, Result};
use artem::{config::ConfigBuilder, convert};
use image::io::Reader as ImageReader;
use std::path::Path;

pub fn convert_image_to_ascii(image_path: &Path) -> Result<String> {
    // Open the image file
    let img = ImageReader::open(image_path)
        .map_err(|e| anyhow!("Failed to open image: {}", e))?
        .decode()
        .map_err(|e| anyhow!("Failed to decode image: {}", e))?;

    // // Conversion Config
    // let mut config_builder = ConfigBuilder::default();
    // if let Some(w) = width {
    //     config_builder = config_builder.width(w);
    // }
    // let config = config_builder.colored(colored).build();

    // Convert image to ASCII
    let ascii_art = convert(img, &artem::config::ConfigBuilder::new().build());

    // println!("{}", ascii_art);

    Ok(ascii_art)
}
