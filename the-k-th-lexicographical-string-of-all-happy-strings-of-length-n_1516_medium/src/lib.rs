// https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/
#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        /*
                            st
                a            b           c
            ab   ac        ba  bc       ca   cb
        abc  aba  acb  aca ..
         */
        let mut result = Vec::new();
        let n = n as usize;
        let letters = ['a', 'b', 'c'];
        for &letter in &letters {
            backtrack(letter, vec![letter], &mut result, &letters, n);
        }
        // dbg!(&result);
        result
            .get(k as usize - 1)
            .map(|target| target.iter().collect())
            .unwrap_or(String::new())
    }
}

fn backtrack(
    prev_letter: char,
    mut cur_vec: Vec<char>,
    result: &mut Vec<Vec<char>>,
    letters: &[char],
    n: usize,
) {
    if cur_vec.len() == n {
        result.push(cur_vec);
        return;
    }

    for &cur_letter in letters {
        if cur_letter == prev_letter {
            continue;
        }

        cur_vec.push(cur_letter);
        backtrack(cur_letter, cur_vec.clone(), result, letters, n);
        cur_vec.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let n = 1;
        let k = 3;
        let output = "c".to_string();
        let result = Solution::get_happy_string(n, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let n = 1;
        let k = 4;
        let output = "".to_string();
        let result = Solution::get_happy_string(n, k);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let n = 3;
        let k = 9;
        let output = "cab".to_string();
        let result = Solution::get_happy_string(n, k);
        assert_eq!(result, output);
    }
}
