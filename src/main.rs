use bloom_filter_rs::BloomFilter;

fn main() {
    let mut b = BloomFilter::default();
    b.insert(&123);
    assert!(b.exist(&123));
    assert!(!b.exist(&456));
}
