use serde::Deserialize;
use std::error::Error;
use std::fmt;
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

#[derive(Debug)]
enum ConfigError {
    Io(io::Error),
    Yaml(serde_yaml::Error),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Io(err) => write!(f, "IO error: {}", err),
            ConfigError::Yaml(err) => write!(f, "YAML error: {}", err),
        }
    }
}

impl Error for ConfigError {}

impl From<io::Error> for ConfigError {
    fn from(err: io::Error) -> ConfigError {
        ConfigError::Io(err)
    }
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(err: serde_yaml::Error) -> ConfigError {
        ConfigError::Yaml(err)
    }
}

impl Config {
    fn from_file(file_path: &str) -> Result<Self, ConfigError> {
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
        Err(e) => eprintln!("Error reading config: {}", e),
    }
}

// Review the code above. While the code is stack-based, it makes the
// happy path (non-error path) heavier than it needs to be and results
// in a lot more code. Remove the `ConfigError` custom error type but
// keep using the "?" operator. Do not re-map errors to strings or
// some other type

// Review questions:

// 1. What the trade offs between the stack-based and heap-based code?
// 2. Which is more idiomatic Rust?
