use crate::cosmos::base::v1beta1::Coin;
use crate::nova::gal::v1::{MsgClaimSnAsset, MsgDeposit, MsgPendingUndelegate, MsgWithdraw};
use crate::AnyMsg;

use cosmos::config::{HostChain, NovaChain};
use cosmos::key::Account;
use cosmos::tx::broadcast as tx_broadcast;
use cosmos::Error;
use cosmrs::tendermint::block::Height;
use cosmrs::tx::Body;
use prost_types::Any;
use tendermint_rpc::endpoint::broadcast::tx_async::Response;
use tendermint_rpc::HttpClient;

async fn sign_and_broadcast(
    client: &HttpClient,
    msg: Vec<Any>,
    account: &Account,
    nova_chain: &NovaChain,
    sequence_number: u64,
    fee_amount: u128,
    gas_limit: u64,
) -> Result<Response, Error> {
    let signed_tx = account.sign(
        nova_chain.id.parse().map_err(Error::TendermintError)?,
        Body::new(msg, "", Height::from(nova_chain.timeout_height)),
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
    host_chain: &HostChain,
    nova_chain: &NovaChain,
    sequence_number: u64,
    depositor: String,
    claimer: String,
    amount: impl ToString,
) -> Result<Response, Error> {
    let msg_deposit = MsgDeposit {
        zone_id: host_chain.id.clone(),
        depositor,
        claimer,
        amount: Some(Coin {
            denom: host_chain.ibc_denom.to_string(),
            amount: amount.to_string(),
        }),
    };

    let any_msg = msg_deposit.try_to_any("/nova.gal.v1.MsgDeposit")?;
    sign_and_broadcast(
        client,
        vec![any_msg],
        account,
        nova_chain,
        sequence_number,
        nova_chain.fee_amount.into(),
        nova_chain.gas_limit,
    )
    .await
}

pub async fn pending_undelegate(
    client: &HttpClient,
    account: &Account,
    host_chain: &HostChain,
    nova_chain: &NovaChain,
    sequence_number: u64,
    delegator: String,
    withdrawer: String,
    amount: impl ToString,
) -> Result<Response, Error> {
    let msg_pending_undelegate = MsgPendingUndelegate {
        zone_id: host_chain.id.clone(),
        delegator,
        withdrawer,
        amount: Some(Coin {
            denom: host_chain.sn_denom.clone(),
            amount: amount.to_string(),
        }),
    };

    let any_msg = msg_pending_undelegate.try_to_any("/nova.gal.v1.MsgPendingUndelegate")?;
    sign_and_broadcast(
        client,
        vec![any_msg],
        account,
        nova_chain,
        sequence_number,
        nova_chain.fee_amount.into(),
        nova_chain.gas_limit,
    )
    .await
}

pub async fn withdraw(
    client: &HttpClient,
    account: &Account,
    host_chain: &HostChain,
    nova_chain: &NovaChain,
    sequence_number: u64,
    withdrawer: String,
) -> Result<Response, Error> {
    let msg_withdraw = MsgWithdraw {
        zone_id: host_chain.id.clone(),
        withdrawer,
    };

    let any_msg = msg_withdraw.try_to_any("/nova.gal.v1.MsgWithdraw")?;
    sign_and_broadcast(
        client,
        vec![any_msg],
        account,
        nova_chain,
        sequence_number,
        nova_chain.fee_amount.into(),
        nova_chain.gas_limit,
    )
    .await
}

pub async fn claim_sn_asset(
    client: &HttpClient,
    account: &Account,
    host_chain: &HostChain,
    nova_chain: &NovaChain,
    sequence_number: u64,
    claimer: String,
) -> Result<Response, Error> {
    let msg_claim_sn_asset = MsgClaimSnAsset {
        zone_id: host_chain.id.clone(),
        claimer,
    };

    let any_msg = msg_claim_sn_asset.try_to_any("/nova.gal.v1.MsgClaimSnAsset")?;
    sign_and_broadcast(
        client,
        vec![any_msg],
        account,
        nova_chain,
        sequence_number,
        nova_chain.fee_amount.into(),
        nova_chain.gas_limit,
    )
    .await
}

impl AnyMsg for MsgDeposit {}
impl AnyMsg for MsgPendingUndelegate {}
impl AnyMsg for MsgWithdraw {}
impl AnyMsg for MsgClaimSnAsset {}
