// https://leetcode.com/problems/check-if-array-is-sorted-and-rotated/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        // easiest way to debug and pass, is to double the array
        // to avoid first and last numbers checking
        let n = nums.len();
        if n == 1 {
            return true;
        }
        let mut double_nums = Vec::with_capacity(n);
        for _ in 0..2 {
            for &num in &nums {
                double_nums.push(num);
            }
        }
        // dbg!(&double_nums);
        let mut count_ascen = 0;
        for i in 1..n * 2 {
            // dbg!(&i);
            // dbg!(&count_ascen);
            if double_nums[i] >= double_nums[i - 1] {
                count_ascen += 1;
            } else {
                count_ascen = 0;
            }
            if count_ascen == n - 1 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![3, 4, 5, 1, 2];
        let output = true;
        let result = Solution::check(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![2, 1, 3, 4];
        let output = false;
        let result = Solution::check(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![1, 2, 3];
        let output = true;
        let result = Solution::check(nums);
        assert_eq!(result, output);
    }
}
