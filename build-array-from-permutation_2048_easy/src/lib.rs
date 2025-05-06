// https://leetcode.com/problems/build-array-from-permutation/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(nums.len());
        for &num in &nums {
            result.push(nums[num as usize]);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![0, 2, 1, 5, 3, 4];
        let output = vec![0, 1, 2, 4, 5, 3];
        let result = Solution::build_array(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![5, 0, 1, 2, 3, 4];
        let output = vec![4, 5, 0, 1, 2, 3];
        let result = Solution::build_array(nums);
        assert_eq!(result, output);
    }
}
