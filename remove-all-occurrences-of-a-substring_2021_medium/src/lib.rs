// https://leetcode.com/problems/remove-all-occurrences-of-a-substring/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let part_len = part.len();
        let mut stack = Vec::new();

        'outer: for letter in s.chars() {
            stack.push(letter);

            if stack.len() >= part_len {
                // input is 1000 so we can do this crazy idea, not really crazy
                // but would be if the input was 10^9
                // the `zip` fn came in handy
                for (&stack_letter, part_letter) in stack.iter().rev().zip(part.chars().rev()) {
                    if stack_letter != part_letter {
                        continue 'outer;
                    }
                }
                // if we reached here we are sure that we encountered substring
                // equlas to "part"
                for _ in 0..part_len {
                    stack.pop();
                }
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
        let s = "daabcbaabcbc".to_string();
        let part = "abc".to_string();
        let output = "dab".to_string();
        let result = Solution::remove_occurrences(s, part);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "axxxxyyyyb".to_string();
        let part = "xy".to_string();
        let output = "ab".to_string();
        let result = Solution::remove_occurrences(s, part);
        assert_eq!(result, output);
    }
}
