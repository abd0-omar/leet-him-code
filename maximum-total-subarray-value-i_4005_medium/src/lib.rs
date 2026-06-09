// https://leetcode.com/problems/maximum-total-subarray-value-i/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let max = *nums.iter().max().unwrap() as i64;
        let min = *nums.iter().min().unwrap() as i64;
        (max - min) * k as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 3, 2];
        let k = 2;
        let output = 4;
        let result = Solution::max_total_value(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![4, 2, 5, 1];
        let k = 3;
        let output = 12;
        let result = Solution::max_total_value(nums, k);
        assert_eq!(result, output);
    }
}
