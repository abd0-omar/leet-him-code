// https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        // the same intiution from a problem we solved before
        // https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips/
        // that's why it's idea seemed similar and came up naturally, but it's
        // an adhoc-ish problem after all so glad I faced it before
        // [0, 1, 1, 1, 0, 0]
        // [1, 0, 0, 1, 0, 0]
        // [1, 1, 1, 0, 0, 0]
        // [1, 1, 1, 1, 1, 1]
        // once you face a 0 flip it, if you faced a 0 and the subarray len is
        // less than 3, then retur false, yeah it's that simple

        let n = nums.len();
        let mut result = 0;
        for i in 0..n {
            if nums[i] == 0 && n - i < 3 {
                return -1;
            }
            if nums[i] == 0 {
                result += 1;
                for j in i..i + 3 {
                    nums[j] = if nums[j] == 0 { 1 } else { 0 };
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
        let nums = vec![0, 1, 1, 1, 0, 0];
        let output = 3;
        let result = Solution::min_operations(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![0, 1, 1, 1];
        let output = -1;
        let result = Solution::min_operations(nums);
        assert_eq!(result, output);
    }
}
