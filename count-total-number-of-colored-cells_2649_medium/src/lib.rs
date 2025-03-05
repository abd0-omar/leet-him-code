// https://leetcode.com/problems/count-total-number-of-colored-cells/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        // there is an obvious bfs solution here
        // but there is a math pattern too
        // 1, 1 + 4 (5), 5 + 8 (13)
        // 1, += 4*1, += 4*2
        let mut result = 1;
        for i in 1..n as i64 {
            result += 4 * i;
            // dbg!(i);
            // dbg!(4i64.pow(i));
            // dbg!(result);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 1;
        let output = 1;
        let result = Solution::colored_cells(n);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 2;
        let output = 5;
        let result = Solution::colored_cells(n);
        assert_eq!(result, output);
    }
    #[test]
    fn it_works2() {
        let n = 3;
        let output = 13;
        let result = Solution::colored_cells(n);
        assert_eq!(result, output);
    }
}
