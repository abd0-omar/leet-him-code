use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let nums = vec![2, 3, 3, 2, 2, 4, 2, 3, 4];
    // Output: 4
    let nums = vec![2, 1, 2, 2, 3, 3];
    println!("{}", min_operations(nums));
}

pub fn min_operations(nums: Vec<i32>) -> i32 {
    let mut hm = HashMap::new();
    for &num in nums.iter() {
        *hm.entry(num).or_insert(0) += 1;
    }
    // println!("hm={:?}", hm);
    let mut count = 0;
    for val in hm.values_mut() {
        if *val == 1 {
            return -1;
        }
        count += f32::ceil(*val as f32 / 3 as f32) as i32;
    }
    count
}
