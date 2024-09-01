use std::num::{NonZero, NonZeroU32};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::SeedableRng;

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

pub fn criterion_benchmark(c: &mut Criterion) {
    for buckets in [1u32, 2, 10, 16, 100].map(|x| NonZeroU32::new(x).unwrap()) {
        for trials in [1usize, 2, 100] {
            c.bench_function(format!("sample_uniform({buckets}, {trials})").as_str(), |b| {
                let mut rng = get_rng(123);
                b.iter(|| sample_uniform(buckets, trials, &mut rng))
            });
        }
    }
}

fn sample_uniform(buckets: NonZero<u32>, trials: usize, rng: &mut rand::rngs::StdRng) -> u32 {
    let mut uniform = kindness::uniform::Uniform::new(buckets);
    let mut sum = 0;
    for _ in 0..(black_box(trials)) {
        let next = uniform.next(rng);
        sum += next;
    }

    sum
}

fn get_rng(seed: u64) -> rand::rngs::StdRng {
    rand::rngs::StdRng::seed_from_u64(seed)
}
