pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut freq = vec![vec![0; 26]; words.len()];
    //  a  b  c    a   b  c   a  b
    //[[1, 1, 1], [1, 1, 1], [1, 1, 0]]
    for (i, word) in words.iter().enumerate() {
        for letter in word.chars() {
            let letter_idx = (letter as u8 - b'a') as usize;
            freq[i][letter_idx] += 1;
        }
    }

    let mut common_chars = Vec::new();

    for idx in 0..26 {
        let mut min_repetation = i32::MAX;
        for i in 0..freq.len() {
            let cur = freq[i][idx];
            if cur < 0 {
                break;
            }
            min_repetation = min_repetation.min(cur);
        }

        if min_repetation != i32::MAX {
            let letter = (char::from(idx as u8 + b'a')).to_string();
            for _ in 0..min_repetation {
                common_chars.push(letter.clone());
            }
        }
    }

    common_chars
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let words = vec![
            "bella".to_string(),
            "label".to_string(),
            "roller".to_string(),
        ];
        let output = vec!["e".to_string(), "l".to_string(), "l".to_string()];
        let result = common_chars(words);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let words = vec!["cool".to_string(), "lock".to_string(), "cook".to_string()];
        let output = vec!["c".to_string(), "o".to_string()];
        let result = common_chars(words);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let words = vec![
            "acabcddd".to_string(),
            "bcbdbcbd".to_string(),
            "baddbadb".to_string(),
            "cbdddcac".to_string(),
            "aacbcccd".to_string(),
            "ccccddda".to_string(),
            "cababaab".to_string(),
            "addcaccd".to_string(),
        ];
        let output: Vec<String> = vec![];
        let result = common_chars(words);
        assert_eq!(result, output);
    }
}
