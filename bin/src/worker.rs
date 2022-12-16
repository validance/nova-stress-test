use cosmos::config::{Config, HostChain, NovaChain};
use cosmos::key::Account;
use std::str::FromStr;
use std::time::Duration;
use tendermint_rpc::{HttpClient, Url};
use tokio::runtime::Runtime;

const TX_TYPES: u64 = 2;

fn fallback(
    chain_id: &str,
    hash: &str,
    sequence_number: &mut u64,
    tx_number: &mut u64,
    total_tx: u64,
    interval: u64,
) {
    println!("{}: {} {}/{}", chain_id, hash, tx_number, total_tx);

    *sequence_number += 1;
    *tx_number += 1;
    std::thread::sleep(Duration::from_millis(interval))
}

fn spawn_task(
    rt: &Runtime,
    nova_client: &HttpClient,
    account: &Account,
    nova_chain_config: &NovaChain,
    host_chain_config: &HostChain,
    sequence_number: &mut u64,
    total_tx: u64,
    tx_number: &mut u64,
) {
    for _ in 0..host_chain_config.num_per_tx {
        let deposit_task = nova::tx::deposit(
            nova_client,
            account,
            host_chain_config,
            nova_chain_config,
            *sequence_number,
            account.get_account_id().unwrap().to_string(),
            account.get_account_id().unwrap().to_string(),
            "1",
        );

        let res = rt.block_on(deposit_task).unwrap();

        fallback(
            &host_chain_config.id,
            &res.hash.to_string(),
            sequence_number,
            tx_number,
            total_tx,
            host_chain_config.interval,
        );

        // let pending_undelegate_task = nova::tx::pending_undelegate(
        //     nova_client,
        //     account,
        //     host_chain_config,
        //     nova_chain_config,
        //     *sequence_number,
        //     account.get_account_id().unwrap().to_string(),
        //     account.get_account_id().unwrap().to_string(),
        //     "1000000000000",
        // );
        //
        // let res = rt.block_on(pending_undelegate_task).unwrap();
        //
        // fallback(
        //     &host_chain_config.id,
        //     &res.hash.to_string(),
        //     &res.deliver_tx.log,
        //     sequence_number,
        //     tx_number,
        //     total_tx,
        //     host_chain_config.interval,
        // );

        let ibc_transfer_task = nova::tx::ibc_transfer(
            nova_client,
            account,
            host_chain_config,
            nova_chain_config,
            *sequence_number,
        );

        let res = rt.block_on(ibc_transfer_task).unwrap();

        fallback(
            &host_chain_config.id,
            &res.hash.to_string(),
            sequence_number,
            tx_number,
            total_tx,
            host_chain_config.interval,
        );
    }
}

pub fn spawn_workers(config_dir: Option<String>, rt: Runtime) {
    let config = match config_dir {
        Some(config_dir) => Config::new(&config_dir),
        None => Config::default(),
    };

    let nova_chain_config = &config.nova;

    let total_tx: u64 = config.hosts.iter().map(|c| c.num_per_tx).sum();
    let mut tx_number: u64 = 1;

    let nova_client = HttpClient::new(Url::from_str(&nova_chain_config.rpc).unwrap()).unwrap();
    let mut account = Account::new(nova_chain_config).unwrap();

    let account_numbers = rt
        .block_on(cosmos::query::account(&account, &nova_chain_config.rest))
        .unwrap();
    account.set_account_number(account_numbers.account_number.parse().unwrap());

    let mut sequence_number: u64 = account_numbers.sequence.parse().unwrap();

    config.hosts.iter().for_each(|host_chain_config| {
        spawn_task(
            &rt,
            &nova_client,
            &account,
            nova_chain_config,
            host_chain_config,
            &mut sequence_number,
            total_tx * TX_TYPES,
            &mut tx_number,
        );
    });

    println!("next sequence: {sequence_number}");
}
