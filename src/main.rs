mod decode;
use serde_json;
use std::env;

use decode::decode;

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

    if command == "decode" {
        let decoded_value = decode_bencoded_value(&args[2]);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
