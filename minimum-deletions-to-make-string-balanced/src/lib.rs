pub fn minimum_deletions(s: String) -> i32 {
    // there is ofcourse a dynamic programming way of doing this
    // but there also might be a greedy way too, let's think of that
    // the final output should be: aaaaabbbbb
    // let's do dynamic programming, I can't think of a greedy way
    //                "aababbab"
    //    "a"
    //    "aa"
    //    "aab"            take  "aa"  leave
    //    "aaba"           "aaa"     "aa"

    // backtracking got tle as it is I think 2^n
    // how to convert it to dynamic programming, idk yet

    // in the topics section it's written there stack and dp
    // a stack solution makes sense and it's far easier but will stick to dp
    // just will do stack and maybe after reviewing lis will get back to this
    // backtrack(s.as_bytes(), 0, Vec::new(), 0)
    stack(s.as_bytes())
}

fn stack(s: &[u8]) -> i32 {
    let mut count = 0;
    let mut stack = vec![];
    for &letter in s {
        if letter == b'b' {
            stack.push(letter);
        } else {
            // it means it has processed a's and b's then a's again
            if !stack.is_empty() {
                stack.pop();
                count += 1;
            }
        }
    }
    count
}

fn backtrack(s: &[u8], idx: usize, mut cur_s: Vec<u8>, score: i32) -> i32 {
    if idx == s.len() {
        if valid(&cur_s) {
            dbg!(score);
            dbg!(cur_s);
            return score;
        } else {
            return i32::MAX;
        }
    }

    // take
    cur_s.push(s[idx]);
    let take = backtrack(s, idx + 1, cur_s.clone(), score);
    // if valid(&cur_s) {
    //     take += 1;
    // }

    cur_s.pop();

    let leave = backtrack(s, idx + 1, cur_s.clone(), score + 1);
    // if valid(&cur_s) {
    //     leave += 1;
    // }

    take.min(leave)
}

fn valid(cur_s: &[u8]) -> bool {
    let mut a_first = true;
    for letter in cur_s {
        if letter == &b'b' {
            a_first = false;
        }

        if letter == &b'a' && a_first == false {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "aababbab".to_string();
        let output = 2;
        let result = minimum_deletions(s);
        assert_eq!(result, output);
    }
}
