use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

fn main() {
    println!("Hello, world!");
    let order = "cba".to_string();
    let s = "abcd".to_string();
    let order = "kqep".to_string();
    let s = "pekeq".to_string();

    // Output:  "cbad"
    println!("{}", custom_sort_string(order, s));
}

pub fn custom_sort_string(order: String, s: String) -> String {
    let mut hm = HashMap::new();
    for (i, c) in order.char_indices() {
        hm.insert(c, i);
    }
    println!("{:?}", hm);
    let mut bh = BinaryHeap::new();
    for c in s.chars() {
        if let Some(n) = hm.get(&c) {
            bh.push((Reverse(*n as i32), c));
        } else {
            bh.push((Reverse(-1), c));
        }
    }
    println!("{:?}", bh);
    let mut f: Vec<_> = bh.iter().collect();
    println!("f: {:?}", f);
    f.sort_by(|&x, &y| y.0.cmp(&x.0));
    // f.sort();
    println!("{:?}", f);
    let rezult: String = f.iter().map(|&x| x.1).collect();

    rezult
}
