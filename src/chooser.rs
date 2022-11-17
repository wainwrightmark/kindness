// use rand::{Rng, RngCore};

// fn calculate_upper(min: u64) -> (u64, u8) {
//     const fn inner_func(min: u64) -> (u64, u8) {
//         let mut product = 1;
//         let mut count: u8 = 0;

//         loop {
//             if let Some(p) = u64::checked_mul(product, min + (count as u64)) {
//                 product = p;
//                 count += 1;
//             } else {
//                 return (product, count);
//             }
//         }
//     }

//     const RESULT2: (u64, u8) = inner_func(2);
//     if min == 2 {
//         return RESULT2;
//     }

//     inner_func(min)
// }

// /// ∀ n ∊ `1..U64::MAX` ∀ k ∊ `0..n`
// /// After n items have been returned, the probability that the highest index where true was returned is k is 1 / n
// pub(crate) struct Chooser<R: RngCore> {
//     pub rng: R,
//     consumed: usize,
//     chunk: u64,
//     chunk_remaining: u8,
// }

// /* */
// impl<R: RngCore> Chooser<R> {
//     /// Create a new choice iterator
//     pub fn new_zero(rng: R) -> Self {
//         Self {
//             rng,
//             consumed: 0,
//             //Set the chunk to one here because the first element is always true
//             chunk: 1,
//             chunk_remaining: 1,
//         }
//     }

//     pub fn new_one(rng: R) -> Self {
//         Self {
//             rng,
//             consumed: 1,
//             //The chunk will be reset the first time we try to get an element
//             chunk: 0,
//             chunk_remaining: 0,
//         }
//     }

//     /// Resets the chunk to one. Does not reset the chunk, but decrements the `chunk_remaining` by 1;
//     /// This is okay because the product of any sequence of n consecutive integers is a multiple of the product of the first n integers
//     pub fn set_consumed_to_one(&mut self) { //TODO remove this
//         self.consumed = 1;
//         if self.chunk_remaining > 0 {
//             self.chunk_remaining -= 1;
//         }
//     }

//     pub fn get_consumed(&self) -> usize {
//         self.consumed
//     }
//     pub fn set_consumed(&mut self, consumed: usize) {
//         self.consumed = consumed;
//         //no need to reset the chunk
//         self.chunk_remaining = 0; //TODO subtract consumed from chunk_remaining instead - check this is valid
//     }

//     #[inline]
//     pub fn next(&mut self) -> bool {
//         self.consumed = self.consumed.saturating_add(1);

//         if self.chunk_remaining == 0 {
//             let (bound, remaining) = calculate_upper(self.consumed as u64);
//             self.chunk_remaining = remaining;
//             self.chunk = self.rng.gen_range(0..bound);
//         }

//         let result = self.chunk % (self.consumed as u64) == 0;
//         self.chunk /= self.consumed as u64;
//         self.chunk_remaining -= 1;
//         result
//     }
// }
