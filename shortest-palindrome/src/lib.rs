pub fn shortest_palindrome(s: String) -> String {
    let n = s.len();
    for i in 0..n {
        if is_palindrome(&s[..n - i].as_bytes()) {
            let end_part = &s[n - i..];
            let end_part_rev: String = end_part.chars().rev().collect();
            let result = format!("{}{}", end_part_rev, s);
            return result;
        }
    }
    s
}

fn is_palindrome(s: &[u8]) -> bool {
    let mut st = 0;
    let mut end = s.len() - 1;
    while st < end {
        if s[st] != s[end] {
            return false;
        }
        st += 1;
        end -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "aacecaaa".to_string();
        let output = "aaacecaaa".to_string();
        let result = shortest_palindrome(s);
        assert_eq!(result, output);
    }
}
