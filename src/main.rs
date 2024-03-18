use serde_json;
use std::env;

// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    // If encoded_value starts with a digit, it's a number
    let first_char= encoded_value.chars().next().unwrap();
    if first_char.is_digit(10) {
        // Example: "5:hello" -> "hello"
        let colon_index = encoded_value.find(':').unwrap();
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<i64>().unwrap();
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
        return serde_json::Value::String(string.to_string());
    } else if first_char ==  'i'  {
        // Example: "i42e" -> 42
        let end_index = encoded_value.find('e').unwrap();
        let number_string = &encoded_value[1..end_index];
        let number = number_string.parse::<i64>().unwrap();
        return serde_json::Value::Number(serde_json::Number::from(number));
    }
    // else if first_char == 'l' {
    //     // Example: "l5:helloi42ee" -> ["hello", 42]
    //     let mut list = vec![];
    //     let mut index = 1;
    //     while encoded_value.chars().nth(index).unwrap() != 'e' {
    //         let value = &encoded_value[index..];
    //         let decoded_value = decode_bencoded_value(value);
    //         list.push(decoded_value);
    //         index += value.len();
    //     }
    //     return serde_json::Value::Array(list);
    // } else if first_char == 'd' {
    //     // Example: "d3:cow3:moo4:spam4:eggse" -> {"cow": "moo", "spam": "eggs"}
    //     let mut map = serde_json::Map::new();
    //     let mut index = 1;
    //     while encoded_value.chars().nth(index).unwrap() != 'e' {
    //         let key = &encoded_value[index..];
    //         let decoded_key = decode_bencoded_value(key);
    //         index += key.len();
    //         let value = &encoded_value[index..];
    //         let decoded_value = decode_bencoded_value(value);
    //         index += value.len();
    //         map.insert(decoded_key.to_string(), decoded_value);
    //     }
    //     return serde_json::Value::Object(map);        
    // }
     else {
        panic!("Unhandled encoded value: {}", encoded_value)
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        // println!("Logs from your program will appear here!");

        // Uncomment this block to pass the first stage
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
