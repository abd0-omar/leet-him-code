pub fn check_valid_string(s: String) -> bool {
    // let mut memory = std::collections::HashMap::new();
    let mut memory = vec![vec![false; s.len()]; s.len()];
    _check_valid_string(s.as_bytes(), 0, 0, &mut memory)
}

pub fn _check_valid_string(
    s: &[u8],
    idx: usize,
    open_parenthesis: i32,
    // memory: &mut std::collections::HashMap<(i32, usize), bool>,
    memory: &mut Vec<Vec<bool>>,
) -> bool {
    if idx == s.len() {
        return open_parenthesis == 0;
    }

    // if let Some(&ret) = memory.get(&(open_parenthesis, idx)) {
    //     return ret;
    // }

    if open_parenthesis >= 0 {
        if memory[open_parenthesis as usize][idx] {
            return true;
        }
    }

    let mut result = false;

    if s[idx] == b'*' {
        // * == ''
        result |= _check_valid_string(s, idx + 1, open_parenthesis, memory);

        // * == '('
        result |= _check_valid_string(s, idx + 1, open_parenthesis + 1, memory);

        // * == ')'
        if open_parenthesis > 0 {
            result |= _check_valid_string(s, idx + 1, open_parenthesis - 1, memory);
        }
    } else if s[idx] == b'(' {
        result |= _check_valid_string(s, idx + 1, open_parenthesis + 1, memory);
    } else if s[idx] == b')' {
        if open_parenthesis > 0 {
            result |= _check_valid_string(s, idx + 1, open_parenthesis - 1, memory);
        }
    }
    // memory.insert((open_parenthesis, idx), result);
    if open_parenthesis >= 0 {
        memory[open_parenthesis as usize][idx] = result;
    }
    result
}

// initial tle solution, the tle happens on test 5
pub fn another_check_valid_string(s: String) -> bool {
    let mut memory = std::collections::HashMap::new();
    _another_check_valid_string(s.as_bytes(), 0, &mut memory)
}

pub fn stack_it(s: &[u8]) -> bool {
    let n = s.len();
    let mut stack = Vec::with_capacity(n);

    for s_char in s.iter() {
        match s_char {
            b'(' => stack.push('('),
            b')' => match stack.pop() {
                Some(_) => (),
                None => return false,
            },
            // b'*' => {
            //     stack.push('(');
            // }
            _ => (),
        }
    }

    println!("stack={:?}", stack);
    stack.is_empty()
}

pub fn _another_check_valid_string(
    s: &[u8],
    idx: usize,
    memory: &mut std::collections::HashMap<(Vec<u8>, usize), bool>,
) -> bool {
    if idx == s.len() {
        return stack_it(s);
    }

    if let Some(&ret) = memory.get(&(s.to_owned(), idx)) {
        return ret;
    }

    // * == ''
    // make * == empty '' in stack_it fn
    let choice1 = _another_check_valid_string(s, idx + 1, memory);

    if s[idx] == b'*' {
        // * == '('
        let mut new_s = s.to_owned();
        new_s[idx] = b'(';
        let choice2 = _another_check_valid_string(&new_s, idx + 1, memory);

        // * == ')'
        let mut new_s = s.to_owned();
        new_s[idx] = b')';
        let choice3 = _another_check_valid_string(&new_s, idx + 1, memory);
        let f = choice3 || choice2 || choice1;
        memory.insert((s.to_owned(), idx), f);
        f
    } else {
        choice1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = String::from("()");
        assert!(check_valid_string(s));
    }

    #[test]
    fn it_works1() {
        let s = String::from("(*)");
        assert!(check_valid_string(s));
    }

    #[test]
    fn it_works2() {
        let s = String::from("(*))");
        assert!(check_valid_string(s));
    }

    #[test]
    fn it_works3() {
        // "(((((*)))**"
        // "(((((*)**))"
        //  (((((()))((
        //  ((((())))))
        let s = String::from("(((((*)))**");
        assert!(check_valid_string(s));
    }

    #[test]
    fn it_works4() {
        let s = String::from("((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((((");
        assert!(!check_valid_string(s));
    }

    #[test]
    fn it_works5() {
        let s = String::from("(((((*(()((((*((**(((()()*)()()()*((((**)())*)*)))))))(())(()))())((*()()(((()((()*(())*(()**)()(())");
        assert!(!check_valid_string(s));
    }
}
