use std::fs::OpenOptions;
use std::io::Read;
use std::os::unix::net::{UnixListener, UnixStream};
use std::{
    fs::{remove_file, File},
    io::Write,
    process::exit,
    thread::spawn,
};

use imrefs::args::Args;
use libc::getpid;
use signal_hook::iterator::Signals;

fn main() {
    let mut signals = Signals::new(&[libc::SIGTERM]).expect("Failed to initialize signal handler");

    let args = Args::parse();
    let socket_name = format!("/tmp/imrefs-{}.sock", args.file_name);
    let tempfile_name = format!("/tmp/imrefs-{}.tmp", args.file_name);
    let socket_name_close = format!("/tmp/imrefs-{}.sock", args.file_name);
    let tempfile_name_sloce = format!("/tmp/imrefs-{}.tmp", args.file_name);
    spawn(move || {
        for signal in signals.forever() {
            match signal {
                libc::SIGTERM => {
                    println!("Received SIGTERM signal.");
                    cleanup(&socket_name_close, &tempfile_name_sloce)
                }
                _ => {}
            }
        }
    });

    match args.arg_type {
        imrefs::args::ArgType::Init => {
            if let Err(e) = File::create(&tempfile_name) {
                eprintln!("Error can't create file: {}", e);
                exit(1);
            };
            match unsafe { libc::fork() } {
                -1 => {
                    eprintln!("fork failed");
                    exit(1);
                }
                0 => {
                    if unsafe { libc::setsid() } == -1 {
                        eprintln!("Failed to create new session");
                        exit(1);
                    }
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
                        socket_name, child_pid
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
                                    // remove the file socket
                                    if let Err(err) = remove_file(&socket_name) {
                                        eprintln!("Failed to remove file socket: {}", err);
                                        exit(1);
                                    }
                                    // remove the file socket
                                    if let Err(err) = remove_file(&tempfile_name) {
                                        eprintln!("Failed to remove temporary file: {}", err);
                                        exit(1);
                                    }

                                    println!("Filesystem {} succesfully removed", &socket_name);
                                    exit(0);
                                }
                                // response = response.replace("msg:", "");

                                let mut file = match OpenOptions::new()
                                    .create(true)
                                    .write(true)
                                    .truncate(true)
                                    .open(&tempfile_name)
                                {
                                    Ok(f) => f,
                                    Err(e) => {
                                        println!("Error can't open file: {}", e);
                                        exit(1);
                                    }
                                };

                                let response = response.replace("msg:", "");
                                println!("Data successfully written to file: {}", &tempfile_name);

                                if let Err(e) = file.write_all(response.as_bytes()) {
                                    println!("Error can't write to file: {}", e);
                                    continue;
                                };
                            }
                            Err(err) => {
                                eprintln!("Failed to accept connection: {}", err);
                            }
                        }
                    }
                }
                _ => {
                    exit(0);
                }
            }
        }
        imrefs::args::ArgType::Send => {
            let mut stream = match UnixStream::connect(socket_name) {
                Ok(s) => s,
                Err(err) => {
                    eprintln!("Failed connect to Unix socket: {}", err);
                    exit(1);
                }
            };
            let message_formatted = format!("msg:{}", args.message);
            if let Err(e) = stream.write_all(message_formatted.as_bytes()) {
                println!("Error can't write to socket: {}", e);
                exit(1);
            };
        }
        imrefs::args::ArgType::Stop => {
            let mut stream = match UnixStream::connect(socket_name) {
                Ok(s) => s,
                Err(err) => {
                    eprintln!("Failed connect to Unix socket: {}", err);
                    exit(1);
                }
            };
            if let Err(e) = stream.write_all("cmd:stop".as_bytes()) {
                println!("Error can't write to socket: {}", e);
                exit(1);
            };
        }
        imrefs::args::ArgType::Others => {}
    };
}

fn cleanup(socket_name: &str, tempfile_name: &str) {
    // remove the file socket
    if let Err(err) = remove_file(socket_name) {
        eprintln!("Failed to remove file socket: {}", err);
        exit(1);
    }
    // remove the file socket
    if let Err(err) = remove_file(tempfile_name) {
        eprintln!("Failed to remove temporary file: {}", err);
        exit(1);
    }
    println!("Filesystem {} successfully removed", socket_name);
}
