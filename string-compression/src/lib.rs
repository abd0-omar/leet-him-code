pub fn compress(chars: &mut Vec<char>) -> i32 {
    // let mut hm = std::collections::HashMap::new();
    // for letter in chars {
    //     hm.entry(letter)
    //         .and_modify(|counter| *counter += 1)
    //         .or_insert(1);
    // }

    // let mut freq = vec![0; 128];
    // for letter in chars.iter() {
    //     freq[*letter as usize] += 1;
    // }

    // let n = chars.len();
    // let mut st = 0;
    // let mut end = 0;
    // let mut st_seq_idx = 0;
    // let mut st_seq_letter = 'a';
    // let mut count = 0;
    // let mut result_len = 0;
    // let mut last_one_char = false;
    // while end < n {
    //     last_one_char = false;
    //     if chars[st] == chars[end] {
    //         st_seq_letter = chars[end];
    //         count += 1;
    //         end += 1;
    //         last_one_char = true;
    //     } else {
    //         // count -> 3
    //         // end -> 2 at b
    //         println!("{:?}", chars);
    //         println!("{:?}", count);
    //         if count == 1 {
    //             st += 1;
    //             result_len += 1;
    //             count = 0;
    //             st_seq_idx += 1;
    //             print!("OOMMG");
    //             last_one_char = true;
    //             continue;
    //         }
    //         chars[st_seq_idx] = st_seq_letter;
    //         // count two digits case
    //         if count >= 10 {
    //             let first = count / 10;
    //             let last = count % 10;
    //             chars[st_seq_idx + 1] = char::from_digit(first, 10).unwrap();
    //             chars[st_seq_idx + 2] = char::from_digit(last, 10).unwrap();
    //             result_len += 1;
    //         } else {
    //             chars[st_seq_idx + 1] = char::from_digit(count, 10).unwrap();
    //         }
    //
    //         while chars[st] != chars[end] {
    //             // go to the next letter, "b"
    //             st += 1;
    //         }
    //         result_len += 2;
    //         count = 0;
    //         st_seq_idx = st;
    //         // while st != end inc st
    //     }
    // }
    // if last_one_char {
    //     return result_len + 1;
    // }
    // dbg!(&chars);
    // chars[st_seq_idx] = st_seq_letter;
    // dbg!(st_seq_idx);
    // dbg!(count);
    // // count two digits case
    // if count >= 10 {
    //     let first = count / 10;
    //     let last = count % 10;
    //     chars[st_seq_idx + 1] = char::from_digit(first, 10).unwrap();
    //     chars[st_seq_idx + 2] = char::from_digit(last, 10).unwrap();
    //     result_len += 1;
    // } else {
    //     chars[st_seq_idx + 1] = char::from_digit(count, 10).unwrap();
    // }
    // result_len += 2;
    // dbg!(chars);
    //
    // result_len
    let mut st = 0;
    let mut end = 1;
    let mut count = 1;
    // dummy insert
    chars.push('%');
    let n = chars.len();

    while end < n {
        if chars[end] != chars[end - 1] {
            chars[st] = chars[end - 1];

            if count > 1 {
                let num_str = count.to_string();
                for &num in num_str.as_bytes() {
                    st += 1;
                    chars[st] = num as char;
                }
            }
            st += 1;
            count = 0;
        }

        end += 1;
        count += 1;
    }
    chars.pop();
    st as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        // Input: chars = ["a","a","b","b","c","c","c"]
        // Output: Return 6, and the first 6 characters of the input array should be: ["a","2","b","2","c","3"]
        // Explanation: The groups are "aa", "bb", and "ccc". This compresses to "a2b2c3".
        let mut chars = vec!['a', 'a', 'b', 'b', 'c', 'c', 'c'];
        let output = 6;

        let result = compress(&mut chars);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let mut chars = vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b',
        ];
        let output = 4;

        let result = compress(&mut chars);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let mut chars = vec!['a'];
        let output = 1;

        let result = compress(&mut chars);
        assert_eq!(result, output);
    }
}
