pub fn min_days(mut grid: Vec<Vec<i32>>) -> i32 {
    // 0 or 1 or 2
    let (n, m) = (grid.len(), grid[0].len());
    let mut visited = std::collections::HashSet::new();

    let mut count = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 1 && !visited.contains(&(i, j)) {
                dfs(&grid, i, j, &mut visited);
                count += 1;
            }
        }
    }

    // 0 islands, or 2 (already disconnected)
    if count != 1 {
        return 0;
    }

    // brute force every visited (1 land) and make it 0
    for (i, j) in visited.into_iter() {
        grid[i][j] = 0;
        let mut new_visited: std::collections::HashSet<(usize, usize)> =
            std::collections::HashSet::new();

        let mut new_count = 0;
        for r in 0..n {
            for c in 0..m {
                if grid[r][c] == 1 && !new_visited.contains(&(r, c)) {
                    dfs(&grid, r, c, &mut new_visited);
                    new_count += 1;
                }
            }
        }

        if new_count != 1 {
            return 1;
        }

        grid[i][j] = 1;
    }

    2
}

fn dfs(
    grid: &Vec<Vec<i32>>,
    i: usize,
    j: usize,
    visited: &mut std::collections::HashSet<(usize, usize)>,
) {
    if !(0..grid.len()).contains(&i)
        || !(0..grid[0].len()).contains(&j)
        || grid[i][j] == 0
        || !visited.insert((i, j))
    {
        return;
    }

    for (di, dj) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
        let ni = (di + (i as i32)) as usize;
        let nj = (dj + (j as i32)) as usize;

        dfs(grid, ni, nj, visited);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid = vec![vec![0, 1, 1, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 0]];
        let output = 2;
        let result = min_days(grid);
        assert_eq!(result, output);
    }
}
