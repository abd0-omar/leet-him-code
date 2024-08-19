pub fn min_steps(n: i32) -> i32 {
    //  copy                                        paste
    //                                A
    //                    A                        AA
    //              A         AA            AA           AAA
    //           A   AA    AA   AAA     AA   AAAA     AAA      AAAA
    // to avoid infinite recursion, we'll copy and paste at the same time

    if n == 1 {
        return 0;
    }

    let mut memory = vec![vec![-1; 1001]; 1001];
    _min_steps(n, 1, 1, &mut memory) + 1
}

fn _min_steps(n: i32, count: i32, paste: i32, memory: &mut Vec<Vec<i32>>) -> i32 {
    if count == n {
        return 0;
    }
    if count > n {
        return 1000;
    }

    if memory[count as usize][paste as usize] != -1 {
        return memory[count as usize][paste as usize];
    }

    let paste_res = _min_steps(n, count + paste, paste, memory) + 1;
    let copy_and_paste_res = _min_steps(n, count * 2, count, memory) + 2;

    let result = paste_res.min(copy_and_paste_res);
    memory[count as usize][paste as usize] = result;

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 3;
        let output = 3;
        let result = min_steps(n);
        assert_eq!(result, output);
    }
}
