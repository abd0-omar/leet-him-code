// https://leetcode.com/problems/construct-k-palindrome-strings/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        // freq, if count odd freq less than k then true

        // edge case
        if k > s.len() as i32 {
            return false;
        }
        let mut freq = std::collections::HashMap::new();
        for letter in s.chars() {
            *freq.entry(letter as u8 - b'a').or_insert(0) += 1;
        }
        let mut count = 0;
        for letter_count in freq.into_values() {
            if letter_count % 2 == 1 {
                count += 1;
            }
            if count > k {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = "annabelle".to_string();
        let k = 2;
        let output = true;
        let result = Solution::can_construct(s, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "leetcode".to_string();
        let k = 3;
        let output = false;
        let result = Solution::can_construct(s, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let s = "true".to_string();
        let k = 4;
        let output = true;
        let result = Solution::can_construct(s, k);
        assert_eq!(result, output);
    }
}
