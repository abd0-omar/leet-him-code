// https://leetcode.com/problems/construct-string-with-repeat-limit/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        let mut hm = std::collections::HashMap::new();
        for letter in s.chars() {
            *hm.entry(letter).or_insert(0) += 1;
        }
        let mut sorted_letters = Vec::new();
        for (&letter, &freq) in hm.iter() {
            sorted_letters.push((letter, freq));
        }
        sorted_letters.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        dbg!(&sorted_letters);
        let mut result = String::new();
        let mut idx = 0;
        let mut count = 0;
        loop {
            while idx < sorted_letters.len() && sorted_letters[idx].1 == 0 {
                idx += 1;
                count = 0;
            }
            if idx == sorted_letters.len() {
                break;
            }
            if count == repeat_limit {
                count = 0;
                if idx + 1 == sorted_letters.len() {
                    break;
                }
                // if sorted_letters[idx + 1].1 == 0 {
                //     break;
                // }
                let mut new_idx = idx + 1;
                while new_idx < sorted_letters.len() && sorted_letters[new_idx].1 == 0 {
                    new_idx += 1;
                }
                if new_idx == sorted_letters.len() {
                    break;
                }
                sorted_letters[new_idx].1 -= 1;
                result.push(sorted_letters[new_idx].0);
                continue;
            }
            if sorted_letters[idx].1 > 0 && count < repeat_limit {
                sorted_letters[idx].1 -= 1;
                count += 1;
                result.push(sorted_letters[idx].0);
            }
            if sorted_letters[idx].1 == 0 {
                idx += 1;
                count = 0;
                continue;
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
        let s = "cczazcc".to_string();
        let repeat_limit = 3;
        let output = "zzcccac".to_string();
        let result = Solution::repeat_limited_string(s, repeat_limit);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "aababab".to_string();
        let repeat_limit = 2;
        let output = "bbabaa".to_string();
        let result = Solution::repeat_limited_string(s, repeat_limit);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let s = "robnsdvpuxbapuqgopqvxdrchivlifeepy".to_string();
        let repeat_limit = 2;
        let output = "yxxvvuvusrrqqppopponliihgfeeddcbba".to_string();
        let result = Solution::repeat_limited_string(s, repeat_limit);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let s = "xyutfpopdynbadwtvmxiemmusevduloxwvpkjioizvanetecnuqbqqdtrwrkgt".to_string();
        let repeat_limit = 1;
        let output = "zyxyxwxwvwvuvuvututstrtrtqpqpqponononmlmkmkjigifiededededcbaba".to_string();
        let result = Solution::repeat_limited_string(s, repeat_limit);
        assert_eq!(result, output);
    }
}
