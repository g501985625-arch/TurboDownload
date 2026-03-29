//! Performance benchmarks for turbo-downloader

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use turbo_downloader::{Strategy, Chunk, Tracker, SpeedCalculator};

/// Benchmark chunk strategy calculation
fn bench_strategy_calculation(c: &mut Criterion) {
    let mut group = c.benchmark_group("strategy");
    
    for size in [1_000_000, 10_000_000, 100_000_000, 1_000_000_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                Strategy::calculate(black_box(size), black_box(0), black_box(1_000_000))
            });
        });
    }
    
    group.finish();
}

/// Benchmark chunk operations
fn bench_chunk_operations(c: &mut Criterion) {
    let chunk = Chunk::new(0, 0, 1_000_000);
    
    c.bench_function("chunk_size", |b| {
        b.iter(|| chunk.size());
    });
    
    c.bench_function("chunk_remaining", |b| {
        b.iter(|| chunk.remaining());
    });
    
    c.bench_function("chunk_is_complete", |b| {
        b.iter(|| chunk.is_complete());
    });
}

/// Benchmark progress tracker
fn bench_tracker(c: &mut Criterion) {
    let tracker = Tracker::new(1_000_000);
    
    c.bench_function("tracker_update", |b| {
        b.iter(|| tracker.update(black_box(1000)));
    });
    
    c.bench_function("tracker_progress", |b| {
        b.iter(|| tracker.get_progress());
    });
}

/// Benchmark speed calculator
fn bench_speed_calculator(c: &mut Criterion) {
    let mut calc = SpeedCalculator::new(10);
    
    // Add some samples first
    for _ in 0..5 {
        calc.add_sample(1000);
    }
    
    c.bench_function("speed_calculator_sample", |b| {
        b.iter(|| calc.add_sample(black_box(1000)));
    });
    
    c.bench_function("speed_calculator_get_speed", |b| {
        b.iter(|| calc.get_speed());
    });
}

criterion_group!(
    benches,
    bench_strategy_calculation,
    bench_chunk_operations,
    bench_tracker,
    bench_speed_calculator
);
criterion_main!(benches);