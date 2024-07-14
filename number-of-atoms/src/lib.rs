pub fn count_of_atoms(formula: String) -> String {
    // K4(ON(SO3)2)2
    // k4(ONS2O6)2
    // k4-O2-N2-S4-O-12
    // add all the similar
    // k4-N2-S4-O14
    // maybe a stack with a hm to track the occurrences
    // stack would be like this
    // k4 ( ON ( SO3
    // and once the cur_letter is ) we pop till the first (
    // if there is a count after the ( we would multiply all of it with the num
    // so check if there is a count to multiply
    // k4 ( ON S2O6
    // let formula: Vec<char> = formula.chars().collect();
    // let mut stack: Vec<char> = vec![];
    // for (i, &cur_letter) in formula.iter().enumerate() {
    //     if cur_letter == ')' {
    //         // check if there is a count after it
    //         if i < formula.len() - 1 && formula[i + 1].is_digit(10) {
    //             let mult = formula[i + 1].to_digit(10).unwrap();
    //             let mut extracted = std::collections::VecDeque::new();
    //             while !stack.is_empty() {
    //                 // if let Some(top_num) = stack.last() {
    //                 //     if let Some(t_n) = top_num.to_digit(10) {
    //                 //         t_
    //                 //     }
    //                 // }
    //                 // extract the (SO3
    //                 if *stack.last().unwrap() == '(' {
    //                     stack.pop();
    //                     break;
    //                 }
    //                 let popped = stack.pop().unwrap();
    //                 if let Some(popping) = popped.to_digit(10) {
    //                     extracted.push_front(char::from_digit(popping * mult, 10).unwrap());
    //                 } else {
    //                     extracted.push_front(popped);
    //                 }
    //             }
    //             dbg!(extracted);
    //         }
    //     } else {
    //         stack.push(cur_letter);
    //     }
    // }
    // K4(ON(SO3)2)2
    // maybe parse it like this
    // k4 ( O N ( S O3 )2 )2
    // let formula: Vec<char> = formula.chars().collect();
    // let mut parsed = vec![];

    // let mut i = 0;
    // while i < formula.len() {
    //     if i < formula.len() - 1 {
    //         // parse till we find a num or ( or ) or to the end
    //         // that would be our symbol
    //         if formula[i + 1].is_digit(10) {
    //             if formula[i].is_alphabetic() || formula[i] == ')' {
    //                 parsed.push((formula[i], formula[i + 1].to_digit(10).unwrap()));
    //                 i += 1;
    //             } else {
    //                 parsed.push((formula[i], 1));
    //             }
    //         } else {
    //             parsed.push((formula[i], 1));
    //         }
    //     } else {
    //         parsed.push((formula[i], 1));
    //     }
    //     i += 1;
    // }

    // while i < formula.len() {
    //     // parse till we find a num or ( or ) or to the end
    //     // that would be our symbol
    //     while i < formula.len() && !formula[i].is_lowercase() {

    //     }
    //     parsed.push((formula[i], 1));
    //     i += 1;
    // }
    // dbg!(&parsed);

    // let mut stack: Vec<(char, u32)> = vec![];
    // for &(symbol, count) in parsed.iter() {
    //     if symbol == ')' {
    //         let mult = count;
    //         let mut extracted: Vec<(char, u32)> = Vec::new();
    //         while !stack.is_empty() {
    //             // k4 ( O N ( S O3 )2 )2
    //             // if count == 1 {
    //             //     // dont go in the while loop
    //             // }

    //             if stack.last().unwrap().0 == '(' {
    //                 stack.pop();
    //                 break;
    //             }

    //             let pop = stack.pop().unwrap();
    //             let new_num = pop.1 * mult;
    //             extracted.push((pop.0, new_num));
    //         }
    //         dbg!(&extracted);
    //         while let Some(pop) = extracted.pop() {
    //             stack.push(pop);
    //         }
    //         dbg!(&stack);
    //         // o6 s2
    //     } else {
    //         stack.push((symbol, count));
    //     }
    // }
    // /*
    //      [src/lib.rs:97:13] &stack = [
    //     (
    //         'K',
    //         4,
    //     ),
    //     (
    //         'O',
    //         2,
    //     ),
    //     (
    //         'N',
    //         2,
    //     ),
    //     (
    //         'S',
    //         4,
    //     ),
    //     (
    //         'O',
    //         12,
    //     ),
    // ]
    //      */
    // // add similar symbols
    // stack.sort_unstable();
    // dbg!(&stack);
    // let mut n = stack.len();
    // let mut i = 1;
    // while i < n {
    //     dbg!(i);
    //     if stack[i].0 == stack[i - 1].0 {
    //         let x = stack[i - 1].1;
    //         let y = stack[i].1;
    //         stack[i - 1].1 = x + y;
    //         stack.remove(i);
    //         n -= 1;
    //     }
    //     i += 1;
    // }
    // dbg!(&stack);
    // //     [src/lib.rs:143:5] stack = [
    // //     (
    // //         'K',
    // //         4,
    // //     ),
    // //     (
    // //         'N',
    // //         2,
    // //     ),
    // //     (
    // //         'O',
    // //         14,
    // //     ),
    // //     (
    // //         'S',
    // //         4,
    // //     ),
    // // ]

    // let mut result = String::with_capacity(n);
    // for i in 0..n {
    //     let symbol = stack[i].0;
    //     result.push(symbol);
    //     if stack[i].1 > 1 {
    //         let count = stack[i].1.to_string();
    //         dbg!(&count);
    //         result.push_str(&count);
    //     }
    // }
    // result

    // for (i, &cur_letter) in formula.iter().enumerate() {
    //     if cur_letter == ')' {
    //         // check if there is a count after it
    //         if i < formula.len() - 1 && formula[i + 1].is_digit(10) {
    //             let mult = formula[i + 1].to_digit(10).unwrap();
    //             let mut extracted = std::collections::VecDeque::new();
    //             while !stack.is_empty() {
    //                 // if let Some(top_num) = stack.last() {
    //                 //     if let Some(t_n) = top_num.to_digit(10) {
    //                 //         t_
    //                 //     }
    //                 // }
    //                 // extract the (SO3
    //                 if *stack.last().unwrap() == '(' {
    //                     stack.pop();
    //                     break;
    //                 }
    //                 let popped = stack.pop().unwrap();
    //                 if let Some(popping) = popped.to_digit(10) {
    //                     extracted.push_front(char::from_digit(popping * mult, 10).unwrap());
    //                 } else {
    //                     extracted.push_front(popped);
    //                 }
    //             }
    //             dbg!(extracted);
    //         }
    //     } else {
    //         stack.push(cur_letter);
    //     }
    // }
    // todo!()
    let mut stack: Vec<std::collections::HashMap<String, i32>> =
        // dummy insert or the start the recursion insert
        // like the paren stack problem when we added '(' at first
        vec![std::collections::HashMap::new()];
    let mut i = 0;
    let n = formula.len();

    while i < n {
        if formula.as_bytes()[i] == b'(' {
            stack.push(std::collections::HashMap::new());
            i += 1;
        } else if formula.as_bytes()[i] == b')' {
            let mut i_end = i + 1;
            // parse the digit after ')'
            while i_end < n && formula.as_bytes()[i_end].is_ascii_digit() {
                i_end += 1;
            }
            // if found a digit after the ')' get it else 1
            let mult = if i + 1 < i_end {
                formula[i + 1..i_end].parse::<i32>().unwrap()
            } else {
                1
            };
            let top = stack.pop().unwrap();
            let mut top_map = stack.pop().unwrap();
            for (key, value) in top.into_iter() {
                *top_map.entry(key).or_insert(0) += value * mult;
            }
            stack.push(top_map);
            i = i_end;
        } else {
            let i_start = i;
            i += 1;
            // parse small letters after symbol
            while i < n && formula.as_bytes()[i].is_ascii_lowercase() {
                i += 1;
            }
            let element = formula[i_start..i].to_string();
            let i_start = i;
            // parse symbol count
            while i < n && formula.as_bytes()[i].is_ascii_digit() {
                i += 1;
            }
            let mult = if i_start < i {
                formula[i_start..i].parse::<i32>().unwrap()
            } else {
                1
            };
            let top_map = stack.last_mut().unwrap();
            *top_map.entry(element).or_insert(0) += mult;
        }
        dbg!(&stack);
    }

    dbg!(&stack);

    let counter = stack.pop().unwrap();
    let mut elements: Vec<_> = counter.into_iter().collect();
    elements.sort();

    let mut result = String::new();
    for (element, count) in elements {
        result.push_str(&element);
        if count > 1 {
            result.push_str(&count.to_string());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let formula = "K4(ON(SO3)2)2".to_string();
        let output = "K4N2O14S4".to_string();
        let result = count_of_atoms(formula);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let formula = "Mg(OH)2".to_string();
        let output = "H2MgO2".to_string();
        let result = count_of_atoms(formula);
        assert_eq!(result, output);
    }
}
