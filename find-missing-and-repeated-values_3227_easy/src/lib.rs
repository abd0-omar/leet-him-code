// https://leetcode.com/problems/find-missing-and-repeated-values/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut hs = std::collections::HashSet::new();
        let n = grid.len();
        let mut repeated = 0;
        let mut missing = 0;
        for i in 0..n {
            for j in 0..n {
                let value = grid[i][j];
                if !hs.insert(value) {
                    repeated = value;
                }
            }
        }

        for i in 1..=(n * n) as i32 {
            if !hs.contains(&i) {
                missing = i;
                break;
            }
        }

        vec![repeated, missing]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let grid = vec![vec![1, 3], vec![2, 2]];
        let output = vec![2, 4];
        let result = Solution::find_missing_and_repeated_values(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let grid = vec![vec![9, 1, 7], vec![8, 9, 2], vec![3, 4, 6]];
        let output = vec![9, 5];
        let result = Solution::find_missing_and_repeated_values(grid);
        assert_eq!(result, output);
    }
}
