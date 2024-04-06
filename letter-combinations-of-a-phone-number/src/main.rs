// use std::collections::HashMap;
//
// fn main() {
//     println!("Hello, world!");
//     letter_combinations("s".to_string());
// }
//
// pub fn letter_combinations(digits: String) -> Vec<String> {
//     let mut res: Vec<String> = Vec::new();
//     let mut digit_to_char = HashMap::from([
//         ("2", "abc"),
//         ("3", "def"),
//         ("4", "ghi"),
//         ("5", "jkl"),
//         ("6", "mno"),
//         ("7", "pqrs"),
//         ("8", "tuv"),
//         ("9", "wxyz"),
//     ]);
//
//     let digit_to_char = Vec::from([
//         vec!['a', 'b', 'c'],
//         vec!['d', 'e', 'f'],
//         vec!['g', 'h', 'i'],
//         vec!['j', 'k', 'l'],
//         vec!['m', 'n', 'o'],
//         vec!['p', 'q', 'r', 's'],
//         vec!['t', 'u', 'v'],
//         vec!['x', 'y', 'z'],
//     ]);
//
//     let digit_to_char = Vec::from([vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]);
//
//     // ["abc", "def"]
//
//     let mut rezulted_vec = Vec::new();
//     _letter_combinations(digit_to_char, 0, Vec::new(), &mut rezulted_vec);
//     println!("rezulted_vec={:?}", rezulted_vec);
//
//     todo!()
// }
//
// pub fn _letter_combinations(
//     digits: Vec<Vec<char>>,
//     idx: usize,
//     mut cur_string: Vec<char>,
//     rezulted_vec: &mut Vec<Vec<char>>,
// ) {
//     if idx == digits.len() {
//         // here will push smthn
//         rezulted_vec.push(cur_string);
//         return;
//     }
//
//     for word in digits.iter() {
//         for &letter in word {
//             // not same word
//             if !cur_string.contains(&letter) {
//                 cur_string.push(letter);
//                 _letter_combinations(digits.clone(), idx + 1, cur_string.clone(), rezulted_vec);
//                 cur_string.pop();
//             }
//         }
//     }
//
//     //     for i in 0..nums.len() {
//     //     if !v.contains(&nums[i]) {
//     //         v.push(nums[i]);
//     //         _permute(nums, vec, idx + 1, v.clone());
//     //         v.pop();
//     //     }
//     // }
// }

use std::collections::HashMap;

pub fn letter_combinations(digits: String) -> Vec<String> {
    let mut digit_to_char = HashMap::new();
    digit_to_char.insert('2', vec!['a', 'b', 'c']);
    digit_to_char.insert('3', vec!['d', 'e', 'f']);
    digit_to_char.insert('4', vec!['g', 'h', 'i']);
    digit_to_char.insert('5', vec!['j', 'k', 'l']);
    digit_to_char.insert('6', vec!['m', 'n', 'o']);
    digit_to_char.insert('7', vec!['p', 'q', 'r', 's']);
    digit_to_char.insert('8', vec!['t', 'u', 'v']);
    digit_to_char.insert('9', vec!['w', 'x', 'y', 'z']);

    let mut result = Vec::new();
    if !digits.is_empty() {
        _letter_combinations(&digit_to_char, &digits, 0, String::new(), &mut result);
    }
    result
}

fn _letter_combinations(
    digit_to_char: &HashMap<char, Vec<char>>,
    digits: &str,
    idx: usize,
    combination: String,
    result: &mut Vec<String>,
) {
    if idx == digits.len() {
        result.push(combination.clone());
        return;
    }

    let digit = digits.chars().nth(idx).unwrap();
    if let Some(letters) = digit_to_char.get(&digit) {
        for &letter in letters {
            let mut new_combination = combination.clone();
            new_combination.push(letter);
            _letter_combinations(digit_to_char, digits, idx + 1, new_combination, result);
        }
    }
}

fn main() {
    let digits = "23".to_string();
    let combinations = letter_combinations(digits);
    println!("{:?}", combinations); // Output: ["ad","ae","af","bd","be","bf","cd","ce","cf"]
}
