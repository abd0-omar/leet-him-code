fn main() {
    println!("Hello, world!");
    let grid = vec![vec![0, 1, 1], vec![1, 0, 1], vec![0, 0, 1]];
    // Output: vec![vec![0,0,4],vec![0,0,4],vec![-2,-2,2]];
    println!("{:?}", ones_minus_zeros(grid));
}

pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut ones_row = vec![0; m];
    let mut ones_col = vec![0; n];
    let mut zeros_row = vec![0; m];
    let mut zeros_col = vec![0; n];

    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 1 {
                ones_row[i] += 1;
                ones_col[j] += 1;
            }

            if grid[i][j] == 0 {
                zeros_row[i] += 1;
                zeros_col[j] += 1;
            }
        }
    }
    // println!("ones_row={:?}", ones_row);
    // println!("ones_col={:?}", ones_col);
    // println!("zeros_row={:?}", zeros_row);
    // println!("zeros_col={:?}", zeros_col);

    let mut diff = vec![vec![0; n]; m];

    for i in 0..m {
        for j in 0..n {
            diff[i][j] = ones_row[i] + ones_col[j] - zeros_row[i] - zeros_col[j];
        }
    }

    diff
}
