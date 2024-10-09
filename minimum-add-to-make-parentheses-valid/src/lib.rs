pub fn min_add_to_make_valid(s: String) -> i32 {
    // it's literally the easy parenthesis stack problem
    // https://tenor.com/bEWgE.gif
    // I have college by 8 but this is just too easy
    let mut stack = Vec::new();

    for letter in s.chars() {
        if let Some(&peek) = stack.last() {
            if peek == '(' && letter == ')' {
                stack.pop();
                continue;
            }
        }
        stack.push(letter);
    }
    stack.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "())".to_string();
        let output = 1;
        let result = min_add_to_make_valid(s);
        assert_eq!(result, output);
    }
}
