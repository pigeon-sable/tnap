use anyhow::Result;
use clap::Parser;
use convert_image_to_ascii::convert_image_to_ascii;
use dotenv::dotenv;
use std::{fs, path::Path};
// use generate_image::{download_image, generate_image};

mod convert_image_to_ascii;
mod display_image;
// mod generate_image;

/// Generate image with DALL-E and print it
#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from Cargo.toml
struct Args {
    /// Prompt to pass to DALL-E
    prompt: String,

    /// Convert an image to ASCII art
    #[arg(short = 'a', long)]
    ascii: bool,
}

fn main() -> Result<()> {
    dotenv().ok(); // Read environment variable from .env file
    let args = Args::parse();

    // println!("Generating an image...");
    // let image_url = generate_image(&args.prompt)?;
    // download_image(&image_url, "generated_image.png")?;

    if args.ascii {
        // let image_path = "./src/img/girl_with_headphone_01.png";
        // convert_image_to_ascii(image_path)?;

        let data = get_ascii_arts("test")?;
        println!("Converted image to ASCII art!");
        display_image::run(&data)?;
    } else {
        println!("Non-ASCII image feature is not implemented yet.");
    }

    Ok(())
}

fn get_ascii_arts(theme: &str) -> Result<Vec<String>> {
    let mut data = vec![];

    let dir = Path::new("examples").join(theme);
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_file() {
            let ascii_art = convert_image_to_ascii(&path)?;
            println!("{}", ascii_art); // TODO: Remove later
            data.push(ascii_art);
        }
    }

    Ok(data)
}
