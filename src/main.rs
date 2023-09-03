use std::io;
use std::{collections::HashMap, fs::File, io::Read};

use clap::Parser;
use reqwest::blocking::multipart::{Form, Part};
use reqwest::blocking::Client;

static CONFIG_DEFAULT_PATH: &str = "./discorder.toml";

/// A cli tool for sending text or file to Discord Webhook
/// discorder --webhook https://discord.com/api/webhooks/1234567890/ABCDEFGHIJKL --text "Hello, World!"
/// discorder --webhook https://discord.com/api/webhooks/1234567890/ABCDEFGHIJKL --file ./message.txt
#[derive(Parser)]
struct Args {
    /// Discord Webhook URL
    #[clap(short, long)]
    webhook: Option<String>,
    /// A text to send
    #[clap(short, long)]
    text: Option<String>,
    /// A file to send
    #[clap(short, long)]
    file: Option<String>,
    /// Config file path
    #[clap(short, long, default_value = CONFIG_DEFAULT_PATH)]
    config: String,
}

fn load_config(path: &str) -> Result<toml::Value, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let config = toml::from_str(&buf)?;
    Ok(config)
}

fn main() {
    let args = Args::parse();

    let config = load_config(&args.config).unwrap();
    let webhook = config["webhook"].as_str().map(|s| s.to_owned());
    let text = config["text"].as_str().map(|s| s.to_owned());
    let file = config["file"].as_str().map(|s| s.to_owned());

    let args = Args {
        webhook: args.webhook.or(webhook),
        text: args.text.or(text),
        file: args.file.or(file),
        config: args.config,
    };

    match (args.text, args.file) {
        (Some(text), None) => send_text_to_discord(&args.webhook.unwrap(), &text).unwrap(),
        (None, Some(file)) => send_file_to_discord(&args.webhook.unwrap(), &file).unwrap(),
        (None, None) => send_stdin_to_discord(&args.webhook.unwrap()).unwrap(),
        _ => println!("Please specify text or file."),
    }
}

fn send_stdin_to_discord(webhook: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf).unwrap();
    send_file_to_discord(webhook, &buf).unwrap();

    Ok(())
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
