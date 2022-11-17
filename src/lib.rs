#![cfg_attr(not(test), no_std)]
#![doc(html_root_url = "https://docs.rs/kindness/0.3.0")]
#![deny(missing_docs)]
#![deny(warnings, dead_code, unused_imports, unused_mut)]
#![warn(clippy::pedantic)]

//! [![github]](https://github.com/wainwrightmark/kindness)&ensp;[![crates-io]](https://crates.io/crates/kindness)&ensp;[![docs-rs]](https://docs.rs/kindness)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! Methods for choosing random elements from an iterator.
//!
//! <br>
//!
//! ## Usage
//!
//! ```
//! use kindness::*;
//!
//!
//! fn main()  {
//!     use rand::SeedableRng;
//!     let mut rng = rand::rngs::StdRng::seed_from_u64(123);
//!     let m =[3,2,1,2,3].iter().choose_max(&mut rng).unwrap();
//!     assert_eq!(*m, 3)
//! }
//! ```
//!
//! ## Readme Docs
//!
//! You can find the crate's readme documentation on the
//! [crates.io] page, or alternatively in the [`README.md`] file on the GitHub project repo.
//!
//! [crates.io]: https://crates.io/crates/kindness
//! [`README.md`]: https://github.com/wainwrightmark/kindness

mod chooser;
mod coin_flipper;

use core::cmp::Ordering;

//use chooser::Chooser;
use coin_flipper::CoinFlipper;

impl<T: Iterator + Sized> Kindness for T {}

