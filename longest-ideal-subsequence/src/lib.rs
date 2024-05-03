pub fn longest_ideal_string(s: String, k: i32) -> i32 {
    // let mut memory = std::collections::HashMap::new();
    let mut memory = vec![vec![None; 27]; s.len()];
    // let mut result = 0;
    // for i in 0..s.len() {
    //     let start = i;
    //     result = result.max(_longest_ideal_string(s.as_bytes(), k, start, &mut memory));
    // }
    // result
    _longest_ideal_string(s.as_bytes(), k, 0, None, &mut memory)
}

pub fn _longest_ideal_string(
    s: &[u8],
    k: i32,
    idx: usize,
    last_letter: Option<usize>,
    // hm: &mut std::collections::HashMap<(usize, Option<u8>), i32>,
    memory: &mut Vec<Vec<Option<i32>>>,
) -> i32 {
    if idx == s.len() {
        return 0;
    }

    if let Some(last_l) = last_letter {
        if let Some(ret) = memory[idx][last_l] {
            return ret;
        }
    }

    // leave
    let leave = _longest_ideal_string(s, k, idx + 1, last_letter, memory);

    //pick
    let cur_char = s[idx] - b'a';
    let pick = if let Some(l_l) = last_letter {
        if (cur_char as i32 - l_l as i32).abs() <= k {
            _longest_ideal_string(s, k, idx + 1, Some(cur_char as usize), memory) + 1
        } else {
            0
        }
    } else {
        _longest_ideal_string(s, k, idx + 1, Some(cur_char as usize), memory) + 1
    };

    let result = leave.max(pick);
    if let Some(last_l) = last_letter {
        memory[idx][last_l] = Some(result);
    }
    result
}

// pub fn _longest_ideal_string(
//     s: &[u8],
//     k: i32,
//     idx: usize,
//     // hm: &mut std::collections::HashMap<(usize, Option<u8>), i32>,
//     memory: &mut std::collections::HashMap<usize, i32>,
//     // memory: &mut Vec<Option<i32>>,
// ) -> i32 {
//     if idx == s.len() {
//         return 0;
//     }
//
//     if let Some(ret) = memory.get(&idx) {
//         return *ret;
//     }
//
//     let mut result = 0;
//
//     for j in idx + 1..s.len() {
//         if (s[j] as i32 - s[idx] as i32).abs() <= k {
//             result = result.max(_longest_ideal_string(s, k, j, memory));
//         }
//     }
//     result += 1;
//     memory.insert(idx, result);
//     result
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "acfgbd".to_string();
        let k = 2;
        let output = 4;
        // Explanation: The longest ideal string is "acbd". The length of this string is 4, so 4 is returned.
        // Note that "acfgbd" is not ideal because 'c' and 'f' have a difference of 3 in alphabet order.
        // straightforward pick or leave
        let result = longest_ideal_string(s, k);
        assert_eq!(result, output);
    }
}
