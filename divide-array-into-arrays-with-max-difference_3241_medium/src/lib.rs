// https://leetcode.com/problems/divide-array-into-arrays-with-max-difference/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn divide_array(mut nums: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let each_size = nums.len() / 3;
        let mut result = vec![Vec::with_capacity(3); each_size];
        let mut i = 0;
        let mut idx = 0;
        while i < nums.len() {
            if nums[i + 2] - nums[i] > k {
                return Vec::new();
            }
            result[idx].push(nums[i]);
            result[idx].push(nums[i + 1]);
            result[idx].push(nums[i + 2]);
            idx += 1;
            i += 3;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 3, 4, 8, 7, 9, 3, 5, 1];
        let k = 2;
        let output = vec![vec![1, 1, 3], vec![3, 4, 5], vec![7, 8, 9]];
        let result = Solution::divide_array(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![2, 4, 2, 2, 5, 2];
        let k = 2;
        let output: Vec<Vec<i32>> = vec![];
        let result = Solution::divide_array(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![4, 2, 9, 8, 2, 12, 7, 12, 10, 5, 8, 5, 5, 7, 9, 2, 5, 11];
        let k = 14;
        let output = vec![
            vec![2, 2, 12],
            vec![4, 8, 5],
            vec![5, 9, 7],
            vec![7, 8, 5],
            vec![5, 9, 10],
            vec![11, 12, 2],
        ];
        let result = Solution::divide_array(nums, k);
        assert_eq!(result, output);
    }
}
