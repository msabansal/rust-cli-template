
use std::pin::Pin;
use std::future::Future;

#[path = "util/tracing.rs"]
pub mod tracing;

pub fn async_fibonacci(n: u64) -> Pin<Box<dyn Future<Output = u64>>> {
    Box::pin(async move {
        match n {
            0 => 1,
            1 => 1,
            n => async_fibonacci(n - 1).await + async_fibonacci(n - 2).await,
        }
    })
}
pub fn sync_fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => sync_fibonacci(n-1) + sync_fibonacci(n-2),
    }
}