use criterion::{criterion_group, criterion_main, Criterion, black_box};
use glstsp::types::graph::Graph;
use glstsp::types::point::Point;

fn criterion_benchmark(c: &mut Criterion) {
    let tsp = include_str!("../data/pcb3038.preprocessed.tsp");
    let tsp: Vec<_> = tsp
        .lines()
        .map(Point::from)
        .collect();

    c.bench_function("Graph new PCB3038", |b| b.iter(|| {
        let graph = Graph::new(&tsp);
        black_box(graph[(1, 2)]);
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);