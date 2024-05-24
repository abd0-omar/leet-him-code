use std::collections::HashMap;

pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
    fn calculate_word_score(word: &str, score: &[i32]) -> i32 {
        word.chars().map(|c| score[(c as u8 - b'a') as usize]).sum()
    }

    fn generate_subsets(
        words: &[Vec<char>],
        index: usize,
        current_set: &mut Vec<Vec<char>>,
        result: &mut Vec<Vec<Vec<char>>>,
    ) {
        if index == words.len() {
            result.push(current_set.clone());
            return;
        }

        current_set.push(words[index].clone());
        generate_subsets(words, index + 1, current_set, result);

        current_set.pop();
        generate_subsets(words, index + 1, current_set, result);
    }

    let words: Vec<Vec<char>> = words
        .into_iter()
        .map(|word| word.chars().collect())
        .collect();

    let mut subsets = Vec::new();
    generate_subsets(&words, 0, &mut Vec::new(), &mut subsets);

    let mut letter_freq = HashMap::new();
    for &letter in &letters {
        *letter_freq.entry(letter).or_insert(0) += 1;
    }

    fn can_form_words(words: &[Vec<char>], letter_freq: &HashMap<char, i32>) -> bool {
        let mut freq = letter_freq.clone();
        for word in words {
            for &letter in word {
                if let Some(count) = freq.get_mut(&letter) {
                    if *count == 0 {
                        return false;
                    }
                    *count -= 1;
                } else {
                    return false;
                }
            }
        }
        true
    }

    let mut max_score = 0;
    for subset in subsets {
        if can_form_words(&subset, &letter_freq) {
            let subset_score: i32 = subset
                .iter()
                .map(|word| calculate_word_score(&word.iter().collect::<String>(), &score))
                .sum();
            max_score = max_score.max(subset_score);
        }
    }

    max_score
}

// pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
//     // get letters to form a word using back track
//     // base case: if the formed word is in vec words then calc it's core
//     let mut letters_freq = std::collections::HashMap::new();
//     for &letter in letters.iter() {
//         *letters_freq.entry(letter).or_insert(0) += 1;
//     }
//     let mut letters: Vec<Option<char>> = letters.into_iter().map(Some).collect();
//     let mut words: Vec<Vec<char>> = words
//         .iter()
//         .map(|f| f.chars().collect::<Vec<char>>())
//         .collect();
//     words.sort_unstable();
//     let mut words_set = std::collections::HashSet::new();
//     for mut word in words.clone() {
//         word.sort_unstable();
//         words_set.insert(word);
//     }
//     println!("words_set={:?}", words_set);
//     println!("letters={:?}", letters);
//     let mut max_score = 0;
//     let mut total_vec = Vec::new();
//
//     _max_score_words(
//         &words_set,
//         &mut letters,
//         // &mut letters_freq,
//         score.clone(),
//         vec![],
//         0,
//         &mut max_score,
//         &mut total_vec,
//     );
//     println!("total_vec_that_came={:?}", total_vec);
//     println!("letters={:?}", letters);
//     println!("happ new year");
//     println!("happ new year");
//     println!("happ new year");
//     println!("happ new year");
//     println!("happ new year");
//     println!("happ new year");
//     println!("letters_freq={:?}", letters_freq);
//     max_score = 0;
//     if total_vec.is_empty() {
//         total_vec = words;
//     }
//     _subsets(
//         &total_vec,
//         &mut letters_freq,
//         0,
//         vec![vec![]],
//         &mut max_score,
//         &score,
//     );
//     max_score
// }
//
// pub fn _subsets(
//     total_vec: &Vec<Vec<char>>,
//     letters_map: &mut std::collections::HashMap<char, i32>,
//     idx: usize,
//     mut cur_vec: Vec<Vec<char>>,
//     max_score: &mut i32,
//     score: &Vec<i32>,
// ) {
//     if idx == total_vec.len() {
//         // if not in the big vec
//         let temp_letters = letters_map.clone();
//         let mut cur_score = 0;
//         println!("cur_vec_up={:?}", cur_vec);
//         for word in cur_vec.iter() {
//             for &letter in word {
//                 if let Some(freq) = letters_map.get_mut(&letter) {
//                     if *freq >= 1 {
//                         *freq -= 1;
//                     } else {
//                         println!("cur_vec_in_loop={:?}", cur_vec);
//                         println!("letters_map={:?}", letters_map);
//                         *letters_map = temp_letters.clone();
//                         println!("happened");
//                         return;
//                     }
//                 }
//                 let letter_idx = (letter as u8) - 'a' as u8;
//                 cur_score += score[letter_idx as usize];
//             }
//         }
//         println!("cur_vec_down={:?}", cur_vec);
//         println!("cur_score={:?}", cur_score);
//         *max_score = (*max_score).max(cur_score);
//         *letters_map = temp_letters.clone();
//         return;
//     }
//     let leave = _subsets(
//         total_vec,
//         letters_map,
//         idx + 1,
//         cur_vec.clone(),
//         max_score,
//         score,
//     );
//
//     cur_vec.push(total_vec[idx].clone());
//     let pick = _subsets(
//         total_vec,
//         letters_map,
//         idx + 1,
//         cur_vec.clone(),
//         max_score,
//         score,
//     );
//     cur_vec.pop();
// }
//
// pub fn _max_score_words(
//     words_set: &std::collections::HashSet<Vec<char>>,
//     letters: &mut Vec<Option<char>>,
//     // letters_freq: &mut std::collections::HashMap<char, i32>,
//     score: Vec<i32>,
//     mut cur_word: Vec<char>,
//     st: usize,
//     max_score: &mut i32,
//     total_vec: &mut Vec<Vec<char>>,
// ) -> () {
//     if words_set.contains(&cur_word) {
//         // calc it's score
//         let mut cur_score = 0;
//         for &letter in cur_word.iter() {
//             println!("cur_word={:?}", cur_word);
//             println!("letters={:?}", letters);
//             let letter_idx = (letter as u8) - 'a' as u8;
//             cur_score += score[letter_idx as usize];
//         }
//         println!("cur_score={:?}", cur_score);
//         if !total_vec.contains(&cur_word) {
//             total_vec.push(cur_word.clone());
//             *max_score += cur_score;
//             // return;
//         }
//         // return;
//         // *max_score = (*max_score).max(cur_score);
//     }
//
//     for end in st..letters.len() {
//         // let letters_place_holder = letters[end];
//         // if let Some(l) = letters[end] {
//         // letters[end] = None;
//         cur_word.push(letters[end].unwrap());
//         _max_score_words(
//             words_set,
//             letters,
//             // letters_freq,
//             score.clone(),
//             cur_word.clone(),
//             end + 1,
//             max_score,
//             total_vec,
//         );
//         cur_word.pop();
//         // letters[end] = letters_place_holder;
//         // }
//     }
// }
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        // Example 1:

        let words = vec![
            "dog".to_string(),
            "cat".to_string(),
            "dad".to_string(),
            "good".to_string(),
        ];
        // only can be used once
        let letters = vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'];
        // from 0 to 25
        let score = vec![
            1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let output = 23;
        let result = max_score_words(words, letters, score);
        assert_eq!(result, output);
        // Explanation:
        // Score  a=1, c=9, d=5, g=3, o=2
        // Given letters, we can form the words "dad" (5+1+5) and "good" (3+2+2+5) with a score of 23.
        // Words "dad" and "dog" only get a score of 21.
    }

    #[test]
    fn it_works1() {
        let words = vec!["leetcode".to_string()];
        let letters = vec!['l', 'e', 't', 'c', 'o', 'd'];
        let score = vec![
            0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        ];
        let output = 0;
        let result = max_score_words(words, letters, score);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let words = vec![
            "xxxz".to_string(),
            "ax".to_string(),
            "bx".to_string(),
            "cx".to_string(),
        ];
        let letters = vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'];
        let score = vec![
            4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10,
        ];
        let output = 27;
        let result = max_score_words(words, letters, score);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let words = vec!["get".to_string(), "set".to_string()];
        let letters = vec!['g', 's', 'e', 't'];
        let score = vec![
            0, 0, 0, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 1, 0, 0, 0, 0, 0, 0,
        ];
        let output = 5;
        let result = max_score_words(words, letters, score);
        assert_eq!(result, output);
    }
}
