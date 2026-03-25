// https://leetcode.com/problems/equal-sum-grid-partition-i/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        /*
        1, 4 -> 5
        2, 3 -> 5

        1, 4
        2, 3
        |  |
        3  7


                0
        1, 4 -> 5
        2, 3 -> 10
        ---------
        2, 0 -> 12
        8, 0 -> 20
                20

        upper row sum -> 10 -  0 = 10
        lower row sum -> 20 - 10 = 10

         */
        let n = grid.len();
        let m = grid[0].len();
        let mut prefix_sum_row = vec![0; n + 1];
        let mut total_row_sum = 0;
        let mut prefix_sum_col = vec![0; m + 1];
        let mut total_col_sum = 0;

        for i in 0..n {
            let row_sum = grid[i].iter().map(|&x| x as i64).sum::<i64>();
            prefix_sum_row[i + 1] = prefix_sum_row[i] as i64 + row_sum;
            total_row_sum += row_sum;
        }

        for j in 0..m {
            let mut col_sum = 0i64;
            for i in 0..n {
                col_sum += grid[i][j] as i64;
            }

            prefix_sum_col[j + 1] = prefix_sum_col[j] + col_sum;
            total_col_sum += col_sum;
        }

        for row_sum in prefix_sum_row {
            if row_sum == total_row_sum - row_sum {
                return true;
            }
        }

        for col_sum in prefix_sum_col {
            if col_sum == total_col_sum - col_sum {
                return true;
            }
        }

        // dbg!(&prefix_sum_row);
        // dbg!(prefix_sum_col);
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let grid = vec![vec![1, 4], vec![2, 3]];
        let output = true;
        let result = Solution::can_partition_grid(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let grid = vec![vec![1, 3], vec![2, 4]];
        let output = false;
        let result = Solution::can_partition_grid(grid);
        assert_eq!(result, output);
    }
}
