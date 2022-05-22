use git2::build::RepoBuilder;
use std::{collections::HashMap, path::Path};

use crate::cli::exit_abnormally;
fn clone_repository(path: &str, url: &str, path_to_ssh_key: &str) {
    match RepoBuilder::new().clone(url, &Path::new(path)) {
        Ok(repo) => {},
        Err(err) => exit_abnormally("")
    };
}

// fn apply_repository_settings(path: &str, settings: HashMap<String, String>) {

// }