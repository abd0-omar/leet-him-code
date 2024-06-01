pub fn score_of_string(s: String) -> i32 {
    let mut score = 0;
    let s = s.as_bytes();
    for i in 1..s.len() {
        score += s[i].abs_diff(s[i - 1]) as i32;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "hello".to_string();
        let output = 13;
        let result = score_of_string(s);
        assert_eq!(result, output);
    }
}
