// https://leetcode.com/problems/find-words-containing-character/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        // fastest is O(n^2) that I could think of right now
        let mut result = Vec::new();
        for (idx, word) in words.into_iter().enumerate() {
            for letter in word.chars() {
                if letter == x {
                    result.push(idx as i32);
                    break;
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
        let words = vec!["leet".to_string(), "code".to_string()];
        let x = 'e';
        let output = vec![0, 1];
        let result = Solution::find_words_containing(words, x);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let words = vec![
            "abc".to_string(),
            "bcd".to_string(),
            "aaaa".to_string(),
            "cbc".to_string(),
        ];
        let x = 'a';
        let output = vec![0, 2];
        let result = Solution::find_words_containing(words, x);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let words = vec![
            "abc".to_string(),
            "bcd".to_string(),
            "aaaa".to_string(),
            "cbc".to_string(),
        ];
        let x = 'z';
        let output = vec![];
        let result = Solution::find_words_containing(words, x);
        assert_eq!(result, output);
    }
}
