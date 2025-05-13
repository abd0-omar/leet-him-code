// https://leetcode.com/problems/total-characters-in-string-after-transformations-i/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        // dp and math are not the solution
        // frequency simulation
        const MOD: i32 = 1_000_000_007;
        let s = s.as_bytes();
        let mut freq = [0; 26];
        for &ch in s.iter() {
            let ch_idx = (ch - b'a') as usize;
            freq[ch_idx] += 1;
        }
        for _ in 0..t {
            let mut next = [0; 26];
            // a <- z
            next[0] = freq[25];
            // b <- a | z
            next[1] = (freq[0] + freq[25]) % MOD;

            // rest of the letters
            for i in 2..26 {
                next[i] = freq[i - 1];
            }

            // update curr frequency
            freq = next;
        }
        let mut result = 0;
        for count in freq {
            result = (result + count) % MOD;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = "abcyy".to_string();
        let t = 2;
        let output = 7;
        let result = Solution::length_after_transformations(s, t);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "azbk".to_string();
        let t = 1;
        let output = 5;
        let result = Solution::length_after_transformations(s, t);
        assert_eq!(result, output);
    }
}
