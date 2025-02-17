// https://leetcode.com/problems/letter-tile-possibilities/
#[allow(dead_code)]
struct Solution;

use std::collections::HashSet;
#[allow(dead_code)]
impl Solution {
    pub fn num_tile_possibilities(tiles: String) -> i32 {
        // remove result later
        let mut result = HashSet::new();
        let mut used = vec![false; tiles.len()];
        backtrack(&tiles, &mut result, String::new(), &mut used);
        // dbg!(&result);
        result.len() as i32
    }
}

fn backtrack(tiles: &str, result: &mut HashSet<String>, mut cur: String, used: &mut [bool]) {
    for (idx, letter) in tiles.char_indices() {
        if used[idx] {
            continue;
        }
        used[idx] = true;
        cur.push(letter);
        backtrack(tiles, result, cur.clone(), used);
        cur.pop();
        used[idx] = false;
    }
    if !cur.is_empty() {
        result.insert(cur);
    }
}

// more memory optimized solution
// impl Solution {
//     pub fn num_tile_possibilities(tiles: String) -> i32 {
//         let mut counts = [0; 26]; // Frequency array for 'A' to 'Z'
//         for c in tiles.chars() {
//             counts[(c as u8 - b'A') as usize] += 1;
//         }
//         _backtrack(&mut counts)
//     }
// }
//
// fn _backtrack(counts: &mut [i32; 26]) -> i32 {
//     let mut sum = 0;
//
//     for i in 0..26 {
//         if counts[i] > 0 {
//             counts[i] -= 1;
//             sum += 1 + _backtrack(counts);
//             counts[i] += 1; // Backtrack
//         }
//     }
//
//     sum
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let tiles = "AAB".to_string();
        let output = 8;
        let result = Solution::num_tile_possibilities(tiles);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let tiles = "AAABBC".to_string();
        let output = 188;
        let result = Solution::num_tile_possibilities(tiles);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let tiles = "V".to_string();
        let output = 1;
        let result = Solution::num_tile_possibilities(tiles);
        assert_eq!(result, output);
    }
}
