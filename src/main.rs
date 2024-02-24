use anyhow::{bail, Result};
use clap::Parser;
use dotenv::dotenv;
use generate_image::{download_image, generate_image};
use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::{fs, thread};
use toml::Value;

mod app;
mod convert_image_to_ascii;
mod generate_image;
mod util;

// Path to the directory containing the images to draw
pub static PATHS: Lazy<Mutex<Vec<PathBuf>>> = Lazy::new(|| Mutex::new(Vec::new()));

// Maximum number of images to generates
// TODO: Change it to 5
const MAX_IMAGES: u8 = 2;

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
    env_logger::init();

    // TODO: If `generated_images` directory does not exist, make it under the current directory

    match (args.theme, args.key, args.prompt) {
        (Some(theme), None, None) => display_theme(&theme, args.ascii),
        (None, Some(key), None) => {
            let prompt = read_config(&key)?;
            display_generated_image(&prompt, args.ascii)
        }
        (None, None, Some(prompt)) => display_generated_image(&prompt, args.ascii),
        // TODO: Set default values
        (None, None, None) => {
            let prompt = read_config("cat")?;
            display_generated_image(&prompt, args.ascii)
        }
        _ => bail!("Invalid arguments combination."),
    }
}

fn display_theme(theme: &str, ascii: bool) -> Result<()> {
    // TODO: Avoid hard-coding
    let path = Path::new("./themes")
        .join(theme)
        .join(format!("{}_01.png", theme));

    // Check if the theme exists and has images
    if path.exists() {
        let dir = path.parent().unwrap();
        log::info!("{:?}", fs::canonicalize(&dir));

        return app::run(&dir, ascii);
    }
    bail!("Theme '{}' not found.", theme);
}

fn display_generated_image(prompt: &str, ascii: bool) -> Result<()> {
    // TODO: Avoid hard-coding
    // TODO: Generate directory name from timestamp and make a directory
    let dir_name = "2024_0224_0000";
    let dir_path = Path::new("./generated_images").join(dir_name);

    // TODO: Avoid hard-coding
    // Add an image path to display while waiting for image generation
    let path_to_sample = Path::new("./examples").join("girl_with_headphone.png");
    PATHS.lock().unwrap().push(path_to_sample);

    let dir_path2 = dir_path.clone();
    let prompt = prompt.to_string();
    let handle = thread::spawn(move || {
        for i in 0..MAX_IMAGES {
            // println!("{}: Generating image...", i);
            let path = dir_path2.join(&format!("{}.png", i));
            let url = generate_image(&prompt).expect("Failed to generate an image.");

            download_image(&url, &path).expect("Failed to download a generated image.");
            // println!("Generated image downloaded to {:?}", path);

            PATHS.lock().unwrap().push(path);
        }
    });

    app::run(&dir_path, ascii)?;
    handle
        .join()
        .expect("Couldn't join on the associated thread.");

    Ok(())
}

fn read_config(key: &str) -> Result<String> {
    // TODO: Avoid hard-coding
    let contents = fs::read_to_string("./config.toml").expect("config.toml does not exist.");
    let value = contents.parse::<Value>().unwrap();

    match value
        .get("prompts")
        .and_then(|v| v.get(key))
        .and_then(|v| v.as_str())
    {
        Some(prompt) => Ok(prompt.to_string()),
        None => bail!("Key not found in config."),
    }
}
