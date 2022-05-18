use serde::{Deserialize, Serialize};

pub struct GlobalConfig {
    pub default: String,
    pub profiles: Vec<Profile>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub ssh_key: String,
    pub config: Config,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub ssh_key: String,
    pub comment: String,
    pub user: UserConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserConfig {
    pub name: String,
    pub email: String,
}
