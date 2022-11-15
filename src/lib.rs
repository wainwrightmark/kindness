#![cfg_attr(not(test), no_std)]
#![doc(html_root_url = "https://docs.rs/kindness/0.1.0")]
#![allow(missing_docs)]
#![allow(warnings, dead_code, unused_imports, unused_mut)]

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
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     println!("Hello world!");
//!
//!     Ok(())
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

use rand::Rng;

/// Return a random element of the iterable.  
/// Returns none if the iterable is empty.  
/// Panics if the iterator has more than usize::Max elements.
/// Will iterate the entire enumerable unless it has a size hint which indicates an exact length
///
/// ```no_run
/// use kindness::random_element;
///
/// assert_eq!(random_element(0..10), Some(7)); //example value
/// ```
pub fn random_element<I, R: rand::RngCore>(iterable: I, rng: &mut R) -> Option<I::Item>
where
    I: IntoIterator,
{
    let mut iterator = iterable.into_iter();
    if let (lower, Some(upper)) = iterator.size_hint() {
        if lower == upper {
            //the iterator has an exact size
            if lower == 0 {
                return None;
            } else {
                let n = rng.gen_range(0..upper);
                return iterator.nth(n);
            }
        }
    }

    let Some(first)  = iterator.next() else {
        return None;
    };

    let mut current = first;
    let mut count: usize = 1;

    for i in iterator {
        count += 1;
        if rng.gen_range(0..count) == 0 {
            current = i;
        }
    }

    Some(current)
}

impl<T: Iterator + Sized> Kindness for T {}

/// An [`Iterator`] blanket implementation that provides extra adaptors and
/// methods for returning random elements.
pub trait Kindness: Iterator
where
    Self: Sized,
{
    /// Return a random element of the iterator.  
    /// Returns none if the iterator is empty.  
    /// Panics if the iterator has more than usize::Max elements.
    /// Will iterate the entire enumerable unless it has a size hint which indicates an exact length
    ///
    /// ```no_run
    /// use kindness::random_element;
    ///
    /// assert_eq!(random_element(0..10), Some(7)); //example value
    /// ```
    fn random_element<R: rand::Rng>(self, rng: &mut R) -> Option<Self::Item> {
        random_element(self, rng)
    }
}

#[cfg(test)]
mod tests {
    use crate::Kindness;
    use rand::{Rng, RngCore, SeedableRng};

    #[test]
    fn test_random_element_with_size_hint() {
        let mut counts: [usize; 100] = [0; 100];
        let mut inner_rng = rand::rngs::StdRng::seed_from_u64(123);
        let mut rng = CountingRng {
            rng: inner_rng,
            count: 0,
        };

        for _ in 0..10000 {
            let range = 0..100;
            assert_eq!((100, Some(100)), range.size_hint());
            let element = range.random_element(&mut rng).unwrap();
            counts[element] += 1;
        }

        insta::assert_debug_snapshot!(counts);
        for x in counts{
            assert!(x > 60);
            assert!(x < 140);
        }

        assert!(rng.count < 20000); // There should be at most two calls per iteration because we are using gen_range only once
    }

    #[inline(never)]
    fn return_true() -> bool {
        true
    }

    #[test]
    fn test_random_element_without_size_hint() {
        let mut counts: [usize; 100] = [0; 100];
        let mut inner_rng = rand::rngs::StdRng::seed_from_u64(123);
        let mut rng = CountingRng {
            rng: inner_rng,
            count: 0,
        };

        for _ in 0..10000 {
            let range = (0..100).filter(|x| return_true());
            assert_eq!((0, Some(100)), range.size_hint());
            let element = range.random_element(&mut rng).unwrap();
            counts[element] += 1;
        }

        insta::assert_debug_snapshot!(counts);

        for x in counts{
            assert!(x > 60);
            assert!(x < 140);
        }

        assert!(rng.count > 1000000);
        assert!(rng.count < 2000000);        
    }

    struct CountingRng<Inner: RngCore> {
        pub rng: Inner,
        pub count: usize,
    }

    impl<Inner: RngCore> RngCore for CountingRng<Inner> {
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
