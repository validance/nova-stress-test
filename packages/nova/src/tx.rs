use crate::cosmos::base::v1beta1::Coin;
use crate::nova::gal::v1::MsgDeposit;

use crate::AnyMsg;
use cosmos::config::Chain;
use cosmos::key::Account;
use cosmos::tx::broadcast as tx_broadcast;
use cosmos::Error;
use cosmrs::tendermint::block::Height;
use cosmrs::tx::Body;
use prost;
use prost_types::Any;
use tendermint_rpc::endpoint::broadcast::tx_async::Response;
use tendermint_rpc::HttpClient;

async fn sign_and_broadcast(
    client: &HttpClient,
    msg: Vec<Any>,
    account: &Account,
    chain: &Chain,
    sequence_number: u64,
    fee_amount: u128,
    gas_limit: u64,
) -> Result<Response, Error> {
    let signed_tx = account.sign(
        chain.id.parse().map_err(Error::TendermintError)?,
        Body::new(msg, "", Height::from(chain.target_height)),
        sequence_number,
        fee_amount,
        gas_limit,
    )?;

    tx_broadcast(client, signed_tx)
        .await
        .map_err(Error::TendermintRpcError)
}

/// Zone id in chain config
pub async fn deposit(
    client: &HttpClient,
    account: &Account,
    chain: &Chain,
    depositor: String,
    claimer: String,
    amount: impl ToString,
    sequence_number: u64,
    fee_amount: u128,
    gas_limit: u64,
) -> Result<Response, Error> {
    let msg_deposit = MsgDeposit {
        zone_id: chain.zone_id.clone(),
        depositor,
        claimer,
        amount: Some(Coin {
            denom: chain.denom.clone(),
            amount: amount.to_string(),
        }),
    };

    let any_msg = msg_deposit.try_to_any("/nova.gal.v1.Msg/Deposit")?;
    sign_and_broadcast(
        client,
        vec![any_msg],
        account,
        chain,
        sequence_number,
        fee_amount,
        gas_limit,
    ).await
}

impl AnyMsg for MsgDeposit {
    fn try_to_any(&self, type_url: &str) -> Result<Any, Error> {
        let mut buf = Vec::new();
        prost::Message::encode(self, &mut buf).map_err(Error::ProstEncodeError)?;
        Ok(Any {
            type_url: type_url.to_string(),
            value: buf,
        })
    }
}
