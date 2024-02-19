use anyhow::{anyhow, Result};
use reqwest;
use serde_json::{json, Value};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub async fn generate_image(prompt: &str) -> Result<String> {
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

pub async fn download_image(url: &str, file_path: &str) -> Result<()> {
    let response = reqwest::get(url).await?.bytes().await?;
    let path = Path::new(file_path);
    let mut file = File::create(path)?;

    file.write_all(&response)?;

    println!("Image saved to {:?}", path);
    Ok(())
}
