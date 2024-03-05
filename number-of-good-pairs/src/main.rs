fn main() {
    let nums = vec![1, 2, 3, 1, 1, 3];
    println!("{}", num_identical_pairs(nums));
}

pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut hs = HashMap::new();

    for num in nums {
        *hs.entry(num).or_insert(0) += 1;
        // println!("DEBUGPRINT[1]: main.rs:10: hs={:#?}", hs);
    }

    let mut count = 0;
    for (_, val) in hs {
        let formula = (val * (val - 1)) / 2;
        count += formula;
    }
    count
}
