use std::cmp;
use std::collections::*;
//dvdf
//dvdd
fn main() {
    let s = String::from("abcabcbb");

    let mut btm = BTreeMap::new();
    let mut l = 0;
    let mut max = 0;
    for (i, c) in s.chars().enumerate() {
        while btm.contains_key(&c) {
            btm.remove(&s.chars().nth(l).unwrap());
            l += 1;
        }
        btm.insert(c, i);
        max = cmp::max(max, i - l + 1);
        println!("{max}, {}", i - l + 1);
    }
}
