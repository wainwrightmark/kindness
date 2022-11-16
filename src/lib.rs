#![cfg_attr(not(test), no_std)]
#![doc(html_root_url = "https://docs.rs/kindness/0.1.0")]
// #![deny(missing_docs)]
// #![deny(warnings, dead_code, unused_imports, unused_mut)]
#![warn(clippy::pedantic)]

//! [![github]](https://github.com/wainwrightmark/kindness)&ensp;[![crates-io]](https://crates.io/crates/kindness)&ensp;[![docs-rs]](https://docs.rs/kindness)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K
//!
//! <br>
//!
//! Methods for returning random elements from an iterator. Includes random_max(), random_max_by(), random_max_by_key(), random_element(), random_min(), random_min_by(), random_min_by_key()
//!
//! <br>
//!
//! ## Usage
//!
//! ```no_run
//! use kindness::*;
//!
//!
//! fn main()  {
//!     use rand::SeedableRng;
//!     let mut rng = rand::rngs::StdRng::seed_from_u64(123);
//!     let m =[3,2,1,2,3].iter().random_max(&mut rng).unwrap();
//!     assert_eq!(*m, 3)
//! }
//! ```
//!
//! ## Examples
//!
//! You can check out sample usage of this crate in the [examples/](https://github.com/wainwrightmark/kindness/tree/main/examples)
//! folder in the project repo on GitHub.
//!
//! ## Readme Docs
//!
//! You can find the crate's readme documentation on the
//! [crates.io] page, or alternatively in the [`README.md`] file on the GitHub project repo.
//!
//! [crates.io]: https://crates.io/crates/kindness
//! [`README.md`]: https://github.com/wainwrightmark/kindness

mod choice_iterator;

use core::cmp::Ordering;

