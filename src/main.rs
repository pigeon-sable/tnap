use anyhow::Result;
use clap::Parser;
use convert_image_to_ascii::convert_image_to_ascii;
use dotenv::dotenv;
// use generate_image::{download_image, generate_image};

mod convert_image_to_ascii;
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
        let image_path = "./src/img/girl_with_headphone_01.png";
        convert_image_to_ascii(image_path)?;
        println!("Converted image to ASCII art!");
    } else {
        println!("Non-ASCII image feature is not implemented yet.");
    }

    Ok(())
}
