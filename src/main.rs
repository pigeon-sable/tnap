use anyhow::{bail, Result};
use chrono::Local;
use clap::Parser;
use dotenv::dotenv;
use generate_image::{download_image, generate_image};
use once_cell::sync::Lazy;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;
use std::sync::Mutex;
use std::{env, fs, thread};
use toml::Value;

mod app;
mod convert_image_to_ascii;
mod generate_image;
mod util;

// Path to the directory containing the images to draw
static PATHS: Lazy<Mutex<Vec<PathBuf>>> = Lazy::new(|| Mutex::new(Vec::new()));

static APP_EXIT: AtomicBool = AtomicBool::new(false);

// Maximum number of images to generates
const MAX_IMAGES: u8 = 3;

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
    env_logger::init();

    let args = Args::parse();
    match (args.theme, args.key, args.prompt) {
        (Some(theme), None, None) => display_theme(&theme, args.ascii),
        (None, Some(key), None) => {
            let prompt = read_config(&key)?;
            display_generated_image(&prompt, args.ascii)
        }
        (None, None, Some(prompt)) => display_generated_image(&prompt, args.ascii),
        // TODO: Set default values
        (None, None, None) => display_theme("cat", args.ascii),
        _ => bail!("Invalid arguments combination."),
    }
}

fn read_config(key: &str) -> Result<String> {
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

    let config_path = format!("{}/config.toml", tnap_root);

    let contents = fs::read_to_string(config_path).unwrap();
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

fn display_generated_image(prompt: &str, ascii: bool) -> Result<()> {
    // TODO: Use an environment variable
    let time = Local::now().format("%Y_%m%d_%H%M").to_string();
    let dir_path = Path::new("./generated_images").join(time);
    create_dir_all(&dir_path)?;

    // TODO: Use an environment variable
    // Add an image path to display while waiting for image generation
    let path_to_sample = Path::new("./examples").join("girl_with_headphone.png");
    PATHS.lock().unwrap().push(path_to_sample);

    let dir = dir_path.clone();
    let prompt = prompt.to_string();
    let handle = thread::spawn(move || {
        let mut url = generate_image(&prompt).unwrap();
        let mut path = dir.join("0.png");
        download_image(&url, &path).expect("Failed to download a generated image.");

        PATHS.lock().unwrap().push(path);
        PATHS.lock().unwrap().remove(0); // Remove a sample image path

        for i in 1..MAX_IMAGES {
            if APP_EXIT.load(SeqCst) {
                break;
            }

            url = generate_image(&prompt).unwrap();
            path = dir.join(&format!("{}.png", i));
            download_image(&url, &path).expect("Failed to download a generated image.");

            PATHS.lock().unwrap().push(path);
        }
    });

    app::run(&dir_path, ascii)?;
    handle
        .join()
        .expect("Couldn't join on the associated thread.");

    Ok(())
}
