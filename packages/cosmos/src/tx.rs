// use tendermint_rpc::endpoint::broadcast::tx_async::Response as AsyncResponse;
use tendermint_rpc::endpoint::broadcast::tx_commit::Response as CommitResponse;
use tendermint_rpc::{Client, Error, HttpClient};

pub async fn broadcast(client: &HttpClient, signed_tx: Vec<u8>) -> Result<CommitResponse, Error> {
    // client.broadcast_tx_async(signed_tx).await
    client.broadcast_tx_commit(signed_tx).await
}