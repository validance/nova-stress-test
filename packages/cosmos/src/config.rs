use serde::Deserialize;
use std::fs;
use toml;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub nova: NovaChain,
    pub hosts: Vec<HostChain>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NovaChain {
    pub id: String,
    pub prefix: String,
    pub denom: String,
    pub mnemonic: String,
    pub account_number: u64,
    pub sequence_number: u64,
    pub rpc: String,
    pub timeout_height: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HostChain {
    pub id: String,
    pub ibc_denom: String,
    pub interval: u64,
    pub total_tx: u64,
}

impl Default for Config {
    fn default() -> Self {
        let config_str = fs::read_to_string("config.toml").expect("config file not found");
        toml::from_str(config_str.as_str()).expect("")
    }
}
