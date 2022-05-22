use git2::{build::RepoBuilder, FetchOptions};
use std::{collections::HashMap, path::Path};
use crate::log_utils::exit_abnormally;

const URL_REGEX: &str = r"/(https|.*@.*):(\/\/.+?\/)?(.*).git/gm";


pub fn clone_repository(path: &str, url: &str, path_to_ssh_key: &str) {

    // TODO : Create repo and use ssh key 
    match RepoBuilder::new().fetch_options(FetchOptions::new()).clone(url, &Path::new(path)) {
        Ok(repo) => {},
        Err(err) => exit_abnormally(&format!("could not clone repo"))
    };
}

// fn apply_repository_settings(path: &str, settings: HashMap<String, String>) {

// }