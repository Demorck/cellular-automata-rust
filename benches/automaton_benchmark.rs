use std::hint::black_box;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use cellular_automaton::automaton::Automaton;
use cellular_automaton::cell::Cell;
use cellular_automaton::diagonal::Fast30;
use cellular_automaton::pattern::Pattern;
use cellular_automaton::row::Row;
use cellular_automaton::rules::{WolframRule};

pub fn automaton_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("AutomatonEvolve");
    group.sample_size(10);
    // &[100, 1_000, 2_000, 3_000, 4_000, 5_000, 10_000, 20_000, 30_000, 40_000, 50_000]
    for &steps in &[100, 1_000, 2_000, 3_000, 5_000, 10_000, 20_000, 30_000, 40_000, 50_000] {
        group.bench_with_input(BenchmarkId::new("Naive", steps), &steps, |b, &s| {
            b.iter(|| {
                let mut config = vec![Cell::new(0); s * 2 + 1];
                config[s] = Cell::new(1);

                let row = Row::new(config);
                let rule = Box::new(WolframRule::new(30));
                let mut automaton = Automaton::new(row.clone(), rule);
                automaton.evolve(black_box(s as u64));
            });
        });

        group.bench_with_input(BenchmarkId::new("Diagonal", steps), &steps, |b, &s| {
            b.iter(|| {
                let mut fast = Fast30::new();
                fast.evolve(black_box(s));
            });
        });
    }

    group.finish();
}

pub fn automaton_pattern(c: &mut Criterion) {
    let mut group = c.benchmark_group("Pattern");
    // &[100, 1_000, 2_000, 3_000, 4_000, 5_000, 10_000, 20_000, 30_000, 40_000, 50_000]
    for &steps in &[100, 5_000, 10_000, 50_000, 100_000, 500_000, 1_000_000, 10_000_000, 100_000_000] {
        group.bench_with_input(BenchmarkId::new("Base", steps), &steps, |b, &s| {
            b.iter(|| {
                let first_diag = vec![Cell::new(1)];
                let second_diag = vec![Cell::new(1)];

                let cell_zero = Cell::new(0);
                let cell_one = Cell::new(1);
                let mut cell_type = Cell::new(1);
                let mut counter = 0;
                let mut pattern = Pattern::new(first_diag.clone(), second_diag.clone());
                for i in 0..s {
                    if !pattern.contains(&cell_one) {
                        if pattern.count_state_in_left(1) % 2 == 0 {
                            if counter % 2 == 0 {
                                cell_type = Cell::new(0);
                            } else {
                                cell_type = Cell::new(1);
                            }
                            counter += 1;
                        } else {
                            cell_type = Cell::new(1);
                        }
                    }

                    pattern = pattern.next(Some(&cell_type.clone()));
                }
            });
        });
    }

    group.finish();
}

pub fn bench_chelou(c: &mut Criterion) {
    let s = 5000;
    c.bench_function("chelou", |b| {
        b.iter(|| {
            let mut config = vec![Cell::new(0); s * 2 + 1];
            config[s] = Cell::new(1);

            let row = Row::new(config);
            let rule = Box::new(WolframRule::new(30));
            let mut automaton = Automaton::new(row.clone(), rule);
            automaton.evolve(black_box(s as u64));
        });
    });
}

criterion_group!(benches, automaton_pattern);
criterion_main!(benches);
