extern crate serde_json;

pub use serde_json::{json, Value};

pub fn parse_response(response: &Vec<u8>) -> std::result::Result<Value, String> {
    if response[0..3] != [0xff, 0xff, 0xff, 0xff] {
        return Err(error_helper_for_parse_response("0..3", &response[0..3]))
    }

    if response[4] != 0x49 {
        return Err(error_helper_for_parse_response("4", &[response[4]]))
    }

    // WIP

    let ret_json = json!("{}");

    Ok(json!("{}"))
}

pub fn bytes_to_char_and_map(bytes: &[u8]) -> String {
    bytes.iter().map(|&s| format!("{}, ", s.to_string())).collect::<String>()
}

fn error_helper_for_parse_response(position: &'static str, buffer: &[u8]) -> String {
    format!("response invalid -> {}: {}", position, bytes_to_char_and_map(buffer))
}
