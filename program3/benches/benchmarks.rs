use criterion::{criterion_group, criterion_main, Criterion};
use program3::{ mergesort, heapsort };

mod gendata;

fn mergesort_benchmark(c: &mut Criterion) {
    c.bench_function("mergesort", |b| b.iter(|| mergesort(gendata::generate_random_list(100))));
}

fn heapsort_benchmark(c: &mut Criterion) {
    c.bench_function("heapsort", |b| b.iter(|| heapsort(gendata::generate_random_list(100))));
}

criterion_group!(benches, mergesort_benchmark, heapsort_benchmark);
criterion_main!(benches);