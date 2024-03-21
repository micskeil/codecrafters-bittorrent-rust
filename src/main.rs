mod decode;
mod file;
use serde_json;
use std::env;
use decode::decode;
use serde_bencode;
use file::Torrent;
use sha1::Digest;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &[u8]) -> serde_json::Value {
    decode(encoded_value).unwrap()
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
        // turn string to bytes [u8]
        let encoded_value = &args[2].as_bytes();
        let decoded_value: serde_json::Value = decode_bencoded_value(encoded_value);
        println!("{:?}", decoded_value);
    }

    if command == "info" {
        let torrent: Torrent = file::file_contents(&args[2]).expect("expected FileData");

        let mut hasher = sha1::Sha1::new();
        let encoded_info = serde_bencode::to_string(&torrent.info).unwrap();
        println!("{:?}", encoded_info);

        let hashed_info = hasher.finalize();

        println!(
            "Tracker URL: {}\nLength: {} \nInfo Hash: {:x}" ,
            torrent.announcement, torrent.info.length, hashed_info
        );
    }
}
