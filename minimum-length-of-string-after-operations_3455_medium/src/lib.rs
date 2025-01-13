// https://leetcode.com/problems/minimum-length-of-string-after-operations/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        // count letters with 3 ocurr. or more
        // if it's count odd, one will remian
        // if it's count even, two will remian
        // the problem is considered easy
        // maybe the semi-little medium part is the problem wording
        let mut freq = vec![0; 26];
        for letter in s.chars() {
            let letter_idx = (letter as u8 - b'a') as usize;
            freq[letter_idx] += 1;
        }
        let mut result = 0;
        for count in freq.into_iter().filter(|&c| c > 0) {
            if count >= 3 {
                if count % 2 == 1 {
                    result += 1;
                } else {
                    result += 2
                }
            } else {
                result += count;
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
        let s = "abaacbcbb".to_string();
        let output = 5;
        let result = Solution::minimum_length(s);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "aa".to_string();
        let output = 2;
        let result = Solution::minimum_length(s);
        assert_eq!(result, output);
    }
}
