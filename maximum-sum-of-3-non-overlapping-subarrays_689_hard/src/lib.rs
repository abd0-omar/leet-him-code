// https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
use std::collections::HashMap;
impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // sum first k elements
        let mut k_sums = vec![*&nums[0..k as usize].iter().sum::<i32>()];
        let n = nums.len();
        for i in (k as usize)..n {
            // move by one
            // remove first one and add new one, for last query
            k_sums.push(k_sums.last().unwrap() + nums[i] - nums[i - k as usize]);
        }
        let mut memory = HashMap::new();
        get_indices(&nums, k, &mut k_sums, &mut memory)
            .into_iter()
            .map(|x| x as i32)
            .collect()
    }
}

fn get_max_sum(
    nums: &[i32],
    k: i32,
    i: usize,
    count: i32,
    k_sums: &mut Vec<i32>,
    memory: &mut HashMap<(i32, usize), i32>,
) -> i32 {
    if count == 3 || i as i32 > nums.len() as i32 - k {
        return 0;
    }
    if let Some(&ret) = memory.get(&(count, i)) {
        return ret;
    }
    // take
    let take = k_sums[i] + get_max_sum(nums, k, i + k as usize, count + 1, k_sums, memory);
    // leave
    let leave = get_max_sum(nums, k, i + 1, count, k_sums, memory);
    let result = take.max(leave);
    memory.insert((count, i), result);
    result
}

fn get_indices(
    nums: &[i32],
    k: i32,
    mut k_sums: &mut Vec<i32>,
    mut memory: &mut HashMap<(i32, usize), i32>,
) -> Vec<usize> {
    let mut i = 0;
    let mut indices = Vec::new();

    let n = nums.len();
    while i <= n - k as usize && indices.len() < 3 {
        let take = k_sums[i]
            + get_max_sum(
                nums,
                k,
                i + k as usize,
                indices.len() as i32 + 1,
                &mut k_sums,
                &mut memory,
            );
        let leave = get_max_sum(
            nums,
            k,
            i + 1,
            indices.len() as i32,
            &mut k_sums,
            &mut memory,
        );
        if take >= leave {
            indices.push(i);
            i += k as usize;
        } else {
            i += 1;
        }
    }
    indices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 2, 1, 2, 6, 7, 5, 1];
        let k = 2;
        let output = vec![0, 3, 5];
        let result = Solution::max_sum_of_three_subarrays(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 2, 1, 2, 1, 2, 1, 2, 1];
        let k = 2;
        let output = vec![0, 2, 4];
        let result = Solution::max_sum_of_three_subarrays(nums, k);
        assert_eq!(result, output);
    }
}
