use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use generate_image::{download_image, generate_image};

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
        ascii::run("test")?;
    } else {
        println!("Non-ASCII image feature is not implemented yet.");
    }

    Ok(())
}
