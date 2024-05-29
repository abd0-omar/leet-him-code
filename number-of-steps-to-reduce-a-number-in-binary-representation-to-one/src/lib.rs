pub fn num_steps(s: String) -> i32 {
    // 1101
    //    (2^0 * 1) -> 1
    //    (2^1 * 0)
    //    (2^2 * 1) -> 4
    //    (2^3 * 1) -> 8

    // odd: add one -> flip all ones to zeros till you find the first zero then flip it
    // if all zeros after flipping or didn't find the first zero then push one to the front
    // even: divde it by 2 -> pop the last num which is zero
    // 1101 -> odd
    // 1110 -> even
    // 111 -> odd
    // 1000 -> even
    // 100 -> even
    // 10 -> even
    // 1 -> Done

    // s=Ok("11001")
    // s=Ok("11000") -1
    // s=Ok("1100")
    // s=Ok("110")
    // s=Ok("11")
    // s=Ok("100")
    // s=Ok("10")

    let mut s = s.into_bytes();
    let mut steps = 0;
    while s != vec![b'1'] {
        println!("s={:?}", String::from_utf8(s.clone()));
        let last_number = *s.iter().last().unwrap();
        // odd
        if last_number == b'1' {
            if let Some(first_zero) = s.iter().rev().position(|&f| f == b'0') {
                for j in (s.len() - first_zero - 1..s.len()).rev() {
                    s[j] = b'0';
                }
                println!("first_zero={:?}", first_zero);
                let n = s.len();
                s[n - first_zero - 1] = b'1';
            } else {
                let new_s = format!("{}{}", '1', "0".to_string().repeat(s.len()));
                s = new_s.into_bytes();
            }
        }
        // even
        else {
            s.pop();
        }
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = "1101".to_string();
        let output = 6;
        // Explanation: "1101" corressponds to number 13 in their decimal representation.
        // Step 1) 13 is odd, add 1 and obtain 14.
        // Step 2) 14 is even, divide by 2 and obtain 7.
        // Step 3) 7 is odd, add 1 and obtain 8.
        // Step 4) 8 is even, divide by 2 and obtain 4.
        // Step 5) 4 is even, divide by 2 and obtain 2.
        // Step 6) 2 is even, divide by 2 and obtain 1.
        let result = num_steps(s);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        // just check big number
        let s = "1101".to_string().repeat(400);
        // Explanation: "1101" corressponds to number 13 in their decimal representation.
        // Step 1) 13 is odd, add 1 and obtain 14.
        // Step 2) 14 is even, divide by 2 and obtain 7.
        // Step 3) 7 is odd, add 1 and obtain 8.
        // Step 4) 8 is even, divide by 2 and obtain 4.
        // Step 5) 4 is even, divide by 2 and obtain 2.
        // Step 6) 2 is even, divide by 2 and obtain 1.
        let _result = num_steps(s);
        assert!(true);
    }

    #[test]
    fn it_works2() {
        // just check big number
        let s = "11001".to_string();
        let output = 8;
        let result = num_steps(s);
        assert_eq!(result, output);
    }
}
