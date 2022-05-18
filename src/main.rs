mod config;
use config::Config;

use clap::{Arg, Command};

const DEFAULT_CONFIG_PATH: &str = "~/.config/git-manager";

fn main() {
    let cmd = Command::new("git-manager")
        .version("0.1.0")
        .arg(
            Arg::new("profile")
                .help("Name of the profile to use")
                .required(false)
                .short('p')
                .long("profile"),
        )
        .subcommand_required(true)
        .subcommand(
            Command::new("clone")
                .about("Clone a new git repository")
                .arg(
                    Arg::new("path")
                        .help("Path to the directory of the repository")
                        .required(false)
                        .index(2),
                )
                .arg(
                    Arg::new("url")
                        .help("URL of the repository to clone")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    let mut profile: Option<&str> = None;

    if cmd.is_present(1) {
        profile = Option::Some(cmd.value_of(1).unwrap());
    }

    match cmd.subcommand().unwrap() {
        ("clone", args) => {
            let path: Option<&str> = args.value_of("path");

            let url: &str = args.value_of("url").unwrap();
        }
        _ => !unreachable!(),
    }
}

fn clone_repository(path: Option<&str>) -> () {
    
}