use crate::{
    config::{Config, ConfigFile},
    git,
    log_utils::exit_abnormally,
};

use clap::{Arg, Command};
use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

const DEFAULT_CONFIG_PATH: &str = "~/.config/gitpm/config.json";

pub fn cli_bootstrap() {
    let cmd = Command::new("gitpm")
        .version("0.1.0")
        .arg(
            Arg::new("config-path")
                .help("The path to the config json file")
                .short('c')
                .long("config-path")
                .takes_value(true),
        )
        .arg(
            Arg::new("profile")
                .help("Name of the profile to use")
                .short('p')
                .long("profile")
                .takes_value(true),
        )
        .subcommand_required(true)
        .subcommand(
            Command::new("clone")
                .about("Clone a new git repository")
                .arg(
                    Arg::new("path")
                        .help("Path to the directory of the repository")
                        .index(2)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("url")
                        .help("URL of the repository to clone")
                        .required(true)
                        .index(1)
                        .takes_value(true),
                ),
        )
        .get_matches();

    let config_path: PathBuf = match cmd.value_of("config-path") {
        Some(path) => Path::new(shellexpand::tilde(path).into_owned().as_str()).to_owned(),
        None => Path::new(
            shellexpand::tilde(DEFAULT_CONFIG_PATH)
                .into_owned()
                .as_str(),
        )
        .to_owned(),
    };

    let file: File = match File::open(config_path.as_path()) {
        Ok(f) => f,
        Err(err) => {
            exit_abnormally(&format!(
                "cannot open file : {}",
                err.raw_os_error().as_ref().unwrap()
            ));
            return;
        }
    };

    let read: BufReader<File> = BufReader::new(file);

    let config_file: ConfigFile = match ConfigFile::read(read) {
        serde_json::Result::Ok(conf_file) => conf_file,
        serde_json::Result::Err(e) => {
            exit_abnormally(&format!("could not parse config_file : {}", e.to_string()));
            return;
        }
    };

    let profile_name: &str = match cmd.value_of("profile") {
        Some(p) => p,
        None => config_file.default.as_str(),
    };

    println!("selected profile : {}", profile_name);

    let config: &Config = load_config(&config_file, profile_name);

    match cmd.subcommand().unwrap() {
        ("clone", args) => {
            let path: &str = match args.value_of("path") {
                Some(path_provided) => path_provided,
                None => ".",
            };

            let url: &str = match args.value_of("url") {
                Some(url_provided) => url_provided,
                None => {
                    exit_abnormally("no repo url to clone");
                    return;
                }
            };

            let ssh_key_path: &str = match &(config.ssh_key) {
                Some(ssh_key_path) => ssh_key_path.as_str(),
                None => "",
            };

            git::clone_repository(path, url, ssh_key_path);
        }
        _ => unreachable!(),
    }
}

fn load_config<'a>(config_file: &'a ConfigFile, profile_name: &str) -> &'a Config {
    match config_file.profiles.iter().find(|p| p.name == profile_name) {
        Some(p) => &p.config,
        None => {
            exit_abnormally(&format!("cannot find profile {}!", profile_name));
            unreachable!()
        }
    }
}
