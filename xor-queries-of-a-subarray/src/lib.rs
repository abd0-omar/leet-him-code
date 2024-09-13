pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
    // the queries input is too long so we would have to do prefix sum
    let n = arr.len();
    let mut prefix_xor = vec![0; n + 1];
    for i in 1..n + 1 {
        prefix_xor[i] = prefix_xor[i - 1] ^ arr[i - 1];
    }

    // 1, 2, 3
    // q: 0, 1 -> 3
    // q: 1, 2 -> 5
    // l
    // r + 1
    // 0, 1, 3, 6
    // 0, 1, 2, 3

    let mut result = Vec::with_capacity(n);

    for q in queries {
        let from = q[0] as usize;
        let to = q[1] as usize + 1;
        let quer_answer = prefix_xor[to] ^ prefix_xor[from];
        dbg!(from);
        dbg!(to);
        dbg!(quer_answer);
        result.push(quer_answer);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr = vec![1, 3, 4, 8];
        let queries = vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]];
        let output = vec![2, 7, 14, 8];
        let result = xor_queries(arr, queries);
        assert_eq!(result, output);
    }
}
