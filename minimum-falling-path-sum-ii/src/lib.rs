pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let mut memo: std::collections::HashMap<(usize, usize), i32> = std::collections::HashMap::new();
    let mut answer = i32::MAX;

    for col in 0..grid.len() {
        answer = answer.min(_min_falling_path_sum(0, col, &grid, &mut memo));
    }

    answer
}

fn _min_falling_path_sum(
    row: usize,
    col: usize,
    grid: &Vec<Vec<i32>>,
    memory: &mut std::collections::HashMap<(usize, usize), i32>,
) -> i32 {
    if row == grid.len() - 1 {
        return grid[row][col];
    }

    if let Some(&result) = memory.get(&(row, col)) {
        return result;
    }

    let mut next_minimum = i32::MAX;
    for next_row_col in 0..grid.len() {
        if next_row_col != col {
            next_minimum =
                next_minimum.min(_min_falling_path_sum(row + 1, next_row_col, grid, memory));
        }
    }

    let result = grid[row][col] + next_minimum;
    memory.insert((row, col), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let output = 13;
        let result = min_falling_path_sum(grid);
        assert_eq!(result, output);
    }
}
