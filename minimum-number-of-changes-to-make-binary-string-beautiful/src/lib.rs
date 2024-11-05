pub fn min_changes(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    let mut cur_letter = s[0];
    let mut freq = 0;
    let mut result = 0;
    for i in 0..n {
        if s[i] == cur_letter {
            freq += 1;
            continue;
        }

        if freq % 2 == 0 {
            freq = 1;
        } else {
            result += 1;
            // start over again
            freq = 0;
        }

        cur_letter = s[i];
    }
    result
    // or iterate by 2s and if prev not eq current then increment result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = "1001".to_string();
        let output = 2;
        let result = min_changes(s);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "11000111".to_string();
        let output = 1;
        let result = min_changes(s);
        assert_eq!(result, output);
    }
}
