use criterion::{criterion_group, criterion_main, Criterion, black_box};
use glstsp::load_problem;

fn criterion_benchmark(c: &mut Criterion) {
    let gls = load_problem();

    c.bench_function("Local Search PCB3038", |b| b.iter(|| {
        gls.solve(black_box(666))
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);