use std::char;

pub fn _find_the_longest_substring(s: String) -> i32 {
    // greedy
    // take the whole string
    // if it has an even number of vowels, that's our answer,
    // if it has odd
    // try to remove from the start and try to remove from the end
    // if we found the odd on the end with fewer moves, that's our answer
    // if we found the odd on the start with fewer moves, that's our answer
    // maybe an edge case there is no even vowels
    // or only one vowel

    // oh it's an each vowel happens an even number of times
    // scratch all that I typed
    // brute-force
    // try every substring and see if each vowel has an even number of occurences
    // starting from full length till our condition is true then we return that answer
    //
    // my solution would be prefix sum for each vowel,
    // then binary search on the answer
    // let's say the binary search would say that our answer is 5
    // the binary search would try every subarray with len 5
    // our answer is at least 0 or 2
    // so it would be something like this
    // T T T T T F F F F F
    // x e f e t
    // 0 1 1 2 2
    // len of 0 -> T
    // len of 1 -> T
    // len of 2 -> F
    // len of 3 -> T
    // let of 4 -> T
    // let of 5 -> T
    // I guess I would just do prefix sum with two loops

    let n = s.len();
    let vowels = std::collections::HashSet::from([b'a', b'e', b'i', b'o', b'u']);
    let mut try_sum_all = vec![0; n + 1];
    let mut prefix_sum_vowels = vec![
        vec![0; n + 1],
        vec![0; n + 1],
        vec![0; n + 1],
        vec![0; n + 1],
        vec![0; n + 1],
    ];

    let mut index = 0;
    for &vowel in vowels.iter() {
        for i in 1..n + 1 {
            let add_one = if s.as_bytes()[i - 1] == vowel { 1 } else { 0 };
            prefix_sum_vowels[index][i] = prefix_sum_vowels[index][i - 1] + add_one;
        }
        index += 1;
    }

    let mut count = 0;

    for i in 0..n + 1 {
        'outer: for j in i + 1..n + 1 {
            let mut index = 0;
            for &vowel in vowels.iter() {
                if (prefix_sum_vowels[index][j] - prefix_sum_vowels[index][i]) % 2 == 1 {
                    continue 'outer;
                }
                index += 1;
            }
            dbg!(j);
            dbg!(i);
            count = count.max(j - i);
        }
    }

    dbg!(&prefix_sum_vowels);

    count as i32
    // gets TLE
    //
}

pub fn find_the_longest_substring(s: String) -> i32 {
    let n = s.len();
    let s_bytes = s.as_bytes();

    // Map vowels to bits: a = 0b00001, e = 0b00010, i = 0b00100, o = 0b01000, u = 0b10000
    let mut mask_to_index = vec![-1; 32]; // There are 2^5 = 32 possible bitmasks
    mask_to_index[0] = 0; // For the case where no vowels or even occurrences of all vowels
    let mut mask = 0;
    let mut max_len = 0;

    for (i, &ch) in s_bytes.iter().enumerate() {
        // Update mask based on the current character
        match ch {
            b'a' => mask ^= 0b00001,
            b'e' => mask ^= 0b00010,
            b'i' => mask ^= 0b00100,
            b'o' => mask ^= 0b01000,
            b'u' => mask ^= 0b10000,
            _ => {}
        }

        // If this mask has been seen before, calculate the length of the substring
        if mask_to_index[mask] != -1 {
            max_len = max_len.max(i + 1 - mask_to_index[mask] as usize);
        } else {
            // Store the first occurrence of this mask
            mask_to_index[mask] = (i + 1) as i32;
        }
        // dbg!(ch as char);
        // dbg!(format!("{:b}", mask));
        // dbg!(&mask_to_index);
    }

    max_len as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "eleetminicoworoep".to_string();
        let output = 13;
        // Explanation: The longest substring is "leetminicowor" which contains two each of the vowels: e,
        // i and o and zero of the vowels: a and u.
        let result = find_the_longest_substring(s);
        assert_eq!(result, output);
    }
}
