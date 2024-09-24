struct Trie {
    child: Box<[Option<Trie>; 10]>,
    // is_leaf is redundant here
    is_leaf: bool,
}

const ARRAY_REPEAT_VALUE: Option<Trie> = None;
impl Trie {
    fn new() -> Self {
        Self {
            child: Box::new([ARRAY_REPEAT_VALUE; 10]),
            is_leaf: false,
        }
    }

    fn add(&mut self, number: &[u8]) {
        let mut cur_trie = self;

        for digit in number {
            let digit_idx = (digit - b'0') as usize;

            if cur_trie.child[digit_idx].is_none() {
                // create one
                // else move to the next trie, or anyway move to the next trie
                cur_trie.child[digit_idx] = Some(Trie::new());
            }

            cur_trie = cur_trie.child[digit_idx].as_mut().unwrap();
        }

        cur_trie.is_leaf = true;
    }

    fn prefix_count(&mut self, number: &[u8]) -> i32 {
        let mut cur_trie = self;
        let mut count = 0;

        for digit in number {
            let digit_idx = (digit - b'0') as usize;

            // if no next trie return
            if cur_trie.child[digit_idx].is_none() {
                return count;
            }

            cur_trie = cur_trie.child[digit_idx].as_mut().unwrap();

            count += 1;
        }

        count
    }
}

pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    // it's trie week
    // I tried my best

    // add arr1, then check with arr2

    let mut trie = Trie::new();
    for number in arr1 {
        let number = format!("{}", number);

        trie.add(number.as_bytes());
    }

    let mut max = 0;
    for number in arr2 {
        let number = format!("{}", number);

        max = max.max(trie.prefix_count(number.as_bytes()));
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr1 = vec![1, 10, 100];
        let arr2 = vec![1000];
        let output = 3;
        let result = longest_common_prefix(arr1, arr2);
        assert_eq!(result, output);
    }
}
