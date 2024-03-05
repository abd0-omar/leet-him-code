fn main() {
    println!("Hello, world!");
}

pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (mat.len(), mat[0].len());
    let mut mat_i = mat.clone();
    let mut mat_j = mat.clone();

    for i in 1..m {
        for j in 0..n {
            mat_i[i][j] += mat_i[i - 1][j - 1];
        }
    }
    println!("mat_i={:?}", mat_i);
    for i in 0..m {
        for j in 1..n {
            mat_j[i][j] += mat_j[i - 1][j - 1];
        }
    }
    //check ones in last row
    let mut res = 0;
    let last_row = n - 1;
    for i in 0..n {
        for j in 0..m {
            if mat_j[last_row][i] == 1 {
                if mat_i[last_row][i] == 1 {
                    res += 1;
                }
            }
        }
    }
    println!("res={:?}", res);
    unimplemented!()
}
