// https://leetcode.com/problems/maximum-ascending-subarray-sum/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        // O(n^2) is trivial as getting windows of the array
        // but O(n) is almost as trivial too
        // count if ascen, reset count if not
        let mut max_sum = 0;
        let mut cur_sum = nums[0];
        let max_element = *nums.iter().max().unwrap();
        let n = nums.len();
        if n == 1 {
            return cur_sum;
        }
        for i in 1..n {
            if nums[i] > nums[i - 1] {
                dbg!(i);
                cur_sum += nums[i]
            } else {
                cur_sum = nums[i];
            }
            max_sum = max_sum.max(cur_sum);
        }
        max_sum.max(max_element)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![10, 20, 30, 5, 10, 50];
        let output = 65;
        let result = Solution::max_ascending_sum(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![10, 20, 30, 40, 50];
        let output = 150;
        let result = Solution::max_ascending_sum(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![12, 17, 15, 13, 10, 11, 12];
        let output = 33;
        let result = Solution::max_ascending_sum(nums);
        assert_eq!(result, output);
    }
}
