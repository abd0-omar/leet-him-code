use std::str::from_utf8;

fn main() {
    println!("Hello, world!");
}

pub fn count_palindromic_subsequence(s: String) -> i32 {
    let n = s.len();
    let s = s.as_bytes();
    let mut hs = std::collections::HashSet::new();
    for i in 0..n - 2 {
        for k in i + 2..n {
            if is_palindrome(s, i, k) {
                // let palindrome_s = format!("{}{}{}", s[i] as char, s[j] as char, s[k] as char);
                // println!("palindrome_s={:?}", (palindrome_s));
                for j in i + 1..k {
                    let new_char = j;
                    let palindrome_s = format!("{}{}{}", s[i] as char, s[j] as char, s[k] as char);
                    println!("palindrome_s={:?}", (palindrome_s));
                    hs.insert(palindrome_s);
                }
            }
        }
    }
    println!("hs={:?}", hs);
    hs.len() as _
}

fn is_palindrome_j(s: &[u8], i: usize, j: usize, k: usize) -> bool {
    if s[i] == s[k] {
        println!("i={:?}", i);
        println!("k={:?}", k);
        return true;
    }
    false
}

fn is_palindrome(s: &[u8], i: usize, k: usize) -> bool {
    if s[i] == s[k] {
        println!("s={:?}", s);
        println!("i={:?}", i);
        println!("k={:?}", k);
        return true;
    }
    false
}
