pub fn make_fancy_string(s: String) -> String {
    let n = s.len();
    let s = s.as_bytes();
    let mut result = String::with_capacity(n);
    for i in 0..n {
        if i >= 2 {
            if s[i] == s[i - 1] && s[i - 1] == s[i - 2] {
                continue;
            }
        }
        result.push(char::from(s[i]));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "aaabaaaa".to_string();
        let output = "aabaa".to_string();
        let result = make_fancy_string(s);
        assert_eq!(result, output);
    }
}
