use std::{io::Write, os::unix::net::UnixStream, process::exit};

pub fn send_message_to_socket(name: &str, command: Vec<u8>) {
    let socket_name = format!("/tmp/imrefs-{}.sock", name);
    let mut stream = match UnixStream::connect(socket_name) {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Failed connect to Unix socket: {}", err);
            exit(1);
        }
    };
    if let Err(e) = stream.write_all(&command) {
        println!("Error can't write to socket: {}", e);
        exit(1);
    };
}
