// https://leetcode.com/problems/take-k-of-each-character-from-left-and-right/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        // take the max invalid window

        let mut freq = [0; 3];
        let s = s.into_bytes();
        for letter in s.iter() {
            freq[(letter - b'a') as usize] += 1;
        }

        if *freq.iter().min().unwrap() < k {
            return -1;
        }

        let n = s.len();
        let mut l = 0;
        let mut res = n as i32;
        for r in 0..n {
            freq[(s[r] - b'a') as usize] -= 1;

            while *freq.iter().min().unwrap() < k {
                freq[(s[l] - b'a') as usize] += 1;
                l += 1;
            }

            res = res.min(n as i32 - (r as i32 - l as i32 + 1));
        }
        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = "aabaaaacaabc".to_string();
        let k = 2;
        let output = 8;
        let result = Solution::take_characters(s, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "a".to_string();
        let k = 1;
        let output = -1;
        let result = Solution::take_characters(s, k);
        assert_eq!(result, output);
    }
}
