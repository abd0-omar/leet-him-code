// https://leetcode.com/problems/maximum-matrix-sum/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut total_sum: i64 = 0;
        let mut smallest_abs = i32::MAX;
        let mut negative_count = 0;

        for row in &matrix {
            for &val in row {
                total_sum += val.abs() as i64;
                if val < 0 {
                    negative_count += 1;
                }
                smallest_abs = smallest_abs.min(val.abs());
            }
        }

        // If the count of negatives is odd, subtract twice the smallest absolute value
        if negative_count % 2 != 0 {
            total_sum -= 2 * smallest_abs as i64;
        }

        total_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let matrix = vec![vec![1, -1], vec![-1, 1]];
        let output = 4;
        let result = Solution::max_matrix_sum(matrix);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let matrix = vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]];
        let output = 16;
        let result = Solution::max_matrix_sum(matrix);
        assert_eq!(result, output);
    }
}
