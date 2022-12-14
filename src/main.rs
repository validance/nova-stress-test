use cosmrs::tendermint::block::Height;
use cosmrs::tx::{Body, Msg};
use cosmrs::{AccountId, Coin};
use nova_load_test::config::Config;
use nova_load_test::key::Account;
use nova_load_test::tx::broadcast;
use std::str::FromStr;
use tendermint_rpc::HttpClient;

#[tokio::main]
async fn main() {
    let client = HttpClient::new("http://localhost:26657").unwrap();

    let chain_config = Config::default();
    let target = chain_config.chains.get(0).unwrap();
    let account = Account::new(
        &target.mnemonic,
        &target.prefix,
        &target.denom,
        target.account_number,
    )
    .unwrap();

    // TODO: Create another method to build txs
    let any_tx = cosmrs::bank::MsgSend {
        from_address: account.get_account_id().unwrap(),
        to_address: AccountId::from_str("cosmos1hnydq0uty6us260mw4vap4ks7l6kendz5fzmlc").unwrap(),
        amount: vec![Coin::new(6789_u128, &target.denom).unwrap()],
    }
    .into_any()
    .unwrap();

    let signed_tx = account
        .sign(
            target.id.parse().unwrap(),
            Body::new(vec![any_tx], "", Height::from(1000u32)),
            target.sequence_number,
            300000,
            100000,
        )
        .unwrap();

    let res = broadcast(&client, signed_tx).await.unwrap();
    res.deliver_tx
        .events
        .into_iter()
        .for_each(|i| println!("{:?}", i));
}