/// An [`Iterator`] blanket implementation that provides extra adaptors and
/// methods for returning random elements.
pub trait Kindness: Iterator
where
    Self: Sized,
{
    /// Return a random element of the iterator.  
    /// Returns none if the iterator is empty.  
    /// If the iterator has more than `usize::Max` elements, later elements will be slightly more likely.
    /// Will iterate the entire enumerable unless it has a size hint which indicates an exact length.
    #[inline]
    fn choose_item<R: rand::Rng>(mut self, rng: &mut R) -> Option<Self::Item> {
        let (mut lower, mut upper) = self.size_hint();
        let mut result = None;

        // Handling for this condition outside the loop allows the optimizer to eliminate the loop
        // when the Iterator is an ExactSizeIterator. This has a large performance impact on e.g.
        // seq_iter_choose_from_1000.
        if upper == Some(lower) {
            return if lower == 0 {
                None
            } else {
                self.nth(gen_index(rng, lower))
            };
        }

        //let mut choice_iterator = Chooser::new_zero(rng);
        let mut coin_flipper = CoinFlipper::new(rng);
        let mut consumed = 0;

        // Continue until the iterator is exhausted
        loop {
            if lower > 1 {
                let ix = gen_index(coin_flipper.rng, lower + consumed);
                let skip = if ix < lower {
                    result = self.nth(ix);
                    lower - (ix + 1)
                } else {
                    lower
                };
                if upper == Some(lower) {
                    return result;
                }
                consumed = consumed + lower;
                if skip > 0 {
                    self.nth(skip - 1);
                }
            } else {
                let elem = self.next();
                if elem.is_none() {
                    return result;
                }
                consumed += 1;
                if coin_flipper.gen_ratio_one_over(consumed) {
                    result = elem;
                }
            }

            let hint = self.size_hint();
            lower = hint.0;
            upper = hint.1;
        }
    }

    /// Returns a random maximum element with respect to the specified comparison function.
    ///
    /// If the iterator is empty, [`None`] is returned.
    /// If the iterator has more than `usize::Max` elements, later elements will be slightly more likely.
    fn choose_max<R: rand::Rng>(self, rng: &mut R) -> Option<Self::Item>
    where
        Self::Item: Ord,
    {
        self.choose_max_by(rng, Ord::cmp)
    }

    /// Returns a random element that gives the maximum value from the
    /// specified function.
    /// If the iterator is empty, [`None`] is returned.
    /// If the iterator has more than `usize::Max` elements, later elements will be slightly more likely.
    fn choose_max_by_key<B: Ord, R: rand::Rng, F: FnMut(&Self::Item) -> B>(
        mut self,
        rng: &mut R,
        mut f: F,
    ) -> Option<Self::Item> {
        let Some(first)  = self.next() else {
            return None;
        };

        let mut current_key = f(&first);
        let mut current = first;
        let mut coin_flipper = coin_flipper::CoinFlipper::new(rng);
        let mut consumed = 1;

        for item in self {
            let item_key = f(&item);
            match item_key.cmp(&current_key) {
                core::cmp::Ordering::Less => {}
                core::cmp::Ordering::Equal => {
                    consumed += 1;
                    //Choose either iter or current randomly, see random_element for more
                    if coin_flipper.gen_ratio_one_over(consumed) {
                        current = item;
                    }
                }
                core::cmp::Ordering::Greater => {
                    current_key = item_key; //this is the new maximum
                    current = item;
                    consumed = 1;
                }
            }
        }

        Some(current)
    }

    /// Returns a random maximum element.
    ///
    /// If the iterator is empty, [`None`] is returned.
    /// If the iterator has more than `usize::Max` elements, later elements will be slightly more likely.
    fn choose_max_by<R: rand::Rng, F: FnMut(&Self::Item, &Self::Item) -> Ordering>(
        mut self,
        rng: &mut R,
        mut compare: F,
    ) -> Option<Self::Item>
    where
        Self::Item: Ord,
    {
        let Some(first)  = self.next() else {
            return None;
        };

        let mut current = first;
        let mut coin_flipper = coin_flipper::CoinFlipper::new(rng);
        let mut consumed = 1;

        for item in self {
            match compare(&item, &current) {
                core::cmp::Ordering::Less => {}
                core::cmp::Ordering::Equal => {
                    consumed += 1;
                    if coin_flipper.gen_ratio_one_over(consumed) {
                        current = item;
                    }
                }
                core::cmp::Ordering::Greater => {
                    current = item; //this is the new maximum
                    consumed = 1;
                }
            }
        }

        Some(current)
    }

    /// Return a random minimum element of the iterator.  
    /// Returns none if the iterator is empty.  
    /// If the iterator has more than `usize::Max` elements, later elements will be slightly more likely.
    fn choose_min<R: rand::Rng>(self, rng: &mut R) -> Option<Self::Item>
    where
        Self::Item: Ord,
    {
        self.choose_min_by(rng, Ord::cmp)
    }

    /// Returns a random element that gives the minimum value from the
    /// specified function.
    /// If the iterator is empty, [`None`] is returned.
    /// If the iterator has more than `usize::Max` elements, later elements will be slightly more likely.
    fn choose_min_by_key<B: Ord, R: rand::Rng, F: FnMut(&Self::Item) -> B>(
        mut self,
        rng: &mut R,
        mut f: F,
    ) -> Option<Self::Item> {
        let Some(first)  = self.next() else {
            return None;
        };

        let mut current_key = f(&first);
        let mut current = first;
        let mut coin_flipper = coin_flipper::CoinFlipper::new(rng);
        let mut consumed = 1;

        for item in self {
            let item_key = f(&item);
            match item_key.cmp(&current_key) {
                core::cmp::Ordering::Greater => {}
                core::cmp::Ordering::Equal => {
                    consumed += 1;
                    //Choose either iter or current randomly, see random_element for more
                    if coin_flipper.gen_ratio_one_over(consumed) {
                        current = item;
                    }
                }
                core::cmp::Ordering::Less => {
                    current_key = item_key; //this is the new maximum
                    current = item;
                    consumed = 1;
                }
            }
        }

        Some(current)
    }

    /// Returns a random minimum element.
    ///
    /// If the iterator is empty, [`None`] is returned.
    /// If the iterator has more than `usize::Max` elements, later elements will be slightly more likely.
    fn choose_min_by<R: rand::Rng, F: FnMut(&Self::Item, &Self::Item) -> Ordering>(
        mut self,
        rng: &mut R,
        mut compare: F,
    ) -> Option<Self::Item>
    where
        Self::Item: Ord,
    {
        let Some(first)  = self.next() else {
        return None;
    };

        let mut current = first;
        let mut coin_flipper = coin_flipper::CoinFlipper::new(rng);
        let mut consumed = 1;

        for item in self {
            match compare(&item, &current) {
                core::cmp::Ordering::Greater => {}
                core::cmp::Ordering::Equal => {
                    consumed += 1;
                    if coin_flipper.gen_ratio_one_over(consumed) {
                        current = item;
                    }
                }
                core::cmp::Ordering::Less => {
                    current = item; //this is the new maximum
                    consumed = 1;
                }
            }
        }

        Some(current)
    }
}

