extern crate criterion;

use count_down::count_down;
use criterion::*;
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("CountDown", |b| {
        b.iter(|| count_down::solutions(vec![7,3,4,5,15,75], 785))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .sample_size(10)
        .warm_up_time(Duration::from_secs(2))
        .measurement_time(Duration::from_secs(3));
    targets = criterion_benchmark
}
criterion_main!(benches);
