use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ff::PrimeField;
use pairing_bn256::bn256;
use crate::bn256::Fr;

fn criterion_benchmark(c: &mut Criterion) {
    let lhs = Fr::from_raw([
        0x5b5f898e5e963f25,
        0x64ec26aad4c86e71,
        0x09226b6e22c6f0ca,
        0x870e56bbe533e9a2,
    ]);
    let rhs = Fr::from_raw([
        0x64ec26aad4c86e71,
        0x09226b6e22c6f0ca,
        0x870e56bbe533e9a2,
        0x5b5f898e5e963f25,
    ]);
    //let expr = [12345678u64, 87654321u64, 21436587u64, 78563412u64, 0u64, 0u64, 0u64, 0u64]
    //c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("add", |b| b.iter(|| lhs.add(&rhs)));
    c.bench_function("sub", |b| b.iter(|| lhs.sub(&rhs)));
    c.bench_function("neg", |b| b.iter(|| lhs.neg()));
    c.bench_function("square", |b| b.iter(|| lhs.square()));
    c.bench_function("double", |b| b.iter(|| lhs.double()));
    c.bench_function("mul", |b| b.iter(|| lhs.mul(&rhs)));
    c.bench_function("montgomery_reduce", |b| b.iter(|| lhs.to_repr()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);