// Sample a number uniformly between 0 and `ubound`. Uses 32-bit sampling where
// possible, primarily in order to produce the same output on 32-bit and 64-bit
// platforms.
#[inline]
fn gen_index<R: rand::Rng + ?Sized>(rng: &mut R, ubound: usize) -> usize {
    if ubound <= (core::u32::MAX as usize) {
        rng.gen_range(0..ubound as u32) as usize
    } else {
        rng.gen_range(0..ubound)
    }
}

#[cfg(test)]
mod tests {
    use core::ops::Range;

    use crate::Kindness;
    use rand::{Rng, RngCore, SeedableRng};

    const RUNS: usize = 10000;
    const LENGTH: usize = 100;
    const LOWER_TOLERANCE: usize = 60;
    const UPPER_TOLERANCE: usize = 140;

    #[test]
    fn test_random_element_with_size_hint() {
        let mut counts: [usize; LENGTH] = [0; LENGTH];
        let mut rng = get_rng();

        for _ in 0..RUNS {
            let range = 0..LENGTH;
            assert_eq!((LENGTH, Some(LENGTH)), range.size_hint());
            let element = range.choose_item(&mut rng).unwrap();
            counts[element] += 1;
        }

        insta::assert_debug_snapshot!(counts);
        for x in counts {
            assert!(x > LOWER_TOLERANCE);
            assert!(x < UPPER_TOLERANCE);
        }

        assert_contains(RUNS..(RUNS * 2), &rng.count); // There should be at most two calls per iteration because we are using gen_range only once
    }

    #[test]
    fn test_random_element_unhinted() {
        let mut counts: [usize; LENGTH] = [0; LENGTH];
        let mut rng = get_rng();

        for _ in 0..RUNS {
            let range = UnhintedIterator(0..LENGTH);
            assert_eq!((0, None), range.size_hint());
            let element = range.choose_item(&mut rng).unwrap();
            counts[element] += 1;
        }

        insta::assert_debug_snapshot!(counts);

        for x in counts {
            assert!(x > LOWER_TOLERANCE);
            assert!(x < UPPER_TOLERANCE);
        }

        //println!("{}", &rng.count);

        assert_contains(RUNS..2000000, &rng.count);
    }

    #[test]
    fn test_random_element_windowed() {
        let mut counts: [usize; LENGTH] = [0; LENGTH];
        let mut rng = get_rng();

        for _ in 0..RUNS {
            let range = UnhintedIterator(0..LENGTH);
            assert_eq!((0, None), range.size_hint());
            let element = range.choose_item(&mut rng).unwrap();
            counts[element] += 1;
        }

        insta::assert_debug_snapshot!(counts);

        for x in counts {
            assert!(x > LOWER_TOLERANCE);
            assert!(x < UPPER_TOLERANCE);
        }

        assert_contains(RUNS..2000000, &rng.count);
    }

    #[test]
    fn test_random_max() {
        let mut counts: [usize; LENGTH] = [0; LENGTH];
        let mut rng = get_rng();

        for _ in 0..RUNS {
            let range = (0..LENGTH).map(RoughNumber);
            let max = range.choose_max(&mut rng).unwrap();
            counts[max.0] += 1;
        }

        insta::assert_debug_snapshot!(counts);

        for (i, &x) in counts.iter().enumerate() {
            if i < 90 {
                assert!(x == 0)
            } else {
                assert!(x > LOWER_TOLERANCE * 10);
                assert!(x < UPPER_TOLERANCE * 10);
            }
        }

        assert_contains(0..1800000, &rng.count);
    }

