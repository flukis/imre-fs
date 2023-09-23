use imrefs::args::Args;

fn main() {
    let args = Args::parse();

    let arg_type: String = match args.arg_type {
        imrefs::args::ArgType::Init => "Init".to_string(),
        imrefs::args::ArgType::Send => "Send".to_string(),
        imrefs::args::ArgType::Stop => "Stop".to_string(),
        imrefs::args::ArgType::Others => "Others".to_string(),
    };

    eprint!(
        "filesystem name: {}\nmessage: {}\ntype: {}\n",
        args.file_name, args.message, arg_type
    )
}
