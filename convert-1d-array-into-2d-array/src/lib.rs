pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
    let len = original.len();
    let (m, n) = (m as usize, n as usize);
    if len != m * n {
        return Vec::new();
    }

    let mut result = vec![vec![0; n]; m];
    for i in 0..len {
        let ni = i / n as usize;
        let nj = i % n as usize;

        result[ni][nj] = original[i];
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let original = vec![1, 2, 3, 4];
        let m = 2;
        let n = 2;
        let output = vec![vec![1, 2], vec![3, 4]];
        let result = construct2_d_array(original, m, n);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works0() {
        let original = vec![1, 2, 3];
        let m = 1;
        let n = 3;
        let output = vec![vec![1, 2, 3]];
        let result = construct2_d_array(original, m, n);
        assert_eq!(result, output);
    }
}