    #[test]
    fn test_random_max_by() {
        let mut counts: [usize; LENGTH] = [0; LENGTH];
        let mut rng = get_rng();

        for _ in 0..RUNS {
            let range = 0..LENGTH;
            let max = range
                .choose_max_by(&mut rng, |&a, &b| (a / 10).cmp(&(b / 10)))
                .unwrap();
            counts[max] += 1;
        }

        insta::assert_debug_snapshot!(counts);

        for (i, &x) in counts.iter().enumerate() {
            if i < 90 {
                assert!(x == 0)
            } else {
                assert!(x > LOWER_TOLERANCE * 10);
                assert!(x < UPPER_TOLERANCE * 10);
            }
        }

        assert_contains(0..1800000, &rng.count);
    }

    #[test]
    fn test_random_max_by_key() {
        let mut counts: [usize; LENGTH] = [0; LENGTH];
        let mut rng = get_rng();

        for _ in 0..RUNS {
            let range = 0..LENGTH;
            let max = range
                .choose_max_by_key(&mut rng, |x| RoughNumber(*x))
                .unwrap();
            counts[max] += 1;
        }

        insta::assert_debug_snapshot!(counts);

        for (i, &x) in counts.iter().enumerate() {
            if i < 90 {
                assert!(x == 0)
            } else {
                assert!(x > LOWER_TOLERANCE * 10);
                assert!(x < UPPER_TOLERANCE * 10);
            }
        }

        assert_contains(0..1800000, &rng.count);
    }

    #[test]
    fn test_random_min() {
        let mut counts: [usize; LENGTH] = [0; LENGTH];
        let mut rng = get_rng();

        for _ in 0..RUNS {
            let range = (0..LENGTH).map(RoughNumber);
            let min = range.choose_min(&mut rng).unwrap();
            counts[min.0] += 1;
        }

        insta::assert_debug_snapshot!(counts);

        for (i, &x) in counts.iter().enumerate() {
            if i >= 10 {
                assert!(x == 0)
            } else {
                assert!(x > LOWER_TOLERANCE * 10);
                assert!(x < UPPER_TOLERANCE * 10);
            }
        }

        assert_contains(0..200000, &rng.count);
    }

    #[test]
    fn test_random_min_by() {
        let mut counts: [usize; LENGTH] = [0; LENGTH];
        let mut rng = get_rng();

        for _ in 0..RUNS {
            let range = 0..LENGTH;
            let max = range
                .choose_min_by(&mut rng, |&a, &b| (a / 10).cmp(&(b / 10)))
                .unwrap();
            counts[max] += 1;
        }

        insta::assert_debug_snapshot!(counts);

        for (i, &x) in counts.iter().enumerate() {
            if i >= 10 {
                assert!(x == 0)
            } else {
                assert!(x > LOWER_TOLERANCE * 10);
                assert!(x < UPPER_TOLERANCE * 10);
            }
        }

        assert_contains(0..200000, &rng.count);
    }

    #[test]
    fn test_random_min_by_key() {
        let mut counts: [usize; LENGTH] = [0; LENGTH];
        let mut rng = get_rng();

        for _ in 0..RUNS {
            let range = 0..LENGTH;
            let max = range
                .choose_min_by_key(&mut rng, |x| RoughNumber(*x))
                .unwrap();
            counts[max] += 1;
        }

        insta::assert_debug_snapshot!(counts);

        for (i, &x) in counts.iter().enumerate() {
            if i >= 10 {
                assert!(x == 0)
            } else {
                assert!(x > LOWER_TOLERANCE * 10);
                assert!(x < UPPER_TOLERANCE * 10);
            }
        }

        assert_contains(0..200000, &rng.count);
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

    /// A number whose ordering is only affected by the tens digit e.g 42 >= 43
    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    struct RoughNumber(pub usize);

    impl Ord for RoughNumber {
        fn cmp(&self, other: &Self) -> core::cmp::Ordering {
            (self.0 / 10).cmp(&(other.0 / 10))
        }
    }

    impl PartialOrd for RoughNumber {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            (self.0 / 10).partial_cmp(&(other.0 / 10))
        }
    }

    fn assert_contains(range: Range<usize>, n: &usize) {
        if !range.contains(n) {
            panic!("The range {:?} does not contain {n}", range)
        }
    }

    fn get_rng() -> CountingRng<rand::rngs::StdRng> {
        let inner = rand::rngs::StdRng::seed_from_u64(123);
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
