// https://leetcode.com/problems/largest-divisible-subset/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        // it's like LIS
        // solved it before with take or leave pattern
        // now we'll do it like LIS for loop pattern
        nums.sort_unstable();
        let n = nums.len();
        let mut memo = vec![None; n];
        let mut result = vec![];

        for i in 0..n {
            let temp = dp(i, &nums, &mut memo);
            if temp.len() > result.len() {
                result = temp;
            }
        }

        result
    }
}

fn dp(i: usize, nums: &[i32], memo: &mut Vec<Option<Vec<i32>>>) -> Vec<i32> {
    if let Some(cached) = &memo[i] {
        return cached.clone();
    }

    let mut best = vec![nums[i]];

    for j in (i + 1)..nums.len() {
        if nums[j] % nums[i] == 0 {
            let subset = dp(j, nums, memo);
            if subset.len() + 1 > best.len() {
                let mut new_best = vec![nums[i]];
                new_best.extend(subset);
                best = new_best;
            }
        }
    }

    memo[i] = Some(best.clone());
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 2, 3];
        let output = vec![1, 2];
        let result = Solution::largest_divisible_subset(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 2, 4, 8];
        let output = vec![1, 2, 4, 8];
        let result = Solution::largest_divisible_subset(nums);
        assert_eq!(result, output);
    }
}
