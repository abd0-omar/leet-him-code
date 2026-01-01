// https://leetcode.com/problems/plus-one/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        for digit in digits.iter_mut().rev() {
            if *digit != 9 {
                *digit += 1;
                return digits;
            }

            *digit = 0;
        }
        digits.insert(0, 1);
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let digits = vec![1, 2, 3];
        let output = vec![1, 2, 4];
        let result = Solution::plus_one(digits);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let digits = vec![4, 3, 2, 1];
        let output = vec![4, 3, 2, 2];
        let result = Solution::plus_one(digits);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let digits = vec![9];
        let output = vec![1, 0];
        let result = Solution::plus_one(digits);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let digits = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let output = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1];
        let result = Solution::plus_one(digits);
        assert_eq!(result, output);
    }
}
