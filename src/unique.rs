#[cfg(any(test, feature = "std"))]
pub mod iterators {
    use core::hash::Hash;
    use core::iter::FusedIterator;
    use hashbrown::hash_map::*;

    pub type Unique<Item, A: allocator_api2::alloc::Allocator + Clone> = IntoKeys<Item, usize, A>;

    #[derive(Debug)]
    /// An iterator adapter to filter out duplicate elements by a key.
    pub struct UniqueByKey<K: Eq + Hash, Item, A: allocator_api2::alloc::Allocator + Clone> {
        map: IntoValues<K, (Item, usize), A>,
    }

    impl<K: Eq + Hash, Item, A: allocator_api2::alloc::Allocator + Clone> UniqueByKey<K, Item, A> {
        pub fn new(map: IntoValues<K, (Item, usize), A>) -> Self {
            Self { map }
        }
    }

    impl<K: Eq + Hash, Item, A: allocator_api2::alloc::Allocator + Clone> ExactSizeIterator for UniqueByKey<K, Item, A> {}

    impl<K: Eq + Hash, Item, A: allocator_api2::alloc::Allocator + Clone> FusedIterator for UniqueByKey<K, Item, A> {}

    impl<K: Eq + Hash, Item, A: allocator_api2::alloc::Allocator + Clone> Iterator for UniqueByKey<K, Item, A> {
        type Item = Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.map.next().map(|x| x.0)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.map.size_hint()
        }

        fn count(self) -> usize
        where
            Self: Sized,
        {
            self.map.count()
        }

        fn last(self) -> Option<Self::Item>
        where
            Self: Sized,
        {
            self.map.map(|x| x.0).last()
        }

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            self.map.nth(n).map(|x| x.0)
        }
    }
}
