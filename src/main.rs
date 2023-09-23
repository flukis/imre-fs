use std::{
    fs::{remove_file, File, OpenOptions},
    io::Write,
    process::exit,
};

use imrefs::args::Args;

fn main() {
    let args = Args::parse();
    let tempfile_name = format!("tmp/imrefs-{}.tmp", args.file_name);

    match args.arg_type {
        imrefs::args::ArgType::Init => {
            if let Err(e) = File::create(tempfile_name) {
                eprintln!("error create temp file: {}", e);
                exit(1);
            }
        }
        imrefs::args::ArgType::Send => {
            let mut file = match OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(tempfile_name)
            {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("error open temp file: {}", e);
                    exit(1)
                }
            };

            if let Err(e) = file.write_all(args.message.as_bytes()) {
                eprintln!("cannot write to temp file: {}", e);
            }
        }
        imrefs::args::ArgType::Stop => {
            if let Err(e) = remove_file(tempfile_name) {
                eprintln!("error create temp file: {}", e);
                exit(1);
            }
        }
        imrefs::args::ArgType::Others => {}
    };
}
