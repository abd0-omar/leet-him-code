pub fn longest_palindrome(s: String) -> i32 {
    // a b c c c c d d
    // i j
    // i   j
    // get freq
    // if there is an extra letter with freq one added it or freq /2
    use std::collections::HashMap;
    let mut freq = HashMap::new();
    for letter in s.chars() {
        *freq.entry(letter).or_insert(0) += 1;
    }

    let mut count = 0;
    let mut take_odd = false;
    for (_, ocurr) in freq {
        if ocurr % 2 == 1 {
            if !take_odd {
                if ocurr >= 1 {
                    count += ocurr;
                    take_odd = true;
                }
            } else {
                count += ocurr - 1;
            }
        } else {
            count += ocurr;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "abccccdd".to_string();
        let output = 7;
        let result = longest_palindrome(s);
        assert_eq!(result, output);
    }
}
