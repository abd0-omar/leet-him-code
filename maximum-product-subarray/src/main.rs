use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let nums = vec![2, 3, -2, 4];
    println!("{}", max_product(nums)); // Output: 6

    let nums = vec![-2, 0, -1];
    println!("{}", max_product(nums)); // Output: 0

    let nums = vec![-3, -1, -1];
    println!("{}", max_product(nums)); // Output: 3
}

pub fn max_product(nums: Vec<i32>) -> i32 {
    let mut memory = HashMap::new();
    _max_product(&nums, 0, None, 1, &mut memory)
}

pub fn _max_product(
    nums: &Vec<i32>,
    idx: usize,
    prev: Option<usize>,
    res: i32,
    memory: &mut HashMap<(usize, Option<usize>, i32), i32>,
) -> i32 {
    if idx == nums.len() {
        return res;
    }

    if let Some(&ret) = memory.get(&(idx, prev, res)) {
        return ret;
    }

    let choice1 = _max_product(nums, idx + 1, prev, res, memory);

    let mut choice2 = 0;
    if let Some(p) = prev {
        if p == idx - 1 {
            choice2 = _max_product(nums, idx + 1, Some(idx), res * nums[idx], memory);
        }
    } else {
        choice2 = _max_product(nums, idx + 1, Some(idx), res * nums[idx], memory);
    }

    let result = choice1.max(choice2.max(res));
    memory.insert((idx, prev, res), result);
    result
}
