// https://leetcode.com/problems/adding-spaces-to-a-string/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        // parse it as you move
        // like how you did it in cargo-leetcode project
        let spaces_len = spaces.len();
        let mut result = String::with_capacity(s.len() + spaces_len);
        let mut spaces_idx = 0;
        for (s_idx, letter) in s.chars().enumerate() {
            if spaces_idx < spaces_len && spaces[spaces_idx] == s_idx as i32 {
                result.push(' ');
                spaces_idx += 1;
            }
            result.push(letter);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = "LeetcodeHelpsMeLearn".to_string();
        let spaces = vec![8, 13, 15];
        let output = "Leetcode Helps Me Learn".to_string();
        let result = Solution::add_spaces(s, spaces);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "icodeinpython".to_string();
        let spaces = vec![1, 5, 7, 9];
        let output = "i code in py thon".to_string();
        let result = Solution::add_spaces(s, spaces);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let s = "spacing".to_string();
        let spaces = vec![0, 1, 2, 3, 4, 5, 6];
        let output = " s p a c i n g".to_string();
        let result = Solution::add_spaces(s, spaces);
        assert_eq!(result, output);
    }
}
