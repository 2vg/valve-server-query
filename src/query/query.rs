extern crate async_std;

use async_std::io;
use async_std::net::UdpSocket;
use async_std::task;

// reference: https://developer.valvesoftware.com/wiki/Server_queries

const SPECIAL_CHARA_VEC: &'static [u8] = &[0xff, 0xff, 0xff, 0xff];
const PAYLOAD: &'static [u8] = "Source Engine Query".as_bytes();

pub fn info_query<'a>(host: &'a str, port: &'a str) -> io::Result<Vec<u8>> {
    let address = format!("{}:{}", host, port);

    return task::block_on(async {
        let socket = UdpSocket::bind("0.0.0.0:12345").await?;

        let query_type = 0x54; // 'T'

        let query = build_query(query_type, None as Option<&[u8]>);

        socket.send_to(&query, &address).await?;

        let mut buf = vec![0u8; 1024];
        let (n, _) = socket.recv_from(&mut buf).await?;

        Ok(Vec::from(&buf[..n]))
    });
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