use choice_iterator::ChoiceIterator;

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
    fn random_item<R: rand::Rng>(mut self, rng: &mut R) -> Option<Self::Item> {
        if let (lower, Some(upper)) = self.size_hint() {
            if lower == upper {
                //the iterator has an exact size, so we don't need to iterate the whole thing.
                if lower == 0 {
                    return None;
                }
                let n = rng.gen_range(0..upper);
                return self.nth(n);
            }
        }

        let choice_iterator = ChoiceIterator::new_zero(rng);

        choice_iterator
            .zip(self)
            .filter_map(|(chosen, item)| chosen.then_some(item))
            .last()
    }

    /// Returns a random maximum element with respect to the specified comparison function.
    ///
    /// If the iterator is empty, [`None`] is returned.
    /// Panics if the iterator has more than `usize::Max` maximum elements.
    fn random_max<R: rand::Rng>(self, rng: &mut R) -> Option<Self::Item>
    where
        Self::Item: Ord,
    {
        self.random_max_by(rng, Ord::cmp)
    }

    /// Returns a random element that gives the maximum value from the
    /// specified function.
    /// If the iterator is empty, [`None`] is returned.
    /// Panics if the iterator has more than `usize::Max` maximum elements.
    fn random_max_by_key<B: Ord, R: rand::Rng, F: FnMut(&Self::Item) -> B>(
        mut self,
        rng: &mut R,
        mut f: F,
    ) -> Option<Self::Item> {
        let Some(first)  = self.next() else {
            return None;
        };

        let mut current_key = f(&first);
        let mut current = first;
        let mut choice_iterator = ChoiceIterator::new_one(rng);

        for item in self {
            let item_key = f(&item);
            match item_key.cmp(&current_key) {
                core::cmp::Ordering::Less => {}
                core::cmp::Ordering::Equal => {
                    //Choose either iter or current randomly, see random_element for more
                    if choice_iterator.next() == Some(true) {
                        current = item;
                    }
                }
                core::cmp::Ordering::Greater => {
                    current_key = item_key; //this is the new maximum
                    current = item;
                    choice_iterator.reset_to_one();
                }
            }
        }

        Some(current)
    }

    /// Returns a random maximum element.
    ///
    /// If the iterator is empty, [`None`] is returned.
    /// Panics if the iterator has more than `usize::Max` maximum elements.
    fn random_max_by<R: rand::Rng, F: FnMut(&Self::Item, &Self::Item) -> Ordering>(
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
        let mut choice_iterator = ChoiceIterator::new_one(rng);

        for item in self {
            match compare(&item, &current) {
                core::cmp::Ordering::Less => {}
                core::cmp::Ordering::Equal => {
                    if choice_iterator.next() == Some(true) {
                        current = item;
                    }
                }
                core::cmp::Ordering::Greater => {
                    current = item; //this is the new maximum
                    choice_iterator.reset_to_one();
                }
            }
        }

        Some(current)
    }

    /// Return a random minimum element of the iterator.  
    /// Returns none if the iterator is empty.  
    /// Panics if the iterator has more than `usize::Max` elements.
    fn random_min<R: rand::Rng>(self, rng: &mut R) -> Option<Self::Item>
    where
        Self::Item: Ord,
    {
        self.random_min_by(rng, Ord::cmp)
    }

    /// Returns a random element that gives the minimum value from the
    /// specified function.
    /// If the iterator is empty, [`None`] is returned.
    /// Panics if the iterator has more than `usize::Max` minimum elements.
    fn random_min_by_key<B: Ord, R: rand::Rng, F: FnMut(&Self::Item) -> B>(
        mut self,
        rng: &mut R,
        mut f: F,
    ) -> Option<Self::Item> {
        let Some(first)  = self.next() else {
            return None;
        };

        let mut current_key = f(&first);
        let mut current = first;
        let mut choice_iterator = ChoiceIterator::new_one(rng);

        for item in self {
            let item_key = f(&item);
            match item_key.cmp(&current_key) {
                core::cmp::Ordering::Greater => {}
                core::cmp::Ordering::Equal => {
                    //Choose either iter or current randomly, see random_element for more
                    if choice_iterator.next() == Some(true) {
                        current = item;
                    }
                }
                core::cmp::Ordering::Less => {
                    current_key = item_key; //this is the new maximum
                    current = item;
                    choice_iterator.reset_to_one();
                }
            }
        }

        Some(current)
    }

    /// Returns a random minimum element.
    ///
    /// If the iterator is empty, [`None`] is returned.
    /// Panics if the iterator has more than `usize::Max` minimum elements.
    fn random_min_by<R: rand::Rng, F: FnMut(&Self::Item, &Self::Item) -> Ordering>(
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
        let mut choice_iterator = ChoiceIterator::new_one(rng);

        for item in self {
            match compare(&item, &current) {
                core::cmp::Ordering::Greater => {}
                core::cmp::Ordering::Equal => {
                    if choice_iterator.next() == Some(true) {
                        current = item;
                    }
                }
                core::cmp::Ordering::Less => {
                    current = item; //this is the new maximum
                    choice_iterator.reset_to_one();
                }
            }
        }

        Some(current)
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
            let element = range.random_item(&mut rng).unwrap();
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
            let element = range.random_item(&mut rng).unwrap();
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
    fn test_random_element_windowed() {
        let mut counts: [usize; LENGTH] = [0; LENGTH];
        let mut rng = get_rng();

        for _ in 0..RUNS {
            let range = UnhintedIterator(0..LENGTH);
            assert_eq!((0, None), range.size_hint());
            let element = range.random_item(&mut rng).unwrap();
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
            let max = range.random_max(&mut rng).unwrap();
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
                .random_max_by(&mut rng, |&a, &b| (a / 10).cmp(&(b / 10)))
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
                .random_max_by_key(&mut rng, |x| RoughNumber(*x))
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
            let min = range.random_min(&mut rng).unwrap();
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
                .random_min_by(&mut rng, |&a, &b| (a / 10).cmp(&(b / 10)))
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
                .random_min_by_key(&mut rng, |x| RoughNumber(*x))
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