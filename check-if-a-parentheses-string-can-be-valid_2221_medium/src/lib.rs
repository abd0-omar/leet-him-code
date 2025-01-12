// https://leetcode.com/problems/check-if-a-parentheses-string-can-be-valid/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        // same as https://leetcode.com/problems/valid-parenthesis-string/
        // but I solved it with dp top-down approach and it passed due it's low constraints
        // won't work this one so stack solution would be sufficient

        let n = s.len();
        if n % 2 == 1 {
            return false;
        }

        let s = s.chars().collect::<Vec<char>>();
        let locked = locked.chars().collect::<Vec<char>>();

        let mut unlocked = Vec::new();
        let mut stack = Vec::new();
        for i in 0..n {
            // 1 locked
            // 0 unlocked
            if locked[i] == '0' {
                unlocked.push(i);
            } else if s[i] == '(' {
                stack.push(i);
            } else if s[i] == ')' {
                if !stack.is_empty() {
                    stack.pop();
                } else if !unlocked.is_empty() {
                    unlocked.pop();
                } else {
                    return false;
                }
            }
        }

        while !stack.is_empty()
            && !unlocked.is_empty()
            && stack.last().unwrap() < unlocked.last().unwrap()
        {
            stack.pop();
            unlocked.pop();
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = "))()))".to_string();
        let locked = "010100".to_string();
        let output = true;
        let result = Solution::can_be_valid(s, locked);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "()()".to_string();
        let locked = "0000".to_string();
        let output = true;
        let result = Solution::can_be_valid(s, locked);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let s = ")".to_string();
        let locked = "0".to_string();
        let output = false;
        let result = Solution::can_be_valid(s, locked);
        assert_eq!(result, output);
    }
}
