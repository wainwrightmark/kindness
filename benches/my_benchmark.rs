use criterion::{black_box, criterion_group, criterion_main, Criterion};
use kindness::Kindness;
use rand::{seq::IteratorRandom, SeedableRng};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("random_item(1x10000)", |b| {
        b.iter(|| random_item(black_box(1), 10000))
    });
    c.bench_function("choose(1x10000)", |b| {
        b.iter(|| choose(black_box(1), 10000))
    });

    c.bench_function("random_item(20x10000)", |b| {
        b.iter(|| random_item(black_box(20), 10000))
    });
    c.bench_function("choose(20x10000)", |b| {
        b.iter(|| choose(black_box(20), 10000))
    });

    c.bench_function("random_item(100x10000)", |b| {
        b.iter(|| random_item(black_box(100), 10000))
    });
    c.bench_function("choose(100x10000)", |b| {
        b.iter(|| choose(black_box(100), 10000))
    });

    c.bench_function("random_item(10000x100)", |b| {
        b.iter(|| random_item(black_box(10000), 100))
    });
    c.bench_function("choose(10000x100)", |b| {
        b.iter(|| choose(black_box(10000), 100))
    });
}

fn random_item(max: u64, iterations: u64) -> u64 {
    let mut i: u64 = 0;
    for seed in 0..iterations {
        let mut rng = get_rng(seed);
        let range = (0..max).into_iter().filter(|_| return_true()); //prevent size hint
        if range.size_hint().0 > 0 {
            panic!("size hint");
        }
        let r = range.random_item(&mut rng).unwrap();
        i = u64::wrapping_add(i, r);
    }
    i
}

fn choose(max: u64, iterations: u64) -> u64 {
    let mut i: u64 = 0;
    for seed in 0..iterations {
        let mut rng = get_rng(seed);
        let range = (0..max).into_iter().filter(|_| return_true()); //prevent size hint
        if range.size_hint().0 > 0 {
            panic!("size hint");
        }
        let r = range.choose(&mut rng).unwrap();
        i = u64::wrapping_add(i, r);
    }
    i
}

/// Used for preventing size hints
#[inline(never)]
fn return_true() -> bool {
    true
}

fn get_rng(seed: u64) -> rand::rngs::StdRng {
    rand::rngs::StdRng::seed_from_u64(seed)
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
