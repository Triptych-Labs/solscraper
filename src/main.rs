use futures::executor::block_on;
use futures::future::join_all;
use futures::{Future, StreamExt};
use std::pin::Pin;

pub mod exhaust_addresses;
pub mod scraper;
pub mod solana;

type HelloResponseFuture = dyn Future<Output = String>;
type HelloResponse = Pin<Box<HelloResponseFuture>>;

pub async fn say_hello(msg: String) -> String {
    let ret = format!("hello {}", msg).to_string();
    ret
}

async fn executor() {
    let mut futures: Vec<HelloResponse> = vec![];
    for i in 0..=19 {
        futures.push(Box::pin(say_hello(format!("{}", i).to_string())));
    }

    let stream = futures::stream::iter(futures).buffer_unordered(5);
    let results = stream.collect::<Vec<_>>().await;
    println!("{:?}", results);
}

fn main() {
    block_on(executor());
}
