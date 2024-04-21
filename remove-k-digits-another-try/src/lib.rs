pub fn remove_kdigits(num: String, k: i32) -> String {
    let mut mono_stack = Vec::with_capacity(num.len());

    let mut count = 0;
    for c in num.chars() {
        while !mono_stack.is_empty() && mono_stack.last().unwrap() > &c && count < k {
            mono_stack.pop();
            count += 1
        }

        if mono_stack.is_empty() && c == '0' {
            continue;
        }

        mono_stack.push(c);
    }

    while count < k {
        mono_stack.pop();
        count += 1;
    }

    if mono_stack.is_empty() {
        "0".to_string()
    } else {
        mono_stack.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works1() {
        let result = remove_kdigits(String::from("1432219"), 3);
        assert_eq!(result, String::from("1219"));
    }

    #[test]
    fn it_works2() {
        let result = remove_kdigits(String::from("10200"), 1);
        assert_eq!(result, String::from("200"));
    }

    #[test]
    fn it_works3() {
        let result = remove_kdigits(String::from("10"), 2);
        assert_eq!(result, String::from("0"));
    }

    #[test]
    fn it_works4() {
        let result = remove_kdigits(String::from("9"), 1);
        assert_eq!(result, String::from("0"));
    }

    #[test]
    fn it_works5() {
        let result = remove_kdigits(String::from("112"), 1);
        assert_eq!(result, String::from("11"));
    }
}
