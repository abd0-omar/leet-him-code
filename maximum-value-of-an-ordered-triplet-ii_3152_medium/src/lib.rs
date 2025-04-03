// https://leetcode.com/problems/maximum-value-of-an-ordered-triplet-ii/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        // maintain prefix max and max diff (could be prefix min but max diff is
        // more appropriate here)
        // and no need for arrays, variables are enough
        let mut prefix_max = nums[0] as i64;
        let mut max_diff = 0 as i64;
        let mut result = 0;
        // our k loop
        for num in nums.iter().skip(1) {
            let num = *num as i64;
            result = result.max(max_diff * num);
            max_diff = max_diff.max(prefix_max - num);
            prefix_max = prefix_max.max(num);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![12, 6, 1, 2, 7];
        let output = 77;
        let result = Solution::maximum_triplet_value(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 10, 3, 4, 19];
        let output = 133;
        let result = Solution::maximum_triplet_value(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![1, 2, 3];
        let output = 0;
        let result = Solution::maximum_triplet_value(nums);
        assert_eq!(result, output);
    }
}
