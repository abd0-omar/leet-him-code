// https://leetcode.com/problems/find-numbers-with-even-number-of-digits/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        // should be count to match the counting month
        let mut result = 0;
        for num in nums {
            if is_digit_count_even(num) {
                result += 1;
            }
        }
        result
    }
}

fn is_digit_count_even(mut num: i32) -> bool {
    let mut count = 0;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![12, 345, 2, 6, 7896];
        let output = 2;
        let result = Solution::find_numbers(nums);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![555, 901, 482, 1771];
        let output = 1;
        let result = Solution::find_numbers(nums);
        assert_eq!(result, output);
    }
}
