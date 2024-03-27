fn main() {
    let matrix = vec![vec![1, 1]];
    let target = 3;
    // Output: true
    search_matrix(matrix, target);
}

fn possible(matrix: &Vec<Vec<i32>>, target: i32, unflattend_i: usize, unflattend_j: usize) -> bool {
    //mid -> 3
    // i -> 1, m -> 4, j -> 1
    // let flattened = i * m + j;

    if matrix[unflattend_i][unflattend_j] == target {
        return true;
    }
    false
}

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let n = matrix.len();
    let m = matrix[0].len();

    if n == 1 && m == 1 {
        return matrix[0][0] == target;
    }

    let mut l = 0;
    let mut r = (n * m) as i32 - 1;

    while l <= r {
        //mid index flattend
        let mid = l + (r - l) / 2;
        println!("DEBUGPRINT[1]: main.rs:28: mid={:#?}", mid);

        let unflattend_i = mid as usize / m;
        println!(
            "DEBUGPRINT[2]: main.rs:31: unflattend_i={:#?}",
            unflattend_i
        );
        let unflattend_j = mid as usize % m;
        println!(
            "DEBUGPRINT[3]: main.rs:33: unflattend_j={:#?}",
            unflattend_j
        );

        println!(
            "DEBUGPRINT[4]: main.rs:13: matrix={:#?}",
            matrix[unflattend_i as usize][unflattend_j as usize]
        );

        if possible(&matrix, target, unflattend_i, unflattend_j) {
            return true;
        } else if matrix[unflattend_i as usize][unflattend_j as usize] < target {
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }

    false
}
