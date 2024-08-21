use std::collections::HashMap;

pub fn strange_printer(s: String) -> i32 {
    /*
        aba
                        aba
                  b
             0

        aaaaabbbbb
                           aaaaaabbbbbb
                bbbbbb                      aaaaaaa
           0                              0
    */
    // that's a dp range problem
    // yeah like the matrix multiplication
    let mut memo = HashMap::new();
    _strange_printer(s.as_bytes(), 0, s.len() as i32 - 1, &mut memo)
}

fn _strange_printer(s: &[u8], st: usize, end: i32, memo: &mut HashMap<(usize, i32), i32>) -> i32 {
    if st as i32 > end {
        return 0;
    }

    if let Some(&result) = memo.get(&(st, end)) {
        return result;
    }

    let mut res = _strange_printer(s, st, end - 1, memo) + 1;

    for i in st..end as usize {
        if s[i] == s[end as usize] {
            res = res.min(
                _strange_printer(s, st, i as i32, memo) + _strange_printer(s, i + 1, end - 1, memo),
            );
        }
    }

    memo.insert((st, end), res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "aba".to_string();
        let output = 2;
        let result = strange_printer(s);
        assert_eq!(result, output);
    }
}
