// https://leetcode.com/problems/minimum-operations-to-make-a-uni-value-grid/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        // The median is the answer for statistics reasons
        let mut grid: Vec<i32> = grid.into_iter().flatten().collect();
        grid.sort_unstable();
        // check first if it's not possible (impossible) then return -1
        let n = grid.len();
        let median = *grid.iter().nth(n / 2).unwrap();
        let mut result = 0;
        for &element in grid.iter() {
            if element % x != median % x {
                return -1;
            }
            result += (element - median).abs() / x
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let grid = vec![vec![2, 4], vec![6, 8]];
        let x = 2;
        let output = 4;
        let result = Solution::min_operations(grid, x);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let grid = vec![vec![1, 5], vec![2, 3]];
        let x = 1;
        let output = 5;
        let result = Solution::min_operations(grid, x);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let x = 2;
        let output = -1;
        let result = Solution::min_operations(grid, x);
        assert_eq!(result, output);
    }
}
