extern crate valve_server_query;

use valve_server_query::{ query::QueryContext, parser };

fn main() {
    let host = "13.73.0.133";
    let port = "27017";
    let ctx = QueryContext::new();

    let buf = ctx.player_query(&host, &port);

    println!("{}", parser::bytes_to_char_and_map(&buf));
}
