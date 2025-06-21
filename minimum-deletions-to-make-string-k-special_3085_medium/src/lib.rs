#![allow(unused)]
struct Solution;

impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let mut freq = [0; 26];
        for c in word.chars() {
            freq[(c as u8 - b'a') as usize] += 1;
        }

        let mut frequencies: Vec<i32> = freq.iter().filter(|&&f| f > 0).copied().collect();
        // dbg!(&frequencies);

        let mut result = word.len() as i32;

        // assumed keep
        for &min_freq in frequencies.iter() {
            let mut deletions = 0;

            for &f in frequencies.iter() {
                // lower than our assumed minimum
                if f < min_freq {
                    deletions += f;
                // higher than our assumed minimum + k
                } else if f > min_freq + k {
                    deletions += f - (min_freq + k);
                }
            }

            result = result.min(deletions);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let word = "aabcaba".to_string();
        let k = 0;
        assert_eq!(Solution::minimum_deletions(word, k), 3);
    }

    #[test]
    fn it_works1() {
        let word = "dabdcbdcdcd".to_string();
        let k = 2;
        assert_eq!(Solution::minimum_deletions(word, k), 2);
    }

    #[test]
    fn it_works2() {
        let word = "aaabaaa".to_string();
        let k = 2;
        assert_eq!(Solution::minimum_deletions(word, k), 1);
    }
}
