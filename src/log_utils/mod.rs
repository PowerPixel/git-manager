pub fn exit_abnormally(error_message: &str) {
    eprintln!("ERROR : {}", error_message);
    std::process::exit(1);
}