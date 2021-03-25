use criterion::{criterion_group, criterion_main, Criterion, black_box};
use glstsp::types::graph::Graph;
use glstsp::load_data;

fn criterion_benchmark(c: &mut Criterion) {
    let tsp = load_data();
    let graph = Graph::new(&tsp);

    c.bench_function("Local Search PCB3038", |b| b.iter(|| {
        graph.gls(black_box(666))
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);