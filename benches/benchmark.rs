use criterion::{criterion_group, criterion_main, Criterion, black_box};
use seive_of_eratosthenes::{find_primes};

fn benchmark_functions(c: &mut Criterion) {
    let input_vector_smart: Vec<i32> = (2..=10_0000).collect();
    c.bench_function("find_primes_smart", |b| {
        b.iter(|| find_primes(black_box(&mut input_vector_smart.clone()), &Default::default()))
    });
}

criterion_group!(benches, benchmark_functions);
criterion_main!(benches);
