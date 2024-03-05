fn main() {
    println!("Hello, world!");
    let nums = vec![3, 2, 3];
    println!("{:?}", majority_element(nums));

    let nums = vec![3, 2, 3, 2, 3, 3];
    println!("{:?}", majority_element(nums));
}

pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::{HashMap, HashSet};
    let mut hm = HashMap::new();
    if nums.len() < 3 {
        let mut hs = HashSet::new();
        for num in nums {
            hs.insert(num);
        }
        return hs.iter().map(|&x| x).collect();
    }
    let division: f32 = nums.len() as f32 / 3.0;
    println!("DEBUGPRINT[2]: main.rs:14: division={:#?}", division);

    let mut hs = HashSet::new();
    for &num in &nums {
        let count = hm.entry(num).or_insert(0);
        *count += 1;
        if *count as f32 > division {
            println!("DEBUGPRINT[1]: main.rs:16: count={:#?}", count);
            hs.insert(num);
        }
    }
    hs.iter().map(|&x| x).collect::<_>()
}
