use tendermint_rpc::endpoint::broadcast::tx_async::Response;
use tendermint_rpc::{Client, Error, HttpClient};

pub async fn broadcast(client: &HttpClient, signed_tx: Vec<u8>) -> Result<Response, Error> {
    client.broadcast_tx_async(signed_tx).await
}
