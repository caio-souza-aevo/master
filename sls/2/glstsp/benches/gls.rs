use criterion::{criterion_group, criterion_main, Criterion, black_box};
use glstsp::load_problem;

fn gls_benchmark(c: &mut Criterion) {
    let gls = load_problem();

    let mut group = c.benchmark_group("PCB3038");
    group.sample_size(50);
    group.bench_function("Local Search: 50 iterations", |b| b.iter(|| {
        gls.solve(black_box(666), black_box(50))
    }));
    group.finish();
}

criterion_group!(benches, gls_benchmark);
criterion_main!(benches);