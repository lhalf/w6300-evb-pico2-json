mod common;

use colored::Colorize;
use common::Connection;
use std::time::{Duration, Instant};

#[test]
fn single_packet() {
    let connection = Connection::new();

    let payload = b"{}";

    let start = Instant::now();

    connection.socket.send(payload).unwrap();
    let received = connection
        .receiver
        .recv_timeout(Duration::from_secs(1))
        .unwrap();

    let elapsed = start.elapsed();

    assert_eq!(received, payload.to_vec());

    print!("{} ", format!("{elapsed:?}").green());
}
