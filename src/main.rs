use std::hash::{BuildHasher, Hash, RandomState};
use std::marker::PhantomData;

const ITEMS_SIZE: usize = 1024;
const HASH_COUNT: usize = 5;

struct BloomFilter<T: Hash> {
    items: Vec<usize>,
    states: Vec<RandomState>,
    phantom: PhantomData<T>,
}

impl<T: Hash> BloomFilter<T> {
    fn new() -> Self {
        let items = vec![0; ITEMS_SIZE];
        let states = vec![0; HASH_COUNT]
            .into_iter()
            .map(|_| RandomState::new())
            .collect();

        BloomFilter {
            items,
            states,
            phantom: PhantomData,
        }
    }

    fn insert(&mut self, item: &T) {
        for state in &self.states {
            let index = state.hash_one(item) as usize % self.items.capacity();
            self.items[index] = 1;
        }
    }

    fn exist(&self, item: &T) -> bool {
        for state in &self.states {
            let index = state.hash_one(item) as usize % self.items.capacity();
            if self.items[index] == 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    let mut b = BloomFilter::new();
    b.insert(&123);
    assert!(b.exist(&123));
    assert!(!b.exist(&456));
}
