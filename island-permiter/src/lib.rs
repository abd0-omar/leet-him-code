// overengineered solution for an "easy" constraints
pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
    let mut perimeter = 0;
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    'outer: for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                _island_perimeter(&grid, &mut perimeter, &mut visited, (i, j));
                break 'outer;
            }
        }
    }
    perimeter
}

pub fn _island_perimeter(
    grid: &Vec<Vec<i32>>,
    perimeter: &mut i32,
    visited: &mut Vec<Vec<bool>>,
    cur_pos: (usize, usize),
) {
    if !(0..grid.len()).contains(&(cur_pos.0))
        || !(0..grid[0].len()).contains(&(cur_pos.1))
        || grid[cur_pos.0][cur_pos.1] == 0
        || visited[cur_pos.0][cur_pos.1]
    {
        return;
    }

    visited[cur_pos.0][cur_pos.1] = true;

    let mut surrounded_water_or_borders = 0;
    for (i, j) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
        let ni = (i + cur_pos.0 as i32) as usize;
        let nj = (j + cur_pos.1 as i32) as usize;
        if !(0..grid.len()).contains(&ni) || !(0..grid[0].len()).contains(&nj) || grid[ni][nj] == 0
        {
            surrounded_water_or_borders += 1;
        }
    }

    *perimeter += surrounded_water_or_borders;
    for (i, j) in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
        let ni = (i + cur_pos.0 as i32) as usize;
        let nj = (j + cur_pos.1 as i32) as usize;
        _island_perimeter(grid, perimeter, visited, (ni, nj));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Output: 16
    // Explanation: The perimeter is the 16 yellow stripes in the image above.
    #[test]
    fn it_works() {
        let grid = vec![
            vec![0, 1, 0, 0],
            vec![1, 1, 1, 0],
            vec![0, 1, 0, 0],
            vec![1, 1, 0, 0],
        ];
        let result = island_perimeter(grid);
        assert_eq!(result, 16);
    }
}
