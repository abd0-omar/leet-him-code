// https://leetcode.com/problems/best-sightseeing-pair/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_score_sightseeing_pair(values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut result = i32::MIN;
        let mut max_left = values[0];
        for j in 1..n {
            let second_pair = values[j] - j as i32;
            result = result.max(max_left + second_pair);
            max_left = max_left.max(second_pair + 2 * j as i32);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let values = vec![8, 1, 5, 2, 6];
        let output = 11;
        let result = Solution::max_score_sightseeing_pair(values);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let values = vec![1, 2];
        let output = 2;
        let result = Solution::max_score_sightseeing_pair(values);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let values = vec![1, 2, 2];
        let output = 3;
        let result = Solution::max_score_sightseeing_pair(values);
        assert_eq!(result, output);
    }
}
