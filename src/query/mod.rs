use std::net::UdpSocket;
use std::time::Duration;

// reference: https://developer.valvesoftware.com/wiki/Server_queries

const SPECIAL_CHARA_VEC: &'static [u8] = &[0xff, 0xff, 0xff, 0xff];
const PAYLOAD: &'static [u8] = "Source Engine Query".as_bytes();

pub struct QueryContext {
    socket: UdpSocket,
}

impl QueryContext {
    pub fn new() -> QueryContext {
        let socket = UdpSocket::bind("0.0.0.0:12345").unwrap();
        let timeout_sec = Some(Duration::new(2, 0));
        let _ = socket.set_read_timeout(timeout_sec);
        let _ = socket.set_write_timeout(timeout_sec);
        return QueryContext {
            socket: socket,
        };
    }

    pub fn info_query<'a>(&self, host: &'a str, port: &'a str) -> Vec<u8> {
        let query_type = 0x54; // 'T'
        return Vec::from(self.send_query(&host, &port, query_type, None as Option<&[u8]>));
    }

    pub fn player_query<'a>(&self, host: &'a str, port: &'a str) -> Vec<u8> {
        let query_type = 0x55; // 'U'
        let result = self.send_query(&host, &port, query_type, None as Option<&[u8]>);
        if result.len() == 0 { return result; } // for timeout error
        let chanllenge_token = &result[5..9]; // challenge token, [0xFF,0xFF,0xFF,0xFF,0x41,hoge,hoge,hoge,hoge]
        return Vec::from(self.send_query(&host, &port, query_type, Some(chanllenge_token)));
    }

    fn send_query<'a>(&self, host: &'a str, port: &'a str, query_type: u8, challenge_token: Option<&[u8]>) -> Vec<u8> {
        let address = format!("{}:{}", host, port);
        let query_type = query_type;
        let query = build_query(query_type, challenge_token);
        match self.socket.send_to(&query, &address) {
            Ok(_) => {},
            Err(_) => { return vec![0u8;0]; }
        }
        let mut buf = vec![0u8; 10240];
        match self.socket.recv_from(&mut buf) {
            Ok(result) => { return Vec::from(&buf[..result.0]); },
            Err(_) => { return vec![0u8;0]; }
        }
    }
}

pub fn build_query(query_type: u8, challenge_token: Option<&[u8]>) -> Vec<u8> {
    let mut query = Vec::new();

    query.extend_from_slice(&SPECIAL_CHARA_VEC);
    query.push(query_type);

    match challenge_token {
        Some(token) => {
            query.extend_from_slice(token);
        },
        None => {
            if query_type == 0x55 {
                query.extend_from_slice(&SPECIAL_CHARA_VEC);
            } else {
                query.extend_from_slice(&PAYLOAD);
                query.push(0x00);
            }
        }
    };

    return query;
}
