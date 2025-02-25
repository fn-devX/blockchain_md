use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub blockchain: BlockchainConfig,
}

#[derive(Deserialize)]
pub struct NetworkConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct BlockchainConfig {
    pub difficulty: u8,
}

impl Config {
    pub fn load() -> Self {
        let config_str = fs::read_to_string("config.toml").expect("Failed to read config file");
        toml::from_str(&config_str).expect("Failed to parse config")
    }
}
