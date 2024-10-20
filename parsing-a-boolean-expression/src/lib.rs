pub fn parse_bool_expr(expression: String) -> bool {
    let mut stack = vec![];

    for letter in expression.chars() {
        if letter == '!'
            || letter == '&'
            || letter == '|'
            || letter == 'f'
            || letter == 't'
            || letter == '('
        {
            stack.push(letter);
        } else if letter == ')' {
            let mut express = vec![];
            while let Some(top) = stack.pop() {
                if top == '(' {
                    break;
                }
                express.push(top);
            }

            let op = stack.pop().unwrap();
            let res = match op {
                '&' => parse_and(&express),
                '|' => parse_or(&express),
                '!' => parse_not(express[0]),
                _ => unreachable!(),
            };

            stack.push(if res { 't' } else { 'f' });
            dbg!(&stack);
        }
    }

    stack.pop().unwrap() == 't'
}

fn parse_not(express: char) -> bool {
    express == 'f'
}

fn parse_and(express: &[char]) -> bool {
    !express.contains(&'f')
}

fn parse_or(express: &[char]) -> bool {
    express.contains(&'t')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_or() {
        let expression = "|(f,f,f,t)".to_string();
        let output = true;
        let result = parse_bool_expr(expression);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works_and() {
        let expression = "&(f,f,f,t)".to_string();
        let output = false;
        let result = parse_bool_expr(expression);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works_not() {
        let expression = "!(&(f,t))".to_string();
        let output = true;
        let result = parse_bool_expr(expression);
        assert_eq!(result, output);
    }
}
