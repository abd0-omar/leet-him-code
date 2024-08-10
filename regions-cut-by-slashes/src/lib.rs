pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
    let grid: Vec<Vec<char>> = grid
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();
    let (rows1, cols1) = (grid.len(), grid[0].len());
    let (rows2, cols2) = (rows1 * 3, cols1 * 3);

    // 0 open, 1 closed
    let mut grid2 = vec![vec![0; cols2]; rows2];

    for r in 0..rows1 {
        for c in 0..cols1 {
            // top left of our imaginary 3x3 grid
            let (r2, c2) = (r * 3, c * 3);

            if grid[r][c] == '/' {
                // top
                grid2[r2][c2 + 2] = 1;
                // mid
                grid2[r2 + 1][c2 + 1] = 1;
                // bot
                grid2[r2 + 2][c2] = 1;
            } else if grid[r][c] == '\\' {
                // top
                grid2[r2][c2] = 1;
                // mid
                grid2[r2 + 1][c2 + 1] = 1;
                // bot
                grid2[r2 + 2][c2 + 2] = 1;
            }
        }
    }

    dbg!(&grid);
    dbg!(&grid2);

    let mut result = 0;
    for r in 0..rows2 {
        for c in 0..cols2 {
            if grid2[r][c] == 0 {
                dfs(&mut grid2, r, c);
                result += 1;
            }
        }
    }

    result
}

fn dfs(grid: &mut Vec<Vec<i32>>, r: usize, c: usize) {
    if !(0..grid.len()).contains(&r) || !(0..grid[0].len()).contains(&c) || grid[r][c] == 1 {
        return;
    }

    grid[r][c] = 1;

    for (dr, dc) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
        let nr = (dr + r as i32) as usize;
        let nc = (dc + c as i32) as usize;

        dfs(grid, nr, nc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid = vec!["/\\".to_string(), "\\/".to_string()];
        let output = 5;
        let result = regions_by_slashes(grid);
        assert_eq!(result, output);
    }
}
