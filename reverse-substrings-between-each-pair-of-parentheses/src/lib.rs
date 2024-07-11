pub fn reverse_parentheses(s: String) -> String {
    /*
    stack: u, love
    evol u
    stack: i
    i love u

    (ed(et(oc))el)
    stack: ed, et, oc
    co
    octe
    // add to the result string then reverse, when you see )
    // then add back to the stack
    stack: ed, octe, el


    le etco de

    tracing once again
    (ed(et(oc))el)
    i = 0
    stack: ed
    i = 2
    stack: ed, et
    i = 3
    stack: ed, et, oc
    i = 4
    found ')'
    stack: ed, et
    *pop*
    oc
    reverse
    co
    *add it again to the last word*
    stack: ed, etco
    i = 5
    found ')'
    // pop
    etco
    reverse
    octe
    // add it again to the last word
    stack: edocte        ed, et, co          ed, te, oc
    i = 6
    stack: edocte, el
    i = 7
    found ')'
    // pop
    el
    reverse
    le
    // add it again to the last word
    stack: edoctele                ed, te, oc, el            ed, et, co, le
    // stack is not empty
    // pop
    leetcode
    */
    // let mut stack: Vec<_> = vec![];
    // let mut string = vec![];
    // let mut inner_string = String::new();

    // for letter in s.chars() {
    //     if letter == '(' {
    //         string.push(inner_string.clone());
    //         inner_string.clear();
    //         continue;
    //     }
    //     if letter == ')' {
    //         string.push(inner_string.clone());
    //         string.push(")".to_owned());
    //         inner_string.clear();
    //         continue;
    //     }
    //     inner_string.push(letter);
    // }
    // dbg!(&string);

    // /*
    // "",
    // "ed",
    // "et",
    // "oc",
    // ")",
    // "",
    // ")",
    // "el",
    // ")",
    // */
    let mut stack: Vec<String> = Vec::new();
    let mut current_string = String::new();

    for ch in s.chars() {
        if ch == '(' {
            stack.push(current_string);
            current_string = String::new();
        } else if ch == ')' {
            let reversed_string: String = current_string.chars().rev().collect();
            current_string = format!("{}{}", stack.pop().unwrap(), &reversed_string);
            // dbg!(&current_string);
        } else {
            current_string.push(ch);
        }
        // dbg!(&stack);
    }

    current_string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = "(u(love)i)".to_string();
        let output = "iloveu".to_string();
        let result = reverse_parentheses(s);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "(ed(et(oc))el)".to_string();
        let output = "leetcode".to_string();
        let result = reverse_parentheses(s);
        assert_eq!(result, output);
    }
}
