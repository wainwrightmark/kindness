use rand::{Rng, RngCore};

/// ∀ n ∊ `1..U64::MAX` ∀ k ∊ `0..n`  
/// After n items have been returned, the probability that the highest index where true was returned is k is 1 / n
pub(crate) struct ChoiceIterator<R: RngCore> {
    rng: R,
    returned: usize,
}

impl<R: RngCore> ChoiceIterator<R> {
    /// Create a new choice iterator
    pub fn new(rng: R, returned: usize) -> Self {
        Self { rng, returned }
    }

    // /// Reset the choice iterator. The next item returned will be true
    // pub fn reset(&mut self){
    //     self.returned = 0;
    // }

    /// Reset the choice iterator to the one state. The next item returned has a fifty percent change of being true.
    pub fn reset_to_one(&mut self) {
        self.returned = 1;
    }
}

impl<R: RngCore> Iterator for ChoiceIterator<R> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        self.returned = self.returned.saturating_add(1);
        let r = self.rng.gen_range(0..self.returned);

        //We only return true if the index is 0.
        // This has a (1 / returned) probability of happening.
        // This ensures that every element has an equal probability of being returned.
        // The first element has (1/2) * (2/3) .. (n-2/n-1)  (n-1/n) = (1/n) probability of being returned
        // The kth element has (1/k+1) * (k+1/k+2) .. (n-2/n-1)  (n-1/n) = (1/n) probability of being returned
        Some(r == 0)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}
