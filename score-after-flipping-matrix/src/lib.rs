pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
    let m = grid.len();
    let n = grid[0].len();

    // if row starts with 0, flip it
    for i in 0..m {
        if grid[i][0] == 0 {
            flip_row(&mut grid, i);
        }
    }

    // if no. of 0 in a col > no. of 1, flip it
    for j in 1..n {
        let mut count_zeros = 0;
        let mut count_ones = 0;
        for i in 0..m {
            if grid[i][j] == 0 {
                count_zeros += 1;
            } else {
                count_ones += 1;
            }
        }
        if count_zeros > count_ones {
            // println!("happened {}", j);
            flip_col(&mut grid, j);
        }
    }

    // calc
    let mut sum = 0;
    for i in 0..m {
        let row: String = grid[i]
            .iter()
            .map(|&f| char::from_digit(f as u32, 10).unwrap())
            .collect();
        // println!("row={}", row);
        // dbg!(i32::from_str_radix(&row, 2).unwrap());
        sum += i32::from_str_radix(&row, 2).unwrap();
    }
    sum
}

fn flip_col(grid: &mut Vec<Vec<i32>>, col: usize) {
    // println!("col{}", col);
    for i in 0..grid.len() {
        for j in col..grid[0].len() {
            // println!("{}{}", i, j);
            if grid[i][j] == 0 {
                grid[i][j] = 1;
            } else {
                grid[i][j] = 0;
            }
        }
    }
}

fn flip_row(grid: &mut Vec<Vec<i32>>, row: usize) {
    for i in row..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                grid[i][j] = 1;
            } else {
                grid[i][j] = 0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let grid = vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]];
        let output = 39;
        let result = matrix_score(grid);
        assert_eq!(result, output);
    }
}
