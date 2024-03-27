use std::collections::HashMap;

fn main() {
    let s = "([{}])".to_string();

    let s = s.chars().collect::<Vec<_>>();

    let mut hs = HashMap::new();
    hs.insert('(', ')');
    hs.insert('[', ']');
    hs.insert('{', '}');

    let mut stack: Vec<char> = Vec::new();
    let mut flag = true;

    for c in s {
        if !hs.contains_key(&c) && flag {
            let top = stack.last().cloned();
            loop {
                if stack.is_empty() {
                    break;
                }
                match top {
                    Some(topz) => {
                        if c == *hs.get(&topz).unwrap() {
                            stack.pop();
                        } else {
                            !flag;
                            break;
                        }
                    }
                    None => break,
                }
                // if top.clone() == c {
                //     stack.pop();
                // } else {
                //     break;
                // }
            }
        } else {
            stack.push(c);
        }
    }
    if stack.is_empty() {
        println!("true");
    } else {
        println!("false");
    }
}
