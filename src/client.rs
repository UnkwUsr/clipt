use crate::shared::SOCKET_PATH;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;

pub fn client_put() {
    let mut stream = connect_socket();

    let mut message = "put\n".to_string();
    std::io::stdin().read_to_string(&mut message).unwrap();
    send_message(&mut stream, &message);
}

pub fn client_list() {
    let mut stream = connect_socket();

    let message = "list\n";
    send_message(&mut stream, message);
    stream.shutdown(std::net::Shutdown::Write).unwrap();

    let buf = recv_message(&mut stream);
    println!("{}", buf);
}

pub fn client_pick(id: &str) {
    let mut stream = connect_socket();

    let message = format!("pick\n{}", id);
    send_message(&mut stream, &message);
    stream.shutdown(std::net::Shutdown::Write).unwrap();

    let buf = recv_message(&mut stream);
    println!("{}", buf);
}

fn send_message(stream: &mut UnixStream, message: &str) {
    stream
        .write_all(message.as_bytes())
        .expect("Can't send message to server");
}

fn recv_message(stream: &mut UnixStream) -> String {
    let mut buf = String::new();
    stream.read_to_string(&mut buf).unwrap();
    buf
}

fn connect_socket() -> UnixStream {
    let socket = Path::new(SOCKET_PATH);
    UnixStream::connect(&socket).expect("Server is not running")
}
