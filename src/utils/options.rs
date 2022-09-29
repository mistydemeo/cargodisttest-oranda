use std::fs::{self};
use std::path::Path;

use toml::Value;
use twelf::config;
pub struct Downloads {}

// enum Theme {
//     Light,
//     Dark,
// }

#[config]
#[derive(Debug, Eq, PartialEq)]
pub struct Options {
    // Your Readme.md name
    pub file: Option<String>,
    pub dist: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    // pub logo: String,
    // pub shareCard: String,
    // pub homepage: String,
    pub no_header: Option<bool>,
    // pub theme: Theme,
}

pub struct FilledOptions {
    // Your Readme.md name
    pub file: String,
    pub dist: String,
    pub name: String,
    pub description: String,
    // pub logo: String,
    // pub shareCard: String,
    // pub homepage: String,
    pub no_header: bool,
    // pub theme: Theme,
}

impl Default for Options {
    fn default() -> Self {
        let mut name = String::new();
        let mut description = String::new();
        let cargo_file = "Cargo.toml";
        if Path::new(cargo_file).exists() {
            let file = fs::read_to_string(cargo_file).unwrap();
            let value = file.parse::<Value>().unwrap();
            name = value["package"]["name"].to_string();
            description = value["package"]["description"].to_string();
        };

        Options {
            file: Some("Readme".to_string()),
            dist: Some("public".to_string()),
            no_header: Some(false),
            name: Some(name),
            description: Some(description),
        }
    }
}
