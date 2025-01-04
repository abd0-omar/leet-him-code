// https://leetcode.com/problems/unique-length-3-palindromic-subsequences/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        // two pointers
        // "aabca"
        // "bbcbaba"
        // set to avoid duplicate answers
        // two pointers or freq array
        let mut freq = std::collections::HashMap::new();
        let mut hs = std::collections::HashSet::new();
        let s = s.into_bytes();
        for (idx, letter) in s.iter().enumerate() {
            match freq.entry(letter) {
                std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                    let (prev_freq, _last_idx) = occupied_entry.get_mut();
                    *prev_freq += 1;
                    // for j in *last_idx..idx {
                    // hs.insert(format!("{}{}{}", letter, s[idx], letter));
                    // }
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    // first time
                    vacant_entry.insert((0, idx));
                }
            }
        }
        let mut visited = std::collections::HashSet::new();
        for i in (0..s.len()).rev() {
            if let Some((freq, first_idx)) = freq.get(&s[i]) {
                if *freq >= 1 {
                    if visited.insert(s[i]) {
                        dbg!(&i);
                        for j in (*first_idx + 1..i).rev() {
                            dbg!(&j);
                            hs.insert(format!("{}{}{}", s[i], s[j], s[i]));
                        }
                    }
                }
            }
        }
        dbg!(&hs);
        hs.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = "aabca".to_string();
        let output = 3;
        let result = Solution::count_palindromic_subsequence(s);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "adc".to_string();
        let output = 0;
        let result = Solution::count_palindromic_subsequence(s);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let s = "bbcbaba".to_string();
        let output = 4;
        let result = Solution::count_palindromic_subsequence(s);
        assert_eq!(result, output);
    }
}
