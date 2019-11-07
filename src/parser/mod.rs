extern crate serde;
extern crate serde_json;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value, Result};

pub fn parse_info_response(response: Vec<u8>) -> std::result::Result<Value, String> {
    if response[0..4] != [0xff, 0xff, 0xff, 0xff] {
        return Err(error_helper_for_parse_response("0..3", &response[0..4]))
    }

    if response[4] != 0x49 {
        return Err(error_helper_for_parse_response("4", &[response[4]]))
    }

    let mut r = response[6..].split(|bin| bin == &0x00);

    let server_name = String::from_utf8_lossy(r.next().unwrap());
    let map_name = String::from_utf8_lossy(r.next().unwrap());
    let folder = String::from_utf8_lossy(r.next().unwrap());
    let game_name = String::from_utf8_lossy(r.next().unwrap());

    let tmp = r.next().unwrap();
    let steam_app_id = (tmp[0] as u16) + ((tmp[1] as u16) << 8);
    let players = tmp[2];
    let max_players = tmp[3];
    let bots = tmp[4];
    let server_type = tmp[5] as char;
    let server_environment = tmp[6] as char;

    Ok(json!({
        "server_name": server_name,
        "map_name": map_name,
        "folder": folder,
        "game_name": game_name,
        "steam_app_id": steam_app_id.to_string(),
        "players": players.to_string(),
        "max_players": max_players.to_string(),
        "bots": bots.to_string(),
        "server_type": server_type,
        "server_environment": server_environment,
    }))
}

pub fn json_to_string(json: &Value) -> Result<String> {
    Ok(serde_json::to_string(json)?)
}

pub fn print_json(json: &Value) -> Result<()> {
    let j = serde_json::to_string(json)?;
    println!("{}", j);
    Ok(())
}

pub fn bytes_to_char_and_map(bytes: &[u8]) -> String {
    bytes.iter().map(|&s| format!("{}, ", s.to_string())).collect::<String>()
}

fn error_helper_for_parse_response(position: &'static str, buffer: &[u8]) -> String {
    format!("response invalid -> {}: {}", position, bytes_to_char_and_map(buffer))
}

