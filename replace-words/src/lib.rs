pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
    let mut trie = Trie::new();
    for word in dictionary {
        trie.insert(word.as_bytes());
    }

    let mut result = String::with_capacity(sentence.len());
    for word in sentence.split_whitespace() {
        if let Some(prefix) = trie.is_prefix(word.as_bytes()) {
            result.push_str(&prefix);
        } else {
            result.push_str(word);
        }
        result.push(' ');
    }
    result.trim_end().to_owned()
}

#[derive(Clone)]
struct Trie {
    child: Vec<Option<Trie>>,
    is_leaf: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            child: vec![None; 26],
            is_leaf: false,
        }
    }

    fn insert(&mut self, word: &[u8]) {
        let mut cur_trie = self;
        for letter in word {
            let letter_idx = (letter - b'a') as usize;
            if cur_trie.child[letter_idx].is_none() {
                cur_trie.child[letter_idx] = Some(Trie::new());
            }
            cur_trie = cur_trie.child[letter_idx].as_mut().unwrap();
        }
        cur_trie.is_leaf = true;
    }

    fn is_prefix(&self, word: &[u8]) -> Option<String> {
        let mut cur_trie = self;
        let mut result = String::with_capacity(word.len());

        for &letter in word {
            let letter_idx = (letter - b'a') as usize;

            if cur_trie.child[letter_idx].is_none() {
                return None;
            }

            result.push(char::from(letter));
            cur_trie = cur_trie.child[letter_idx].as_ref().unwrap();

            if cur_trie.is_leaf {
                return Some(result);
            }
        }

        None
    }
}

//                      put dict in trie and see if sentence have a prefix in the trie
//                      trie
//                       []
//              b        c            r
//             a         a            a
//             t        t             t

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let dictionary = vec!["cat".to_string(), "bat".to_string(), "rat".to_string()];
        let sentence = "the cattle was rattled by the battery".to_string();
        let output = "the cat was rat by the bat".to_string();
        let result = replace_words(dictionary, sentence);
        assert_eq!(result, output);
    }
}
