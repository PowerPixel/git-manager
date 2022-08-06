use crate::log_utils::exit_abnormally;
use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks};
use std::{borrow::Borrow, collections::HashMap, path::Path};

const URL_REGEX: &str = r"/(https|.*@.*):(\/\/.+?\/)?(.*).git/gm";

pub fn clone_repository(path: &str, url: &str, path_to_ssh_key: &str) {
    let mut fetch_option = FetchOptions::new();
    let mut remote_callback = RemoteCallbacks::new();
    println!("DEBUG : {}", path_to_ssh_key);
    remote_callback.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(shellexpand::tilde(path_to_ssh_key).into_owned().as_str())
                .to_owned()
                .as_path(),
            None,
        )
    });
    fetch_option.remote_callbacks(remote_callback);
    match RepoBuilder::new()
        .fetch_options(fetch_option)
        .bare(true)
        .clone(url, &Path::new(path))
    {
        Ok(_) => {
            println!("Successfully cloned the repo !");
        }
        Err(err) => exit_abnormally(&format!(
            "could not clone repo {} : {}",
            url,
            err.message()
        )),
    };
}

// fn apply_repository_settings(path: &str, settings: HashMap<String, String>) {

// }
