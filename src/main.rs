use std::{collections::HashMap, fs::File, io::Read};

use clap::Parser;
use reqwest::blocking::multipart::{Form, Part};
use reqwest::blocking::Client;

/// A cli tool for sending text or file to Discord Webhook
/// discorder --webhook https://discord.com/api/webhooks/1234567890/ABCDEFGHIJKL --text "Hello, World!"
/// discorder --webhook https://discord.com/api/webhooks/1234567890/ABCDEFGHIJKL --file ./message.txt
#[derive(Parser)]
struct Args {
    /// Discord Webhook URL
    #[clap(short, long)]
    webhook: String,
    /// A text to send
    #[clap(short, long)]
    text: Option<String>,
    /// A file to send
    #[clap(short, long)]
    file: Option<String>,
}

fn main() {
    let args = Args::parse();

    if args.text.is_none() && args.file.is_none() {
        println!("Please specify text or file.");
        return;
    }

    if let Some(text) = args.text {
        send_text_to_discord(&args.webhook, &text).unwrap();
    }

    if let Some(file) = args.file {
        send_file_to_discord(&args.webhook, &file).unwrap();
    }
}

fn send_text_to_discord(webhook: &str, text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut json = HashMap::new();
    json.insert("content", text);
    let res = client.post(webhook).json(&json).send()?;

    if res.status().is_success() {
        println!("Success!");
    } else {
        println!("Failed!");
        println!("{:?}", res.text());
    }

    Ok(())
}

fn send_file_to_discord(webhook: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let mut file = File::open(file_path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let form = Form::new();
    let file_name = file_path.split('/').last().unwrap().to_owned();

    let res = client
        .post(webhook)
        .multipart(form.part("file", Part::bytes(buf).file_name(file_name)))
        .send()?;

    if res.status().is_success() {
        println!("Success!");
    } else {
        println!("Failed!");
        println!("{:?}", res.text());
    }

    Ok(())
}
