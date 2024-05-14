pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
    let mut max_path = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
            if grid[i][j] != 0 {
                _get_maximum_gold(&grid, i, j, &mut visited, &mut max_path, 0);
            }
        }
    }
    max_path
}

pub fn _get_maximum_gold(
    grid: &Vec<Vec<i32>>,
    i: usize,
    j: usize,
    visited: &mut Vec<Vec<bool>>,
    max_path: &mut i32,
    mut curr_path: i32,
) -> () {
    if !(0..grid.len()).contains(&i)
        || !(0..grid[0].len()).contains(&j)
        || visited[i][j]
        || grid[i][j] == 0
    {
        *max_path = *max_path.max(&mut curr_path);
        return;
    }

    visited[i][j] = true;
    curr_path += grid[i][j];

    for (di, dj) in &[(0, 1), (1, 0), (-1, 0), (0, -1)] {
        let ni = (di + i as i32) as usize;
        let nj = (dj + j as i32) as usize;

        _get_maximum_gold(grid, ni, nj, visited, max_path, curr_path);
    }

    visited[i][j] = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid = vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]];
        let output = 24;
        let result = get_maximum_gold(grid);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let grid = vec![
            vec![1, 0, 7, 0, 0, 0],
            vec![2, 0, 6, 0, 1, 0],
            vec![3, 5, 6, 7, 4, 2],
            vec![4, 3, 1, 0, 2, 0],
            vec![3, 0, 5, 0, 20, 0],
        ];
        let output = 60;
        let result = get_maximum_gold(grid);
        assert_eq!(result, output);
    }
}
