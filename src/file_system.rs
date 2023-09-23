use std::fs::{remove_file, File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::net::UnixListener;
use std::process::exit;

use libc::getpid;

pub fn initiate_filesystem(name: &str) {
    let socket_name = format!("/tmp/imrefs-{}.sock", name);
    let listener = match UnixListener::bind(&socket_name) {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Failed to bind to Unix socket: {}", err);
            exit(1);
        }
    };
    let child_pid = unsafe { getpid() };
    println!(
        "Filesystem {} successfully created with PID: {}",
        &socket_name, child_pid
    );
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut response = String::new();
                if let Err(err) = stream.read_to_string(&mut response) {
                    eprintln!("Failed to read from socket: {}", err);
                    exit(1);
                }
                if response == "cmd:stop" {
                    remove_temp_and_sock(name);
                }
                write_to_temp_file(name, &response)
            }
            Err(err) => {
                eprintln!("Failed to accept connection: {}", err);
            }
        }
    }
}

pub fn create_temp_file(name: &str) {
    let tempfile_name = format!("/tmp/imrefs-{}.tmp", name);
    if let Err(e) = File::create(tempfile_name) {
        eprintln!("error can't create file: {}", e);
        exit(1);
    };
}

pub fn write_to_temp_file(name: &str, message: &str) {
    let tempfile_name = format!("/tmp/imrefs-{}.tmp", name);
    let mut file = match OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&tempfile_name)
    {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error can't open file: {}", e);
            exit(1);
        }
    };
    let response = message.replace("msg:", "");
    println!("Data successfully written to file: {}", &tempfile_name);
    if let Err(e) = file.write_all(response.as_bytes()) {
        eprintln!("error can't write to file: {}", e);
    };
}

pub fn remove_temp_and_sock(name: &str) {
    let socket_name = format!("/tmp/imrefs-{}.sock", name);
    let tempfile_name = format!("/tmp/imrefs-{}.tmp", name);
    // remove the file socket
    if let Err(err) = remove_file(&socket_name) {
        eprintln!("Failed to remove file socket: {}", err);
        exit(1);
    }
    // remove the file socket
    if let Err(err) = remove_file(tempfile_name) {
        eprintln!("Failed to remove temporary file: {}", err);
        exit(1);
    }

    println!("Filesystem {} succesfully removed", &socket_name);
    exit(0);
}
