pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
    // clearly a dp matrix problem
    // because instead of recomputing a square to find the max moves starting from that square
    // we could have computed it before and just reuse it

    // (row - 1, col + 1), (row, col + 1) and (row + 1, col + 1)
    let n = grid.len();
    let m = grid[0].len();
    let mut max_count = 0;
    let mut memory = vec![vec![-1; m]; n];
    for i in 0..n {
        // start at any cell in the first column
        // dummy prev
        let prev = i32::MIN;
        max_count = max_count.max(dp(i, 0, &grid, n, m, prev, &mut memory) - 1);
    }
    max_count
}

fn dp(
    i: usize,
    j: usize,
    grid: &[Vec<i32>],
    n: usize,
    m: usize,
    prev: i32,
    memory: &mut Vec<Vec<i32>>,
) -> i32 {
    // next cell must be bigger
    if !(0..n).contains(&i) || !(0..m).contains(&j) || grid[i][j] <= prev {
        return 0;
    }

    if memory[i][j] != -1 {
        return memory[i][j];
    }

    let prev = grid[i][j];

    let right = dp(i, j + 1, grid, n, m, prev, memory) + 1;
    let up_right = dp(i.wrapping_sub(1), j + 1, grid, n, m, prev, memory) + 1;
    let down_right = dp(i + 1, j + 1, grid, n, m, prev, memory) + 1;

    let result = right.max(up_right.max(down_right));
    memory[i][j] = result;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid = vec![
            vec![2, 4, 3, 5],
            vec![5, 4, 9, 3],
            vec![3, 4, 2, 11],
            vec![10, 9, 13, 15],
        ];
        let output = 3;
        let result = max_moves(grid);
        assert_eq!(result, output);
    }
}
