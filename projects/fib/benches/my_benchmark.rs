use criterion::{Criterion, criterion_group, criterion_main};
use fib::fib_recursive;
use std::hint::black_box;


fn bench_fib1(c: &mut Criterion) {
    c.bench_function("fib_recursive", 
    |b|
        b.iter(|| fib_recursive(black_box(20))));
}

criterion_group!(benches, bench_fib1);
criterion_main!(benches);