#[cfg(any(test, feature = "std"))]
pub mod iterators {
    use core::hash::Hash;
    use core::iter::FusedIterator;
    use hashbrown::{hash_map::*, HashTable};

    #[derive(Debug)]
    pub struct Unique<Item, A: allocator_api2::alloc::Allocator + Clone> {
        table: hashbrown::hash_table::IntoIter<(Item, usize), A>,
    }

    impl<Item, A: allocator_api2::alloc::Allocator + Clone> Unique<Item, A> {
        pub fn new(table: hashbrown::hash_table::IntoIter<(Item, usize), A>) -> Self {
            Self { table }
        }
    }

    impl<Item, A: allocator_api2::alloc::Allocator + Clone> ExactSizeIterator for Unique<Item, A> {}

    impl<Item, A: allocator_api2::alloc::Allocator + Clone> FusedIterator for Unique<Item, A> {}

    impl<Item, A: allocator_api2::alloc::Allocator + Clone> Iterator for Unique<Item, A> {
        type Item = Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.table.next().map(|x| x.0)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            self.table.size_hint()
        }

        fn count(self) -> usize
        where
            Self: Sized,
        {
            self.table.count()
        }

        fn last(self) -> Option<Self::Item>
        where
            Self: Sized,
        {
            self.table.map(|x| x.0).last()
        }

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            self.table.nth(n).map(|x| x.0)
        }
    }

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

    impl<K: Eq + Hash, Item, A: allocator_api2::alloc::Allocator + Clone> ExactSizeIterator
        for UniqueByKey<K, Item, A>
    {
    }

    impl<K: Eq + Hash, Item, A: allocator_api2::alloc::Allocator + Clone> FusedIterator
        for UniqueByKey<K, Item, A>
    {
    }

    impl<K: Eq + Hash, Item, A: allocator_api2::alloc::Allocator + Clone> Iterator
        for UniqueByKey<K, Item, A>
    {
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
