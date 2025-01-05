// https://leetcode.com/problems/shifting-letters-ii/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let n = s.len();
        let mut s = s.chars().collect::<Vec<char>>();
        let mut prefix_diff = vec![0; n + 1];

        for shift in shifts {
            let from = shift[0] as usize;
            let to = shift[1] as usize;
            let direction = shift[2];
            let new_val = if direction == 1 { 1 } else { -1 };
            prefix_diff[from] += new_val;
            prefix_diff[to + 1] -= new_val;
        }

        let mut diff = 0;
        for i in 0..n {
            diff += prefix_diff[i];
            let letter_idx = (s[i] as u8 - b'a') as i32;
            let new_letter_idx = (letter_idx + diff % 26 + 26) % 26;
            s[i] = ((new_letter_idx as u8) + b'a') as char;
        }

        s.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = "abc".to_string();
        let shifts = vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]];
        let output = "ace".to_string();
        let result = Solution::shifting_letters(s, shifts);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "dztz".to_string();
        let shifts = vec![vec![0, 0, 0], vec![1, 1, 1]];
        let output = "catz".to_string();
        let result = Solution::shifting_letters(s, shifts);
        assert_eq!(result, output);
    }
}
