use std::collections::HashMap;

fn main() {
    let s1 = String::from("abc");
    let s2 = String::from("baxyzabc");
    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();

    let mut hs = HashMap::new();
    for &c in s1 {
        *hs.entry(c).or_insert(0) += 1;
    }

    let mut st = 0;
    let mut end = 0;
    // if s1.len == end - st - 1 -> tru
}
