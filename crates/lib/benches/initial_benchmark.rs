use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_cli_lib::{async_fibonacci, sync_fibonacci};

fn criterion_benchmark(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
    c.bench_function("async fib 20", |b| {
        b.to_async(&rt).iter(|| async {
            async_fibonacci(black_box(20)).await;
        });
    });
    c.bench_function("fib 20", |b| b.iter(|| sync_fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
