pub fn minimum_pushes(word: String) -> i32 {
    // highest freq must be mapped with one push
    // pq of letter and freq
    /*
    h | 2
    i | 6
    */
    use std::collections::{BinaryHeap, HashMap};
    let mut pq = BinaryHeap::new();
    // get normal freq
    let mut hm = HashMap::new();
    for letter in word.chars() {
        hm.entry(letter)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    for (key, val) in hm.into_iter() {
        pq.push((val, key));
    }
    // 2 -> 9 inclusive
    let mut pushes = 1;
    let mut count = 2;
    let mut result = 0;
    while let Some((freq, letter)) = pq.pop() {
        dbg!(letter);
        dbg!(freq);
        dbg!(pushes);
        dbg!("----");
        // let freq = freq.0;
        result += freq * pushes;
        if count == 9 {
            pushes += 1;
            count = 1;
        }
        count += 1;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let word = "abcde".to_string();
        let output = 5;
        let result = minimum_pushes(word);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let word = "aabbccddeeffgghhiiiiii".to_string();
        let output = 24;
        let result = minimum_pushes(word);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let word = "abzaqsqcyrbzsrvamylmyxdjl".to_string();
        let output = 32;
        let result = minimum_pushes(word);
        assert_eq!(result, output);
    }
}
