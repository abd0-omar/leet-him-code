// https://leetcode.com/problems/check-if-one-string-swap-can-make-strings-equal/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        // I think true brute-force would be to try every combination until we
        // run out of combination or they become equal, whichever comes first
        // or just O(n^2) check every letter
        // but the solution is just as simple as checking indices, and they must
        // have the same frequencies
        if s1.len() != s2.len() {
            return false;
        }
        let mut freq1 = vec![0; 26];
        let mut freq2 = vec![0; 26];
        // فترة سماح
        let mut allow = 0;
        for (letter1, letter2) in s1.chars().zip(s2.chars()) {
            if letter1 != letter2 {
                allow += 1;
            }
            if allow > 2 {
                return false;
            }
            freq1[(letter1 as u8 - b'a') as usize] += 1;
            freq2[(letter2 as u8 - b'a') as usize] += 1;
        }
        for i in 0..26 {
            if freq1[i] != freq2[i] {
                return false;
            }
        }
        allow == 2 || allow == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s1 = "bank".to_string();
        let s2 = "kanb".to_string();
        let output = true;
        let result = Solution::are_almost_equal(s1, s2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s1 = "attack".to_string();
        let s2 = "defend".to_string();
        let output = false;
        let result = Solution::are_almost_equal(s1, s2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let s1 = "kelb".to_string();
        let s2 = "kelb".to_string();
        let output = true;
        let result = Solution::are_almost_equal(s1, s2);
        assert_eq!(result, output);
    }
}
