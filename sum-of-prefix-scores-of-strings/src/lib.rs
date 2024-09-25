struct Trie {
    child: Box<[Option<Trie>; 26]>,
    is_leaf: bool,
    count: i32,
}

const REPEAT_VALUE: Option<Trie> = None;

impl Trie {
    fn new() -> Self {
        Self {
            child: Box::new([REPEAT_VALUE; 26]),
            is_leaf: false,
            count: 0,
        }
    }

    fn add(&mut self, word: &[u8]) {
        let mut cur_trie = self;

        for letter in word {
            let letter_idx = (letter - b'a') as usize;

            if cur_trie.child[letter_idx].is_none() {
                cur_trie.child[letter_idx] = Some(Trie::new());
            }

            cur_trie = cur_trie.child[letter_idx].as_mut().unwrap();
            cur_trie.count += 1;
        }

        cur_trie.is_leaf = true;
    }

    fn count_prefix(&mut self, word: &[u8]) -> i32 {
        let mut cur_trie = self;
        let mut ans = 0;

        for letter in word {
            let letter_idx = (letter - b'a') as usize;

            if cur_trie.child[letter_idx].is_none() {
                break;
            }

            cur_trie = cur_trie.child[letter_idx].as_mut().unwrap();
            ans += cur_trie.count;
        }

        ans
    }
}

pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
    let mut trie = Trie::new();

    // I don't think this should be "hard", more like a medium
    // just a classical trie implementation

    // all words to the trie
    for word in words.iter() {
        trie.add(word.as_bytes());
    }

    //check every prefix of every word if there is a prefix to it in the trie

    let n = words.len();
    let mut result = vec![0; n];

    // for (idx, word) in words.into_iter().enumerate() {
    //     let n = word.len();
    //     dbg!(&word);
    //     for i in 1..n + 1 {
    //         let prefix = &word[0..i];
    //         dbg!(prefix);
    //         result[idx] += trie.count_prefix(prefix.as_bytes());
    //     }
    // }
    for (idx, word) in words.into_iter().enumerate() {
        result[idx] += trie.count_prefix(word.as_bytes());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let words: Vec<String> = ["abc", "ab", "bc", "b"]
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        let output = vec![5, 4, 3, 2];
        let result = sum_prefix_scores(words);
        assert_eq!(result, output);
    }
}
