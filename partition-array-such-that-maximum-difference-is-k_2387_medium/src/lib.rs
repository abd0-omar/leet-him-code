// https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn partition_array(nums: Vec<i32>, k: i32) -> i32 {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![3, 6, 1, 2, 5];
        let k = 2;
        let output = 2;
        let result = Solution::partition_array(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![1, 2, 3];
        let k = 1;
        let output = 2;
        let result = Solution::partition_array(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![2, 2, 4, 5];
        let k = 0;
        let output = 3;
        let result = Solution::partition_array(nums, k);
        assert_eq!(result, output);
    }
}
