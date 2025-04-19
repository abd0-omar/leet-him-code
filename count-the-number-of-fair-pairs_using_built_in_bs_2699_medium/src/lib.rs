// https://leetcode.com/problems/count-the-number-of-fair-pairs/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();
        let mut result = 0;

        for (idx, &num) in nums.iter().enumerate() {
            let start_range = nums[idx + 1..].partition_point(|&x| x < lower - num);
            let end_range = nums[idx + 1..].partition_point(|&x| x <= upper - num);

            result += (end_range - start_range) as i64;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![0, 1, 7, 4, 4, 5];
        let lower = 3;
        let upper = 6;
        let output = 6;
        let result = Solution::count_fair_pairs(nums, lower, upper);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 7, 9, 2, 5];
        let lower = 11;
        let upper = 11;
        let output = 1;
        let result = Solution::count_fair_pairs(nums, lower, upper);
        assert_eq!(result, output);
    }
}
