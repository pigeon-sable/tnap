use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn generate_image(prompt: &str) -> Result<String> {
    let api_key =
        env::var("OPENAI_API_KEY").expect("Expected an environment variable OPENAI_API_KEY");
    log::info!("prompt: {}", prompt);

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://api.openai.com/v1/images/generations")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": "dall-e-3",
            "prompt": prompt,
            "n": 1,
            "size": "1024x1024"
        }))
        .send()?
        .json::<Value>()?;
    log::info!("API Response: {:?}", response);

    let image_url = response["data"][0]["url"]
        .as_str()
        .ok_or(anyhow!("Failed to extract image URL"))?
        .to_string();

    log::info!("image_url {}", image_url);
    Ok(image_url)
}

pub fn download_image(url: &str, path: &PathBuf) -> Result<()> {
    let response = reqwest::blocking::get(url)?.bytes()?;
    let mut file = File::create(path)?;

    file.write_all(&response)?;
    log::info!("Image saved to {:?}", path);

    Ok(())
    // let response = reqwest::blocking::get(url)?;
    // let bytes = response.bytes()?;
    // Ok(bytes.to_vec())
}
