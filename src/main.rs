extern crate async_std;
extern crate valve_server_query;

use async_std::io;
use async_std::net::UdpSocket;
use async_std::task;

use valve_server_query::{ query, parser };

fn main() {
    let host = "13.73.0.133";
    let port = "27017";

    match query::info_query(&host, &port) {
        Ok(result) => {
            println!("result: {}", String::from_utf8_lossy(&result));
            println!("result: {}", parser::bytes_to_char_and_map(&result));
        },
        Err(msg) => {},
    }
}
