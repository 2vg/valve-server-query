extern crate binary_reader;
extern crate serde;
extern crate serde_json;
 
use binary_reader::{Endian, BinaryReader};

use serde_json::{json, Value, Result};

pub fn parse_info_response(response: Vec<u8>) -> std::result::Result<Value, String> {
    if response.len() == 0 { return Ok(json!({})) }

    if response[0..4] != [0xff, 0xff, 0xff, 0xff] {
        return Err(error_helper_for_parse_response("0..3", &response[0..4]))
    }

    if response[4] != 0x49 {
        return Err(error_helper_for_parse_response("4", &[response[4]]))
    }

    let mut binary = BinaryReader::from_vec(&response);
    binary.set_endian(Endian::Little);
    let _ = binary.read_u32(); // 0xFF,0xFF,0xFF,0xFF
    let _ = binary.read_u8(); // Header
    let _ = binary.read_u8(); // packet type

    let server_name = binary.read_cstr();
    let map_name = binary.read_cstr();
    let folder = binary.read_cstr();
    let game_name = binary.read_cstr();
    let steam_app_id = binary.read_u16().unwrap_or(0);
    let players = binary.read_i8().unwrap_or(0);
    let max_players = binary.read_i8().unwrap_or(0);
    let bots = binary.read_i8().unwrap_or(0);
    let server_type = binary.read_u8().unwrap_or(0) as char;
    let server_environment = binary.read_u8().unwrap_or(0) as char;
    let visibility = binary.read_i8().unwrap_or(0);
    let vac = binary.read_i8().unwrap_or(0);

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
        "visibility": visibility,
        "vac": vac,
    }))
}

pub fn parse_player_response(response: Vec<u8>) -> std::result::Result<Vec<Value>, String> {
    if response.len() == 0 { return Ok(vec![json!({})]) }

    let mut binary = BinaryReader::from_vec(&response);
    binary.set_endian(Endian::Little);
    let _ = binary.read_u32();
    let _ = binary.read_u8();

    let players = binary.read_u8().unwrap_or(0);
    let mut players_vec = Vec::new();

    for _ in 0..players {
        let index = binary.read_u8().unwrap_or(0);
        let name = binary.read_cstr();
        let score = binary.read_i32().unwrap_or(0);
        let time = binary.read_u32().unwrap_or(0);

        players_vec.push(json!({"index": index, "name": name, "score": score, "time": f32::from_bits(time)}));
    }

    Ok(players_vec)
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

