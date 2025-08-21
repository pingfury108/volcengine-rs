use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use clap::{Args, Subcommand};
use log::error;
use std::fs;
use std::process;
use volcengine_rs::services::visual::{OcrNormalRequest, VisualClient};

#[derive(Args, Debug)]
pub struct VisualCommand {
    #[command(subcommand)]
    subcommand: VisualSubcommand,
}

#[derive(Subcommand, Debug)]
enum VisualSubcommand {
    /// Perform General OCR on an image
    OcrNormal(OcrNormalArgs),
}

#[derive(Args, Debug)]
struct OcrNormalArgs {
    /// The URL of the image for OCR
    #[arg(long, group = "image_input", required = true)]
    image_url: Option<String>,

    /// Path to the local image file for OCR
    #[arg(long, name = "image-file", group = "image_input", required = true)]
    image_file: Option<String>,
}

/// Handles the logic for the `visual` subcommand.
pub async fn handle(
    command: VisualCommand,
    access_key: &str,
    secret_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match command.subcommand {
        VisualSubcommand::OcrNormal(args) => {
            let client = VisualClient::new(access_key, secret_key, "cn-north-1");

            // Create the request based on whether a file or URL is provided
            let request = if let Some(file_path) = args.image_file {
                let file_bytes = match fs::read(&file_path) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        error!("Failed to read image file at '{}': {}", file_path, e);
                        process::exit(1);
                    }
                };
                let base64_string = STANDARD.encode(file_bytes);
                OcrNormalRequest {
                    image_base64: Some(base64_string),
                    ..Default::default()
                }
            } else {
                // image_url is the only other option due to the required clap group
                OcrNormalRequest {
                    image_url: args.image_url,
                    ..Default::default()
                }
            };

            println!("Sending OCR request...");
            let result = client.ocr_normal(request).await;

            match result {
                Ok(response) => {
                    println!("Request successful!");
                    println!("{:#?}", response);
                }
                Err(e) => {
                    println!("Request failed: {}", e);
                }
            }
        }
    }
    Ok(())
}
