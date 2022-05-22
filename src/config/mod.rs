use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigFile {
    pub default: String,
    pub profiles: Vec<Profile>,
}

impl ConfigFile {
    pub fn read<R: Read>(reader: R) -> serde_json::Result<ConfigFile> {
        let config_file = serde_json::from_reader(reader)?;
        serde_json::Result::Ok(config_file)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub name: String,
    pub config: Config,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub ssh_key: Option<String>,
    pub comment: Option<String>,
    pub user: Option<UserConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserConfig {
    pub name: Option<String>,
    pub email: Option<String>,
}
