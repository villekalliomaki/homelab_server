use std::{env::current_dir, fs::read_to_string, process::exit};

use log::{error, info};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub backups: Vec<Backup>,
    pub email_settings: EmailSettings,
}

#[derive(Deserialize)]
pub struct EmailSettings {
    pub from: String,
    pub to: String,
    pub host: String,
    pub port: u64,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Backup {
    pub name: String,
    // Comma separated list
    pub source_paths: String,
    pub target_repo: String,
    pub repo_password: String,
}

impl Config {
    pub fn from_file(path: &str) -> Config {
        info!("Building config from file {path}");

        match toml::from_str(&get_file_content(path)) {
            Ok(c) => c,
            Err(error) => {
                error!("Failed to build configuration: {error}");
                exit(1);
            }
        }
    }
}

fn get_file_content(path: &str) -> String {
    let dir = match current_dir() {
        Ok(v) => v,
        Err(error) => {
            error!("Failed to get current working directory: {error}");
            exit(1);
        }
    };

    match read_to_string(path) {
        Ok(s) => s,
        Err(error) => {
            error!(
                "Failed to read config file: {} in {}, {}",
                path,
                dir.display(),
                error
            );
            exit(1);
        }
    }
}
