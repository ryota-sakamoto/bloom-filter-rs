use bloom_filter_rs::BloomFilter;

fn main() {
    let inserted_ratio = [0.01, 0.05, 0.1, 0.25, 0.5, 0.75, 1.0];

    println!(
        "| items_size | hash_count |{}|",
        inserted_ratio
            .iter()
            .map(|v| format!(" {v} "))
            .collect::<Vec<_>>()
            .join("|")
    );

    println!(
        "| --- | --- |{}|",
        inserted_ratio
            .iter()
            .map(|_| " --- ")
            .collect::<Vec<_>>()
            .join("|")
    );

    let mut result = Vec::new();
    for items_size in [10, 100, 1000, 10000, 100000] {
        for hash_count in 1..10 {
            let mut inserted_result = Vec::new();
            for ratio in inserted_ratio {
                let mut b = BloomFilter::new(items_size, hash_count);

                let max_num = (items_size as f64 * ratio) as usize;
                for i in 0..max_num {
                    b.insert(&i);
                }

                let mut count = 0;
                for i in max_num..(1000000 + max_num) {
                    if b.exist(&i) {
                        count += 1;
                    }
                }

                inserted_result.push(count as f64 / 1000000.0);
            }

            result.push((items_size, hash_count, inserted_result));
        }
    }

    for (items_size, hash_count, inserted_result) in result {
        println!(
            "| {items_size} | {hash_count} |{}|",
            inserted_result
                .iter()
                .map(|v| format!(" {v:.3} "))
                .collect::<Vec<_>>()
                .join("|")
        )
    }
}
