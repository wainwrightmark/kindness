use rand::{Rng, RngCore};

pub(crate) struct CoinFlipper<R: RngCore> {
    pub rng: R,
    chunk: u32,
    chunk_remaining: u32,
}

impl<R: RngCore> CoinFlipper<R> {
    pub fn new(rng: R) -> Self {
        Self {
            rng,
            chunk: 0,
            chunk_remaining: 0,
        }
    }

    pub fn gen_ratio_one_over(&mut self, denominator: usize) -> bool {
        let n = usize::BITS - denominator.leading_zeros() - 1;

        if !self.all_next(n) {
            return false;
        }

        return self.gen_ratio(1 << n, denominator);
    }

    pub fn gen_ratio(&mut self, mut numerator: usize, denominator: usize) -> bool {
        while numerator < denominator {
            if let Some(new_numerator) = numerator.checked_mul(2) {
                if self.next() {
                    numerator = new_numerator;
                } else {
                    if new_numerator < denominator {
                        return false;
                    } else {
                        numerator = new_numerator - denominator;
                    }
                }
            } else {
                panic!("Reached branch");
                if self.next() {
                    return true; //numerator must have been more than half the denominator
                } else {
                    numerator.wrapping_sub(denominator).wrapping_add(numerator);
                }
            }
        }
        return true;
    }

    pub fn next(&mut self) -> bool {
        if self.chunk_remaining == 0 {
            self.chunk = self.rng.next_u32();
            self.chunk_remaining = u32::BITS;
        }

        let result = self.chunk.trailing_zeros() > 0;
        self.chunk = self.chunk.wrapping_shr(1);
        self.chunk_remaining = self.chunk_remaining.saturating_sub(1);
        return result;
    }

    pub fn all_next(&mut self, mut n: u32) -> bool {
        let mut zeros = self.chunk.trailing_zeros();
        while self.chunk_remaining < n {
            if zeros >= self.chunk_remaining {
                n -= self.chunk_remaining;
            } else {
                self.chunk_remaining -= (zeros + 1);
                self.chunk = self.chunk >> (zeros + 1);
                return false;
            }
            self.chunk = self.rng.next_u32();
            self.chunk_remaining = u32::BITS;
            zeros = self.chunk.trailing_zeros();
        }

        let result = zeros >= n;
        let shrink = if result { n } else { zeros + 1 };
        self.chunk = self.chunk.wrapping_shr(shrink);
        self.chunk_remaining = self.chunk_remaining.saturating_sub(shrink);

        return result;
    }
}
#[cfg(test)]
mod tests {
    use core::ops::Range;

    use crate::CoinFlipper;
    use rand::{Rng, RngCore, SeedableRng};

    const RUNS: usize = 10000;
    const LENGTH: usize = 10000;
    const START: usize = 1;
    const SEED: u64 = 1;

    #[test]
    pub fn test_coin_flipper_gen_ratio() {
        let rng = get_rng();
        let mut coin_flipper = CoinFlipper::new(rng);

        let mut counts: Vec<_> = Default::default();
        for d in START..=LENGTH {
            let mut count = 0;
            for _ in 0..RUNS {
                if coin_flipper.gen_ratio_one_over(d) {
                    count += 1;
                }
            }
            counts.push(count);
        }

        let adjusted_counts: Vec<_> = counts
            .iter()
            .enumerate()
            .map(|(i, &x)| (i + START) * x)
            .map(|z| (z as f64) / (RUNS as f64))
            .collect();

        // println!(
        //     "{}",
        //     adjusted_counts
        //         .iter()
        //         .map(|z| z.to_string())
        //         .collect::<Vec<_>>()
        //         .join(", ")
        // );

        let average_gens = ((RUNS * LENGTH) as f64) / (coin_flipper.rng.count as f64);

        println!(
            "Gens: {} (1 per {} gens)",
            coin_flipper.rng.count, average_gens
        );

        let (mean, variance, standard_deviation) = get_stats(adjusted_counts);

        println!("mean: {mean}, variance: {variance}, standard deviation: {standard_deviation}");

        //assert_contains(15.5..16.5, &average_gens); //Should be just over 16 gens per gen_ratio
        assert_contains(0.95..1.05, &mean); //Should be about 1 because we are adjusting
        assert_contains(0.0..10.0, &standard_deviation);
    }

    fn get_rng() -> CountingRng<rand::rngs::StdRng> {
        let inner = rand::rngs::StdRng::seed_from_u64(SEED);
        CountingRng {
            rng: inner,
            count: 0,
        }
    }

    pub fn get_stats(vec: Vec<f64>) -> (f64, f64, f64) {
        let mean: f64 = vec.iter().map(|&x| x as f64 / (vec.len() as f64)).sum();
        let variance: f64 = vec
            .iter()
            .map(|&x| f64::powi((x as f64) - mean, 2) / (vec.len() as f64))
            .sum();
        let standard_deviation = f64::sqrt(variance);

        (mean, variance, standard_deviation)
    }

    fn assert_contains(range: Range<f64>, n: &f64) {
        if !range.contains(n) {
            panic!("The range {:?} does not contain {n}", range)
        }
    }

    struct CountingRng<Inner: Rng> {
        pub rng: Inner,
        pub count: usize,
    }

    impl<Inner: Rng> RngCore for CountingRng<Inner> {
        fn next_u32(&mut self) -> u32 {
            self.count += 1;
            self.rng.next_u32()
        }

        fn next_u64(&mut self) -> u64 {
            self.count += 1;
            self.rng.next_u64()
        }

        fn fill_bytes(&mut self, dest: &mut [u8]) {
            self.count += 1;
            self.rng.fill_bytes(dest)
        }

        fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
            self.count += 1;
            self.rng.try_fill_bytes(dest)
        }
    }
}
