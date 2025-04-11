// https://leetcode.com/problems/count-symmetric-integers/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let mut result = 0;
        for x in low..high + 1 {
            if is_symmetric(x) {
                result += 1;
            }
        }
        result
    }
}

fn is_symmetric(x: i32) -> bool {
    let x = x.to_string();
    let n = x.len();
    if n % 2 == 1 {
        return false;
    }
    let mut sum_first_half = 0;
    let mut sum_last_half = 0;
    let x = x.as_bytes();
    for i in 0..n / 2 {
        sum_first_half += x[i];
        sum_last_half += x[n - i - 1];
    }
    sum_first_half == sum_last_half
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let low = 1;
        let high = 100;
        let output = 9;
        let result = Solution::count_symmetric_integers(low, high);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let low = 1200;
        let high = 1230;
        let output = 4;
        let result = Solution::count_symmetric_integers(low, high);
        assert_eq!(result, output);
    }
}
