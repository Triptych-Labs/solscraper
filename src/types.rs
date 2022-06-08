use futures::Future;
use std::io;
use std::pin::Pin;

pub type FutureResponse = Pin<Box<dyn Future<Output = Result<String, io::Error>>>>;
