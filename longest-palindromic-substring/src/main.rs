fn main() {
    println!("Hello, world!");
    let s = "babad".to_string();
    // Output: "bab"
    println!("{}", longest_palindrome(s));
}

pub fn longest_palindrome(s: String) -> String {
    let mut max = 0;
    let mut rez = String::new();
    let x = 5;
    for i in 0..s.len() {
        for j in i + 1..s.len() + 1 {
            let f = &s[i..j];
            // println!("f={:?}", f);
            // println!("{}", is_palindrome(f));
            if is_palindrome(f) {
                if max < f.len() {
                    max = f.len();
                    rez = f.to_string();
                    // println!("rez={:?}", rez);
                }
            }
        }
    }
    rez
}

pub fn is_palindrome(s: &str) -> bool {
    for i in 0..s.len() / 2 {
        if s.get(i..i + 1).unwrap() != s.get((s.len() - i - 1)..(s.len() - i)).unwrap() {
            return false;
        }
    }
    true
}
