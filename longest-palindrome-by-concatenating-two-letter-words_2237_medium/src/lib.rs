// https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        use std::collections::HashMap;
        let mut hm = HashMap::new();
        let mut result = 0;
        let mut center = false;

        for word in words {
            if word.as_bytes()[0] == word.as_bytes()[1] {
                let c = hm.entry(word).or_insert(0);
                if *c > 0 {
                    result += 4;
                    *c -= 1;
                } else {
                    *c += 1;
                }
            } else {
                let word_rev: String = word.chars().rev().collect();
                let c = hm.entry(word_rev).or_insert(0);
                if *c > 0 {
                    result += 4;
                    *c -= 1;
                } else {
                    *hm.entry(word).or_insert(0) += 1;
                }
            }
        }

        for (k, v) in hm.iter() {
            if k.as_bytes()[0] == k.as_bytes()[1] && *v > 0 {
                center = true;
                break;
            }
        }
        if center {
            result += 2;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let words = vec!["lc".to_string(), "cl".to_string(), "gg".to_string()];
        let output = 6;
        let result = Solution::longest_palindrome(words);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let words = vec![
            "ab".to_string(),
            "ty".to_string(),
            "yt".to_string(),
            "lc".to_string(),
            "cl".to_string(),
            "ab".to_string(),
        ];
        let output = 8;
        let result = Solution::longest_palindrome(words);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let words = vec!["cc".to_string(), "ll".to_string(), "xx".to_string()];
        let output = 2;
        let result = Solution::longest_palindrome(words);
        assert_eq!(result, output);
    }
}
