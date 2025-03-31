// https://leetcode.com/problems/put-marbles-in-bags/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn put_marbles(weights: Vec<i32>, k: i32) -> i64 {
        let mut pairs = Vec::new();
        for i in 0..weights.len() - 1 {
            pairs.push(weights[i] as i64 + weights[i + 1] as i64);
        }
        pairs.sort();
        let k = k as usize - 1;
        let max = pairs.iter().rev().take(k).sum::<i64>();
        let min = pairs.iter().take(k).sum::<i64>();
        max - min
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let weights = vec![1, 3, 5, 1];
        let k = 2;
        let output = 4;
        let result = Solution::put_marbles(weights, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let weights = vec![1, 3];
        let k = 2;
        let output = 0;
        let result = Solution::put_marbles(weights, k);
        assert_eq!(result, output);
    }
}
