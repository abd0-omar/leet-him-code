fn main() {
    let s = "0100".to_string();
    println!("{}", min_operations(s));
    let s = "10".to_string();
    println!("{}", min_operations(s));
    let s = "0000".to_string();
    println!("{}", min_operations(s));
    let s = "110010".to_string();
    println!("{}", min_operations(s));
}

pub fn min_operations(s: String) -> i32 {
    let s = s.as_bytes();
    let n = s.len();
    if n < 4 {
        return 0;
    }
    let mut prev = s[0] == b'1';
    let mut curr = s[1] == b'1';
    let mut next = s[2] == b'1';
    let mut after = s[3] == b'1';

    for i in 0..=(n - 4) {
        // or end of string
        dbg!(i + 3 == n - 1);
        if (!prev && curr && !next && i + 2 == n - 1) || (prev && !curr && next && i + 2 == n - 1) {
            return 0;
        }
        if (prev && !curr && next && after) || (!prev && curr && !next && !after) {
            return 1;
        }
        if (prev && curr && next && after) || (!prev && !curr && !next && !after) {
            return 2;
        }
        prev = s[i] == b'1';
        curr = s[i + 1] == b'1';
        next = s[i + 2] == b'1';
        after = s[i + 3] == b'1';
    }
    -1
}
