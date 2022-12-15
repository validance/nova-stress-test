use cosmos::config::{Config, HostChain};
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

fn get_host_chain_config<'a>(config: &'a Config, chain_id: &'a str) -> Option<&'a HostChain> {
    config.hosts.iter().find(|chain| chain.id.eq(chain_id))
}

pub fn spawn_workers(rt: Runtime) {
    let config = Config::default();
    let nova_chain_config = &config.nova;

    if let Some(host_chain_config) = get_host_chain_config(&config, "gaia") {
        let nova_client = HttpClient::new(Url::from_str(&nova_chain_config.rpc).unwrap()).unwrap();
        let account = Account::new(&nova_chain_config).unwrap();

        let mut counter = nova_chain_config.sequence_number;

        for _ in 0..10 {
            let deposit_task = nova::tx::deposit(
                &nova_client,
                &account,
                &host_chain_config,
                &nova_chain_config,
                100000_u128,
                1000000_u64,
                counter,
                account.get_account_id().unwrap().to_string(),
                account.get_account_id().unwrap().to_string(),
                "1",
            );
            let res = rt.block_on(deposit_task).unwrap();
            println!("{:?}", res);

            counter += 1;
        }
        println!("{counter}");
    }
}
