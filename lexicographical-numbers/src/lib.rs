pub fn lexical_order(n: i32) -> Vec<i32> {
    // dfs
    // trie
    let mut result = Vec::with_capacity(n as usize);
    for i in 1..10 {
        dfs(i, n, &mut result)
    }
    result
}

fn dfs(og_i: i32, n: i32, result: &mut Vec<i32>) {
    if og_i > n {
        return;
    }

    result.push(og_i);

    for i in 0..10 {
        let new_number = og_i * 10 + i;
        if new_number <= n {
            dfs(new_number, n, result)
        } else {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 13;
        let output = vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9];
        let result = lexical_order(n);
        assert_eq!(result, output);
    }
}
