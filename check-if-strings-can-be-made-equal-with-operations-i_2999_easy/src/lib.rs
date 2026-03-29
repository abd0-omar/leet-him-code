// https://leetcode.com/problems/check-if-strings-can-be-made-equal-with-operations-i/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        // s1
        // "abcd"
        //
        // hashmap
        //
        // 0 | a
        // 1 | b
        // 2 | a, c. 0 | a, c
        //
        // cuz input is too low, hashmap is an overkill
        // we'll use an array instead
        // also array is overkill cuz it's a fixed 4 len
        // will use if checks instead
        //
        // [[a], [b]]
        // [[a, c], [b], [c, a]]

        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());

        if s1[0] != s2[0] && s1[0] != s2[2] {
            false
        } else if s1[1] != s2[1] && s1[1] != s2[3] {
            false
        } else if s1[2] != s2[2] && s1[2] != s2[0] {
            false
        } else if s1[3] != s2[3] && s1[3] != s2[1] {
            false
        } else {
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s1 = "abcd".to_string();
        let s2 = "cdab".to_string();
        let output = true;
        let result = Solution::can_be_equal(s1, s2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s1 = "abcd".to_string();
        let s2 = "dacb".to_string();
        let output = false;
        let result = Solution::can_be_equal(s1, s2);
        assert_eq!(result, output);
    }
}
