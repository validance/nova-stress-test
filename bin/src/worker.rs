use cosmos::config::{Config, HostChain, NovaChain};
use cosmos::key::Account;
use std::str::FromStr;
use std::time::Duration;
use tendermint_rpc::{HttpClient, Url};
use tokio::runtime::Runtime;

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
            nova_client,
            account,
            host_chain_config,
            nova_chain_config,
            100000_u128,
            1000000_u64,
            *counter,
            account.get_account_id().unwrap().to_string(),
            account.get_account_id().unwrap().to_string(),
            "1",
        );
        let res = rt.block_on(deposit_task).unwrap();
        println!("{}: {}", &host_chain_config.id, res.hash);

        *counter += 1;
        std::thread::sleep(Duration::from_millis(host_chain_config.interval))
    }
}

pub fn spawn_workers(config_dir: Option<String>, rt: Runtime) {
    let config = match config_dir {
        Some(config_dir) => Config::new(&config_dir),
        None => Config::default()
    };

    let nova_chain_config = &config.nova;

    let nova_client = HttpClient::new(Url::from_str(&nova_chain_config.rpc).unwrap()).unwrap();
    let account = Account::new(nova_chain_config).unwrap();

    let mut counter = nova_chain_config.sequence_number;

    config.hosts.iter().for_each(|host_chain_config| {
        spawn_task(
            &rt,
            &nova_client,
            &account,
            nova_chain_config,
            host_chain_config,
            &mut counter,
        );
    });

    println!("next sequence: {counter}");
}
