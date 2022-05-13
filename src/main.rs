mod config;
use config::Config;

use clap::{Command, Arg};

const DEFAULT_CONFIG_PATH: &str = "~/.git-manager";

fn main() {
    let cmd = Command::new("git-manager")
                    .version("0.1.0")
                    .arg(
                        Arg::new("profile")
                            .help("Name of the profile to use")
                            .required(false)
                            .short('p')
                            .long("profile")
                    )
                    .subcommand_required(true)
                    .subcommand(
                        Command::new("clone")
                            .about("Clone a new git repository")
                            .arg(
                                Arg::new("path")
                                .help("Path to the directory of the repository")
                                .required(false)
                                .index(2)
                            )
                            .arg(
                                Arg::new("url")
                                .help("URL of the repository to clone")
                                .required(true)
                                .index(1)
                            )
                    ).get_matches();

        // Default profile dir, maybe we can add in the future the possibility to use a specific profile file ?
        let profile_dir: &str = "~/.config/git-manager";

        let profile: Option<&str> = None;

        if cmd.is_present(1) {
            profile = Option::Some(cmd.value_of(1).unwrap());
        }


        match cmd.subcommand().unwrap() {
            ("clone", args) => {
                
                
            },
            _ => !unreachable!(),
        }
}