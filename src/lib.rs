use std::hash::{BuildHasher, Hash, RandomState};
use std::marker::PhantomData;

const ITEMS_SIZE: usize = 1024;
const HASH_COUNT: usize = 5;

pub struct BloomFilter<T: Hash> {
    items: Vec<usize>,
    states: Vec<RandomState>,
    phantom: PhantomData<T>,
}

impl<T: Hash> Default for BloomFilter<T> {
    fn default() -> Self {
        Self::new(ITEMS_SIZE, HASH_COUNT)
    }
}

impl<T: Hash> BloomFilter<T> {
    pub fn new(items_size: usize, hash_count: usize) -> Self {
        let items = vec![0; items_size];
        let states = vec![0; hash_count]
            .into_iter()
            .map(|_| RandomState::new())
            .collect();

        BloomFilter {
            items,
            states,
            phantom: PhantomData,
        }
    }

    pub fn insert(&mut self, item: &T) {
        for state in &self.states {
            let index = state.hash_one(item) as usize % self.items.capacity();
            self.items[index] = 1;
        }
    }

    pub fn exist(&self, item: &T) -> bool {
        for state in &self.states {
            let index = state.hash_one(item) as usize % self.items.capacity();
            if self.items[index] == 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::BloomFilter;
    use proptest::prelude::*;

    #[test]
    fn simple_exist() {
        let mut b = BloomFilter::default();
        b.insert(&123);
        assert!(b.exist(&123));
        assert!(!b.exist(&456));
    }

    proptest! {
        #[test]
        fn insert_exist(v in any::<i32>()) {
            let mut b = BloomFilter::default();
            b.insert(&v);
            assert!(b.exist(&v));
        }
    }
}
