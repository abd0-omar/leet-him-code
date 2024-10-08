pub fn min_swaps(s: String) -> i32 {
    // I know that it is a stack problem
    // but it's not the same as the classical parenthesis one
    // ]][][[
    // ][[]][
    //
    // most simple cases
    // []
    // 0
    //
    // ][
    // 1
    //
    // ][][
    // 1
    //
    // ][][][
    // 1
    //
    // ]
    //
    // if there is [], remove it by using the stack
    // after filtering you would have a string that looks like this somehow ]]][[[
    // how did we managed to get two tho 6/3?
    //
    // let's think about this test case
    // ]][[
    // it would be I guess also 2
    // 4/2
    //
    // so the answer is stack.len() * 0.5 ?
    // let's try it
    //
    // I was dead wrong

    let mut stack = Vec::new();
    let mut unbalanced = 0;

    // filtering the string from "[]"s
    for letter in s.chars() {
        if letter == '[' {
            stack.push(letter);
        } else {
            if !stack.is_empty() {
                stack.pop();
            } else {
                unbalanced += 1;
            }
        }
        // if let Some(&peek) = stack.last() {
        //     if peek == '[' && letter == ']' {
        //         stack.pop();
        //         continue;
        //     }
        // }
        // stack.push(letter);
    }
    // dbg!(&stack);
    //
    // let divisor = stack.len() as f64 * 0.5;
    // dbg!(divisor);
    // let answer = stack.len() as f64 / divisor;
    // dbg!(stack.len() as f64 / divisor);

    (unbalanced + 1) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "]]][[[".to_string();
        let output = 2;
        // Explanation: You can do the following to make the string balanced:
        //                                   012345
        //                                   ]]][[[
        // - Swap index 0 with index 4. s = "[]][][".
        // - Swap index 1 with index 5. s = "[[][]]".
        // The resulting string is "[[][]]".
        let result = min_swaps(s);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "][][".to_string();
        let output = 1;
        let result = min_swaps(s);
        assert_eq!(result, output);
    }
}
