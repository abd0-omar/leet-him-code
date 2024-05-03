pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    let modo = 1_000_000_007;
    // let mut memory = vec![vec![None; target as usize + 1]; n as usize + 1];
    let mut memory = std::collections::HashMap::new();
    _num_rolls_to_target(n, k, target, &mut memory, &modo)
}

pub fn _num_rolls_to_target(
    n: i32,
    k: i32,
    target: i32,
    // memory: &mut Vec<Vec<Option<i32>>>,
    memory: &mut std::collections::HashMap<(i32, i32), i32>,
    modo: &i32,
) -> i32 {
    if n == 0 {
        if target == 0 {
            return 1;
        }
        return 0;
    }

    // if let Some(ret) = memory[n as usize][target as usize] {
    //     return ret;
    // }

    if let Some(ret) = memory.get(&(n, target)) {
        return *ret;
    }

    let mut result = 0;
    for j in 1..=k {
        result += _num_rolls_to_target(n - 1, k, target - j, memory, modo);
        result %= modo;
    }

    // memory[n as usize][target as usize] = Some(result);
    memory.insert((n, target), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let n = 1;
        let k = 6;
        let target = 3;
        let output = 1;
        let result = num_rolls_to_target(n, k, target);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 2;
        let k = 6;
        let target = 7;
        let output = 6;
        let result = num_rolls_to_target(n, k, target);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let n = 30;
        let k = 30;
        let target = 500;
        let output = 222616187;
        let result = num_rolls_to_target(n, k, target);
        assert_eq!(result, output);
    }
}
