// https://leetcode.com/problems/minimum-number-of-operations-to-make-elements-in-array-distinct/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut result = 0;
        let n = nums.len();
        while i < n {
            if !is_unique(&nums[i..n]) {
                result += 1;
                i += 3
            } else {
                break;
            }
        }
        result
    }
}

fn is_unique(nums: &[i32]) -> bool {
    let mut hs = std::collections::HashSet::new();
    for num in nums {
        if !hs.insert(num) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1, 2, 3, 4, 2, 3, 3, 5, 7];
        let output = 2;
        let result = Solution::minimum_operations(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![4, 5, 6, 4, 4];
        let output = 2;
        let result = Solution::minimum_operations(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![6, 7, 8, 9];
        let output = 0;
        let result = Solution::minimum_operations(nums);
        assert_eq!(result, output);
    }
}
