use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use generate_image::{download_image, generate_image};

mod generate_image;

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

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok(); // Read environment variable from .env file
    let args = Args::parse();

    println!("Generating an image...");
    let image_url = generate_image(&args.prompt).await?;

    if args.ascii {
        println!("Converted image to ASCII art!");
    } else {
        download_image(&image_url, "generated_image.png").await?;
    }

    Ok(())
}
