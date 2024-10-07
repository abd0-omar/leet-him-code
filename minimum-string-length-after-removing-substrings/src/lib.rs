pub fn min_length(s: String) -> i32 {
    // first thought is to use stack
    // like the parenthesis stack problem
    let mut stack = Vec::new();

    for letter in s.chars() {
        if let Some(&peek) = stack.last() {
            if peek == letter_map(letter).unwrap_or('🧊') {
                stack.pop();
                continue;
            }
        }
        stack.push(letter);
    }

    stack.len() as i32
}

// instead of using a hashmap
fn letter_map(letter: char) -> Option<char> {
    match letter {
        'D' => Some('C'),
        'B' => Some('A'),
        _ => None,
    }
}

// // other way to do it
// if !stack.is_empty() {
//     if *stack.last().unwrap() == letter_map(letter).unwrap_or('a') {
//         stack.pop();
//         count += 2;
//         continue;
//     }
//     // if let Some(cur_l) = letter_map(letter) {
//     //     if *stack.last().unwrap() == cur_l {
//     //         stack.pop();
//     //         count += 2;
//     //         continue;
//     //     }
//     // }
// }

// fn letter_key(letter: &char) -> bool {
//     *letter == 'A' || *letter == 'C'
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "ABFCACDB".to_string();
        let output = 2;
        let result = min_length(s);
        assert_eq!(result, output);
    }
}
