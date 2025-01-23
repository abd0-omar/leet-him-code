// https://leetcode.com/problems/count-servers-that-communicate/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        // could be solved in many different ways, arrays would be the most efficient one I guess
        // I will go with brute-force as the input is too low

        let n = grid.len();
        let m = grid[0].len();

        let mut result = 0;

        for i in 0..n {
            'outer: for j in 0..m {
                if grid[i][j] == 1 {
                    // right
                    for right in 1..m {
                        let nj = right + j;
                        if (0..m).contains(&nj) {
                            if grid[i][nj] == 1 {
                                result += 1;
                                continue 'outer;
                            }
                        }
                    }
                    for left in 1..m {
                        if let (_, true) = j.overflowing_sub(left) {
                            continue;
                        }
                        let nj = j - left;
                        if (0..m).contains(&nj) {
                            if grid[i][nj] == 1 {
                                result += 1;
                                continue 'outer;
                            }
                        }
                    }

                    for down in 1..n {
                        let ni = i + down;
                        if (0..n).contains(&ni) {
                            if grid[ni][j] == 1 {
                                result += 1;
                                continue 'outer;
                            }
                        }
                    }

                    for up in 1..n {
                        if let (_, true) = i.overflowing_sub(up) {
                            continue;
                        }
                        let ni = i - up;
                        if (0..n).contains(&ni) {
                            if grid[ni][j] == 1 {
                                result += 1;
                                continue 'outer;
                            }
                        }
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let grid = vec![vec![1, 0], vec![0, 1]];
        let output = 0;
        let result = Solution::count_servers(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let grid = vec![vec![1, 0], vec![1, 1]];
        let output = 3;
        let result = Solution::count_servers(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let grid = vec![
            vec![1, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
        ];
        let output = 4;
        let result = Solution::count_servers(grid);
        assert_eq!(result, output);
    }
}
