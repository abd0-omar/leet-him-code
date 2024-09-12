pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
    // allowed input is too small, so a hashset would be an overkill, would use a freq array
    // instead
    let mut freq = [false; 26];
    for letter in allowed.chars() {
        let letter_idx = (letter as u8 - b'a') as usize;
        freq[letter_idx] = true;
    }

    let mut count = 0;

    'outer: for word in words {
        for letter in word.chars() {
            let letter_idx = (letter as u8 - b'a') as usize;
            if !freq[letter_idx] {
                continue 'outer;
            }
        }
        count += 1;
    }

    count
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let allowed = "ab".to_string();
        let words = vec![
            "ad".to_string(),
            "bd".to_string(),
            "aaab".to_string(),
            "baa".to_string(),
            "badab".to_string(),
        ];
        let output = 2;
        let result = count_consistent_strings(allowed, words);
        assert_eq!(result, output);
    }
}
