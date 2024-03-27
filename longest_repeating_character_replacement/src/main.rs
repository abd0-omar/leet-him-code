use std::collections::HashMap;

fn main() {
    let s = String::from("AABABBA");
    let s: &[u8] = s.as_bytes();
    let k: i32 = 2;

    let mut hs = HashMap::new();
    let mut max_length = 0;
    let mut start = 0;

    for end in 0..s.len() {
        let count = hs.entry(s[end]).or_insert(0);
        *count += 1;

        let max_max = *hs.values().max().unwrap();

        if (end - start + 1) - max_max > k as usize {
            let left_char = s[start];
            let left_count = hs.get_mut(&left_char).unwrap();
            *left_count -= 1;
            start += 1;
        }

        max_length = max_length.max(end - start + 1);
    }

    println!("{}", max_length);
}
