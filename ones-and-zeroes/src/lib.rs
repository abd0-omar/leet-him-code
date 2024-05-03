pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
    let strs: Vec<Vec<char>> = strs
        .iter()
        .map(|f| f.chars().collect::<Vec<char>>())
        .collect();
    let mut memory = vec![vec![vec![None; (m + 1) as usize]; (n + 1) as usize]; strs.len()];
    _find_max_form(&strs, m, n, 0, &mut memory)
}

pub fn _find_max_form(
    strs: &Vec<Vec<char>>,
    m: i32,
    n: i32,
    idx: usize,
    memory: &mut Vec<Vec<Vec<Option<i32>>>>,
    // hashmap version
    // memory: &mut std::collections::HashMap<(i32, i32, usize), i32>,
) -> i32 {
    if idx == strs.len() || (m <= 0 && n <= 0) {
        return 0;
    }

    if let Some(ret) = memory[idx][n as usize][m as usize] {
        return ret;
    }

    // leave
    let leave = _find_max_form(strs, m, n, idx + 1, memory);

    // pick
    let mut zeros_count = 0;
    let mut ones_count = 0;

    for number in &strs[idx] {
        if number == &'0' {
            zeros_count += 1;
        } else if number == &'1' {
            ones_count += 1;
        } else {
            unreachable!("only zeros and ones");
        }
    }

    let pick = if zeros_count <= m && ones_count <= n {
        _find_max_form(strs, m - zeros_count, n - ones_count, idx + 1, memory) + 1
    } else {
        0
    };

    let result = leave.max(pick);

    memory[idx][n as usize][m as usize] = Some(result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let strs = vec![
            "10".to_string(),
            "0001".to_string(),
            "111001".to_string(),
            "1".to_string(),
            "0".to_string(),
        ];
        let m = 5;
        let n = 3;
        let output = 4;
        let result = find_max_form(strs, m, n);
        assert_eq!(result, output);
    }
    #[test]
    fn it_works1() {
        let strs = vec!["10".to_string(), "0".to_string(), "1".to_string()];
        let m = 1;
        let n = 1;
        let output = 2;
        let result = find_max_form(strs, m, n);
        assert_eq!(result, output);
    }
}
