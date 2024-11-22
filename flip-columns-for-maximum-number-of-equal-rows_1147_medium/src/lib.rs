// https://leetcode.com/problems/flip-columns-for-maximum-number-of-equal-rows/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        // very clear solution
        // made with observation on the test cases from neetcode
        use std::collections::HashMap;
        let mut rows_count_map = HashMap::new();
        for row in matrix {
            if row[0] == 0 {
                *rows_count_map.entry(row).or_insert(0) += 1;
            } else {
                *rows_count_map
                    .entry(
                        row.iter()
                            .map(|&x| if x == 0 { 1 } else { 0 })
                            .collect::<Vec<_>>(),
                    )
                    .or_insert(0) += 1;
            }
        }
        *rows_count_map.values().max().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let matrix = vec![vec![0, 1], vec![1, 1]];
        let output = 1;
        let result = Solution::max_equal_rows_after_flips(matrix);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let matrix = vec![vec![0, 1], vec![1, 0]];
        let output = 2;
        let result = Solution::max_equal_rows_after_flips(matrix);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let matrix = vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]];
        let output = 2;
        let result = Solution::max_equal_rows_after_flips(matrix);
        assert_eq!(result, output);
    }
}
