mod decode;
mod file;
use serde_json;
use std::env;
use decode::decode;
use file::FileData;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    // If encoded_value starts with a digit, it's a number
    decode(encoded_value).map(|decoded_value| {
        decoded_value.value
    }).unwrap_or_else(|err| {
        panic!("Error decoding value: {}", err)
    })
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    const VALID_COMMANDS: [&str; 2] = ["decode", "info"];
    if !VALID_COMMANDS.contains(&command.as_str()) {
        println!("Invalid command: {}", command);
        return;
    }

    if command == "decode" {
        let decoded_value = decode_bencoded_value(&args[2]);
        println!("{}", decoded_value.to_string());
    }

    if command == "info" {
        let contents: FileData = file::file_contents(&args[2]).expect("expected FileData");
        println!(
            "Tracker URL: {}\nLength: {}",
            contents.announcement, contents.info.length
        );
    }
}
