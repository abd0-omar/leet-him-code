fn main() {
    println!("Hello, world!");
}

pub fn max_depth(s: String) -> i32 {
    let mut count = 0;
    let mut max_count = 0;
    for s_char in s.chars() {
        match s_char {
            '(' => {
                count += 1;
                max_count = max_count.max(count);
            }
            ')' => count -= 1,
            _ => (),
        }
    }
    max_count
}

#[cfg(test)]
mod tests {
    use super::max_depth;

    #[test]
    fn example1() {
        //     Example 1:
        //
        let s = "(1+(2*3)+((8)/4))+1".to_string();
        // Output: 3
        // Explanation: Digit 8 is inside of 3 nested parentheses in the string.}
        assert_eq!(max_depth(s), 3)
    }
}
