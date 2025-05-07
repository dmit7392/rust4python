use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    r#type: String,
    host: String,
    port: u16,
    username: String,
    password: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Settings {
    pool_size: Option<u32>,
    timeout: Option<u32>,
    sslmode: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    database: DatabaseConfig,
    settings: Option<Settings>,
}

impl Config {
    fn from_file(file_path: &str) -> Result<Self, Box<dyn Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Self = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}

fn main() {
    match Config::from_file("config.yml") {
        Ok(config) => println!("{:#?}", config),
        Err(e) => {
            // downcast e to io:Error
            if let Some(io_err) = e.downcast_ref::<io::Error>() {
                eprintln!("IO error: {}", io_err);
            }

            // downcast e to serde_yaml::Error
            if let Some(yaml_err) = e.downcast_ref::<serde_yaml::Error>() {
                eprintln!("YAML error: {}", yaml_err);
            }

            eprintln!("Error reading config: {}", e);
        }
    }
}
