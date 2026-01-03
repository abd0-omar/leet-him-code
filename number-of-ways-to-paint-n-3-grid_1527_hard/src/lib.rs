// https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/
#[allow(dead_code)]
struct Solution;

#[derive(PartialEq, Eq, Clone, Copy)]
// Congo flag the black horse of the AFCON
enum Color {
    Green = 0,
    Yellow = 1,
    Red = 2,
}

const COLORS: [Color; 3] = [Color::Green, Color::Yellow, Color::Red];

fn to_idx(c: Option<Color>) -> usize {
    match c {
        Some(Color::Green) => 0,
        Some(Color::Yellow) => 1,
        Some(Color::Red) => 2,
        None => 3,
    }
}

#[allow(dead_code)]
impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let mut memo = vec![vec![vec![vec![-1; 4]; 4]; 4]; n as usize + 1];
        Self::dfs(n, None, None, None, &mut memo)
    }

    fn dfs(
        n: i32,
        pa: Option<Color>,
        pb: Option<Color>,
        pc: Option<Color>,
        memo: &mut Vec<Vec<Vec<Vec<i32>>>>,
    ) -> i32 {
        if n == 0 {
            return 1;
        }

        let (ia, ib, ic) = (to_idx(pa), to_idx(pb), to_idx(pc));
        if memo[n as usize][ia][ib][ic] != -1 {
            return memo[n as usize][ia][ib][ic];
        }

        let mut ans = 0i64;
        let m = 1_000_000_007i64;

        for &a in &COLORS {
            if pa.is_some_and(|prev_a| prev_a == a) {
                continue;
            }

            for &b in &COLORS {
                if pb.is_some_and(|prev_b| prev_b == b) || b == a {
                    continue;
                }

                for &c in &COLORS {
                    if pc.is_some_and(|prev_c| prev_c == c) || c == b {
                        continue;
                    }

                    ans = (ans + Self::dfs(n - 1, Some(a), Some(b), Some(c), memo) as i64) % m;
                }
            }
        }

        memo[n as usize][ia][ib][ic] = ans as i32;
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 1;
        let output = 12;
        let result = Solution::num_of_ways(n);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 5000;
        let output = 30228214;
        let result = Solution::num_of_ways(n);
        assert_eq!(result, output);
    }
}
