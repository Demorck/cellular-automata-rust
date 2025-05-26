use criterion::{criterion_group, criterion_main, Criterion};
use cellular_automaton::automaton::Automaton;
use cellular_automaton::cell::Cell;
use cellular_automaton::row::Row;
use cellular_automaton::rules::{Rule, WolframRule};

fn automaton_benchmark(c: &mut Criterion) {
    let mut config = vec![Cell::new(0); 201];
    config[50] = Cell::new(1);

    let row = Row::new(config);

    c.bench_function("evolve 100 steps", |b| {
        b.iter(|| {
            let rule = Box::new(WolframRule::new(30));
            let mut automaton = Automaton::new(row.clone(), rule);
            automaton.evolve(100);
        });
    });
}

criterion_group!(benches, automaton_benchmark);
criterion_main!(benches);
