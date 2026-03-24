// https://leetcode.com/problems/construct-product-matrix/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        /*
         *  [1, 2]
         *  [3, 4]
         *
         *  prefix_mult
         *  [1, 2]
         *  [6,24]
         *
         *      1, 2, 3, 4
         *   1, 1, 2, 6, 24
         * 24, 24, 12, 4, 1
         *
         * I remember solving something similar when I first started out years ago, and came up with that conclusion
         * myself, it was 1d tho
         *
         * 6 * 1   under no. left * under under
         * 2 * 4
         * 1 * 12
         * 1 * 24
         */

        // flatten array for simplicity

        let n = grid.len();
        let m = grid[0].len();
        let mut prefix_mult = vec![1; n * m + 1];
        let mut suffix_mult = vec![1; n * m + 1];

        for i in 0..n {
            for j in 0..m {
                let idx = i * m + j;
                prefix_mult[idx + 1] =
                    (prefix_mult[idx] as i64 * grid[i][j] as i64).rem_euclid(12345) as i32;
                suffix_mult[(n * m) - idx - 1] = (suffix_mult[(n * m) - idx] as i64
                    * grid[n - i - 1][m - j - 1] as i64)
                    .rem_euclid(12345) as i32;
            }
        }

        let mut result = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                let idx = i * m + j;
                result[i][j] = (prefix_mult[idx] as i64 * suffix_mult[idx + 1] as i64)
                    .rem_euclid(12345) as i32;
            }
        }
        // dbg!(prefix_mult);
        // dbg!(suffix_mult);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let output = vec![vec![24, 12], vec![8, 6]];
        let result = Solution::construct_product_matrix(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let grid = vec![vec![12345], vec![2], vec![1]];
        let output = vec![vec![2], vec![0], vec![0]];
        let result = Solution::construct_product_matrix(grid);
        assert_eq!(result, output);
    }
}
