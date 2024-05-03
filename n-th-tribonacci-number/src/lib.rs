pub fn tribonacci(n: i32) -> i32 {
    let mut memory = vec![None; (n + 1) as usize];
    _tribonacci(n, &mut memory)
}

pub fn _tribonacci(n: i32, memory: &mut Vec<Option<i32>>) -> i32 {
    if n == 0 {
        return 0;
    }

    if n == 1 || n == 2 {
        return 1;
    }
    if let Some(ret) = memory[n as usize] {
        return ret;
    }

    let result =
        _tribonacci(n - 1, memory) + _tribonacci(n - 2, memory) + _tribonacci(n - 3, memory);

    memory[n as usize] = Some(result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //         T_3 = 0 + 1 + 1 = 2
        // T_4 = 1 + 1 + 2 = 4
        let result = tribonacci(4);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works1() {
        let result = tribonacci(25);
        assert_eq!(result, 1389537);
    }

    #[test]
    fn it_works2() {
        let result = tribonacci(35);
        assert_eq!(result, 615693474);
    }
}
