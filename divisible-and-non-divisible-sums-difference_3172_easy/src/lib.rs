// https://leetcode.com/problems/divisible-and-non-divisible-sums-difference/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn difference_of_sums(n: i32, m: i32) -> i32 {
        let sum1 = (1..n + 1).into_iter().filter(|&x| x % m != 0).sum::<i32>();
        let sum2 = (1..n + 1).into_iter().filter(|&x| x % m == 0).sum::<i32>();
        sum1 - sum2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 10;
        let m = 3;
        let output = 19;
        let result = Solution::difference_of_sums(n, m);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 5;
        let m = 6;
        let output = 15;
        let result = Solution::difference_of_sums(n, m);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let n = 5;
        let m = 1;
        let output = -15;
        let result = Solution::difference_of_sums(n, m);
        assert_eq!(result, output);
    }
}
