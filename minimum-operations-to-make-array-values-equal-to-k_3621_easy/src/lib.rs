// https://leetcode.com/problems/minimum-operations-to-make-array-values-equal-to-k/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut hs: std::collections::HashSet<i32> = std::collections::HashSet::new();
        for num in nums {
            if num < k {
                return -1;
            }
            if num != k {
                hs.insert(num);
            }
        }
        hs.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![5, 2, 5, 4, 5];
        let k = 2;
        let output = 2;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![2, 1, 2];
        let k = 2;
        let output = -1;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![9, 7, 5, 3];
        let k = 1;
        let output = 4;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, output);
    }
}
