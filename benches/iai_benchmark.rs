use iai_callgrind::{
    library_benchmark, library_benchmark_group, main, FlamegraphConfig, LibraryBenchmarkConfig,
};
use kindness::Kindness;
use rand::{seq::IteratorRandom, SeedableRng};

#[library_benchmark]
#[bench::one(1, 1000)]
#[bench::ten(10, 100)]
#[bench::one_hundred(100, 10)]
#[bench::one_thousand(1000, 1)]
fn random_item(max: usize, trials: usize) -> usize {
    let mut rng = get_rng(123);
    let mut total = 0usize;
    for _ in 0..trials {
        let range = UnhintedIterator(0..max);
        let r = range.choose_item(&mut rng).unwrap();
        total = total.wrapping_add(r);
    }
    total
}

#[library_benchmark]
#[bench::one(1, 1000)]
#[bench::ten(10, 100)]
#[bench::one_hundred(100, 10)]
#[bench::one_thousand(1000, 1)]
fn choose(max: usize, trials: usize) -> usize {
    let mut rng = get_rng(123);
    let mut total = 0usize;
    for _ in 0..trials {
        let range = UnhintedIterator(0..max);
        let r = range.choose(&mut rng).unwrap();
        total = total.wrapping_add(r);
    }
    total
}

#[library_benchmark]
#[bench::one(1, 1000)]
#[bench::ten(10, 100)]
#[bench::one_hundred(100, 10)]
#[bench::one_thousand(1000, 1)]
fn random_item_windowed(max: usize, trials: usize) -> usize {
    let mut rng = get_rng(123);
    let mut total = 0usize;
    for _ in 0..trials {
        let range = WindowHintedIterator(0..max, 100);
        let r = range.choose_item(&mut rng).unwrap();
        total = total.wrapping_add(r);
    }
    total
}

#[library_benchmark]
#[bench::one(1, 1000)]
#[bench::ten(10, 100)]
#[bench::one_hundred(100, 10)]
#[bench::one_thousand(1000, 1)]
fn choose_windowed(max: usize, trials: usize) -> usize {
    let mut total = 0usize;
    let mut rng = get_rng(123);
    for _ in 0..trials {
        let range = WindowHintedIterator(0..max, 100);
        let r = range.choose(&mut rng).unwrap();
        total = total.wrapping_add(r);
    }
    total
}

#[library_benchmark]
#[bench::one(1, 2)]
#[bench::ten(10, 2)]
#[bench::one_hundred(100, 2)]
#[bench::one_thousand(1000, 2)]
fn choose_unique_default(max: usize, duplicates: usize) -> usize {
    let mut rng = get_rng(123);
    let range = (0..max).flat_map(|x| std::iter::repeat(x).take(duplicates));
    range.choose_unique(&mut rng).len()
}

#[library_benchmark]
#[bench::one(1, 2)]
#[bench::ten(10, 2)]
#[bench::one_hundred(100, 2)]
#[bench::one_thousand(1000, 2)]
fn choose_unique_ahash(max: usize, duplicates: usize) -> usize {
    let mut rng = get_rng(123);
    let range = (0..max).flat_map(|x| std::iter::repeat(x).take(duplicates));

    let hash_builder = hashbrown::hash_map::DefaultHashBuilder::default();
        let alloc = allocator_api2::alloc::Global;

    range.choose_unique_with_hasher_in(&mut rng, hash_builder, alloc).len()
}

library_benchmark_group!(
    name = unhinted;
    benchmarks = random_item, choose
);

library_benchmark_group!(
    name = windowed;
    benchmarks = random_item_windowed, choose_windowed
);

library_benchmark_group!(
    name = choose_unique;
    benchmarks = choose_unique_default, choose_unique_ahash
);

main!(library_benchmark_groups = unhinted, windowed, choose_unique);

fn get_rng(seed: u64) -> rand::rngs::StdRng {
    rand::rngs::StdRng::seed_from_u64(seed)
}

#[derive(Clone)]
struct UnhintedIterator<I: Iterator + Clone>(I);
impl<I: Iterator + Clone> Iterator for UnhintedIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

#[derive(Clone)]
struct WindowHintedIterator<I: ExactSizeIterator + Iterator + Clone>(I, usize);

impl<I: ExactSizeIterator + Iterator + Clone> Iterator for WindowHintedIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (core::cmp::min(self.0.len(), self.1), None)
    }
}
