use std::process::exit;

use imrefs::args::Args;
use imrefs::{file_system, sender};

fn main() {
    let args = Args::parse();
    match args.arg_type {
        imrefs::args::ArgType::Init => {
            file_system::create_temp_file(&args.file_name);
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
                    file_system::initiate_filesystem(&args.file_name);
                }
                _ => {
                    exit(0);
                }
            }
        }
        imrefs::args::ArgType::Send => {
            let message_formatted = format!("msg:{}", args.message);
            sender::send_message_to_socket(&args.file_name, message_formatted.as_bytes().to_vec())
        }
        imrefs::args::ArgType::Stop => {
            sender::send_message_to_socket(&args.file_name, "cmd:stop".as_bytes().to_vec())
        }
        imrefs::args::ArgType::Others => {}
    };
}
