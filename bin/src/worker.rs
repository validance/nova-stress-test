use cosmos::config::{Config, HostChain, NovaChain};
use cosmos::key::Account;
use cosmos::Error;
use futures::future::{join_all, try_join_all, TryJoinAll};
use futures::{FutureExt, TryFutureExt};
use std::future::Future;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tendermint_rpc::endpoint::broadcast::tx_async::Response;
use tendermint_rpc::{Client, HttpClient, HttpClientUrl, Url};
use tokio::runtime::{Handle, Runtime};
use tokio::task::JoinHandle;

fn spawn_task(
    rt: &Runtime,
    nova_client: &HttpClient,
    account: &Account,
    nova_chain_config: &NovaChain,
    host_chain_config: &HostChain,
    counter: &mut u64,
) {
    for _ in 0..host_chain_config.total_tx {
        let deposit_task = nova::tx::deposit(
            &nova_client,
            &account,
            &host_chain_config,
            &nova_chain_config,
            100000_u128,
            1000000_u64,
            *counter,
            account.get_account_id().unwrap().to_string(),
            account.get_account_id().unwrap().to_string(),
            "1",
        );
        let res = rt.block_on(deposit_task).unwrap();
        println!("{}: {}", &host_chain_config.id, res.hash.to_string());

        *counter += 1;
        std::thread::sleep(Duration::from_millis(host_chain_config.interval))
    }
    println!("next sequence: {counter}");
}

fn get_host_chain_config<'a>(config: &'a Config, chain_id: &'a str) -> Option<&'a HostChain> {
    config.hosts.iter().find(|chain| chain.id.eq(chain_id))
}

pub fn spawn_workers(rt: Runtime) {
    let config = Config::default();
    let nova_chain_config = &config.nova;

    let nova_client = HttpClient::new(Url::from_str(&nova_chain_config.rpc).unwrap()).unwrap();
    let account = Account::new(&nova_chain_config).unwrap();

    let mut counter = nova_chain_config.sequence_number;

    config.hosts.iter().for_each(|host_chain_config| {
        spawn_task(&rt, &nova_client, &account, nova_chain_config, host_chain_config, &mut counter);
    })
}
