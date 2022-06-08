use futures::StreamExt;
use std::io::Result;

use crate::types::*;

use crate::exhaust_addresses::start_exhaustion;

pub mod exhaust_addresses;
pub mod scraper;
pub mod solana;
pub mod types;

#[tokio::main]
async fn main() -> Result<()> {
    let mut futures: Vec<FutureResponse> = vec![];

    futures
        .append(&mut start_exhaustion("https://explorer-api.devnet.solana.com/".to_string()).await);

    // process 5 at a time - batch 5 per
    let stream = futures::stream::iter(futures).buffer_unordered(5);
    let results = stream.collect::<Vec<_>>().await;
    println!("{:?}", results);

    Ok(())
}
