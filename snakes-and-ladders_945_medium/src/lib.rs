// https://leetcode.com/problems/snakes-and-ladders/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        // 2D board but we "treat" it as a 1D array
        let n = board.len();
        let mut q = std::collections::VecDeque::from([1usize]);
        let mut visited = vec![false; n * n + 1];
        visited[1] = true;
        let mut lvl = 0;

        while !q.is_empty() {
            let size = q.len();
            for _ in 0..size {
                let cur = q.pop_front().unwrap();
                if cur == n * n {
                    return lvl;
                }

                for next_move in cur + 1..=cur + 6 {
                    if next_move > n * n {
                        continue;
                    }

                    let next_cell_value = {
                        // the "treatment"
                        // i = i * m + j
                        // i = i / n
                        // j = i % n

                        let pos_0_indexed = next_move - 1;
                        let row = pos_0_indexed / n;
                        let col = pos_0_indexed % n;

                        // Board is numbered from bottom-left, so flip the row
                        let board_row = n - 1 - row;

                        if row % 2 == 0 {
                            // even row (from bottom), left to right
                            board[board_row][col]
                        } else {
                            // odd row (from bottom), right to left
                            board[board_row][n - 1 - col]
                        }
                    };

                    let destination = if next_cell_value == -1 {
                        next_move as usize
                    } else {
                        next_cell_value as usize
                    };

                    if !visited[destination as usize] {
                        q.push_back(destination);
                        visited[destination as usize] = true;
                    }
                }
            }
            lvl += 1;
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let board = vec![
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 35, -1, -1, 13, -1],
            vec![-1, -1, -1, -1, -1, -1],
            vec![-1, 15, -1, -1, -1, -1],
        ];
        let output = 4;
        let result = Solution::snakes_and_ladders(board);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let board = vec![vec![-1, -1], vec![-1, 3]];
        let output = 1;
        let result = Solution::snakes_and_ladders(board);
        assert_eq!(result, output);
    }
}
