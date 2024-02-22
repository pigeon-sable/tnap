use anyhow::Result;
use clap::Parser;
use convert_image_to_ascii::convert_image_to_ascii;
use dotenv::dotenv;
use generate_image::{download_image, generate_image};
use std::fs;
use std::path::Path;
use toml::Value;

mod ascii;
mod convert_image_to_ascii;
mod generate_image;
mod non_ascii;
mod util;

/// You can use sample themes for tnap and generate image with default prompts or your own prompts.
#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from Cargo.toml
struct Args {
    /// Use the sample theme without generating images
    #[arg(short, long)]
    theme: Option<String>,

    /// Generate Image by looking up the corresponding value in config.toml
    /// using the subsequent string as a key and using it as a prompt.
    #[arg(short, long)]
    key: Option<String>,

    /// Generate images with user-considered prompt
    #[arg(short, long)]
    prompt: Option<String>,

    /// Convert an image to ASCII art
    #[arg(short, long)]
    ascii: bool,
}

fn main() -> Result<()> {
    dotenv().ok(); // Read environment variable from .env file
    let args = Args::parse();

    match (args.theme, args.key, args.prompt) {
        (Some(theme), None, None) => display_theme(&theme, args.ascii)?,
        (None, Some(key), None) => display_generated_image_from_config(&key, args.ascii)?,
        (None, None, Some(prompt)) => display_generated_image_from_prompt(&prompt, args.ascii)?,
        _ => println!("Error: Invalid arguments combination."),
    }
    Ok(())
}

fn display_theme(theme: &str, ascii: bool) -> Result<()> {
    let path = format!("./themes/{}/{}_01.png", theme, theme);
    // println!("{}", path);
    if Path::new(&path).exists() {
        display_image(&path, ascii)?;
    } else {
        println!("Error: Theme not found.");
    }
    Ok(())
}

fn display_generated_image_from_config(key: &str, ascii: bool) -> Result<()> {
    let contents = fs::read_to_string("./config.toml").unwrap();
    let value = contents.parse::<Value>().unwrap();

    if let Some(prompt) = value
        .get("prompts")
        .and_then(|v| v.get(key))
        .and_then(|v| v.as_str())
    {
        display_generated_image_from_prompt(&prompt, ascii)?;
    } else {
        println!("Error: Key not found in config.");
    }
    Ok(())
}

fn display_generated_image_from_prompt(prompt: &str, ascii: bool) -> Result<()> {
    println!("Generating image...");
    let image_url = generate_image(&prompt)?;
    let image_path = "./generate_image.png";
    download_image(&image_url, image_path)?;
    println!("Generated image downloaded to {}", image_path);
    display_image(image_path, ascii)?;
    Ok(())
}

fn display_image(path: &str, ascii: bool) -> Result<()> {
    if ascii {
        convert_image_to_ascii(&Path::new(path))?;
    } else {
        println!("Displaying image: {}", path);
        // render_image(&path)?;
    }
    Ok(())
}
