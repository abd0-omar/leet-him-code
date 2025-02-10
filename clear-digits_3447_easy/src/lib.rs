// https://leetcode.com/problems/clear-digits/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn clear_digits(s: String) -> String {
        // basic stack problem, can't even call it a problem you know whamsayin
        // you know whamsayinsdkfhajdhfh xD
        // الواحد محتاج يخرج شوية

        let mut stack = Vec::with_capacity(s.len());
        for letter in s.chars() {
            if letter.is_ascii_digit() {
                stack.pop();
            } else {
                stack.push(letter);
            }
        }
        stack.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = "abc".to_string();
        let output = "abc".to_string();
        let result = Solution::clear_digits(s);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "cb34".to_string();
        let output = "".to_string();
        let result = Solution::clear_digits(s);
        assert_eq!(result, output);
    }
}
