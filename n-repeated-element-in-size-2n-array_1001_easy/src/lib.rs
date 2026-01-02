// https://leetcode.com/problems/n-repeated-element-in-size-2n-array/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn repeated_n_times(nums: Vec<i32>) -> i32 {
        let mut hs = std::collections::HashSet::new();
        for num in nums {
            if !hs.insert(num) {
                return num;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 2, 3, 3];
        let output = 3;
        let result = Solution::repeated_n_times(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![2, 1, 2, 5, 3, 2];
        let output = 2;
        let result = Solution::repeated_n_times(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![5, 1, 5, 2, 5, 3, 5, 4];
        let output = 5;
        let result = Solution::repeated_n_times(nums);
        assert_eq!(result, output);
    }
}
