// https://leetcode.com/problems/number-of-subsequences-that-satisfy-the-given-sum-condition/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn num_subseq(mut nums: Vec<i32>, target: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        nums.sort_unstable();
        let n = nums.len();
        let mut result = 0i64;
        for i in 0..n {
            if i == n - 1 {
                if nums[i] * 2 <= target {
                    result = (result + 1) % MOD;
                }
                break;
            }
            let mut l = i;
            let mut r = n - 1;
            let first = nums[i];
            let mut ans: Option<usize> = None;
            // T T T T F F F F
            // we want last 'F'
            // dbg!(i);
            while l <= r {
                let mid = l + (r - l) / 2;
                // dbg!(first);
                // dbg!(&mid);
                // dbg!(nums[mid]);
                // dbg!(first + nums[mid]);
                if first + nums[mid] <= target {
                    ans = Some(mid);
                    l = mid + 1;
                } else {
                    if let (_, true) = mid.overflowing_sub(1) {
                        break;
                    }
                    r = mid - 1;
                }
            }
            // dbg!(&ans);
            if let Some(ans) = ans {
                // result = (result + 2_i64.pow((ans - i) as u32)) % MOD;
                result = (result + mod_pow(2, (ans - i) as usize, MOD)) % MOD;
            } else {
                break;
            }
            // dbg!(&result);
        }
        result as i32
    }
}

fn mod_pow(mut base: i64, mut exp: usize, modulo: i64) -> i64 {
    let mut result = 1;
    base %= modulo;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulo;
        }
        base = (base * base) % modulo;
        exp /= 2;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let nums = vec![3, 5, 6, 7];
        let target = 9;
        let output = 4;
        let result = Solution::num_subseq(nums, target);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let nums = vec![3, 3, 6, 8];
        let target = 10;
        let output = 6;
        let result = Solution::num_subseq(nums, target);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let nums = vec![2, 3, 3, 4, 6, 7];
        let target = 12;
        let output = 61;
        let result = Solution::num_subseq(nums, target);
        assert_eq!(result, output);
    }
}
