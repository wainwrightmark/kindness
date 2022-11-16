use rand::{Rng, RngCore};

fn calculate_upper(min: u64) -> (u64, u8) {
    const fn inner_func(min: u64) -> (u64, u8) {
        let mut product = 1;
        let mut count: u64 = 0;

        loop {
            let n = if min + count == 0 { 1 } else { min + count };
            if let Some(p) = u64::checked_mul(product, n) {
                product = p;
                count += 1;
            } else {
                return (product, count as u8);
            }
        }
    }

    const RESULT2: (u64, u8) = inner_func(2);
    if min == 2 {        
        return RESULT2;
    }

    inner_func(min)
}

/// ∀ n ∊ `1..U64::MAX` ∀ k ∊ `0..n`  
/// After n items have been returned, the probability that the highest index where true was returned is k is 1 / n
pub(crate) struct ChoiceIterator<R: RngCore> {
    pub rng: R,
    consumed: usize,
    chunk: u64,
    chunk_remaining: u8
}

/* */
impl<R: RngCore> ChoiceIterator<R> {
    /// Create a new choice iterator
    pub fn new_zero(rng: R) -> Self {
        Self {
            rng,
            consumed : 0,
            //Set the chunk to one here because the first element is always true
            chunk: 1,
            chunk_remaining: 1
        }
    }
    
    pub fn new_one(rng: R) -> Self {
        Self {
            rng,
            consumed : 1,
            //The chunk will be reset the first time we try to get an element
            chunk: 0,
            chunk_remaining: 0
        }
    }

    pub fn get_consumed(&self)-> usize{self.consumed}
    pub fn set_consumed(&mut self, consumed:usize){
        self.consumed = consumed;
        self.chunk = 0;
        self.chunk_remaining = 0;
    }
}

impl<R: RngCore> Iterator for ChoiceIterator<R> {
    type Item = bool;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {    
        self.consumed = self.consumed.saturating_add(1);

        if self.chunk_remaining == 0 {
            let (bound, remaining) = calculate_upper(self.consumed as u64);
            self.chunk_remaining = remaining;
            self.chunk = self.rng.gen_range(0..bound);            
        }

        
        let result = self.chunk % (self.consumed  as u64) == 0;
        self.chunk = self.chunk / (self.consumed  as u64);
        self.chunk_remaining -= 1;
        Some(result)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}

/*
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
 */