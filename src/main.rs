use anyhow::{anyhow, Result};
use clap::Parser;
use dotenv::dotenv;
use reqwest;
use serde_json::{json, Value};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Generate image with DALL-E and print it
#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from Cargo.toml
struct Args {
    /// Prompt to pass to DALL-E
    #[arg(short, long)]
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok(); // Read environment variable from .env file
    let args = Args::parse();

    let image_url = generate_image(&args.prompt).await?;
    download_image(&image_url, "generated_image.png").await?;

    Ok(())
}

async fn generate_image(prompt: &str) -> Result<String> {
    let api_key = env::var("OPENAI_API_KEY").expect("Expected an environment variable OPENAI_API_KEY");

    let client = reqwest::Client::new();
    let response = client.post("https://api.openai.com/v1/images/generations")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "dall-e-3",
            "prompt": prompt,
            "n": 1,
            "size": "1024x1024"
        }))
        .send()
        .await?
        .json::<Value>()
        .await?;

    let image_url = response["data"][0]["url"].as_str().ok_or(anyhow!("Failed to extract image URL"))?.to_string();

    Ok(image_url)
}

async fn download_image(url: &str, file_path: &str) -> Result<()> {
    let response = reqwest::get(url).await?.bytes().await?;
    let path = Path::new(file_path);
    let mut file = File::create(path)?;

    file.write_all(&response)?;

    println!("Image saved to {:?}", path);
    Ok(())
}
