use crate::key::Account;
use crate::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AccountResponse {
    pub account: AccountNumbers,
}

#[derive(Debug, Deserialize)]
pub struct AccountNumbers {
    pub sequence: String,
    pub account_number: String,
}

pub async fn account(account: &Account, base_url: &str) -> Result<AccountNumbers, Error> {
    let path = "cosmos/auth/v1beta1/accounts";
    let account_addr = account.get_account_id().unwrap().to_string();

    let raw = reqwest::get(format!("{}/{}/{}", base_url, path, account_addr))
        .await
        .map_err(Error::ReqwestError)?
        .json::<AccountResponse>()
        .await
        .map_err(Error::ReqwestError)?;

    Ok(raw.account)
}
