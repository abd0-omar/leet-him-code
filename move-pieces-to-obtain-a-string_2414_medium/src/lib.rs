// https://leetcode.com/problems/move-pieces-to-obtain-a-string/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_change(start: String, target: String) -> bool {
        // two-pointers
        let (start, target) = (start.into_bytes(), target.into_bytes());
        let mut start_idx = 0;
        let mut target_idx = 0;
        while start_idx < start.len() || target_idx < target.len() {
            while start_idx < start.len() && start[start_idx] == b'_' {
                start_idx += 1;
            }
            while target_idx < target.len() && target[target_idx] == b'_' {
                target_idx += 1;
            }

            // if one ended and the other one didn't, return false
            if start_idx == start.len() || target_idx == start.len() {
                return start_idx == start.len() && target_idx == start.len();
            }

            if start[start_idx] != target[target_idx] {
                return false;
            }
            if start[start_idx] == b'L' && start_idx < target_idx {
                return false;
            }
            if start[start_idx] == b'R' && start_idx > target_idx {
                return false;
            }
            start_idx += 1;
            target_idx += 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let start = "__l___r___r_".to_string();
        let target = "_l_______r_r".to_string();
        let output = true;
        let result = Solution::can_change(start, target);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let start = "_r__l_".to_string();
        let target = "___l_r".to_string();
        let output = false;
        let result = Solution::can_change(start, target);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let start = "__r".to_string();
        let target = "_r_".to_string();
        let output = false;
        let result = Solution::can_change(start, target);
        assert_eq!(result, output);
    }
}
