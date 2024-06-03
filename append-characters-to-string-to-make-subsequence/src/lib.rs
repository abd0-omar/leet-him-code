pub fn append_characters(s: String, t: String) -> i32 {
    let s = s.as_bytes();
    let t = t.as_bytes();
    let mut i = 0;
    let mut j = 0;
    while i < t.len() && j < s.len() {
        if t[i] == s[j] {
            i += 1;
        }
        j += 1;
    }
    (t.len() - i) as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = "coaching".to_string();
        let t = "coding".to_string();
        let output = 4;
        let result = append_characters(s, t);
        assert_eq!(result, output);
    }
    #[test]
    fn it_works1() {
        let s = "abcde".to_string();
        let t = "a".to_string();
        let output = 0;
        let result = append_characters(s, t);
        assert_eq!(result, output);
    }
    #[test]
    fn it_works2() {
        let s = "z".to_string();
        let t = "abcde".to_string();
        let output = 5;
        let result = append_characters(s, t);
        assert_eq!(result, output);
    }
}
