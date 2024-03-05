fn main() {
    println!("Hello, world!");
}

pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut hm = std::collections::HashMap::new();
    for num in nums {
        *hm.entry(num).or_insert(0) += 1;
    }
    let (max, _) = hm.iter().max_by(|k, v| k.1.cmp(v.1)).unwrap();
    *max
}
