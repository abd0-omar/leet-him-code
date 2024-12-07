// https://leetcode.com/problems/minimum-limit-of-balls-in-a-bag/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        // binary search on result
        let mut l = 1;
        let mut r = *nums.iter().max().unwrap();
        let mut ans = -1;
        while l <= r {
            let mid = l + (r - l) / 2;
            if possible(&nums, mid, max_operations) {
                ans = mid;
                r = mid - 1;
            } else {
                l = mid + 1;
            }
        }
        ans
    }
}

fn possible(nums: &[i32], max_number_balls_cand: i32, mut max_operations: i32) -> bool {
    for &num in nums {
        if num < max_number_balls_cand {
            continue;
        }
        let operations = ((num as f64 / max_number_balls_cand as f64).ceil() - 1.0) as i32;
        max_operations -= operations;
        if max_operations < 0 {
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
        let nums = vec![9];
        let max_operations = 2;
        let output = 3;
        let result = Solution::minimum_size(nums, max_operations);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![2, 4, 8, 2];
        let max_operations = 4;
        let output = 2;
        let result = Solution::minimum_size(nums, max_operations);
        assert_eq!(result, output);
    }
}
