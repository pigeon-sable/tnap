use anyhow::{bail, Result};
use clap::Parser;
use convert_image_to_ascii::convert_image_to_ascii;
use dotenv::dotenv;
use generate_image::{download_image, generate_image};
use std::fs;
use std::path::Path;
use toml::Value;

mod app;
mod convert_image_to_ascii;
mod generate_image;
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
        (Some(theme), None, None) => return display_theme(&theme, args.ascii),
        (None, Some(key), None) => return display_generated_image_from_config(&key, args.ascii),
        (None, None, Some(prompt)) => {
            return display_generated_image_from_prompt(&prompt, args.ascii)
        }
        _ => bail!("Invalid arguments combination."),
    }
}

fn display_theme(theme: &str, ascii: bool) -> Result<()> {
    let path = format!("./themes/{}/{}_01.png", theme, theme);
    // println!("{}", path);

    // Check if the theme exists and has images.
    if Path::new(&path).exists() {
        let dir = Path::new("./themes").join(theme);
        return app::run(&dir, ascii);
    }
    bail!("Theme '{}' not found.", theme);
}

fn display_generated_image_from_config(key: &str, ascii: bool) -> Result<()> {
    // TODO: Check if config file exists
    let contents = fs::read_to_string("./config.toml").unwrap();
    let value = contents.parse::<Value>().unwrap();

    if let Some(prompt) = value
        .get("prompts")
        .and_then(|v| v.get(key))
        .and_then(|v| v.as_str())
    {
        return display_generated_image_from_prompt(&prompt, ascii);
    }
    bail!("Key not found in config.");
}

fn display_generated_image_from_prompt(prompt: &str, ascii: bool) -> Result<()> {
    println!("Generating image...");
    let image_url = generate_image(&prompt)?;

    let path = Path::new("./generated_images").join("2024_0224_0000/generate_image.png");
    download_image(&image_url, &path)?;
    println!("Generated image downloaded to {:?}", path);

    let dir = path
        .parent()
        .expect("Failed to get path to a generated image.");
    app::run(dir, ascii)?;
    Ok(())
}

#[allow(dead_code)]
fn display_image(path: &str, ascii: bool) -> Result<()> {
    if ascii {
        let ascii_art = convert_image_to_ascii(Path::new(&path));
        match ascii_art {
            Ok(art) => println!("{}", art), // 成功した場合、ASCII アートを出力
            Err(e) => println!("Error converting image to ASCII: {:?}", e), // エラーが発生した場合、エラーメッセージを出力
        }
    } else {
        println!("Displaying image: {}", path);
        // render_image(&path)?;
    }
    Ok(())
}
