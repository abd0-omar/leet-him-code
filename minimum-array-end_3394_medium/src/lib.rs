// https://leetcode.com/problems/minimum-array-end/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        // start with `x`
        // then we must reserve the ones I believe
        let x = x as i64;
        let mut result = x;
        for _ in (1..n).rev() {
            result = (result + 1) | x;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 3;
        let x = 4;
        let output = 6;
        let result = Solution::min_end(n, x);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 2;
        let x = 7;
        let output = 15;
        let result = Solution::min_end(n, x);
        assert_eq!(result, output);
    }
}
