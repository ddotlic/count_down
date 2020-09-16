extern crate criterion;

use count_down::count_down;
use criterion::*;
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("CountDown", |b| {
        b.iter(|| count_down::solutions(vec![1, 1, 4, 7, 15, 50], 522))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(10)
        .warm_up_time(Duration::from_secs(4))
        .measurement_time(Duration::from_secs(8));
    targets = criterion_benchmark
}
criterion_main!(benches);
