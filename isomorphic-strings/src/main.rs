fn main() {
    println!("Hello, world!");
}

pub fn is_isomorphic(s: String, t: String) -> bool {
    let s = s.into_bytes();
    let t = t.into_bytes();
    let mut hm1 = std::collections::HashMap::new();
    let mut hm2 = std::collections::HashMap::new();
    for i in 0..s.len() {
        let a = hm1.insert(s[i], i);
        let b = hm2.insert(t[i], i);
        if a != b {
            return false;
        }
    }
    true
}
