use crate::scraper::exhaust_transaction_signatures_for_address::decode_transfer_instruction;
use crate::scraper::exhaust_transaction_signatures_for_address::get_genesis_transaction;
use crate::types::*;
use std::fs::File;
use std::path::Path;

pub async fn start_exhaustion(network: String) -> Vec<FutureResponse> {
    let mut futures: Vec<FutureResponse> = vec![];
    let path = Path::new("./mint-addresses.json");
    let file = File::open(path).unwrap();

    let addresses: Vec<String> = serde_json::from_reader(file).unwrap();

    for address in addresses {
        let future = Box::pin(get_genesis_transaction(network.clone(), address));

        futures.push(future);
    }
    /*
    let future = Box::pin(decode_transfer_instruction(network.clone()));
    futures.push(future);
    */

    return futures;
}
