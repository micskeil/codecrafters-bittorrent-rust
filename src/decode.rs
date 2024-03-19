pub struct DecodedValue {
  // length: i64,
  encoded: String,
  pub value: serde_json::Value,
}


pub fn decode_string(encoded_string: &str) -> Result<DecodedValue, String> {
  let colon_index = encoded_string.find(':').ok_or_else(|| {
    format!("Invalid encoded string, missing semicolon: {}", encoded_string)
  })?;

  let length: usize = encoded_string[..colon_index].parse::<usize>().map_err(|_| {
    format!("Invalid encoded string, length is not a number: {}", encoded_string)
  })?;

  // find the end index of the string
  let end_index = colon_index + length + 1 ;
  // get the value of the string
  let value = &encoded_string[colon_index + 1 ..end_index ];

  Ok(DecodedValue {
    encoded: encoded_string[..end_index].to_string(),
    value: serde_json::Value::String(value.to_string()),
  })
}

pub fn decode_number(encoded_number: &str) -> Result<DecodedValue, String> {
  let end_index = encoded_number.find('e').ok_or_else(|| {
    format!("Invalid encoded number, missing e: {}", encoded_number)
  })?;

  let number_string = &encoded_number[1..end_index];
  let number = number_string.parse::<i64>().map_err(|_| {
    format!("Invalid encoded number, number is not a number: {}", encoded_number)
  })?;

  Ok(DecodedValue {
    encoded: encoded_number[..end_index + 1].to_string(),
    value: serde_json::Value::Number(serde_json::Number::from(number)),
})
}

pub fn decode_list(encoded_list: &str) -> Result<DecodedValue, String> {
  let mut to_encode = &encoded_list[1..encoded_list.len()-1];
  let mut list: Vec<serde_json::Value> = Vec::new();

  while to_encode.len() > 0 {
    if to_encode.starts_with("e") {
      to_encode = &to_encode[1..];
      break;
    }
    decode(to_encode).map(|decoded_value| {
      list.push(decoded_value.value);
      to_encode = &to_encode[decoded_value.encoded.len()..];
    }).map_err(|err| {
      format!("Error decoding list: {}", err)
    })?;
  }

  // Calculate the encoded value
  let encoded: String = encoded_list[..encoded_list.len()-to_encode.len()-1].to_string();
  Ok(DecodedValue {
    encoded,
    value: serde_json::Value::Array(list),
  })
}

pub fn decode(encoded_value: &str) -> Result<DecodedValue, String> {
  if encoded_value.chars().next().unwrap().is_digit(10) {
    return decode_string(encoded_value);
  }
  if encoded_value.chars().next().unwrap() == 'i' {
    return decode_number(encoded_value);
  }
  if encoded_value.chars().next().unwrap() == 'l' {
    return decode_list(encoded_value);
  }
  Err(format!("Unhandled encoded value: {}", encoded_value))
}