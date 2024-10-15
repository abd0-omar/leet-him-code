pub fn minimum_steps(s: String) -> i64 {
    // black "1" right
    // white "0" left
    //
    // 000111 -> 0
    // 01100111
    // 00011111 -> 2
    // count the zeros (or ones if you want) that are not on the most left
    // or wait no that's not right
    //
    // 0010101
    // zeros -> 2
    // ones -> 2
    // but the answer is 1
    // maybe it's a simulation
    // get the most distant zero and replace it with 1
    // 0000111 -> 1
    // maybe sort and see the zeros diff and ones diff
    // yeah the answer is either one of them
    //
    // 101
    // 011
    //
    // 100
    // 001
    //
    // oh adjacent balls not any ball
    let mut zero_diffs = 0;
    let mut zeros_so_far = 0;
    for end in s.chars().rev() {
        if end == '0' {
            zeros_so_far += 1;
        } else {
            zero_diffs += zeros_so_far;
        }
    }

    zero_diffs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "101".to_string();
        let output = 1;
        let result = minimum_steps(s);
        assert_eq!(result, output);
    }
}
