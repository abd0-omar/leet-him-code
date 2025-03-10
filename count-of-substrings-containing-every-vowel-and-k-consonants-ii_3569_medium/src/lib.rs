// https://leetcode.com/problems/count-of-substrings-containing-every-vowel-and-k-consonants-ii/
#[allow(dead_code)]
struct Solution;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let n = word.len();
        let word = word.as_bytes();
        let vowels = [b'a', b'e', b'i', b'o', b'u'];
        let mut vowel_index = std::collections::HashMap::new();
        for (i, &v) in vowels.iter().enumerate() {
            vowel_index.insert(v, i);
        }

        let mut vowel_prefix = vec![vec![0; 6]; n + 1];
        for i in 0..n {
            vowel_prefix[i + 1] = vowel_prefix[i].clone();
            if let Some(&idx) = vowel_index.get(&word[i]) {
                vowel_prefix[i + 1][idx] += 1;
                vowel_prefix[i + 1][5] += 1;
            }
        }

        // dbg!(&vowel_prefix);
        let mut ans = 0_i64;
        for i in 0..n {
            // may include more than k constants (at most k constants), but a valid vowels window
            let first_vowel_valid_index =
                Solution::get_valid_index(&word, i as i32, &vowel_prefix, 0, k);
            // may include a non complete vowels window, but exactly k constants
            let first_valid_k_constants =
                Solution::get_valid_index(&word, i as i32, &vowel_prefix, 1, k);
            dbg!(first_vowel_valid_index);
            dbg!(first_valid_k_constants);
            if first_vowel_valid_index <= first_valid_k_constants {
                ans += (first_valid_k_constants - first_vowel_valid_index + 1) as i64;
            }
            dbg!(&i);
            dbg!(&ans);
        }
        ans
    }

    fn get_valid_index(
        word: &[u8],
        index: i32,
        vowel_prefix: &Vec<Vec<i32>>,
        choice: i32,
        k: i32,
    ) -> i32 {
        let mut start = index;
        let mut end = (word.len() - 1) as i32;
        let mut ans = if choice == 1 {
            start - 1
        } else {
            word.len() as i32
        };

        while start <= end {
            let mid = (start + end) / 2;
            if choice == 0 {
                let mut valid = true;
                for i in 0..5 {
                    let vowel_count_prefix = vowel_prefix[(mid + 1) as usize][i];
                    let current_vowel_prefix = vowel_prefix[index as usize][i];
                    if vowel_count_prefix - current_vowel_prefix == 0 {
                        valid = false;
                        break;
                    }
                }
                let consonant_count = (mid - index + 1) as i32
                    - (vowel_prefix[(mid + 1) as usize][5] - vowel_prefix[index as usize][5]);
                if valid && consonant_count >= k {
                    ans = mid;
                    end = mid.saturating_sub(1);
                } else {
                    start = mid + 1;
                }
            } else {
                let consonant_count = (mid - index + 1) as i32
                    - (vowel_prefix[(mid + 1) as usize][5] - vowel_prefix[index as usize][5]);
                if consonant_count <= k {
                    ans = mid;
                    start = mid + 1;
                } else {
                    end = mid.saturating_sub(1);
                }
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let word = "aeioqq".to_string();
        let k = 1;
        let output = 0;
        let result = Solution::count_of_substrings(word, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let word = "aeiou".to_string();
        let k = 0;
        let output = 1;
        let result = Solution::count_of_substrings(word, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let word = "ieaouqqieaouqq".to_string();
        let k = 1;
        let output = 3;
        let result = Solution::count_of_substrings(word, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let word = "buoeia".to_string();
        let k = 0;
        let output = 1;
        let result = Solution::count_of_substrings(word, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works4() {
        let word = "ieaoubcde".to_string();
        let k = 3;
        let output = 2;
        let result = Solution::count_of_substrings(word, k);
        assert_eq!(result, output);
    }
}
