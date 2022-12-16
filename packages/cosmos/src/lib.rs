use cosmrs::proto::prost;

pub mod config;
pub mod key;
pub mod query;
pub mod tx;

#[derive(Debug)]
pub enum Error {
    AnyError(anyhow::Error),
    Bip32Error(cosmrs::bip32::Error),
    ErrorReport(cosmrs::ErrorReport),
    TendermintError(cosmrs::tendermint::Error),
    TendermintRpcError(tendermint_rpc::Error),
    ProstEncodeError(prost::EncodeError),
    ReqwestError(reqwest::Error),
    ParseIntError(std::num::ParseIntError),
    TryFromIntError(std::num::TryFromIntError),
}
