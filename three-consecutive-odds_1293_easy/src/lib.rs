// https://leetcode.com/problems/three-consecutive-odds/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        arr.windows(3)
            .any(|window| window.iter().all(|&x| x % 2 != 0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let arr = vec![2, 6, 4, 1];
        let output = false;
        let result = Solution::three_consecutive_odds(arr);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let arr = vec![1, 2, 34, 3, 4, 5, 7, 23, 12];
        let output = true;
        let result = Solution::three_consecutive_odds(arr);
        assert_eq!(result, output);
    }
}
