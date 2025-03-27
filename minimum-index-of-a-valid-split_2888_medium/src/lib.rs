// https://leetcode.com/problems/minimum-index-of-a-valid-split/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_index(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let (mut left, mut right): (HashMap<i32, i32>, HashMap<i32, i32>) =
            (HashMap::new(), HashMap::new());
        for &num in &nums {
            *right.entry(num).or_insert(0) += 1;
        }
        let n = nums.len();
        for (idx, num) in nums.iter().enumerate() {
            let left = left.entry(*num).or_insert(0);
            *left += 1;
            let right = right.get_mut(num).unwrap();
            *right -= 1;
            let left_len = idx + 1;
            let right_len = n - idx - 1;

            if 2 * *left > left_len as i32 && 2 * *right > right_len as i32 {
                return idx as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 2, 2, 2];
        let output = 2;
        let result = Solution::minimum_index(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![2, 1, 3, 1, 1, 1, 7, 1, 2, 1];
        let output = 4;
        let result = Solution::minimum_index(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![3, 3, 3, 3, 7, 2, 2];
        let output = -1;
        let result = Solution::minimum_index(nums);
        assert_eq!(result, output);
    }
}
