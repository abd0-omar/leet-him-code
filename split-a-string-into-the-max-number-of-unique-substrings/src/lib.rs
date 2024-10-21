use std::collections::HashSet;

pub fn max_unique_split(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect();
    let mut all_splits = Vec::new();
    let mut seen = HashSet::new();
    _backtrack(&s, 0, &mut seen, Vec::new(), &mut all_splits);

    all_splits.iter().map(|v| v.len() as i32).max().unwrap_or(0)
}

fn _backtrack(
    s: &[char],
    start: usize,
    seen: &mut HashSet<String>,
    mut current_split: Vec<String>,
    all_splits: &mut Vec<Vec<String>>,
) {
    if start == s.len() {
        all_splits.push(current_split);
        return;
    }

    // dp range
    //
    // this will result in not working
    // so the `=` matters a lot here
    // for end in start + 1..s.len() {
    //     let substring: String = s[start..=end].iter().collect();
    for end in start + 1..=s.len() {
        let substring: String = s[start..end].iter().collect();

        if !seen.contains(&substring) {
            seen.insert(substring.clone());
            current_split.push(substring.clone());

            _backtrack(s, end, seen, current_split.clone(), all_splits);

            current_split.pop();
            seen.remove(&substring);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "ababccc".to_string();
        let output = 5;
        let result = max_unique_split(s);
        assert_eq!(result, output);
    }
}
