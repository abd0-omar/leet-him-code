pub fn make_good(s: String) -> String {
    let mut stack: Vec<char> = Vec::with_capacity(s.len());
    for s_char in s.chars() {
        match stack.last() {
            Some(top) => {
                if top.is_uppercase() && s_char.is_uppercase() {
                    stack.push(s_char);
                } else if top.to_uppercase().to_string() == s_char.to_string()
                    || s_char.to_uppercase().to_string() == top.to_string()
                {
                    stack.pop();
                } else {
                    stack.push(s_char);
                }
            }
            None => {
                stack.push(s_char);
            }
        }
    }
    stack.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let result = make_good(String::from("leEeetcode"));
        assert_eq!(result, String::from("leetcode"))
    }

    #[test]
    fn it_works2() {
        let result = make_good(String::from("abBAcC"));
        assert_eq!(result, String::from(""))
    }
}
