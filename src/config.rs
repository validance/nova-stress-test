use serde::Deserialize;
use std::fs;
use toml;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub chains: Vec<Chain>,
}

#[derive(Debug, Deserialize)]
pub struct Chain {
    pub id: String,
    pub prefix: String,
    pub denom: String,
    pub mnemonic: String,
    pub account_number: u64,
    pub sequence_number: u64,
    pub rpc: String,
}

impl Default for Config {
    fn default() -> Self {
        let config_str = fs::read_to_string("config.toml").expect("config file not found");
        toml::from_str(config_str.as_str()).expect("")
    }
}
