pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
    /*
    cdbcbbaaabab
    // remove all y (ba) first because y is larger than x
    cdbcbaabab      5
    cdbcabab        10
    cdbcab          15
    cdbc            19

    example 2
    aabbaaxybbaabb
    // remove all x (ab) first
    abaaxybbaabb     5
    aaxybbaabb       10
    aaxybbab        15
    aaxybb          20
    aaxybb          20

    using stack
    if the top is b and the curr_val in s is a
    pop b and increase the total
    cdbcab
    // then go again in the string and remove all ab with the same technique
    cdbc


    test case
    "paaaabdbabfbybbbtaaab"
    x =
    8132
    y =
    1776
    "paaadbabfbybbbtaaab"   x
    "paaadbfbybbbtaaab"   x
    "paaadbfbybbbtaa"   x
    */

    let mut total = 0;
    let mut stack = vec![];

    if y >= x {
        y_bigger(s.as_bytes(), &mut total, y, &mut stack);
        let new_s = std::str::from_utf8(&stack).unwrap();
        dbg!(new_s);
        dbg!(total);
        x_bigger(new_s.as_bytes(), &mut total, x);
        dbg!(total);
    } else {
        x_bigger2(s.as_bytes(), &mut total, x, &mut stack);
        let new_s = std::str::from_utf8(&stack).unwrap();
        dbg!(new_s);
        y_bigger2(new_s.as_bytes(), &mut total, y);
    }

    total
}

fn x_bigger(s: &[u8], total: &mut i32, x: i32) {
    let mut stack = vec![];
    for &letter in s.iter() {
        if let Some(&top) = stack.last() {
            if letter == b'b' && top == b'a' {
                stack.pop();
                *total += x;
            } else {
                stack.push(letter);
            }
        } else {
            stack.push(letter);
        }
    }
    dbg!(stack);
}

fn y_bigger2(s: &[u8], total: &mut i32, y: i32) {
    let mut stack = vec![];
    for &letter in s.iter() {
        if let Some(&top) = stack.last() {
            if letter == b'a' && top == b'b' {
                stack.pop();
                *total += y;
            } else {
                stack.push(letter);
            }
        } else {
            stack.push(letter);
        }
    }
    dbg!(stack);
}

fn y_bigger(s: &[u8], total: &mut i32, y: i32, stack: &mut Vec<u8>) {
    for &letter in s.iter() {
        if let Some(&top) = stack.last() {
            if letter == b'a' && top == b'b' {
                stack.pop();
                *total += y;
            } else {
                stack.push(letter);
            }
        } else {
            stack.push(letter);
        }
    }
}

fn x_bigger2(s: &[u8], total: &mut i32, x: i32, stack: &mut Vec<u8>) {
    for &letter in s.iter() {
        if let Some(&top) = stack.last() {
            if letter == b'b' && top == b'a' {
                stack.pop();
                *total += x;
            } else {
                stack.push(letter);
            }
        } else {
            stack.push(letter);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works0() {
        let s = "cdbcbbaaabab".to_string();
        let x = 4;
        let y = 5;
        let output = 19;
        let result = maximum_gain(s, x, y);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "aabbaaxybbaabb".to_string();
        let x = 5;
        let y = 4;
        let output = 20;
        let result = maximum_gain(s, x, y);
        assert_eq!(result, output);
    }
    #[test]
    fn it_works2() {
        let s = "paaaabdbabfbybbbtaaab".to_string();
        let x = 8132;
        let y = 1776;
        let output = 24396;
        let result = maximum_gain(s, x, y);
        assert_eq!(result, output);
    }
}
