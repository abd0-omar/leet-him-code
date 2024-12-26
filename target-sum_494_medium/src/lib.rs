// https://leetcode.com/problems/target-sum/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
use std::collections::HashMap;
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        // solved before, but we'll redo it anyway
        let mut memory = HashMap::new();
        dp(&nums, 0, target, &mut memory)
    }
}

fn dp(nums: &[i32], idx: usize, target: i32, memory: &mut HashMap<(usize, i32), i32>) -> i32 {
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
    // positive
    let choice1 = dp(nums, idx + 1, target - nums[idx], memory);
    // negative
    let choice2 = dp(nums, idx + 1, target + nums[idx], memory);
    let result = choice1 + choice2;
    memory.insert((idx, target), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 1, 1, 1, 1];
        let target = 3;
        let output = 5;
        let result = Solution::find_target_sum_ways(nums, target);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1];
        let target = 1;
        let output = 1;
        let result = Solution::find_target_sum_ways(nums, target);
        assert_eq!(result, output);
    }
}
