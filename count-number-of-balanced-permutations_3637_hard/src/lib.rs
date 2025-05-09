// https://leetcode.com/problems/count-number-of-balanced-permutations/
#[allow(dead_code)]
struct Solution;

const MOD: i64 = 1_000_000_007;

#[allow(dead_code)]
impl Solution {
    pub fn count_balanced_permutations(num: String) -> i32 {
        let n = num.len();
        let mut freq = [0; 10];
        let mut total = 0;
        for ch in num.chars() {
            let digit = ch.to_digit(10).unwrap() as usize;
            freq[digit] += 1;
            total += digit;
        }
        if total % 2 != 0 {
            return 0;
        }

        let target = total / 2;
        let max_odd = (n + 1) / 2;

        let mut combinations = vec![vec![0; max_odd + 1]; max_odd + 1];
        for i in 0..=max_odd {
            combinations[i][i] = 1;
            combinations[i][0] = 1;
            for j in 1..i {
                combinations[i][j] = (combinations[i - 1][j] + combinations[i - 1][j - 1]) % MOD;
            }
        }

        let mut psum = [0; 11];
        for i in (0..=9).rev() {
            psum[i] = psum[i + 1] + freq[i];
        }

        let mut memo = vec![vec![vec![-1; max_odd + 1]; target + 1]; 10];

        dfs(
            0,
            0,
            max_odd,
            &freq,
            &psum,
            target,
            max_odd,
            &combinations,
            &mut memo,
        ) as i32
    }
}

fn dfs(
    pos: usize,
    curr: usize,
    odd_cnt: usize,
    freq: &[usize],
    psum: &[usize],
    target: usize,
    max_odd: usize,
    combinations: &Vec<Vec<i64>>,
    memo: &mut Vec<Vec<Vec<i64>>>,
) -> i64 {
    if odd_cnt > max_odd || psum[pos] < odd_cnt || curr > target {
        return 0;
    }
    if pos > 9 {
        return if curr == target && odd_cnt == 0 { 1 } else { 0 };
    }
    if memo[pos][curr][odd_cnt] != -1 {
        return memo[pos][curr][odd_cnt];
    }

    let even_cnt = psum[pos] - odd_cnt;
    let start = (freq[pos] as i32 - even_cnt as i32).max(0) as usize;
    let end = freq[pos].min(odd_cnt);
    let mut res = 0;
    for i in start..=end {
        let ways = combinations[odd_cnt][i] * combinations[even_cnt][freq[pos] - i] % MOD;
        res = (res
            + ways
                * dfs(
                    pos + 1,
                    curr + i * pos,
                    odd_cnt - i,
                    freq,
                    psum,
                    target,
                    max_odd,
                    combinations,
                    memo,
                )
                % MOD)
            % MOD;
    }
    memo[pos][curr][odd_cnt] = res;
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let num = "123".to_string();
        let output = 2;
        let result = Solution::count_balanced_permutations(num);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let num = "112".to_string();
        let output = 1;
        let result = Solution::count_balanced_permutations(num);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let num = "12345".to_string();
        let output = 0;
        let result = Solution::count_balanced_permutations(num);
        assert_eq!(result, output);
    }
}
