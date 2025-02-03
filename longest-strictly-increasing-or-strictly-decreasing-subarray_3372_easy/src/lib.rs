// https://leetcode.com/problems/longest-strictly-increasing-or-strictly-decreasing-subarray/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        // two loops (O(n + n)), one check for increasing and other check for decreasing
        // after solving
        // the two loops could've been combined into one, but it didn't take more than 5 mins to
        // solve any way
        let n = nums.len();
        if n == 1 {
            return 1;
        }

        // increasing
        let mut max_increasing = 0;
        let mut cur_increasing = 0;
        for i in 1..n {
            if nums[i] > nums[i - 1] {
                cur_increasing += 1;
            } else {
                cur_increasing = 0;
            }
            max_increasing = max_increasing.max(cur_increasing + 1);
        }

        // decreasing
        let mut max_decreasing = 0;
        let mut cur_decreasing = 0;
        for i in 1..n {
            if nums[i] < nums[i - 1] {
                cur_decreasing += 1;
            } else {
                cur_decreasing = 0;
            }
            max_decreasing = max_decreasing.max(cur_decreasing + 1);
        }

        max_increasing.max(max_decreasing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 4, 3, 3, 2];
        let output = 2;
        let result = Solution::longest_monotonic_subarray(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![3, 3, 3, 3];
        let output = 1;
        let result = Solution::longest_monotonic_subarray(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![3, 2, 1];
        let output = 3;
        let result = Solution::longest_monotonic_subarray(nums);
        assert_eq!(result, output);
    }
}
