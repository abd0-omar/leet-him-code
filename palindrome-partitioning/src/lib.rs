pub fn partition(s: String) -> Vec<Vec<String>> {
    let mut result_susbset = Vec::new();
    _partition(s.as_bytes(), 0, vec![], &mut result_susbset);
    result_susbset
}

pub fn _partition(
    s: &[u8],
    st: usize,
    mut cur_subset: Vec<String>,
    result_susbset: &mut Vec<Vec<String>>,
) -> () {
    if st == s.len() {
        result_susbset.push(cur_subset.clone());
    }

    for end in st..s.len() {
        if is_palindrom(&s[st..end + 1]) {
            cur_subset.push(String::from_utf8(s[st..end + 1].to_vec()).unwrap());
            _partition(s, end + 1, cur_subset.clone(), result_susbset);
            cur_subset.pop();
        }
    }
}

fn is_palindrom(word: &[u8]) -> bool {
    for i in 0..word.len() / 2 {
        if word[i] != word[word.len() - i - 1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "aab".to_string();
        let output = vec![
            vec!["a".to_string(), "a".to_string(), "b".to_string()],
            vec!["aa".to_string(), "b".to_string()],
        ];
        let result = partition(s);
        assert_eq!(result, output);
    }
}
