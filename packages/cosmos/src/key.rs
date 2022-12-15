use crate::Error;

use crate::config::{HostChain, NovaChain};
use bip39::{Language, Mnemonic, Seed};
use cosmrs::bip32::DerivationPath;
use cosmrs::crypto::secp256k1::SigningKey;
use cosmrs::tendermint::chain::Id;
use cosmrs::tx::{Body, Fee, SignDoc, SignerInfo};
use cosmrs::{AccountId, Coin, Denom};
use std::str::FromStr;

pub struct Account {
    signing_key: SigningKey,
    prefix: String,
    denom: String,
    account_number: u64,
}

impl Account {
    pub fn new(chain_config: &NovaChain) -> Result<Self, Error> {
        let mnemonic = Mnemonic::from_phrase(&chain_config.mnemonic, Language::English)
            .map_err(Error::AnyError)?;
        let seed = Seed::new(&mnemonic, "");
        let hd_path = DerivationPath::from_str("m/44'/118'/0'/0/0").map_err(Error::Bip32Error)?;
        let signing_key =
            SigningKey::derive_from_path(seed, &hd_path).map_err(Error::Bip32Error)?;

        Ok(Self {
            signing_key,
            prefix: chain_config.prefix.to_string(),
            denom: chain_config.denom.to_string(),
            account_number: chain_config.account_number,
        })
    }

    pub fn sign(
        &self,
        chain_id: Id,
        tx_body: Body,
        sequence_number: u64,
        fee_amount: u128,
        gas_limit: u64,
    ) -> Result<Vec<u8>, Error> {
        let public_key = self.signing_key.public_key();
        let signer_info = SignerInfo::single_direct(Some(public_key), sequence_number);
        let auth_info = signer_info.auth_info(Fee::from_amount_and_gas(
            Coin {
                denom: Denom::from_str(&self.denom).map_err(Error::ErrorReport)?,
                amount: fee_amount,
            },
            gas_limit,
        ));

        let sign_doc = SignDoc::new(&tx_body, &auth_info, &chain_id, self.account_number)
            .map_err(Error::ErrorReport)?;

        let tx_signed = sign_doc
            .sign(&self.signing_key)
            .map_err(Error::ErrorReport)?;

        tx_signed.to_bytes().map_err(Error::ErrorReport)
    }

    pub fn get_account_id(&self) -> Result<AccountId, Error> {
        self.signing_key
            .public_key()
            .account_id(&self.prefix)
            .map_err(Error::ErrorReport)
    }
}
