use anyhow::Result;
use clap::Parser;
use convert_image_to_ascii::convert_image_to_ascii;
use crossterm::terminal::size;
use dotenv::dotenv;
use generate_image::{download_image, generate_image};
use std::{fs, path::Path};

mod ascii;
mod convert_image_to_ascii;
mod generate_image;
mod util;

/// Generate image with DALL-E and print it
#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from Cargo.toml
struct Args {
    /// Prompt to pass to DALL-E
    #[arg(short = 'p', long)]
    prompt: Option<String>,

    /// Convert an image to ASCII art
    #[arg(short = 'a', long)]
    ascii: bool,
}

fn main() -> Result<()> {
    dotenv().ok(); // Read environment variable from .env file
    let args = Args::parse();

    if let Some(prompt) = args.prompt {
        println!("Generating an image...");
        let image_url = generate_image(&prompt)?;
        download_image(&image_url, "./src/img/generated_image.png")?;
    }

    if args.ascii {
        let (columns, rows) = size()?;
        let size = (std::cmp::min(columns, rows) * 2) as u32;
        println!("size: {}", size);
        // let image_path = "./src/img/girl_with_headphone_01.png";
        // convert_image_to_ascii(image_path, Some(size))?;

        let data = get_ascii_arts("test", Some(size))?;
        println!("Converted image to ASCII art!");

        ascii::run(&data)?;
    } else {
        println!("Non-ASCII image feature is not implemented yet.");
    }

    Ok(())
}

fn get_ascii_arts(theme: &str, size: Option<u32>) -> Result<Vec<String>> {
    let mut data = vec![];

    let dir = Path::new("examples").join(theme);
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_file() {
            let ascii_art = convert_image_to_ascii(&path, size)?;
            data.push(ascii_art);
        }
    }

    Ok(data)
}
