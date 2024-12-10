pub fn maximum_length(s: String) -> i32 {
    // binary search on the result, which is the len of one substring
    // T T T F F F
    // we want the last T
    // aaaa
    // aa aa aa

    // the `possible()` fn will brute force to check if the candidate len
    // would work

    let s = s.as_bytes();
    let mut l = 1;
    let mut r = s.len();
    let mut ans = -1;

    while l <= r {
        let mid = l + (r - l) / 2;
        if possible(&s, mid) {
            l = mid + 1;
            ans = mid as i32;
        } else {
            if let (_, true) = mid.overflowing_sub(1) {
                break;
            }
            r = mid - 1;
        }
    }
    ans
}

fn possible(s: &[u8], candidate_len: usize) -> bool {
    // count substrings that have same letters and same candidate len
    let mut hm = std::collections::HashMap::new();
    'outer: for idx in 0..s.len() - candidate_len + 1 {
        // input is so small that I can get away with doing this
        // instead of one loop
        let slice = &s[idx..candidate_len + idx];
        for j in 1..slice.len() {
            if slice[j] != slice[j - 1] {
                continue 'outer;
            }
        }
        *hm.entry(slice).or_insert(0) += 1;
    }
    // dbg!(&hm);
    for &val in hm.values() {
        if val >= 3 {
            return true;
        }
    }
    false
}
