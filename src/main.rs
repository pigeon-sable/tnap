use anyhow::{bail, Result};
use clap::Parser;
use convert_image_to_ascii::convert_image_to_ascii;
use dotenv::dotenv;
use generate_image::{download_image, generate_image};
use std::{env, fs};
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
    env_logger::init();

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
    // Obtained from the environment variable TNAP_ROOT, or if it does not exist, the path of the current directory is selected
    let tnap_root = match env::var("TNAP_ROOT") {
        Ok(val) => {
            log::info!("The data was obtained from the environment variable TNAP_ROOT");
            val
        },
        Err(err) => {
            log::info!("{}", err);
            log::info!("The data was obtained from the current directory");
            Path::new(".").canonicalize().unwrap().to_str().unwrap().to_string()
        }
    };
    log::info!("TNAP_ROOT: {}", tnap_root);

    let themes_path = format!("{}/themes", tnap_root);
    log::info!("themes_path: {}", themes_path);

    let theme_path = format!("{}/{}", themes_path, theme);
    log::info!("{}_path: {}", theme, theme_path);

    if Path::new(&theme_path).exists() {
        let files = fs::read_dir(Path::new(&theme_path)).unwrap();
        let mut flag = false;

        for file in files {
            let file_path = file.unwrap().path();

            // Checks if a file exists and if it is an image file.
            if file_path.is_file() && is_image_file(&file_path){
                log::info!("image_path： {}", file_path.display());
            } else {
                flag = true;
                break;
            }
        }

        // If the file contains anything other than an image file
        if flag {
            bail!("{} ({}) include non-image file", theme, theme_path);
        }

        return app::run(Path::new(&theme_path), ascii);
    }
    bail!("{} ({}) is not found", theme, theme_path);
}

fn is_image_file(path: &Path) -> bool {
    let image_extensions = ["png", "jpg", "jpeg", "PNG", "JPG", "JPEG"];
    let extension = path.extension().unwrap().to_str().unwrap();

    image_extensions.contains(&extension)
}

fn display_generated_image_from_config(key: &str, ascii: bool) -> Result<()> {
    let tnap_root = env::var("TNAP_ROOT").expect("Expected an environment variable TNAP_ROOT");
    log::info!("TNAP_ROOT: {:?}", tnap_root);

    let config_path = format!("{}/config.toml", tnap_root);
    log::info!("config_path: {:?}", config_path);

    // Check if config.toml exists
    if !(Path::new(&config_path).exists()) {
        bail!("config.toml is not found")
    }

    let contents = fs::read_to_string(config_path).unwrap();
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
            Ok(art) => println!("{}", art), // If successful, output ASCII art
            Err(e) => println!("Error converting image to ASCII: {:?}", e), // Output error messages when errors occur
        }
    } else {
        println!("Displaying image directly is not supported in this context.");
        // render_image(&path)?;
    }
    Ok(())
}
