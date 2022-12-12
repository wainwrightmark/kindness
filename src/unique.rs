#[cfg(any(test, feature = "std"))]
pub mod iterators {
    use core::iter::FusedIterator;

    #[derive(Debug)]
    /// An iterator adapter to filter out duplicate elements.
    pub struct Unique<K: Eq + std::hash::Hash> {
        map: std::collections::hash_map::IntoKeys<K, usize>,
    }

    impl<K: Eq + std::hash::Hash> Unique<K> {
        pub fn new(map: std::collections::hash_map::IntoKeys<K, usize>) -> Self {
            Self { map }
        }
    }

    impl<K: Eq + std::hash::Hash> ExactSizeIterator for Unique<K> {}

    impl<K: Eq + std::hash::Hash> FusedIterator for Unique<K> {}

    impl<K: Eq + std::hash::Hash> Iterator for Unique<K> {
        type Item = K;

        fn next(&mut self) -> Option<Self::Item> {
            self.map.next()
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
            self.map.last()
        }

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            self.map.nth(n)
        }
    }

    #[derive(Debug)]
    /// An iterator adapter to filter out duplicate elements by a key.
    pub struct UniqueByKey<K: Eq + std::hash::Hash, Item> {
        map: std::collections::hash_map::IntoValues<K, (Item, usize)>,
    }

    impl<K: Eq + std::hash::Hash, Item> UniqueByKey<K, Item> {
        pub fn new(map: std::collections::hash_map::IntoValues<K, (Item, usize)>) -> Self {
            Self { map }
        }
    }

    impl<K: Eq + std::hash::Hash, Item> ExactSizeIterator for UniqueByKey<K, Item> {}

    impl<K: Eq + std::hash::Hash, Item> FusedIterator for UniqueByKey<K, Item> {}

    impl<K: Eq + std::hash::Hash, Item> Iterator for UniqueByKey<K, Item> {
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
