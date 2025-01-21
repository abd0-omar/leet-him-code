// https://leetcode.com/problems/grid-game/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let mut sum_top = grid[0].iter().map(|&x| x as i64).sum::<i64>() - grid[0][0] as i64;
        let mut sum_bot = 0i64;

        let mut res = sum_top.max(sum_bot);
        for j in 1..grid[0].len() {
            sum_top -= grid[0][j] as i64;
            sum_bot += grid[1][j - 1] as i64;
            res = res.min(sum_top.max(sum_bot));
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let grid = vec![vec![2, 5, 4], vec![1, 5, 1]];
        let output = 4;
        let result = Solution::grid_game(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let grid = vec![vec![3, 3, 1], vec![8, 5, 2]];
        let output = 4;
        let result = Solution::grid_game(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let grid = vec![vec![1, 3, 1, 15], vec![1, 3, 3, 1]];
        let output = 7;
        let result = Solution::grid_game(grid);
        assert_eq!(result, output);
    }
}
