use bloom_filter_rs::BloomFilter;
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};

fn insert_benchmark(c: &mut Criterion) {
    c.bench_function("add 10000 items", |b| {
        b.iter_batched(
            BloomFilter::default,
            |mut bf| {
                for i in 0..10000 {
                    bf.insert(std::hint::black_box(&i));
                }
            },
            BatchSize::SmallInput,
        );
    });
}

fn exist_benchmark(c: &mut Criterion) {
    let mut bf = BloomFilter::default();
    for i in 0..10000 {
        bf.insert(&i);
    }

    c.bench_function("exist 10000 items", |b| {
        b.iter(|| {
            for i in 0..10000 {
                std::hint::black_box(bf.exist(std::hint::black_box(&i)));
            }
        });
    });
}

criterion_group!(benches, insert_benchmark, exist_benchmark);
criterion_main!(benches);
