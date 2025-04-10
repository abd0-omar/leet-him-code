// https://leetcode.com/problems/count-the-number-of-powerful-integers/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    const MAX_DIGITS: usize = 17;

    fn check_subtract(num_str: &str, num_digits: usize, suffix: &str, limit: i32) -> bool {
        if num_digits < suffix.len() {
            return false;
        }

        let suffix_len = suffix.len();
        let suffix_of_num = &num_str[num_digits - suffix_len..];
        let subtract = suffix_of_num.parse::<i64>().unwrap() < suffix.parse::<i64>().unwrap();

        if subtract {
            for i in 0..num_digits - suffix.len() {
                if num_str.chars().nth(i).unwrap().to_digit(10).unwrap() > limit as u32 {
                    return false;
                }
            }
        }
        subtract
    }

    fn count_valid_numbers(
        number_str: &str,
        idx: usize,
        max_digits: usize,
        is_tight: bool,
        limit: i32,
        suffix: &str,
        dp: &mut Vec<Vec<i64>>,
    ) -> i64 {
        if idx == max_digits {
            return 1;
        }
        if dp[idx][is_tight as usize] != -1 {
            return dp[idx][is_tight as usize];
        }

        let suffix_len = suffix.len();
        let (low, high) = if idx >= max_digits - suffix_len {
            let suffix_idx = idx - (max_digits - suffix_len);
            let digit = suffix
                .chars()
                .nth(suffix_idx)
                .unwrap()
                .to_digit(10)
                .unwrap() as i64;
            (digit, digit)
        } else {
            let current_digit = number_str.chars().nth(idx).unwrap().to_digit(10).unwrap() as i64;
            let upper_bound = if is_tight {
                std::cmp::min(limit as i64, current_digit)
            } else {
                limit as i64
            };
            (0, upper_bound)
        };

        let mut total = 0;
        for digit in low..=high {
            let new_tight = is_tight
                && (digit == number_str.chars().nth(idx).unwrap().to_digit(10).unwrap() as i64);
            total += Self::count_valid_numbers(
                number_str,
                idx + 1,
                max_digits,
                new_tight,
                limit,
                suffix,
                dp,
            );
        }

        dp[idx][is_tight as usize] = total;
        total
    }

    fn solve_up_to(num_str: &str, num_digits: usize, limit: i32, suffix: &str) -> i64 {
        let mut dp = vec![vec![-1; 2]; Self::MAX_DIGITS];
        let result =
            Self::count_valid_numbers(num_str, 0, num_digits, true, limit, suffix, &mut dp);
        if Self::check_subtract(num_str, num_digits, suffix, limit) {
            result - 1
        } else {
            result
        }
    }

    pub fn number_of_powerful_int(start: i64, finish: i64, limit: i32, s: String) -> i64 {
        let suffix_val = s.parse::<i64>().unwrap();
        let finish_str = finish.to_string();
        let start_str = (start - 1).to_string();

        let finish_digits = finish_str.len();
        let start_digits = start_str.len();

        let upto_finish = if finish >= suffix_val {
            Self::solve_up_to(&finish_str, finish_digits, limit, &s)
        } else {
            0
        };
        let upto_start = if suffix_val < start {
            Self::solve_up_to(&start_str, start_digits, limit, &s)
        } else {
            0
        };

        upto_finish - upto_start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let start = 1;
        let finish = 6000;
        let limit = 4;
        let s = "124".to_string();
        let output = 5;
        let result = Solution::number_of_powerful_int(start, finish, limit, s);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let start = 15;
        let finish = 215;
        let limit = 6;
        let s = "10".to_string();
        let output = 2;
        let result = Solution::number_of_powerful_int(start, finish, limit, s);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let start = 1000;
        let finish = 2000;
        let limit = 4;
        let s = "3000".to_string();
        let output = 0;
        let result = Solution::number_of_powerful_int(start, finish, limit, s);
        assert_eq!(result, output);
    }
}
