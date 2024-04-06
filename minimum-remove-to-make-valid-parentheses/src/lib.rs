pub fn min_remove_to_make_valid(s: String) -> String {
    // found out a boolean should_delete vec was a better choice
    let n = s.len();
    let mut open_parentheses_to_remove = std::collections::VecDeque::with_capacity(n);
    let mut result = String::with_capacity(n);

    let mut remove_closed_parentheses = 0;

    for (i, s_char) in s.char_indices() {
        match s_char {
            '(' => {
                open_parentheses_to_remove.push_back(i);
            }
            ')' => match open_parentheses_to_remove.pop_back() {
                Some(_) => (),
                None => remove_closed_parentheses += 1,
            },
            _ => (),
        }
    }

    let mut should_remove = open_parentheses_to_remove.pop_front();
    for (i, s_char) in s.char_indices() {
        if let Some(should_remove_index) = should_remove {
            if should_remove_index == i {
                should_remove = open_parentheses_to_remove.pop_front();
                continue;
            }
        }
        match s_char {
            ')' => {
                if remove_closed_parentheses > 0 {
                    remove_closed_parentheses -= 1;
                } else {
                    // don't ignore
                    result.push(')');
                }
            }
            _ => {
                result.push(s_char);
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        // Example 1:
        // if )))(((
        // remove 1 )

        // remove 1 )
        let s = "lee(t(c)o)de)".to_string();
        // Output: "lee(t(c)o)de"
        assert_eq!(
            "lee(t(co)de)".to_string(),
            min_remove_to_make_valid(s.clone())
        );
        // Explanation: "lee(t(co)de)" , "lee(t(c)ode)" would also be accepted.
    }

    #[test]
    fn it_works2() {
        let s = ")))(((".to_string();
        assert_eq!("".to_string(), min_remove_to_make_valid(s.clone()));
    }

    #[test]
    fn it_works3() {
        // )
        let s = "())()(((".to_string();
        assert_eq!("()()".to_string(), min_remove_to_make_valid(s.clone()));
    }
}
