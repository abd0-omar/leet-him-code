// https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        // at least k subarrays pattern
        let max_element = *nums.iter().max().unwrap();
        let mut l = 0;
        let n = nums.len();
        let mut count = 0;
        let mut result = 0;
        for r in 0..n {
            if nums[r] == max_element {
                count += 1;
            }
            // while valid
            while count >= k {
                result += n - r;
                if nums[l] == max_element {
                    count -= 1;
                }
                l += 1
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
        let nums = vec![1, 3, 2, 3, 3];
        let k = 2;
        let output = 6;
        let result = Solution::count_subarrays(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 4, 2, 1];
        let k = 3;
        let output = 0;
        let result = Solution::count_subarrays(nums, k);
        assert_eq!(result, output);
    }
}
