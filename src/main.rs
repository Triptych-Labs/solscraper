use std::io;

use futures::executor::block_on;
use futures::future::join_all;
use futures::Future;
use std::pin::Pin;

type HelloResponseFuture = dyn Future<Output = String>;
type HelloResponse = Pin<Box<HelloResponseFuture>>;

pub async fn say_hello(msg: String) -> String {
    let ret = format!("hello {}", msg).to_string();
    ret
}

fn executor() {
    let mut futures: Vec<HelloResponse> = vec![];
    futures.push(Box::pin(say_hello("".to_string())));

    block_on(join_all(futures));
}

fn main() {
    executor();
    println!("Hello, world!");
}
