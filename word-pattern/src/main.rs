fn main() {
    println!("Hello, world!");
    let pattern = "abba".to_string();
    let s = "dog cat cat dog".to_string();
    // Output: true
    println!("{}", word_pattern(pattern, s));
}

pub fn word_pattern(pattern: String, s: String) -> bool {
    let mut hm: std::collections::HashMap<char, &str> = std::collections::HashMap::new();
    let mut s_splitted = s.split_whitespace();
    let f = s_splitted.clone().count();
    if f != pattern.len() {
        return false;
    }
    for char_pattern in pattern.chars() {
        let char_s_splitted = s_splitted.next().unwrap_or("asdfa");
        if let Some(val) = hm.get(&char_pattern) {
            println!("val={:?}", val);
            if val != &char_s_splitted {
                println!("char_s_splitted={:?}", char_s_splitted);
                return false;
            }
        } else {
            for &val in hm.values() {
                println!("char_pattern={:?}", char_pattern);
                println!("val={:?}", val);
                if val.to_string() == char_s_splitted {
                    return false;
                }
            }
            hm.insert(char_pattern, char_s_splitted);
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_return_true() {
        let pattern = "abba".to_string();
        let s = "dog cat cat dog".to_string();
        assert!(word_pattern(pattern, s))
    }

    #[test]
    fn should_return_false() {
        let pattern = "abba".to_string();
        let s = "dog cat cat fish".to_string();
        assert!(!word_pattern(pattern, s))
    }

    #[test]
    fn not_same_length() {
        let pattern = "abbaz".to_string();
        let s = "dog cat cat dog".to_string();
        assert!(!word_pattern(pattern, s))
    }

    #[test]
    fn not_same_length2() {
        let pattern = "abba".to_string();
        let s = "dog cat cat dog cat".to_string();
        assert!(!word_pattern(pattern, s))
    }

    #[test]
    fn dog_dog() {
        let pattern = "abba".to_string();
        let s = "dog dog dog dog".to_string();
        assert!(!word_pattern(pattern, s))
    }
}
