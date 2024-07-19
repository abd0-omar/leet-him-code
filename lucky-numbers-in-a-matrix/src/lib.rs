pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    /*
    3, 7, 8
    9, 11, 13
    15, 16, 17
    input is too small btw so I can get away with brute force
    min row, max col
    i
    0, 0, 0
    1, 1, 1
    2, 2, 2
    j
    0, 1, 2
    0, 1, 2
    0, 1, 2
    all distinct numbers btw so check max or min gonna be easier
    */
    let mut result = vec![];

    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if check_lucky_number(&matrix, i, j) {
                result.push(matrix[i][j]);
            }
        }
    }
    result
}

fn check_lucky_number(matrix: &Vec<Vec<i32>>, i: usize, j: usize) -> bool {
    // check min row
    let cur = matrix[i][j];
    for n_j in 0..matrix[0].len() {
        if n_j == j {
            continue;
        }
        // println!("check row");
        // dbg!(cur);
        // dbg!(i);
        // dbg!(n_j);
        if matrix[i][n_j] < cur {
            return false;
        }
    }
    // check max col
    for n_i in 0..matrix.len() {
        if n_i == i {
            continue;
        }
        // println!("check col");
        // dbg!(cur);
        // dbg!(n_i);
        // dbg!(j);
        if matrix[n_i][j] > cur {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]];
        let output = vec![15];
        // Explanation: 15 is the only lucky number since it is the minimum in its row and the maximum in its column.
        let result = lucky_numbers(matrix);
        assert_eq!(result, output);
    }
}
