use acme_rust_template::add;
use criterion::{Criterion, black_box, criterion_group, criterion_main};

// We need to import the run function from main
// Since main.rs is a binary, we'll benchmark the add function directly
// and create a similar benchmark for the main logic

fn benchmark_add_function(c: &mut Criterion) {
    c.bench_function("add 5 + 3", |b| {
        b.iter(|| {
            // Use black_box to prevent the compiler from optimizing away the computation
            black_box(add(black_box(5), black_box(3)))
        })
    });
}

fn benchmark_main_logic(c: &mut Criterion) {
    c.bench_function("main logic", |b| {
        b.iter(|| {
            // Benchmark the same computation that happens in main
            let result = black_box(add(black_box(5), black_box(3)));
            black_box(result)
        })
    });
}

criterion_group!(benches, benchmark_add_function, benchmark_main_logic);
criterion_main!(benches);
