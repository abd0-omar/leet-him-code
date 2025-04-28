// https://leetcode.com/problems/count-subarrays-with-score-less-than-k/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        // two ways to solve this
        // sliding window with (j - i + 1), which is a cool formula, as to count the windows O(n)
        // prefix_sum + binary search, which i is our start and binary search is
        // our end, O(nlogn)
        let mut result = 0;
        let mut total = 0i64;
        let mut i = 0;
        for j in 0..nums.len() {
            total += nums[j] as i64;
            while i <= j && total * (j - i + 1) as i64 >= k {
                total -= nums[i] as i64;
                i += 1;
            }
            result += j - i + 1;
        }
        result as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![2, 1, 4, 3, 5];
        let k = 10;
        let output = 6;
        let result = Solution::count_subarrays(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 1, 1];
        let k = 5;
        let output = 5;
        let result = Solution::count_subarrays(nums, k);
        assert_eq!(result, output);
    }
}
