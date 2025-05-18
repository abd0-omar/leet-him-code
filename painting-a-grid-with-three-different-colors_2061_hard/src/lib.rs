// https://leetcode.com/problems/painting-a-grid-with-three-different-colors/
#[allow(dead_code)]
struct Solution;

use std::collections::HashMap;
const MOD: i32 = 1_000_000_007;

#[allow(dead_code)]
impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let patterns = gen_col_patterns(m);
        dbg!(&patterns);

        // Precompute compatibility
        let mut compat = vec![vec![]; patterns.len()];
        for (i, a) in patterns.iter().enumerate() {
            for (j, b) in patterns.iter().enumerate() {
                if is_compatible(a, b) {
                    compat[i].push(j);
                }
            }
        }
        dbg!(&compat);

        let mut memo = HashMap::new();
        let mut ans = 0;
        for i in 0..patterns.len() {
            ans = (ans + dp(1, i, n, &patterns, &compat, &mut memo)) % MOD;
        }
        ans
    }
}

fn is_compatible(a: &[u8], b: &[u8]) -> bool {
    a.iter().zip(b.iter()).all(|(&x, &y)| x != y)
}

// backtrack
fn gen_col_patterns(m: usize) -> Vec<Vec<u8>> {
    let mut res = vec![];
    dfs(m, &mut vec![], &mut res);
    res
}
fn dfs(m: usize, col: &mut Vec<u8>, res: &mut Vec<Vec<u8>>) {
    if col.len() == m {
        res.push(col.clone());
        return;
    }
    for color in 0..3 {
        if col.last().map_or(false, |&last| last == color) {
            continue;
        }
        col.push(color);
        dfs(m, col, res);
        col.pop();
    }
}
fn dp(
    col: usize,
    prev_idx: usize,
    n: usize,
    patterns: &Vec<Vec<u8>>,
    compat: &Vec<Vec<usize>>,
    memo: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    if col == n {
        return 1;
    }
    if let Some(&v) = memo.get(&(col, prev_idx)) {
        return v;
    }
    let mut ans = 0;
    for &next_idx in &compat[prev_idx] {
        ans = (ans + dp(col + 1, next_idx, n, patterns, compat, memo)) % MOD;
    }
    memo.insert((col, prev_idx), ans);
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let m = 1;
        let n = 1;
        let output = 3;
        let result = Solution::color_the_grid(m, n);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let m = 1;
        let n = 2;
        let output = 6;
        let result = Solution::color_the_grid(m, n);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let m = 5;
        let n = 5;
        let output = 580986;
        let result = Solution::color_the_grid(m, n);
        assert_eq!(result, output);
    }
}
