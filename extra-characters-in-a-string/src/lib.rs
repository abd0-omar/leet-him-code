use std::collections::HashMap;

#[derive(Debug)]
struct Trie {
    child: Box<[Option<Trie>; 26]>,
    is_leaf: bool,
}

const ARRAY_REPEAT_VALUE: Option<Trie> = None;
impl Trie {
    fn new() -> Self {
        Trie {
            child: Box::new([ARRAY_REPEAT_VALUE; 26]),
            is_leaf: false,
        }
    }

    fn add(&mut self, word: &[u8]) {
        let mut cur_trie = self;

        for &letter in word {
            let letter_idx = (letter - b'a') as usize;
            if cur_trie.child[letter_idx].is_none() {
                cur_trie.child[letter_idx] = Some(Trie::new());
            }
            cur_trie = cur_trie.child[letter_idx].as_mut().unwrap();
        }
        cur_trie.is_leaf = true;
    }

    fn search_from(&self, s: &[u8], start: usize, memo: &mut HashMap<usize, i32>, n: usize) -> i32 {
        if start == n {
            return 0;
        }
        if let Some(&ret) = memo.get(&start) {
            return ret;
        }

        let mut ans = self.search_from(s, start + 1, memo, n) + 1;

        let mut cur_trie = self;
        for end in start..n {
            let letter_idx = (s[end] - b'a') as usize;
            if cur_trie.child[letter_idx].is_none() {
                break;
            }
            cur_trie = cur_trie.child[letter_idx].as_ref().unwrap();

            if cur_trie.is_leaf {
                ans = ans.min(self.search_from(s, end + 1, memo, n));
            }
        }

        memo.insert(start, ans);
        ans
    }
}

struct Solution;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let mut trie = Trie::new();
        for word in dictionary {
            trie.add(word.as_bytes());
        }

        let n = s.len();
        let mut memo = HashMap::new();
        trie.search_from(s.as_bytes(), 0, &mut memo, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_extra_char() {
        let s = "kevlplxozaizdhxoimmraiakbak".to_string();
        let dictionary = vec![
            "yv", "bmab", "hv", "bnsll", "mra", "jjqf", "g", "aiyzi", "ip", "pfctr", "flr",
            "ybbcl", "biu", "ke", "lpl", "iak", "pirua", "ilhqd", "zdhx", "fux", "xaw", "pdfvt",
            "xf", "t", "wq", "r", "cgmud", "aokas", "xv", "jf", "cyys", "wcaz", "rvegf", "ysg",
            "xo", "uwb", "lw", "okgk", "vbmi", "v", "mvo", "fxyx", "ad", "e",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
        let result = Solution::min_extra_char(s, dictionary);
        assert_eq!(result, 9);
    }
}
