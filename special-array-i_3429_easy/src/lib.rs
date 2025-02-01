// https://leetcode.com/problems/special-array-i/
#[allow(dead_code)]
struct Solution;

enum Parity {
    Even,
    Odd,
}

impl Parity {
    fn new(num: i32) -> Self {
        if num % 2 == 0 {
            Self::Even
        } else {
            Self::Odd
        }
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut prev_parity = Parity::new(nums[0]);

        for i in 1..n {
            let cur_parity = Parity::new(nums[i]);

            match (&prev_parity, &cur_parity) {
                (Parity::Even, Parity::Even) | (Parity::Odd, Parity::Odd) => return false,
                (Parity::Even, Parity::Odd) | (Parity::Odd, Parity::Even) => (),
            }

            prev_parity = cur_parity;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![1];
        let output = true;
        let result = Solution::is_array_special(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![2, 1, 4];
        let output = true;
        let result = Solution::is_array_special(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![4, 3, 1, 6];
        let output = false;
        let result = Solution::is_array_special(nums);
        assert_eq!(result, output);
    }
}
