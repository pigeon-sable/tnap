use anyhow::Result;
use clap::Parser;

/// Generate image with DALL·E and prnit it
#[derive(Parser)]
#[command(version, about, long_about = None)] // Read from Cargo.toml
struct Args {
    /// Prompt to pass to DALL·E
    prompt: String,

    /// Convert an image to ASCII art
    #[arg(short, long)]
    ascii: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("{}", args.prompt);
    if args.ascii {
        println!("Convert an image to ASCII art and output");
    } else {
        println!("Output an image as it is");
    }

    Ok(())
}
