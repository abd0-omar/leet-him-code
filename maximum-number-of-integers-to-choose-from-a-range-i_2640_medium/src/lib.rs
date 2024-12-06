// https://leetcode.com/problems/maximum-number-of-integers-to-choose-from-a-range-i/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_count(mut banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        banned.retain(|&x| x <= max_sum.min(n));
        banned.sort_unstable();
        banned.dedup();
        let mut sum = 0;
        let mut count = 0;

        let mut banned_idx = 0;
        for i in 1..n + 1 {
            if banned_idx < banned.len() && banned[banned_idx] == i {
                banned_idx += 1;
                continue;
            }
            sum += i;
            if sum > max_sum {
                break;
            }
            count += 1;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let banned = vec![1, 6, 5];
        let n = 5;
        let max_sum = 6;
        let output = 2;
        let result = Solution::max_count(banned, n, max_sum);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let banned = vec![1, 2, 3, 4, 5, 6, 7];
        let n = 8;
        let max_sum = 1;
        let output = 0;
        let result = Solution::max_count(banned, n, max_sum);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let banned = vec![11];
        let n = 7;
        let max_sum = 50;
        let output = 7;
        let result = Solution::max_count(banned, n, max_sum);
        assert_eq!(result, output);
    }
}
