use std::path::Path;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
};

use clap::Parser;
use reqwest::blocking::multipart::{Form, Part};
use reqwest::blocking::Client;
use yaml_rust::{Yaml, YamlLoader};

static CONFIG_DEFAULT_PATHS: [&str; 4] = [
    "./discorder.yml",
    "./discorder.yaml",
    "~/.config/discorder/discorder.yml",
    "~/.config/discorder/discorder.yaml",
];

static CONFIG_PATH_ENV_KEY: &str = "DISCORDER_CONFIG_PATH";

/// A cli tool for sending text or file to Discord Webhook
#[derive(Parser)]
#[command(version)]
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
    #[clap(short, long)]
    config: Option<String>,
}

fn resolve_path(path: &str) -> String {
    if path.starts_with('~') {
        let home = std::env::var("HOME").unwrap();
        return path.replace('~', &home);
    }
    path.to_owned()
}

fn load_config(path: Option<String>) -> Result<Option<Yaml>, Box<dyn std::error::Error>> {
    let path = path.unwrap_or_else(|| {
        CONFIG_DEFAULT_PATHS
            .iter()
            .map(|path| resolve_path(path))
            .find(|path| Path::new(path).exists())
            .unwrap_or_else(|| "".to_owned())
    });
    let path = std::env::var(CONFIG_PATH_ENV_KEY)
        .map(|path| resolve_path(&path))
        .unwrap_or(path);
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Ok(None),
    };
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    let docs = YamlLoader::load_from_str(&buf)?;
    Ok(docs.first().cloned())
}

fn main() {
    let mut args = Args::parse();

    let config = load_config(args.config.clone()).unwrap();
    if let Some(config) = config {
        let webhook = config["webhook"].as_str().map(|s| s.to_owned());
        let text = config["text"].as_str().map(|s| s.to_owned());
        let file = config["file"].as_str().map(|s| s.to_owned());

        args = Args {
            webhook: args.webhook.or(webhook),
            text: args.text.or(text),
            file: args.file.or(file),
            config: args.config,
        };
    }

    match (args.webhook, args.text, args.file) {
        (Some(webhook), Some(text), None) => send_text_to_discord(&webhook, &text).unwrap(),
        (Some(webhook), None, Some(file)) => {
            let file_name = Path::new(&file).file_name().unwrap().to_str().unwrap();
            let file_bytes = std::fs::read(&file).unwrap();
            send_file_to_discord(&webhook, file_bytes, file_name).unwrap()
        }
        (Some(webhook), None, None) => {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf).unwrap();
            send_file_to_discord(&webhook, buf.into_bytes(), "stdin.txt").unwrap()
        }
        (None, _, _) => {
            println!("Please input webhook url");
        }
        _ => {
            println!("Please input text or file path, or you can use stdin");
        }
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

fn send_file_to_discord(
    webhook: &str,
    file_bytes: Vec<u8>,
    file_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let form = Form::new();
    let part = Part::bytes(file_bytes).file_name(file_name.to_owned());

    let res = client
        .post(webhook)
        .multipart(form.part("file", part))
        .send()?;

    if res.status().is_success() {
        println!("Success!");
    } else {
        println!("Failed!");
        println!("{:?}", res.text());
    }

    Ok(())
}
