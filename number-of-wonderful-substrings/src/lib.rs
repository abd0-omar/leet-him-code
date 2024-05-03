// tle needs to be bitmasked
pub fn wonderful_substrings(word: String) -> i64 {
    let word = word.into_bytes();
    let len = (b'j' - b'a') as usize + 1;
    let mut freq = vec![0; len];
    let mut prefix_freq = vec![vec![0; len]; word.len() + 1];
    // for &letter in word.iter() {
    //     freq[(letter as u8 - b'a') as usize] += 1;
    // }
    for i in 1..=word.len() {
        let letter = word[i - 1];
        let letter_idx = (letter as u8 - b'a') as usize;
        freq[letter_idx] += 1;
        prefix_freq[i] = freq.clone();
    }
    // println!("prefix_freq={:?}", prefix_freq);
    // println!("freq={:?}", freq);
    let mut result = 0;
    for i in 1..=word.len() {
        for j in 0..i {
            // calc curr freq
            // let mut freq_i = &mut prefix_freq[i];
            // let freq_j = &prefix_freq[j];
            for k in 0..len {
                prefix_freq[i][k] = prefix_freq[i][k] - prefix_freq[j][k];
            }
            // check how many odds
            // 1 odd good
            // 0 odd good
            // println!("i={}", i);
            // println!("j={}", j);
            // println!("freq_i={:?}", prefix_freq[i]);
            let mut count_odd = 0;
            for k in 0..len {
                if prefix_freq[i][k] % 2 == 1 {
                    count_odd += 1;
                    if count_odd == 2 {
                        break;
                    }
                }
            }
            if count_odd < 2 {
                result += 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "aba".to_string();
        let output = 4;
        let result = wonderful_substrings(word);
        assert_eq!(result, output);
        // Explanation: The four wonderful substrings are underlined below:
        // - "aba" -> "a"
        // - "aba" -> "b"
        // - "aba" -> "a"
        // - "aba" -> "aba"
    }
}
