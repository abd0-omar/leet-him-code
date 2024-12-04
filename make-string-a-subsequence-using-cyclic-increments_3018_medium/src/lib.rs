// https://leetcode.com/problems/make-string-a-subsequence-using-cyclic-increments/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        if str1.len() < str2.len() {
            return false;
        }
        let mut l = 0;
        let (str1, str2) = (str1.into_bytes(), str2.into_bytes());
        // make it 0 indexed
        for letter in str1 {
            let letter1 = letter - b'a';
            let letter2 = str2[l] - b'a';
            if letter1 == letter2 || (letter1 + 1) % 26 == letter2
            // || (letter1 + 26 - 1) % 26 == letter2 // minus condition is not required here
            // and mkae the solution wrong
            {
                l += 1;
            }
            // dbg!(letter1);
            // dbg!((letter1 + 1) % 26);
            // dbg!((letter1 - 1) % 26);
            if l == str2.len() {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let str1 = "abc".to_string();
        let str2 = "ad".to_string();
        let output = true;
        let result = Solution::can_make_subsequence(str1, str2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let str1 = "zc".to_string();
        let str2 = "ad".to_string();
        let output = true;
        let result = Solution::can_make_subsequence(str1, str2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let str1 = "ab".to_string();
        let str2 = "d".to_string();
        let output = false;
        let result = Solution::can_make_subsequence(str1, str2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let str1 = "abc".to_string();
        let str2 = "ac".to_string();
        let output = true;
        let result = Solution::can_make_subsequence(str1, str2);
        assert_eq!(result, output);
    }
}
