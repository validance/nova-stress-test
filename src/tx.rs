use tendermint_rpc::{Client, Error, HttpClient};
// use tendermint_rpc::endpoint::broadcast::tx_async::Response as AsyncResponse;
use tendermint_rpc::endpoint::broadcast::tx_commit::Response as CommitResponse;

pub async fn broadcast(client: &HttpClient, signed_tx: Vec<u8>) -> Result<CommitResponse, Error> {
    client.broadcast_tx_commit(signed_tx).await
}
