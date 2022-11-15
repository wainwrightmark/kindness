#![cfg_attr(not(test), no_std)]
#![doc(html_root_url = "https://docs.rs/kindness/0.1.0")]
#![deny(missing_docs)]
#![deny(warnings, dead_code, unused_imports, unused_mut)]
#![deny(clippy::integer_arithmetic)]

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

use core::cmp::Ordering;

impl<T: Iterator + Sized> Kindness for T {}

/// An [`Iterator`] blanket implementation that provides extra adaptors and
/// methods for returning random elements.
pub trait Kindness: Iterator
where
    Self: Sized,
{
    /// Return a random element of the iterator.  
    /// Returns none if the iterator is empty.  
    /// If the iterator has more than usize::Max elements, later elements will be slightly more likely.
    /// Will iterate the entire enumerable unless it has a size hint which indicates an exact length.
    fn random_element<R: rand::Rng>(mut self, rng: &mut R) -> Option<Self::Item> {
        if let (lower, Some(upper)) = self.size_hint() {
            if lower == upper {
                //the iterator has an exact size, so we don't need to iterate the whole thing.
                if lower == 0 {
                    return None;
                } else {
                    let n = rng.gen_range(0..upper);
                    return self.nth(n);
                }
            }
        }

        let Some(first)  = self.next() else {
            return None; //The iterator is empty
        };

        let mut current = first;
        let mut count: usize = 1;

        for item in self {
            count = count.saturating_add(1);
            if rng.gen_range(0..count) == 0 {
                //We only change to a new item if the index is 0.
                // This has a (1 / count) probability of happening.
                // This ensures that every element has an equal probability of being returned.
                // The first element has (1/2) * (2/3) .. (n-2/n-1)  (n-1/n) = (1/n) probability of being returned
                // The kth element has (1/k+1) * (k+1/k+2) .. (n-2/n-1)  (n-1/n) = (1/n) probability of being returned
                current = item;
            }
        }

        Some(current)
    }

    /// Returns a random maximum element with respect to the specified comparison function.
    ///
    /// If the iterator is empty, [`None`] is returned.
    /// Panics if the iterator has more than usize::Max maximum elements.
    fn random_max<R: rand::Rng>(self, rng: &mut R) -> Option<Self::Item>
    where
        Self::Item: Ord,
    {
        self.random_max_by(rng, Ord::cmp)
    }

    /// Returns a random element that gives the maximum value from the
    /// specified function.
    /// If the iterator is empty, [`None`] is returned.
    /// Panics if the iterator has more than usize::Max maximum elements.
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
        let mut count: usize = 1;

        for item in self {
            let item_key = f(&item);
            match item_key.cmp(&current_key) {
                core::cmp::Ordering::Less => {}
                core::cmp::Ordering::Equal => {
                    //Choose either iter or current randomly, see random_element for more
                    count = count.saturating_add(1);
                    if rng.gen_range(0..count) == 0 {
                        current_key = item_key;
                        current = item;
                    }
                }
                core::cmp::Ordering::Greater => {
                    current_key = item_key; //this is the new maximum
                    current = item;
                    count = 1;
                }
            }
        }

        Some(current)
    }

    /// Returns a random maximum element.
    ///
    /// If the iterator is empty, [`None`] is returned.
    /// Panics if the iterator has more than usize::Max maximum elements.
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
        let mut count: usize = 1;

        for item in self {
            match compare(&item, &current) {
                core::cmp::Ordering::Less => {}
                core::cmp::Ordering::Equal => {
                    //Choose either iter or current randomly, see random_element for more
                    count = count.saturating_add(1);
                    if rng.gen_range(0..count) == 0 {
                        current = item;
                    }
                }
                core::cmp::Ordering::Greater => {
                    current = item; //this is the new maximum
                    count = 1;
                }
            }
        }

        Some(current)
    }

    /// Return a random minimum element of the iterator.  
    /// Returns none if the iterator is empty.  
    /// Panics if the iterator has more than usize::Max elements.
    fn random_min<R: rand::Rng>(mut self, rng: &mut R) -> Option<Self::Item>
    where
        Self::Item: Ord,
    {
        let Some(first)  = self.next() else {
        return None;
    };

        let mut current = first;
        let mut count: usize = 1;

        for item in self {
            match item.cmp(&current) {
                core::cmp::Ordering::Greater => {}
                core::cmp::Ordering::Equal => {
                    //Choose either iter or current randomly, see random_element for more
                    count = count.saturating_add(1);
                    if rng.gen_range(0..count) == 0 {
                        current = item;
                    }
                }
                core::cmp::Ordering::Less => {
                    current = item; //this is the new minimum
                    count = 1;
                }
            }
        }

        Some(current)
    }

    /// Returns a random element that gives the minimum value from the
    /// specified function.
    /// If the iterator is empty, [`None`] is returned.
    /// Panics if the iterator has more than usize::Max minimum elements.
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
        let mut count: usize = 1;

        for item in self {
            let item_key = f(&item);
            match item_key.cmp(&current_key) {
                core::cmp::Ordering::Greater => {}
                core::cmp::Ordering::Equal => {
                    //Choose either iter or current randomly, see random_element for more
                    count = count.saturating_add(1);
                    if rng.gen_range(0..count) == 0 {
                        current_key = item_key;
                        current = item;
                    }
                }
                core::cmp::Ordering::Less => {
                    current_key = item_key; //this is the new minimum
                    current = item;
                    count = 1;
                }
            }
        }

        Some(current)
    }

    /// Returns a random minimum element.
    ///
    /// If the iterator is empty, [`None`] is returned.
    /// Panics if the iterator has more than usize::Max minimum elements.
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
        let mut count: usize = 1;

        for item in self {
            match compare(&item, &current) {
                core::cmp::Ordering::Greater => {}
                core::cmp::Ordering::Equal => {
                    //Choose either iter or current randomly, see random_element for more
                    count = count.saturating_add(1);
                    if rng.gen_range(0..count) == 0 {
                        current = item;
                    }
                }
                core::cmp::Ordering::Less => {
                    current = item; //this is the new minimum
                    count = 1;
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

    #[test]
    fn test_random_element_with_size_hint() {
        let mut counts: [usize; 100] = [0; 100];
        let mut rng = get_rng();

        for _ in 0..10000 {
            let range = 0..100;
            assert_eq!((100, Some(100)), range.size_hint());
            let element = range.random_element(&mut rng).unwrap();
            counts[element] += 1;
        }

        insta::assert_debug_snapshot!(counts);
        for x in counts {
            assert!(x > 60);
            assert!(x < 140);
        }

        assert_contains(10000..20000, &rng.count); // There should be at most two calls per iteration because we are using gen_range only once

        assert!(rng.count < 20000);
    }

    #[test]
    fn test_random_element_without_size_hint() {
        let mut counts: [usize; 100] = [0; 100];
        let mut rng = get_rng();

        #[inline(never)]
        fn return_true() -> bool {
            true
        }

        for _ in 0..10000 {
            let range = (0..100).filter(|_| return_true());
            assert_eq!((0, Some(100)), range.size_hint());
            let element = range.random_element(&mut rng).unwrap();
            counts[element] += 1;
        }

        insta::assert_debug_snapshot!(counts);

        for x in counts {
            assert!(x > 60);
            assert!(x < 140);
        }

        assert_contains(1000000..2000000, &rng.count);
    }

    #[test]
    fn test_random_max() {
        let mut counts: [usize; 100] = [0; 100];
        let mut rng = get_rng();

        for _ in 0..10000 {
            let range = (0..100).map(RoughNumber);
            let max = range.random_max(&mut rng).unwrap();
            counts[max.0] += 1;
        }

        insta::assert_debug_snapshot!(counts);

        for (i, &x) in counts.iter().enumerate() {
            if i < 90 {
                assert!(x == 0)
            } else {
                assert!(x > 600);
                assert!(x < 1400);
            }
        }

        assert_contains(900000..1800000, &rng.count);
    }

    #[test]
    fn test_random_max_by() {
        let mut counts: [usize; 100] = [0; 100];
        let mut rng = get_rng();

        for _ in 0..10000 {
            let range = 0..100;
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
                assert!(x > 600);
                assert!(x < 1400);
            }
        }

        assert_contains(900000..1800000, &rng.count);
    }

    #[test]
    fn test_random_max_by_key() {
        let mut counts: [usize; 100] = [0; 100];
        let mut rng = get_rng();

        for _ in 0..10000 {
            let range = 0..100;
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
                assert!(x > 600);
                assert!(x < 1400);
            }
        }

        assert_contains(900000..1800000, &rng.count);
    }

    #[test]
    fn test_random_min() {
        let mut counts: [usize; 100] = [0; 100];
        let mut rng = get_rng();

        for _ in 0..10000 {
            let range = (0..100).map(RoughNumber);
            let min = range.random_min(&mut rng).unwrap();
            counts[min.0] += 1;
        }

        insta::assert_debug_snapshot!(counts);

        for (i, &x) in counts.iter().enumerate() {
            if i >= 10 {
                assert!(x == 0)
            } else {
                assert!(x > 600);
                assert!(x < 1400);
            }
        }

        assert_contains(100000..200000, &rng.count);
    }

    #[test]
    fn test_random_min_by() {
        let mut counts: [usize; 100] = [0; 100];
        let mut rng = get_rng();

        for _ in 0..10000 {
            let range = 0..100;
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
                assert!(x > 600);
                assert!(x < 1400);
            }
        }

        assert_contains(100000..200000, &rng.count);
    }

    #[test]
    fn test_random_min_by_key() {
        let mut counts: [usize; 100] = [0; 100];
        let mut rng = get_rng();

        for _ in 0..10000 {
            let range = 0..100;
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
                assert!(x > 600);
                assert!(x < 1400);
            }
        }

        assert_contains(100000..200000, &rng.count);
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
