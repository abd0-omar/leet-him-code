#![allow(non_snake_case)]
fn main() {
    println!("Hello, world!");
    let nums = vec![8, 8, 8, 8, 2, 4, 4, 2, 4];
    println!("{:?}", find_matrix(nums));
}

pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut hm = std::collections::HashMap::new();
    let mut bm = std::collections::HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let count = hm.entry(num).or_insert(0);
        *count += 1;
        // *bm.entry(num).or_insert(0) += 1;
        bm.insert(i, *count);
    }
    let mut bm = bm.iter().collect::<Vec<_>>();
    bm.sort_unstable_by(|a, b| a.1.cmp(b.1));
    let mut v = vec![];
    let mut vec = vec![];
    // for i in 1..bm.len() - 1 {
    //     if bm[i].1 > bm[i - 1].1 {
    //         println!("hello");
    //         vec.push(v.clone());
    //         v.clear();
    //     }
    //     v.push(nums[*bm[i].0]);
    // }
    for i in 0..bm.len() {
        if i != 0 && bm[i].1 > bm[i - 1].1 {
            vec.push(v.clone());
            v.clear()
        }
        v.push(nums[*bm[i].0]);
    }
    // if !v.is_empty() {
    vec.push(v);
    // }
    // println!("hm={:?}", hm);
    // println!("bm={:?}", bm);
    vec
}
