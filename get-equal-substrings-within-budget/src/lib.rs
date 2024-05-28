pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
    let n = s.len();
    let s = s.into_bytes();
    let t = t.into_bytes();

    let mut changes = Vec::with_capacity(n);
    for i in 0..n {
        let diff = (s[i] as i32 - t[i] as i32).abs();
        changes.push(diff);
    }
    // println!("changes={:?}", changes);
    let mut prefix_changes = vec![0; n + 1];
    for i in 1..=n {
        prefix_changes[i] = prefix_changes[i - 1] + changes[i - 1];
    }
    // println!("prefix_changes={:?}", prefix_changes);
    // changes=[15, 8, 6, 12, 4]
    // prefix_changes=[0, 15, 23, 29, 41, 45]

    // changes=[3, 6, 15, 11, 3, 9, 8, 15, 22, 10, 3, 7, 3, 4, 0]
    // prefix_changes=[0, 3, 9, 24, 35, 38, 47, 55, 70, 92, 102, 105, 112, 115, 119, 119]
    // 14

    let mut max_valid_len = 0;
    let mut st = 0;
    let mut end = 1;
    while end <= n {
        if prefix_changes[end] - prefix_changes[st] <= max_cost || end == st {
            max_valid_len = max_valid_len.max(end - st);
            end += 1;
        } else {
            st += 1;
        }
    }
    max_valid_len as _
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
        Example 1:

    Input: s = "abcd", t = "bcdf", maxCost = 3
    Output: 3
    Explanation: "abc" of s can change to "bcd".
    That costs 3, so the maximum length is 3.
    Example 2:

    Input: s = "abcd", t = "cdef", maxCost = 3
    Output: 1
    Explanation: Each character in s costs 2 to change to character in t,  so the maximum length is 1.
    Example 3:

    Input: s = "abcd", t = "acde", maxCost = 0
    Output: 1
    Explanation: You cannot make any change, so the maximum length is 1.
        */

    #[test]
    fn it_works0() {
        let s = "abcd".to_string();
        let t = "bcdf".to_string();
        let max_cost = 3;
        let output = 3;
        // Explanation: "abc" of s can change to "bcd".
        // That costs 3, so the maximum length is 3.
        let result = equal_substring(s, t, max_cost);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works1() {
        let s = "abcd".to_string();
        let t = "cdef".to_string();
        let max_cost = 3;
        let output = 1;
        let result = equal_substring(s, t, max_cost);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works2() {
        let s = "abcd".to_string();
        let t = "acde".to_string();
        let max_cost = 0;
        let output = 1;
        let result = equal_substring(s, t, max_cost);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works3() {
        let s = "krrgw".to_string();
        let t = "zjxss".to_string();
        let max_cost = 19;
        let output = 2;
        let result = equal_substring(s, t, max_cost);
        assert_eq!(result, output);
    }

    #[test]
    fn it_works4() {
        let s = "krpgjbjjznpzdfy".to_string();
        let t = "nxargkbydxmsgby".to_string();
        let max_cost = 14;
        let output = 4;
        let result = equal_substring(s, t, max_cost);
        assert_eq!(result, output);
    }
}
