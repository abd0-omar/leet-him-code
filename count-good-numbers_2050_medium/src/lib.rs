// https://leetcode.com/problems/count-good-numbers/
#[allow(dead_code)]
struct Solution;

const MOD: i64 = 1_000_000_007;
#[allow(dead_code)]
impl Solution {
    pub fn count_good_numbers(n: i64) -> i32 {
        // similar to https://leetcode.com/problems/powx-n/description/

        let n0 = n / 2 + n % 2;
        let n1 = n / 2;
        ((pow(5, n0) * pow(4, n1)) % MOD) as i32
    }
}

fn pow(x: i64, n: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    let y = pow(x, n / 2);
    let y = (y * y) % MOD;
    if n % 2 == 0 { y } else { (y * x) % MOD }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 1;
        let output = 5;
        let result = Solution::count_good_numbers(n);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 4;
        let output = 400;
        let result = Solution::count_good_numbers(n);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let n = 50;
        let output = 564908303;
        let result = Solution::count_good_numbers(n);
        assert_eq!(result, output);
    }
}
