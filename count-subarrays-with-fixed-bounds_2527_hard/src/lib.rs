// https://leetcode.com/problems/count-subarrays-with-fixed-bounds/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        let mut l = 0;
        let n = nums.len();
        let mut min_idx = None;
        let mut max_idx = None;
        let mut result = 0;
        for r in 0..n {
            if nums[r] > max_k || nums[r] < min_k {
                min_idx = None;
                max_idx = None;
                l = r + 1;
            }

            if nums[r] == min_k {
                min_idx = Some(r);
            }

            if nums[r] == max_k {
                max_idx = Some(r);
            }

            if let (Some(min_idx), Some(max_idx)) = (min_idx, max_idx) {
                result += min_idx.min(max_idx) - l + 1;
            }
        }
        result as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 3, 5, 2, 7, 5];
        let min_k = 1;
        let max_k = 5;
        let output = 2;
        let result = Solution::count_subarrays(nums, min_k, max_k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 1, 1, 1];
        let min_k = 1;
        let max_k = 1;
        let output = 10;
        let result = Solution::count_subarrays(nums, min_k, max_k);
        assert_eq!(result, output);
    }
}
