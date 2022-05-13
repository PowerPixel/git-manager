use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigProfiles {
    #[serde(flatten)]
    pub profile: HashMap<String, Profile>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub ssh_key: String,
    pub config: Config
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub ssh_key: String,
    pub comment: String,
    pub user: UserConfig
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserConfig {
    pub name: String,
    pub email: String
}
