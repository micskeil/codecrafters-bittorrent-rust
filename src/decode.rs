use std::str::FromStr;
pub struct DecodedValue {
  encoded: Vec<u8>,
  pub value: Vec<u8>,
}


pub fn decode_string(bytes: &[u8]) -> Result<DecodedValue, String> {
    // find the index of the colon
    let colon_index: usize = bytes.iter().position(|&x| x == b':').ok_or_else(|| {
        format!("Invalid encoded string, no colon: {:?}", bytes)
    })?;

    // get the length of the string
    let indicated_length = &bytes[..colon_index].iter().map(|&x| x as char).collect::<String>();
    let length = i64::from_str(indicated_length).map_err(|err| {
        format!("Error parsing number: {}", err)
    })?;

    // get the string value from the bytes
    let value = &bytes[colon_index + 1..colon_index + 1 + length as usize].to_vec();



    // find the end index of the string, where the characters
    let end_index: usize = colon_index + 1 + value.len();
    let encoded = bytes[..end_index].to_vec();

    return Ok(DecodedValue {
        encoded,
        value: value.to_owned(),
    });
}

pub fn decode_number(bytes: &[u8]) -> Result<DecodedValue, String> {
    // find the element with value 'e' and get the index
    let end_index: usize = bytes.iter().position(|&x| x == b'e').ok_or_else(|| {
        format!("Invalid encoded number, no end character: {:?}", bytes)
    })?;

    let number = &bytes[1..end_index].to_vec();
    let encoded_number = bytes[..end_index + 1].to_vec();

    Ok(DecodedValue {
        encoded: encoded_number,
        value: number.to_owned(),
    })
}

// pub fn decode_list(bytes: &[u8]) -> Result<DecodedValue, String> {
//     let encoded_string = String::from_utf8_lossy(bytes);
//     let mut to_encode = &encoded_string[1..encoded_string.len()-1];

//     let mut list: Vec<serde_json::Value> = Vec::new();
//     while to_encode.len() > 0 {
//         if to_encode.starts_with("e") {
//         to_encode = &to_encode[1..];
//         break;
//         }

//         to_encode.
//         decode(to_encode).map(|decoded_value| {
//         list.push(decoded_value.value);
//         to_encode = &to_encode[decoded_value.encoded.len()..];
//         }).map_err(|err| {
//         format!("Error decoding list: {}", err)
//         })?;
//     }
//     // Calculate the encoded value
//     let encoded: String = encoded_list[..encoded_list.len()-to_encode.len()-1].to_string();
//     Ok(DecodedValue {
//         encoded,
//         value: serde_json::Value::Array(list),
//     });
// }

// pub fn decode_dictionary(encoded_value: &[u8]) -> Result<DecodedValue, String> {
//     let mut to_encode = &encoded_dictionary[1..encoded_dictionary.len()-1];
//     let mut dictionary: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
//     while to_encode.len() > 0 {
//         if to_encode.starts_with("e") {
//             to_encode = &to_encode[1..];
//             break;
//         }
//         decode(to_encode).map(|decoded_key| {
//             to_encode = &to_encode[decoded_key.encoded.len()..];
//             let key = decoded_key.value.as_str().ok_or_else(|| {
//                 format!("Dictionary key is not a string: {}", decoded_key.value)
//             });
//             let value = decode(to_encode).map(|decoded_value: DecodedValue| {
//                 to_encode = &to_encode[decoded_value.encoded.len()..];
//                 decoded_value.value
//             }).map_err(|err| {
//                 format!("Error decoding dictionary value: {}", err)
//             });
//             dictionary.insert(key.unwrap().to_string(), value.unwrap());
//         }).map_err(|err| {
//             format!("Could not insert to the dictionary: {}", err)
//         })?;
//     }

//     // Calculate the encoded value
//     let encoded: String = encoded_dictionary[..encoded_dictionary.len()-to_encode.len()-1].to_string();
//     Ok(DecodedValue {
//         encoded,
//         value: serde_json::Value::Object(dictionary),
//     })
// }

pub fn decode(bytes: &[u8]) -> Result<serde_json::Value, String> {
    if bytes[0] == b'i' {
        let string = String::from_utf8_lossy(&decode_number(&bytes).unwrap().value).to_string().parse::<i64>().unwrap();
        return Ok(serde_json::Value::Number(serde_json::Number::from(string)));
    }

    // if encoded_string.chars().next().unwrap().is_digit(10) {
    //     return decode_string(&bytes);
    // }
    // if encoded_string.chars().next().unwrap() == 'i' {
    //     return decode_number(&bytes);
    // }
    // if encoded_value_as_string.chars().next().unwrap() == 'l' {
    //     return decode_list(encoded_value);
    // }
    // if encoded_value_as_string.chars().next().unwrap() == 'd' {
    //     return decode_dictionary(encoded_value);
    // }

    Err(format!("Unhandled encoded value: {:?}", String::from_utf8(vec![bytes[0]]).unwrap()))
}