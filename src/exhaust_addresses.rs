use crate::scraper::exhaust_transaction_signatures_for_address::get_genesis_transaction;
use crate::types::*;

pub async fn start_exhaustion(network: String) -> Vec<FutureResponse> {
    let mut futures: Vec<FutureResponse> = vec![];

    let addresses: Vec<String> = vec![
        "8qwreJRtNaR1GA5odbV6tzyxwnTu2TStaRRZ7hqy1w4k".to_string(),
        // "3wyvezHryrDnCefZi9dEyBevaLutf6vz69rzaxfxQ3Gv".to_string(),
    ];
    for address in addresses {
        let future = Box::pin(get_genesis_transaction(network.clone(), address));

        futures.push(future);
    }

    return futures;
}
