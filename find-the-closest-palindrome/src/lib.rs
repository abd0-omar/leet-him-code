pub fn nearest_palindromic(n: String) -> String {
    let n_num = n.parse::<i64>().unwrap();

    let mut lower = n_num - 1;
    let mut higher = n_num + 1;

    loop {
        if is_palindrome(lower.to_string().as_str()) {
            return lower.to_string();
        }

        if is_palindrome(higher.to_string().as_str()) {
            return higher.to_string();
        }

        lower -= 1;
        higher += 1;
    }
}

fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
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
        let n = "123".to_string();
        let output = "121".to_string();
        let result = nearest_palindromic(n);
        assert_eq!(result, output);
    }
}
