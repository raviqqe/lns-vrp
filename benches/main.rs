use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use vrp::solve::DynamicProgrammingSolver;

fn delivery(bencher: &mut Bencher) {
    let solver = DynamicProgrammingSolver::new();
    bencher.iter(|| {
        let mut map = HashMap::new();

        for key in &keys {
            map = map.clone();

            map.insert(key, key);
        }
    });
}

fn benchmark(criterion: &mut Criterion) {
    criterion.bench_function("delivery", delivery);
}

criterion_group!(benchmark_group, benchmark);
criterion_main!(benchmark_group);
