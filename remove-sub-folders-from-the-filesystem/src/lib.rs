use std::collections::HashMap;

pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
    // trie
    // two pass trie
    // 1st pass just add to trie
    // second pass
    // if it is a leaf and the cur_string == the string in `folder` then add it to result
    // as simple as that
    // there is also a sorting solution too I guess
    //
    // parse input
    let split_paths: Vec<Vec<&str>> = folder
        .iter()
        .map(|path| path.split('/').collect())
        .collect();

    // dbg!(&split_paths);
    let mut trie = Trie::new();
    for path_parts in &split_paths {
        trie.add_path(path_parts.iter().copied());
    }

    let mut result = Vec::new();

    for (path_parts, raw_path) in split_paths.into_iter().zip(&folder) {
        if trie.path_exists(path_parts.iter().copied()) {
            result.push(raw_path.to_owned());
        }
    }

    result
}

struct Trie {
    // hashmap cuz the input might be a whole string not only chars
    child: HashMap<String, Trie>,
    is_leaf: bool,
}

impl Trie {
    fn new() -> Self {
        Self {
            child: HashMap::new(),
            is_leaf: false,
        }
    }

    fn add_path<'a, I>(&'a mut self, input: I)
    where
        I: Iterator<Item = &'a str>,
    {
        let mut cur_trie = self;

        for word in input {
            cur_trie = cur_trie
                .child
                .entry(word.to_string())
                .or_insert(Trie::new());
        }

        cur_trie.is_leaf = true;
    }

    fn path_exists<'a, I>(&'a self, input: I) -> bool
    where
        I: Iterator<Item = &'a str>,
    {
        let mut cur_trie = self;
        for word in input {
            // if cur_trie.child.get(&word).is_none() {
            //     // path doesn't exist اصلا
            //     // but this won't happen
            //     return false;
            // }
            if cur_trie.is_leaf {
                return false;
            }
            cur_trie = cur_trie
                .child
                .get(word)
                .expect("we added all the path before, it's a two pass trie problem");
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let folder = vec![
            "/a".to_string(),
            "/a/b".to_string(),
            "/c/d".to_string(),
            "/c/d/e".to_string(),
            "/c/f".to_string(),
        ];
        let output = ["/a".to_string(), "/c/d".to_string(), "/c/f".to_string()];
        let result = remove_subfolders(folder);
        assert_eq!(result, output);
    }
}
