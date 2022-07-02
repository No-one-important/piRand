use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[allow(dead_code)]
fn gcd(a: u64, b: u64) -> u64 {
    if b != 0 {
        return gcd(b, a % b);
    }

    a
}

fn gcd_loop(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("gcd", |b| b.iter(|| gcd_loop(black_box(324987), black_box(987654))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);