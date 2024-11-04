// https://leetcode.com/problems/string-compression-iii/
#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn compressed_string(word: String) -> String {
        let word = word.as_bytes();

        let mut freq = 1;
        let n = word.len();

        let mut result = String::new();
        let mut last_letter = word[0];

        for i in 1..n {
            if word[i] != last_letter || freq == 9 {
                result.push_str(&format!("{}{}", freq, char::from(last_letter)));
                freq = 0;
            }

            last_letter = word[i];
            freq += 1;
        }

        result.push_str(&format!("{}{}", freq, char::from(last_letter)));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let word = "abcde".to_string();
        let output = "1a1b1c1d1e".to_string();
        let result = Solution::compressed_string(word);
        // to memorize that result is first
        // RadiOhead
        // result, output
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let word = "aaaaaaaaaaaaaabb".to_string();
        let output = "9a5a2b".to_string();
        let result = Solution::compressed_string(word);
        assert_eq!(result, output);
    }
}
