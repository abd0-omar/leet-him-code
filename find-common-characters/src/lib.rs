pub fn common_chars(words: Vec<String>) -> Vec<String> {
    let mut freq = vec![vec![0; 26]; words.len()];
    for (i, word) in words.iter().enumerate() {
        // println!("freq={:?}", freq);
        for letter in word.chars() {
            let letter_idx = (letter as u8 - b'a') as usize;
            freq[i][letter_idx] += 1;
        }
    }
    println!("freq={:?}", freq);
    let mut common_chars = Vec::new();

    let mut idx = 0;
    while idx < 26 {
        // let mut prev = None;
        let mut all_same = true;
        let mut min_repetation = i32::MAX;
        for i in 0..freq.len() {
            // print!("{}, ", freq[i][idx]);
            let cur = freq[i][idx];
            min_repetation = min_repetation.min(cur);
            if cur < 0 {
                all_same = false;
                break;
            }
            // match prev {
            //     Some(p) => {
            //         if {
            //             all_same = false;
            //             break;
            //         }
            //         prev = Some(cur);
            //     }
            //     None => {
            //         if cur == 0 {
            //             all_same = false;
            //             break;
            //         }
            //         prev = Some(cur)
            //     }
            // };
        }
        // println!();
        if all_same {
            let letter = (char::from(idx as u8 + b'a')).to_string();
            for _ in 0..min_repetation {
                common_chars.push(letter.clone());
            }
        }
        idx += 1
    }

    // for f in 0..freq.len() {
    //     let mut are_equal = true;
    //     let mut prev = None;
    //     let mut idx = 0;
    //     // [[1, 1], [1, 1], [1, 0]]
    //     for i in 0..freq[f].len() {
    //         idx = i;
    //         match prev {
    //             Some(p) => {
    //                 if p != k {
    //                     are_equal = false;
    //                     break;
    //                 }
    //                 prev = Some(k);
    //             }
    //             None => prev = Some(k),
    //         }
    //     }
    //     if are_equal {
    //         let letter = (idx as u8 + b'a').to_string();
    //         common_chars.push(letter);
    //     }
    // }
    //
    // for (key, val) in freq {
    //     // if the val is greater than or equal words.len(), then it's common
    //     let letter_count = val / words.len();
    //     println!("letter_count={:?}", letter_count);
    //     println!("key={:?}", key);
    //     println!("val={:?}", val);
    //     println!("words={:?}", words.len());
    //     // letter_count=1
    //     // key='a'
    //     // val=14
    //     // words=8
    //     for _ in 0..letter_count {
    //         common_chars.push(key.to_string());
    //     }
    // }
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
