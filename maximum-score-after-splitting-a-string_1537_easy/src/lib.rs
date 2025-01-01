// https://leetcode.com/problems/maximum-score-after-splitting-a-string/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_score(s: String) -> i32 {
        // prefix_sum of ones backward
        let n = s.len();
        let s = s.into_bytes();
        let mut prefix_ones = vec![0; n + 1];
        for i in (0..n).rev() {
            prefix_ones[i] = prefix_ones[i + 1] + if s[i] == b'1' { 1 } else { 0 };
        }
        dbg!(&prefix_ones);
        let mut zeros_count = 0;
        let mut result = 0;
        for i in 0..n - 1 {
            zeros_count += if s[i] == b'0' { 1 } else { 0 };
            result = result.max(zeros_count + prefix_ones[i + 1]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = "011101".to_string();
        let output = 5;
        let result = Solution::max_score(s);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "00111".to_string();
        let output = 5;
        let result = Solution::max_score(s);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let s = "1111".to_string();
        let output = 3;
        let result = Solution::max_score(s);
        assert_eq!(result, output);
    }
}
