// https://leetcode.com/problems/count-ways-to-build-good-strings/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
const MOD: i32 = 1_000_000_007;
impl Solution {
    pub fn count_good_strings(mut low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let mut memory = vec![-1; high as usize + 1];
        dp(
            0,
            low as usize,
            high as usize,
            zero as usize,
            one as usize,
            &mut memory,
        )
    }
}

fn dp(idx: usize, low: usize, high: usize, zero: usize, one: usize, memory: &mut Vec<i32>) -> i32 {
    if idx > high {
        return 0;
    }
    let mut result = if idx >= low { 1 } else { 0 };

    if memory[idx] != -1 {
        return memory[idx];
    }

    let take_zero = dp(idx + zero, low, high, zero, one, memory) % MOD;
    let take_one = dp(idx + one, low, high, zero, one, memory) % MOD;

    result = (result + take_zero + take_one) % MOD;
    memory[idx] = result;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let low = 3;
        let high = 3;
        let zero = 1;
        let one = 1;
        let output = 8;
        let result = Solution::count_good_strings(low, high, zero, one);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let low = 2;
        let high = 3;
        let zero = 1;
        let one = 2;
        let output = 5;
        let result = Solution::count_good_strings(low, high, zero, one);
        assert_eq!(result, output);
    }
}
