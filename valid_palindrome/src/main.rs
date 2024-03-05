fn main() {
    let s = String::from("0p");
    let ans = is_palindrome(s);
    println!("{:?}", ans);
}

fn is_palindrome(s: String) -> bool {
    let sz: String = s.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    let sz_lower = sz.to_lowercase();

    if sz_lower.len() == 2 && sz_lower.chars().nth(0) == sz_lower.chars().nth(1) {
        return true;
    }

    let len = sz_lower.len();
    let mut start = 0;
    let mut end = len - 1;

    while start < end {
        let start_char = sz_lower.chars().nth(start).unwrap();
        let end_char = sz_lower.chars().nth(end).unwrap();

        if start_char != end_char {
            return false;
        }

        start += 1;
        end -= 1;
    }

    true
}
