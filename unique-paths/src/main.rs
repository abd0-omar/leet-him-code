fn main() {
    println!("Hello, world!");
    let m = 3;
    let n = 7;
    // Output: 28
    println!("{}", unique_paths(m, n));
    let m = 3;
    let n = 2;
    // Output: 3
    println!("{}", unique_paths(m, n));
}

pub fn unique_paths(m: i32, n: i32) -> i32 {
    let mut memory = vec![vec![None; (n + 1) as usize]; (m + 1) as usize];
    _unique_paths(m, n, 0, 0, &mut memory)
}

pub fn _unique_paths(m: i32, n: i32, i: i32, j: i32, memory: &mut Vec<Vec<Option<i32>>>) -> i32 {
    if i == m - 1 && j == n - 1 {
        return 1;
    }

    if i > m || j > n {
        return 0;
    }

    if let Some(ret) = memory[i as usize][j as usize] {
        return ret;
    }

    let choice1 = _unique_paths(m, n, i + 1, j, memory);

    let choice2 = _unique_paths(m, n, i, j + 1, memory);

    memory[i as usize][j as usize] = Some(choice1 + choice2);
    memory[i as usize][j as usize].unwrap()
}
