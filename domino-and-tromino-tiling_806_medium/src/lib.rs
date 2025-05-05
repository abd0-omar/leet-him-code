// https://leetcode.com/problems/domino-and-tromino-tiling/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    const MOD: i32 = 1_000_000_007;
    pub fn num_tilings(n: i32) -> i32 {
        let mut memo = vec![vec![-1; n as usize + 1]; n as usize + 1];
        Self::helper(n, n, &mut memo)
    }

    fn helper(first_row: i32, second_row: i32, memo: &mut [Vec<i32>]) -> i32 {
        if first_row == 0 && second_row == 0 {
            return 1;
        }
        if first_row < 0 || second_row < 0 {
            return 0;
        }

        if memo[first_row as usize][second_row as usize] != -1 {
            return memo[first_row as usize][second_row as usize];
        }

        let result = if first_row == second_row {
            // Case 1: One vertical domino
            (((Self::helper(first_row - 1, second_row - 1, memo) % Self::MOD
                + Self::helper(first_row - 2, second_row - 2, memo) % Self::MOD)
                % Self::MOD
                + Self::helper(first_row - 2, second_row - 1, memo) % Self::MOD)
                % Self::MOD
                + Self::helper(first_row - 1, second_row - 2, memo) % Self::MOD)
                % Self::MOD
        } else if first_row > second_row {
            // Case 2: Horizontal domino at first row or tromino
            (Self::helper(first_row - 2, second_row, memo) % Self::MOD
                + Self::helper(first_row - 2, second_row - 1, memo) % Self::MOD)
                % Self::MOD
        } else {
            // Case 3: Horizontal domino at second row or inverted tromino
            (Self::helper(first_row, second_row - 2, memo) % Self::MOD
                + Self::helper(first_row - 1, second_row - 2, memo) % Self::MOD)
                % Self::MOD
        };

        memo[first_row as usize][second_row as usize] = result;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 3;
        let output = 5;
        let result = Solution::num_tilings(n);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 1;
        let output = 1;
        let result = Solution::num_tilings(n);
        assert_eq!(result, output);
    }
}
