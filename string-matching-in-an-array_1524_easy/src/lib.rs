// https://leetcode.com/problems/string-matching-in-an-array/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        // I only could think of O(n^420) solution
        // and the input is low enough to do it
        //
        // I saw in the editorial a way to do it more efficiently with tries,
        // by storing the suffixes and their frequency, don't know exactly how
        // but it was interesting

        let mut words: Vec<Vec<char>> = words.iter().map(|word| word.chars().collect()).collect();
        words.sort_unstable_by(|a, b| a.len().cmp(&b.len()));
        // dbg!(&words);
        let n = words.len();
        let mut result: Vec<&Vec<char>> = Vec::new();
        'outer: for i in 0..n {
            let cur_word = &words[i];
            for j in i + 1..n {
                let other_word = &words[j];
                // window of the `cur_word`
                // dbg!("current");
                // dbg!(&cur_word);
                // dbg!("other");
                // dbg!(&other_word);
                // rust built-in .contains function is better
                for sub_string in other_word.windows(cur_word.len()) {
                    // dbg!("substring");
                    // dbg!(&sub_string);
                    if cur_word == sub_string {
                        result.push(cur_word);
                        continue 'outer;
                    }
                }
            }
        }

        result.iter().map(|word| word.iter().collect()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let words = vec![
            "mass".to_string(),
            "as".to_string(),
            "hero".to_string(),
            "superhero".to_string(),
        ];
        let output = vec!["as".to_string(), "hero".to_string()];
        let result = Solution::string_matching(words);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let words = vec!["leetcode".to_string(), "et".to_string(), "code".to_string()];
        let output = vec!["et".to_string(), "code".to_string()];
        let result = Solution::string_matching(words);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let words = vec!["blue".to_string(), "green".to_string(), "bu".to_string()];
        let output: Vec<String> = vec![];
        let result = Solution::string_matching(words);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let words = vec![
            "leetcoder".to_string(),
            "leetcode".to_string(),
            "od".to_string(),
            "hamlet".to_string(),
            "am".to_string(),
        ];
        let output: Vec<String> = vec!["leetcode".to_string(), "od".to_string(), "am".to_string()];
        let result = Solution::string_matching(words);
        assert_eq!(result, output);
    }
}
