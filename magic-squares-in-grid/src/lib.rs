pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
    // traverse every 3x3 and if it's a "magic square" then mark all it's cells visited
    let n = grid.len();
    let m = grid[0].len();

    let mut count = 0;
    for row in 0..n as i32 - 2 {
        for col in 0..m as i32 - 2 {
            if is_magic_square(&grid, row as usize, col as usize) {
                count += 1
            }
        }
    }

    count
}

fn is_magic_square(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
    let mut visited: [bool; 10] = [false; 10];
    for i in 0..3 {
        for j in 0..3 {
            let nr = i + row;
            let nc = j + col;
            if !(1..=9).contains(&grid[nr][nc]) {
                return false;
            }
            if visited[grid[nr][nc] as usize] {
                return false;
            }
            visited[grid[nr][nc] as usize] = true;
        }
    }

    // rows sum
    let mut row_sum = None;
    for i in 0..3 {
        let nr = i + row;
        let mut cur_row_sum = 0;
        for j in 0..3 {
            let nc = j + col;
            cur_row_sum += grid[nr][nc];
        }
        if let Some(sum) = row_sum {
            if cur_row_sum != sum {
                return false;
            }
        } else {
            // first time assigning to `row_sum`
            row_sum = Some(cur_row_sum);
        }
    }

    // cols sum
    for j in 0..3 {
        let nc = j + col;
        let mut cur_col_sum = 0;
        for i in 0..3 {
            let nr = i + row;
            dbg!(row);
            dbg!(col);
            dbg!(nr);
            dbg!(nc);
            cur_col_sum += grid[nr][nc];
        }
        if row_sum.unwrap() != cur_col_sum {
            return false;
        }
    }

    // diag sum
    let mut diag_sum = 0;
    let mut inverse_diag_sum = 0;
    for i in 0..3 {
        let nr = i + row;
        diag_sum += grid[nr][col + i];
        inverse_diag_sum += grid[nr][col + 2 - i];
    }

    if diag_sum != row_sum.unwrap() || diag_sum != inverse_diag_sum {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let grid = vec![vec![4, 3, 8, 4], vec![9, 5, 1, 9], vec![2, 7, 6, 2]];
        let output = 1;
        let result = num_magic_squares_inside(grid);
        assert_eq!(result, output);
    }
    #[test]
    fn it_works1() {
        let grid = vec![vec![8]];
        let output = 0;
        let result = num_magic_squares_inside(grid);
        assert_eq!(result, output);
    }
}
