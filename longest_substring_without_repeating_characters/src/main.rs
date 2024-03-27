fn main() {
    let s = "abcabcbb".to_string();

    let mut a = [0; 128];
    let mut curr_size = 0;
    let mut max = 0;
    let mut st = 0;
    for (end, end_char) in s.chars().enumerate() {
        a[end_char as usize] += 1;
        while a[end_char as usize] > 1 {
            a[s.chars().nth(st).unwrap() as usize] -= 1;
            st += 1;
        }
        curr_size = end - st + 1;
        max = max.max(curr_size);
    }
    println!("{}", max);
}
