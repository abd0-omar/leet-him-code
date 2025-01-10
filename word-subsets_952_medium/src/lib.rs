// https://leetcode.com/problems/word-subsets/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    // pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
    //     // brute-forc-y solution
    //     // just compare by the freq
    //
    //     // converting the strings to bytes cuz I don't want to اوجع دماغي
    //     let words1 = words1
    //         .iter()
    //         .map(|word| word.chars().map(|letter| letter as u8).collect::<Vec<u8>>())
    //         .collect::<Vec<Vec<u8>>>();
    //
    //     let words2 = words2
    //         .iter()
    //         .map(|word| word.chars().map(|letter| letter as u8).collect::<Vec<u8>>())
    //         .collect::<Vec<Vec<u8>>>();
    //
    //     // freq for both
    //     let (mut freq1, mut freq2) = (
    //         // word len is too low
    //         vec![vec![0u8; 26]; words1.len()],
    //         vec![vec![0u8; 26]; words2.len()],
    //     );
    //
    //     for (i, word) in words1.iter().enumerate() {
    //         for letter in word.iter() {
    //             let letter_idx = (letter - b'a') as usize;
    //             freq1[i][letter_idx] += 1;
    //         }
    //     }
    //
    //     for (i, word) in words2.iter().enumerate() {
    //         for letter in word.iter() {
    //             let letter_idx = (letter - b'a') as usize;
    //             freq2[i][letter_idx] += 1;
    //         }
    //     }
    //
    //     // dbg!(&freq1);
    //     // dbg!(&freq2);
    //
    //     let mut result = std::collections::HashSet::new();
    //
    //     'outer: for (i, word1) in words1.iter().enumerate() {
    //         // if word1.as_slice() == "apple".as_bytes() {
    //         //     dbg!("apple");
    //         // } else {
    //         //     dbg!("not apple");
    //         // }
    //         for (j, word2) in words2.iter().enumerate() {
    //             for letter in word2 {
    //                 let letter_idx = (letter - b'a') as usize;
    //                 dbg!(&freq2[j][letter_idx]);
    //                 dbg!(&freq1[i][letter_idx]);
    //                 if freq2[j][letter_idx] > freq1[i][letter_idx] {
    //                     continue 'outer;
    //                 }
    //             }
    //         }
    //         result.insert(word1);
    //     }
    //
    //     // next time do vec<vec<char>>, cuz it's easier to convert back to Vec<String>
    //     result
    //         .iter()
    //         .map(|vec| {
    //             vec.iter()
    //                 .map(|&byte| char::from_u32(byte as u32).unwrap())
    //                 .collect()
    //         })
    //         .collect()
    // }
    // TLE!
    // the optimized trick is to merge the freq of `words2` to show the max freq for each
    // letter across `words2`
    // eg.: ['eo', 'ee']
    // instead of
    // e -> 1
    // o -> 1
    //
    // e -> 2
    // it'll be
    // e -> 2
    // o -> 1
    // max_freq[letter_idx"e"] = 2
    // we will have on freq vec instead of vec![vec![0; 26]; words.len()]

    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        // Optimized solution: Merge frequency requirements of words2
        // Calculate the max frequency of each letter across all words in words2

        // converting the strings to bytes cuz I don't want to اوجع دماغي
        let words1 = words1
            .iter()
            .map(|word| word.chars().map(|letter| letter as u8).collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>();

        let words2 = words2
            .iter()
            .map(|word| word.chars().map(|letter| letter as u8).collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>();

        // Calculate the max frequency for each letter in words2
        let mut max_freq = vec![0u8; 26];
        for word in &words2 {
            let mut freq = vec![0u8; 26];
            for &letter in word {
                let letter_idx = (letter - b'a') as usize;
                freq[letter_idx] += 1;
            }
            for i in 0..26 {
                max_freq[i] = max_freq[i].max(freq[i]);
            }
        }

        // dbg!(&max_freq);

        let mut result = Vec::new();

        'outer: for word1 in &words1 {
            let mut freq1 = vec![0u8; 26];
            for &letter in word1 {
                let letter_idx = (letter - b'a') as usize;
                freq1[letter_idx] += 1;
            }

            // the better way to do it is the uncommented below code, looping over 26 letter
            // for word2 in &words2 {
            //     for &letter in word2 {
            //         let letter_idx = (letter - b'a') as usize;
            //         if max_freq[letter_idx] > freq1[letter_idx] {
            //             continue 'outer;
            //         }
            //     }
            // }

            for i in 0..26 {
                if freq1[i] < max_freq[i] {
                    continue 'outer;
                }
            }

            result.push(word1);
        }

        // next time do vec<vec<char>>, cuz it's easier to convert back to Vec<String>
        result
            .iter()
            .map(|vec| {
                vec.iter()
                    .map(|&byte| char::from_u32(byte as u32).unwrap())
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let words1 = vec![
            "amazon".to_string(),
            "apple".to_string(),
            "facebook".to_string(),
            "google".to_string(),
            "leetcode".to_string(),
        ];
        let words2 = vec!["e".to_string(), "o".to_string()];
        let output = vec![
            "facebook".to_string(),
            "google".to_string(),
            "leetcode".to_string(),
        ];
        let result = Solution::word_subsets(words1, words2);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let words1 = vec![
            "amazon".to_string(),
            "apple".to_string(),
            "facebook".to_string(),
            "google".to_string(),
            "leetcode".to_string(),
        ];
        let words2 = vec!["l".to_string(), "e".to_string()];
        let output = vec![
            "apple".to_string(),
            "google".to_string(),
            "leetcode".to_string(),
        ];
        let result = Solution::word_subsets(words1, words2);
        assert_eq!(result, output);
    }
}
