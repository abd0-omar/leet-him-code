pub fn rotate_string(s: String, goal: String) -> bool {
    let s = s.as_bytes();
    let goal = goal.as_bytes();

    let n = s.len();
    if n != goal.len() {
        return false;
    }

    'outer: for j in 0..n {
        if s[0] == goal[j] {
            let mut k = (j + 1) % n;
            for i in 1..n {
                if s[i] != goal[k] {
                    continue 'outer;
                }
                k = (k + 1) % n;
            }
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let s = "abcde".to_string();
        let goal = "cdeab".to_string();
        let output = true;
        let result = rotate_string(s, goal);
        assert_eq!(result, output);
    }
}
