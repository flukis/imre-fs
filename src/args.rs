use clap::{App, Arg};

pub enum ArgType {
    Init,
    Send,
    Stop,
    Others,
}

pub struct Args {
    pub arg_type: ArgType,
    pub file_name: String,
    pub message: String,
}

impl Args {
    pub fn parse() -> Self {
        let mut file_name = "";
        let mut message = "";
        let mut arg_type: ArgType = ArgType::Others;
        let matches = App::new("imrefs")
            .version("1.0")
            .author("Fahmi Lukistriya")
            .about("Imre Filesystem")
            .subcommand(
                App::new("init").about("Initialize a filesystem").arg(
                    Arg::with_name("file_name")
                        .help("Name of the filesystem to initialize")
                        .required(true)
                        .index(1),
                ),
            )
            .subcommand(
                App::new("send")
                    .about("Send a message using a file")
                    .arg(
                        Arg::with_name("file_name")
                            .help("Name of the filesystem to initialize")
                            .required(true)
                            .index(1),
                    )
                    .arg(
                        Arg::with_name("message")
                            .help("Message to send")
                            .required(true)
                            .index(2),
                    ),
            )
            .subcommand(
                App::new("stop").about("Stop a filesystem").arg(
                    Arg::with_name("file_name")
                        .help("Name of the filesystem to initialize")
                        .required(true)
                        .index(1),
                ),
            )
            .get_matches();
        match matches.subcommand() {
            ("init", Some(init_matches)) => {
                file_name = init_matches.value_of("file_name").unwrap();
                arg_type = ArgType::Init;
            }
            ("send", Some(send_matches)) => {
                file_name = send_matches.value_of("file_name").unwrap();
                message = send_matches.value_of("message").unwrap();
                arg_type = ArgType::Send;
            }
            ("stop", Some(stop_matches)) => {
                file_name = stop_matches.value_of("file_name").unwrap();
                arg_type = ArgType::Stop;
            }
            _ => {
                println!("Invalid command. Use 'init' or 'send'")
            }
        }
        Self {
            file_name: file_name.to_string(),
            message: message.to_string(),
            arg_type,
        }
    }
}
