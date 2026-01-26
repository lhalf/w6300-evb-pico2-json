mod common;

use common::Connection;
use std::sync::mpsc::RecvTimeoutError;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(1);

#[test]
fn valid_json_is_echoed() {
    let connection = Connection::new();

    let payload = b"{}";

    connection.socket.send(payload).unwrap();

    assert_eq!(
        connection.receiver.recv_timeout(TIMEOUT).unwrap(),
        payload.to_vec()
    );
}

#[test]
fn invalid_json_is_dropped() {
    let connection = Connection::new();

    let payload = b"{";

    connection.socket.send(payload).unwrap();

    assert_eq!(
        RecvTimeoutError::Timeout,
        connection.receiver.recv_timeout(TIMEOUT).unwrap_err()
    );
}
