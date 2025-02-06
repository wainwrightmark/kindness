use core::{
    num::{NonZero, NonZeroU32},
    u32,
};

use rand::{Rng, RngCore};

/// A uniform distribution where n is a power of two
#[derive(Debug, Clone, Copy)]
pub struct PowerOfTwo{
    bits: u32,
    ///
    mask: u32,
    ///
    counter: u32,
    ///
    chunk: u32,
}

/// A uniform distribution where n is not a power of two
#[derive(Debug, Clone, Copy)]
pub struct NonPowerOfTwo {
    ///
    inclusive_upper: u32,
    ///
    max_count: u32,
    ///
    counter: u32,
    ///
    chunk: u32,
    ///
    n: NonZero<u32>,
}

impl PowerOfTwo{
    /// Get the next random value in 0..n
    pub fn next(&mut self, rng: &mut impl Rng) -> u32 {
        if let Some(new_count) = (self.counter).checked_sub(self.bits) {
            self.counter = new_count;
        } else {
            self.chunk = rng.next_u32();
            self.counter = u32::BITS - self.bits;
        }

        let value = self.chunk & self.mask;
        self.chunk = self.chunk >> self.bits;
        value
    }
}


impl NonPowerOfTwo{
    /// Get the next random value in 0..n
    pub fn next(&mut self, rng: &mut impl Rng)-> u32{
        if let Some(new_count) = (self.counter).checked_sub(1) {
            self.counter = new_count;
        } else {
            self.counter = self.max_count;
            'random: loop {
                let next = rng.next_u32() >> self.inclusive_upper.leading_zeros() ;
                if next <= self.inclusive_upper {
                    self.chunk = next;
                    break 'random;
                }
            }
        }

        let value = self.chunk % self.n;
        self.chunk = self.chunk / self.n;
        value
    }
}

///Uniform Distribution
pub enum Uniform{
    ///
    PowerOfTwo(PowerOfTwo),
    ///
    NonPowerOfTwo(NonPowerOfTwo)
}

impl Uniform{
    /// Get the next random value in 0..n
    pub fn next(&mut self, rng: &mut impl Rng)-> u32{
        match self {
            Uniform::PowerOfTwo(x) => x.next(rng),
            Uniform::NonPowerOfTwo(x) => x.next(rng),
        }
    }
    ///Create a new uniform distribution
    pub fn new(n: NonZeroU32)-> Self{
        let u = n.get();
        if u.is_power_of_two() {
            let bits = u.trailing_zeros();
            let mask = u32::MAX.checked_shr(u32::BITS - bits).unwrap_or_default();

            Self::PowerOfTwo(PowerOfTwo {
                bits,
                mask,
                counter: 0,
                chunk: 0,
             })
        } else {
            let bits_used = u32::BITS - u.leading_zeros();
            let log_floor = u32::BITS / bits_used;

            let mut inclusive_upper = u.pow(log_floor);
            let mut max_count = log_floor - 1;
            'count_up: loop {
                if let Some(new_upper) = inclusive_upper.checked_mul(u) {
                    inclusive_upper = new_upper;
                    max_count += 1;
                } else {
                    break 'count_up;
                }
            }
            Self::NonPowerOfTwo(NonPowerOfTwo {inclusive_upper,
                max_count,
                counter: 0,
                chunk: 0,
                n, })
        }
    }
}




#[cfg(test)]
mod tests {
    use core::{fmt::Write, num::NonZeroU32};


    use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};

    use crate::uniform::Uniform;

    const RUNS: usize = 10000;

    #[test]
    pub fn test_generate_10() {
        let summary = test_generate(10);
        insta::assert_snapshot!(summary);
    }

    #[test]
    pub fn test_generate_100() {
        let summary = test_generate(100);
        insta::assert_snapshot!(summary);
    }

    #[test]
    pub fn test_generate_1() {
        let summary = test_generate(1);
        insta::assert_snapshot!(summary);
    }

    #[test]
    pub fn test_generate_2() {
        let summary = test_generate(2);
        insta::assert_snapshot!(summary);
    }

    #[test]
    pub fn test_generate_8() {
        let summary = test_generate(8);
        insta::assert_snapshot!(summary);
    }

    #[test]
    pub fn test_generate_16() {
        let summary = test_generate(16);
        insta::assert_snapshot!(summary);
    }

    #[must_use]
    pub fn test_generate(buckets: usize) -> String {
        let mut counts: Vec<usize> = std::iter::repeat(0).take(buckets).collect();
        let mut rng = get_rng();

        let mut uniform = Uniform::new(NonZeroU32::new(buckets as u32).unwrap());
        for _ in 0..RUNS {
            let v = uniform.next(&mut rng);

            counts[v as usize] += 1;
        }

        let mean: f64 = RUNS as f64 / buckets as f64;
        let mut sum_of_squares = 0f64;
        for c in counts.iter() {
            let diff = ((*c as f64) - mean).abs();
            sum_of_squares += (diff * diff);
        }

        ///Note - chi squared being less than twice the mean is a good sign
        let chi_squared = sum_of_squares / mean;

        let mut summary: String = String::new();

        summary
            .write_fmt(format_args!("Chi squared: {chi_squared:.3}\n"))
            .unwrap();
        summary
            .write_fmt(format_args!("Random values used: {}\n", rng.count))
            .unwrap();
        summary.write_fmt(format_args!("Values:\n")).unwrap();

        for x in counts.iter() {
            summary.write_fmt(format_args!("{x:4}\n")).unwrap();
        }

        summary
    }

    fn get_rng() -> CountingRng<StdRng> {
        let inner = StdRng::seed_from_u64(123);
        CountingRng {
            rng: inner,
            count: 0,
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
