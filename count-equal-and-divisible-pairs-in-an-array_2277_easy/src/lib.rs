// https://leetcode.com/problems/count-equal-and-divisible-pairs-in-an-array/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        // brute force
        let n = nums.len();
        let mut result = 0;
        for i in 0..n {
            for j in i + 1..n {
                if nums[i] == nums[j] && (i * j) as i32 % k == 0 {
                    result += 1;
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![3, 1, 2, 2, 2, 1, 3];
        let k = 2;
        let output = 4;
        let result = Solution::count_pairs(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 2, 3, 4];
        let k = 1;
        let output = 0;
        let result = Solution::count_pairs(nums, k);
        assert_eq!(result, output);
    }
}
