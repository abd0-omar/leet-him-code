fn main() {
    println!("Hello, world!");
    let nums = vec![-1, 0, 1, 2, -1, -4, -2, -3, 3, 0, 4];
    println!("{:?}", three_sum(nums));
}

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut hm = std::collections::HashMap::new();
    let mut result = std::collections::HashSet::new();

    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            //          -1         2   -> 1 so we need to search for -1
            let diff = nums[i] + nums[j];
            hm.entry(-diff).or_insert(vec![]).push((i, j));
        }
    }

    for i in 0..nums.len() {
        if let Some(f) = hm.get(&nums[i]) {
            // println!("f={:?}", f);
            for &(x, y) in f {
                if i != x && i != y {
                    let mut v = vec![nums[x], nums[y], nums[i]];
                    v.sort_unstable();
                    result.insert(v);
                }
            }
        }
    }
    result.iter().cloned().collect()
}
