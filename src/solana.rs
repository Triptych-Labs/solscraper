use solana_client::rpc_client::RpcClient;

pub async fn rpc_client(url: &str) -> RpcClient {
    RpcClient::new(url.to_string())
}
