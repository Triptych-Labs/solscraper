use futures::executor::block_on;
use futures::StreamExt;

use crate::types::*;

use crate::exhaust_addresses::start_exhaustion;

pub mod exhaust_addresses;
pub mod scraper;
pub mod solana;
pub mod types;

async fn executor() {
    let mut futures: Vec<FutureResponse> = vec![];

    let addresses: Vec<String> = vec!["aasd".to_string(), "sdfgsdfg".to_string()];
    futures.append(&mut start_exhaustion(addresses).await);

    let stream = futures::stream::iter(futures).buffer_unordered(5);
    let results = stream.collect::<Vec<_>>().await;
    println!("{:?}", results);
}

fn main() {
    block_on(executor());
}
