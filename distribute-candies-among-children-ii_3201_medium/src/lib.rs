// https://leetcode.com/problems/distribute-candies-among-children-ii/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn distribute_candies(n: i32, limit: i32) -> i64 {
        let mut ans = 0;
        for i in 0..=n.min(limit) {
            if n - i > 2 * limit {
                continue;
            }
            ans += (n - i).min(limit) as i64 - 0.max(n - i - limit) as i64 + 1;
        }

        return ans;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 5;
        let limit = 2;
        let output = 3;
        let result = Solution::distribute_candies(n, limit);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 3;
        let limit = 3;
        let output = 10;
        let result = Solution::distribute_candies(n, limit);
        assert_eq!(result, output);
    }
}
