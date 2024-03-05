fn main() {
    println!("Hello, world!");
}

pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    // calculate the freq of chars and freq of words
    use std::collections::HashMap;

    let mut hm_chars = HashMap::new();
    for char in chars.chars() {
        *hm_chars.entry(char).or_insert(0) += 1;
    }
    println!("hm_chars={:#?}", hm_chars);

    let mut len_sum = 0;
    // calc freq of words
    for word in words {
        let mut hm_words = HashMap::new();
        for char in word.chars() {
            *hm_words.entry(char).or_insert(0) += 1;
        }
        println!("hm_words={:#?}", hm_words);
        let mut flag = true;
        for (key, val) in hm_words {
            if let Some(&k) = hm_chars.get(&key) {
                if val > k {
                    flag = false;
                    break;
                }
            } else {
                flag = false;
                break;
            }
        }
        if flag {
            len_sum += word.len()
        }
    }

    len_sum as _
}
