use cosmos::config::{Chain, Config};
use cosmos::key::Account;
use cosmos::Error;
use futures::future::{join_all, try_join_all, TryJoinAll};
use futures::{FutureExt, TryFutureExt};
use std::future::Future;
use std::str::FromStr;
use std::sync::Arc;
use tendermint_rpc::endpoint::broadcast::tx_async::Response;
use tendermint_rpc::{Client, HttpClient, HttpClientUrl, Url};
use tokio::runtime::{Handle, Runtime};
use tokio::task::JoinHandle;

fn get_chain_config<'a>(config: &'a Config, chain_id: &'a str) -> Option<&'a Chain> {
    config.chains.iter().find(|chain| chain.id.eq(chain_id))
}

pub fn spawn_workers(rt: Runtime) {
    let config = Config::default();

    if let Some(nova_config) = get_chain_config(&config, "nova") {
        let nova_client = HttpClient::new(Url::from_str(&nova_config.rpc).unwrap()).unwrap();
        let account = Account::new(&nova_config).unwrap();

        for i in 0..=0 {
            let nova_task = nova::tx::deposit(
                &nova_client,
                &account,
                &nova_config,
                100000_u128,
                1000000_u64,
                nova_config.sequence_number + i,
                account.get_account_id().unwrap().to_string(),
                account.get_account_id().unwrap().to_string(),
                "1000",
            );
            let res = rt.block_on(nova_task).unwrap();
            println!("{:?}", res);
        }
    }
}
