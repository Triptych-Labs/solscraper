use crate::types::*;
use std::io;

pub async fn get_genesis_transaction(msg: String) -> Result<String, io::Error> {
    let ret = format!("hello {}", msg).to_string();
    Ok(ret)
}

pub async fn start_exhaustion(addresses: Vec<String>) -> Vec<FutureResponse> {
    let mut futures: Vec<FutureResponse> = vec![];
    for address in addresses {
        let future = Box::pin(get_genesis_transaction(format!("{}", address).to_string()));

        futures.push(future);
    }

    futures
}
