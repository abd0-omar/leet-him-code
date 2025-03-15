// https://leetcode.com/problems/house-robber-iv/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let (mut l, mut r) = (*nums.iter().min().unwrap(), *nums.iter().max().unwrap());
        let mut ans = -1;
        let n = nums.len();
        while l <= r {
            let candidate = l + (r - l) / 2;
            let mut i = 0;
            let mut house_theft = 0;
            while i < n {
                if nums[i] <= candidate {
                    house_theft += 1;
                    i += 1;
                }
                i += 1;
            }
            if house_theft >= k {
                ans = candidate;
                if let (_, true) = candidate.overflowing_sub(1) {
                    break;
                }
                r = candidate - 1;
            } else {
                l = candidate + 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![2, 3, 5, 9];
        let k = 2;
        let output = 5;
        let result = Solution::min_capability(nums, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![2, 7, 9, 3, 1];
        let k = 2;
        let output = 2;
        let result = Solution::min_capability(nums, k);
        assert_eq!(result, output);
    }
}
