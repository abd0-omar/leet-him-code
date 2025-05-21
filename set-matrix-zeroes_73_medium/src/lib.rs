// https://leetcode.com/problems/set-matrix-zeroes/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        // // for O(n*m) space a bfs solution is trivial
        // // for O(1) space the trick of adding the top row & col with zeros is
        // // smart
        // let mut flag_top_row = false;
        // let mut flag_top_col = false;
        // let n = matrix.len();
        // let m = matrix[0].len();
        // for i in 0..n {
        //     for j in 0..m {
        //         if matrix[i][j] == 0 {
        //             matrix[i][0] = 0;
        //             matrix[0][j] = 0;
        //             if i == 0 {
        //                 flag_top_row = true;
        //             }
        //             if j == 0 {
        //                 flag_top_col = true;
        //             }
        //         }
        //     }
        // }
        // for i in 1..n {
        //     for j in 1..m {
        //         if matrix[i][0] == 0 || matrix[0][j] == 0 {
        //             matrix[i][j] = 0;
        //         }
        //     }
        // }
        // if flag_top_row {
        //     for j in 1..m {
        //         matrix[0][j] = 0;
        //     }
        // }
        // if flag_top_col {
        //     for i in 1..n {
        //         matrix[i][0] = 0;
        //     }
        // }

        // the queue solution
        let mut q = std::collections::VecDeque::new();
        let (n, m) = (matrix.len(), matrix[0].len());
        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == 0 {
                    q.push_back((i, j));
                }
            }
        }
        while let Some((i, j)) = q.pop_front() {
            // wrapping add
            // overflowing add
            // saturating add
            for k in 0..m {
                matrix[i][k] = 0;
            }
            for k in 0..n {
                matrix[k][j] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let mut matrix = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        let output = vec![vec![1, 0, 1], vec![0, 0, 0], vec![1, 0, 1]];
        let _result = Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, output);
    }

    #[test]
    fn it_works1() {
        let mut matrix = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        let output = vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]];
        let _result = Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, output);
    }
}
