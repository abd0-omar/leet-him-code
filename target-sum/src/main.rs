use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let nums = vec![1, 1, 1, 1, 1];
    let target = 3;
    // Output: 5
    println!("{}", find_target_sum_ways(nums, target));
}

pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
    let mut memory = HashMap::new();
    _find_target_sum_ways(&nums, target, 0, &mut memory)
}

pub fn _find_target_sum_ways(
    nums: &Vec<i32>,
    target: i32,
    idx: usize,
    memory: &mut HashMap<(usize, i32), i32>,
) -> i32 {
    if idx == nums.len() {
        if target == 0 {
            return 1;
        } else {
            return 0;
        }
    }

    if let Some(&ret) = memory.get(&(idx, target)) {
        return ret;
    }

    // take normal
    let choice1 = _find_target_sum_ways(nums, target - nums[idx], idx + 1, memory);
    // take nega
    let choice2 = _find_target_sum_ways(nums, target + nums[idx], idx + 1, memory);
    memory.insert((idx, target), choice1 + choice2);
    *memory.get(&(idx, target)).unwrap()
}
