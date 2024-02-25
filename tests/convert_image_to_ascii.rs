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
    log::info!("target size: {}", target_size);

    let config = ConfigBuilder::new()
        .center_x(true)
        .center_y(true)
        .scale(0.380025f32) // magic number!
        .target_size(target_size)
        .build();

    // Convert image to ASCII
    let ascii_art = convert(img, &config);

    Ok(ascii_art)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_convert_image_to_ascii() {
        // Prepare image files for testing.
        let image_path = PathBuf::from("test_images/test_image.png");

        // Convert the image to ASCII art.
        let result = convert_image_to_ascii(&image_path);

        // Compare with the expected ASCII art.
        let expected_ascii_art = "expected_ascii_art";
        assert_eq!(result, Ok(expected_ascii_art));
    }
}


// static mut COLUMNS:u16 = 0;
// static mut ROWS:u16 = 0;
static mut SIZE:u32 = 0;

fn ascii_size() -> Result<u32> {
    let (columns, rows) = size()?;
    // COLUMNS = size()?.0;
    // ROWS = size()?.1;
    let size = (std::cmp::min(columns, rows) * 2) as u32;
    SIZE = (std::cmp::min(columns, rows) * 2) as u32;
    log::info!("ascii size: {}", size);

    Ok(size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_size() {
        

        assert_eq!(SIZE, ascii_size());
    }
}

