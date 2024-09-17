// improved solution
pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
    use std::collections::HashMap;

    let mut count_words: HashMap<&str, u32> = HashMap::new();

    for word in s1.split_whitespace().chain(s2.split_whitespace()) {
        *count_words.entry(word).or_insert(0) += 1
    }

    count_words
        .into_iter()
        .filter(|(_, count)| *count == 1)
        .map(|(word, _)| word.to_string())
        .collect()
}

// first ugly solution
// pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
//     let mut first_vec = _uncommon_from_sentences(&s1, &s2);
//     let second_vec = _uncommon_from_sentences(&s2, &s1);
//
//     first_vec.extend_from_slice(&second_vec);
//     first_vec
// }
//
// pub fn _uncommon_from_sentences(s1: &str, s2: &str) -> Vec<String> {
//     let common_words: HashSet<&str> = HashSet::from_iter(s1.split_whitespace());
//
//     let duplicated_words = duplicated_words(s2);
//
//     // duplicates within string edge case
//
//     s2.split_whitespace()
//         .filter(|&word| !common_words.contains(word) && !duplicated_words.contains(word))
//         .map(|word| word.to_string())
//         .collect()
// }
//
// fn duplicated_words(s2: &str) -> HashSet<&str> {
//     let mut common_words_within_string: HashSet<&str> = HashSet::new();
//     let mut duplicated_words = HashSet::new();
//     for word in s2.split_whitespace() {
//         if !common_words_within_string.insert(word) {
//             duplicated_words.insert(word);
//         }
//     }
//     duplicated_words
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s1 = "this apple is sweet".to_string();
        let s2 = "this apple is sour".to_string();
        let output = vec!["sweet".to_string(), "sour".to_string()];
        let result = uncommon_from_sentences(s1, s2);
        assert_eq!(result, output);
    }
}
