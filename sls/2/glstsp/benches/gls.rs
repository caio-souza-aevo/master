use criterion::{criterion_group, criterion_main, Criterion, black_box, BenchmarkId};
use glstsp::load_problem;

fn gls_benchmark(c: &mut Criterion) {
    let gls = load_problem();

    let mut group = c.benchmark_group("PCB3038");
    group.sample_size(10);

    for step in [1, 5, 10].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(format!("gls(666, {})", step)), step, |b, &step| {
            b.iter(|| gls.solve(black_box(666), black_box(step)))
        });
    }

    group.finish();
}

criterion_group!(benches, gls_benchmark);
criterion_main!(benches);
